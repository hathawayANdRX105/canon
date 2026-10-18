//! Catalog: workspace packages, their placement specs, and the checks that
//! map changed files onto those specs.
//!
//! The catalog is the single source of truth for both the canon gate
//! (`crate-placement` kind) and the member-side `xtask-crate` report: the
//! gate converts [`Diagnostic`]s into findings, the report renders them
//! together with the spec text.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use check_core::finding::Severity;
use check_deps::metadata;
use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use serde::Serialize;

use crate::spec::{self, PlacementSpec, SpecError};

/// How the catalog discovers specs and which workspace packages it owns.
#[derive(Debug, Clone)]
pub struct LoadConfig {
    /// Spec file name relative to each package directory.
    pub spec_file: String,
    /// Package directory globs that own a spec.
    pub members_include: Vec<String>,
    /// Package directory globs skipped by the catalog.
    pub members_exclude: Vec<String>,
}

impl Default for LoadConfig {
    fn default() -> Self {
        Self {
            spec_file: "placement.toml".to_string(),
            members_include: vec!["crates/**".to_string(), "bin/**".to_string()],
            members_exclude: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum LoadError {
    Config(String),
    Metadata(String),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Config(message) => write!(f, "invalid catalog config: {message}"),
            Self::Metadata(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for LoadError {}

/// One validated spec, with compiled matchers ready for the hot path.
#[derive(Debug)]
pub struct LoadedSpec {
    pub package: String,
    pub dir: PathBuf,
    pub spec_path: PathBuf,
    pub spec: PlacementSpec,
    pub allow: GlobSet,
    pub deny: GlobSet,
    pub files_scope: GlobSet,
    pub files: Vec<CompiledFileRule>,
    pub cjk_line: Option<u32>,
}

#[derive(Debug)]
pub struct CompiledFileRule {
    pub pattern: String,
    pub glob: GlobSet,
    pub responsibility: String,
    pub entry: bool,
}

impl LoadedSpec {
    /// `None` when the path is outside the file-responsibility scope.
    /// `Some(None)` when it is in scope but no `[[files]]` entry matches.
    pub fn file_rule_in_scope(&self, relative: &Path) -> Option<Option<&CompiledFileRule>> {
        if !self.files_scope.is_match(relative) {
            return None;
        }
        Some(self.files.iter().find(|rule| rule.glob.is_match(relative)))
    }
}

/// A load-time problem attached to one package. Problems are reported once.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecProblem {
    pub id: String,
    pub package: String,
    pub path: PathBuf,
    pub line: u32,
    pub message: String,
}

impl SpecProblem {
    fn severity(&self) -> Severity {
        match self.id.as_str() {
            "CRATE-SPEC-EMPTY" | "CRATE-SPEC-CJK" | "CRATE-FILE-ENTRY-GLOB" => Severity::Warn,
            _ => Severity::Fail,
        }
    }
}

/// A placement finding. The gate converts it into a canon finding; the
/// `xtask-crate` report prints it as-is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Diagnostic {
    pub id: String,
    pub severity: Severity,
    pub path: String,
    pub line: u32,
    pub message: String,
}

#[derive(Debug, Clone, Copy)]
pub struct DiagnoseOptions {
    /// Report packages that are missing a spec (and were touched).
    pub require_spec: bool,
    /// Report every package, not only the touched ones.
    pub all: bool,
    /// Allow CJK characters in specs.
    pub allow_cjk: bool,
}

/// Everything the report needs to show one changed file.
#[derive(Debug, Clone)]
pub struct Assignment {
    pub package: String,
    pub crate_dir: PathBuf,
    pub spec_path: Option<PathBuf>,
    pub relative: PathBuf,
    pub file_in_scope: bool,
    pub file_pattern: Option<String>,
    pub responsibility: Option<String>,
    pub entry: bool,
    pub path_allowed: bool,
    pub path_denied: bool,
}

pub struct Catalog {
    root: PathBuf,
    config: LoadConfig,
    package_dirs: BTreeMap<String, PathBuf>,
    /// Package directories sorted deepest-first for longest-prefix matching.
    packages_by_dir: Vec<(String, PathBuf)>,
    specs: BTreeMap<String, LoadedSpec>,
    problems: Vec<SpecProblem>,
    include: GlobSet,
    exclude: GlobSet,
}

impl Catalog {
    pub fn load(root: impl Into<PathBuf>, config: &LoadConfig) -> Result<Self, LoadError> {
        let root = root.into();
        let include =
            compile_globs(&config.members_include).map_err(|message| LoadError::Config(message))?;
        let exclude =
            compile_globs(&config.members_exclude).map_err(|message| LoadError::Config(message))?;

        let graph = metadata::load(&root).ok_or_else(|| {
            LoadError::Metadata(
                "cargo metadata --no-deps is unavailable; the placement catalog cannot map files to packages"
                    .to_string(),
            )
        })?;

        let mut package_dirs = BTreeMap::new();
        for (package, dir) in &graph.package_dirs {
            package_dirs.insert(package.clone(), PathBuf::from(dir));
        }

        let mut packages_by_dir: Vec<(String, PathBuf)> = package_dirs
            .iter()
            .map(|(package, dir)| (package.clone(), dir.clone()))
            .collect();
        packages_by_dir.sort_by(|a, b| {
            b.1.as_os_str()
                .len()
                .cmp(&a.1.as_os_str().len())
                .then_with(|| a.0.cmp(&b.0))
        });

        let mut catalog = Self {
            root,
            config: config.clone(),
            package_dirs,
            packages_by_dir,
            specs: BTreeMap::new(),
            problems: Vec::new(),
            include,
            exclude,
        };
        catalog.load_specs();
        Ok(catalog)
    }

    fn load_specs(&mut self) {
        let packages: Vec<(String, PathBuf)> = self
            .package_dirs
            .iter()
            .map(|(package, dir)| (package.clone(), dir.clone()))
            .collect();
        for (package, dir) in packages {
            if !self.includes(&dir) {
                continue;
            }
            let spec_path = dir.join(&self.config.spec_file);
            match spec::load(&self.root, &spec_path) {
                Ok(parsed) => self.validate_spec(package, dir, spec_path, parsed),
                Err(error) => {
                    let (id, line) = match &error {
                        SpecError::Missing { .. } => ("CRATE-SPEC-MISSING", 0),
                        SpecError::Io { .. } => ("CRATE-SPEC-IO", 0),
                        SpecError::Parse { line, .. } => ("CRATE-SPEC-PARSE", *line),
                        SpecError::Schema { .. } => ("CRATE-SPEC-SCHEMA", 0),
                        SpecError::CrateEmpty { .. } => ("CRATE-SPEC-CRATE", 0),
                    };
                    self.problems.push(SpecProblem {
                        id: id.to_string(),
                        package,
                        path: spec_path,
                        line,
                        message: error.to_string(),
                    });
                }
            }
        }
    }

    fn validate_spec(
        &mut self,
        package: String,
        dir: PathBuf,
        spec_path: PathBuf,
        parsed: spec::ParsedSpec,
    ) {
        let spec = parsed.spec;

        if spec.crate_name != package {
            self.problems.push(SpecProblem {
                id: "CRATE-SPEC-CRATE".to_string(),
                package: package.clone(),
                path: spec_path.clone(),
                line: 0,
                message: format!(
                    "spec crate name {:?} does not match package {:?}",
                    spec.crate_name, package
                ),
            });
        }
        if spec.responsibility.trim().is_empty() {
            self.problems.push(SpecProblem {
                id: "CRATE-SPEC-EMPTY".to_string(),
                package: package.clone(),
                path: spec_path.clone(),
                line: 0,
                message: "responsibility is empty".to_string(),
            });
        }

        let allow =
            self.compile_or_problem(&package, &spec_path, &spec.paths.allow, "CRATE-SPEC-GLOB");
        let deny =
            self.compile_or_problem(&package, &spec_path, &spec.paths.deny, "CRATE-SPEC-GLOB");
        let files_scope = self.compile_or_problem(
            &package,
            &spec_path,
            &spec.effective_files_scope(),
            "CRATE-SPEC-GLOB",
        );

        let mut files = Vec::new();
        let mut entry_count = 0usize;
        for rule in &spec.files {
            if rule.responsibility.trim().is_empty() {
                self.problems.push(SpecProblem {
                    id: "CRATE-FILE-EMPTY".to_string(),
                    package: package.clone(),
                    path: spec_path.clone(),
                    line: 0,
                    message: format!("[[files]] {:?} has an empty responsibility", rule.path),
                });
            }
            let glob = match compile_glob(&rule.path) {
                Ok(glob) => glob,
                Err(message) => {
                    self.problems.push(SpecProblem {
                        id: "CRATE-SPEC-GLOB".to_string(),
                        package: package.clone(),
                        path: spec_path.clone(),
                        line: 0,
                        message: format!("[[files]] {:?}: {message}", rule.path),
                    });
                    GlobSet::empty()
                }
            };
            if rule.entry {
                entry_count += 1;
                if rule
                    .path
                    .chars()
                    .any(|ch| matches!(ch, '*' | '?' | '[' | '{'))
                {
                    self.problems.push(SpecProblem {
                        id: "CRATE-FILE-ENTRY-GLOB".to_string(),
                        package: package.clone(),
                        path: spec_path.clone(),
                        line: 0,
                        message: format!(
                            "[[files]] {:?} is an entry but uses a glob; entry must be an exact path",
                            rule.path
                        ),
                    });
                } else if !self.root.join(&dir).join(&rule.path).is_file() {
                    self.problems.push(SpecProblem {
                        id: "CRATE-FILE-ENTRY-MISSING".to_string(),
                        package: package.clone(),
                        path: spec_path.clone(),
                        line: 0,
                        message: format!("entry file {:?} does not exist", rule.path),
                    });
                }
            }
            files.push(CompiledFileRule {
                pattern: rule.path.clone(),
                glob,
                responsibility: rule.responsibility.clone(),
                entry: rule.entry,
            });
        }
        if entry_count > 1 {
            self.problems.push(SpecProblem {
                id: "CRATE-FILE-ENTRY-MULTI".to_string(),
                package: package.clone(),
                path: spec_path.clone(),
                line: 0,
                message: format!("{entry_count} [[files]] entries are marked as entry; keep one"),
            });
        }

        for redirect in &spec.does_not_belong {
            if self.redirect_target_exists(&redirect.to) {
                continue;
            }
            self.problems.push(SpecProblem {
                id: "CRATE-SPEC-REDIRECT".to_string(),
                package: package.clone(),
                path: spec_path.clone(),
                line: 0,
                message: format!(
                    "does_not_belong {:?} points to {:?}, which is neither a workspace package, a package glob, nor an existing path",
                    redirect.concern, redirect.to
                ),
            });
        }

        self.specs.insert(
            package.clone(),
            LoadedSpec {
                package,
                dir,
                spec_path,
                spec,
                allow,
                deny,
                files_scope,
                files,
                cjk_line: parsed.cjk_line,
            },
        );
    }

    fn compile_or_problem(
        &mut self,
        package: &str,
        spec_path: &Path,
        patterns: &[String],
        id: &str,
    ) -> GlobSet {
        match compile_globs(patterns) {
            Ok(glob) => glob,
            Err(message) => {
                self.problems.push(SpecProblem {
                    id: id.to_string(),
                    package: package.to_string(),
                    path: spec_path.to_path_buf(),
                    line: 0,
                    message,
                });
                GlobSet::empty()
            }
        }
    }

    fn includes(&self, dir: &Path) -> bool {
        dir_matches(&self.include, dir) && !dir_matches(&self.exclude, dir)
    }

    /// A redirect target resolves when it is an exact package name, an existing
    /// path, or a glob that matches at least one package name or package dir.
    fn redirect_target_exists(&self, target: &str) -> bool {
        if self.package_dirs.contains_key(target) || self.root.join(target).exists() {
            return true;
        }
        if !target.chars().any(|ch| matches!(ch, '*' | '?' | '[' | '{')) {
            return false;
        }
        let Ok(glob) = GlobBuilder::new(target).literal_separator(true).build() else {
            return false;
        };
        let matcher = glob.compile_matcher();
        self.package_dirs.iter().any(|(package, dir)| {
            matcher.is_match(package.as_str())
                || matcher.is_match(dir)
                || matcher.is_match(dir.join("Cargo.toml"))
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn config(&self) -> &LoadConfig {
        &self.config
    }

    pub fn specs(&self) -> &BTreeMap<String, LoadedSpec> {
        &self.specs
    }

    pub fn loaded(&self, package: &str) -> Option<&LoadedSpec> {
        self.specs.get(package)
    }

    pub fn problems(&self) -> &[SpecProblem] {
        &self.problems
    }

    pub fn package_dirs(&self) -> &BTreeMap<String, PathBuf> {
        &self.package_dirs
    }

    /// Longest-prefix package match for a repo-relative path.
    pub fn assign_package(&self, rel: &Path) -> Option<&str> {
        self.packages_by_dir
            .iter()
            .find_map(|(package, dir)| rel.starts_with(dir).then_some(package.as_str()))
    }

    pub fn assignment(&self, rel: &Path) -> Option<Assignment> {
        let package = self.assign_package(rel)?.to_string();
        let dir = self.package_dirs.get(&package)?.clone();
        let relative = rel.strip_prefix(&dir).ok()?.to_path_buf();
        let loaded = self.specs.get(&package);
        let path_denied = loaded.is_some_and(|spec| spec.deny.is_match(&relative));
        let path_allowed =
            loaded.is_none_or(|spec| spec.allow.is_empty() || spec.allow.is_match(&relative));
        let in_scope = loaded
            .and_then(|spec| spec.file_rule_in_scope(&relative))
            .flatten();
        let file_in_scope = loaded.is_some_and(|spec| spec.files_scope.is_match(&relative));
        Some(Assignment {
            package,
            crate_dir: dir,
            spec_path: loaded.map(|spec| spec.spec_path.clone()),
            relative,
            file_in_scope,
            file_pattern: in_scope.map(|rule| rule.pattern.clone()),
            responsibility: in_scope.map(|rule| rule.responsibility.clone()),
            entry: in_scope.is_some_and(|rule| rule.entry),
            path_allowed,
            path_denied,
        })
    }

    pub fn diagnose(&self, changed: &[PathBuf], options: &DiagnoseOptions) -> Vec<Diagnostic> {
        let touched: BTreeSet<String> = changed
            .iter()
            .filter_map(|path| self.assign_package(path).map(str::to_string))
            .collect();

        let mut diagnostics = Vec::new();

        for problem in &self.problems {
            let in_scope = options.all || touched.contains(&problem.package);
            let include = if problem.id == "CRATE-SPEC-MISSING" {
                options.require_spec && in_scope
            } else {
                in_scope
            };
            if include {
                diagnostics.push(Diagnostic {
                    id: problem.id.clone(),
                    severity: problem.severity(),
                    path: problem.path.display().to_string(),
                    line: problem.line,
                    message: problem.message.clone(),
                });
            }
        }

        if !options.allow_cjk {
            for (package, spec) in &self.specs {
                let Some(line) = spec.cjk_line else { continue };
                if options.all || touched.contains(package) {
                    diagnostics.push(Diagnostic {
                        id: "CRATE-SPEC-CJK".to_string(),
                        severity: Severity::Warn,
                        path: spec.spec_path.display().to_string(),
                        line,
                        message: "placement spec must be English; CJK characters found".to_string(),
                    });
                }
            }
        }

        for rel in changed {
            let Some(package) = self.assign_package(rel) else {
                continue;
            };
            let Some(spec) = self.specs.get(package) else {
                continue;
            };
            let Ok(relative) = rel.strip_prefix(&spec.dir) else {
                continue;
            };
            if spec.deny.is_match(relative) {
                diagnostics.push(Diagnostic {
                    id: "CRATE-PATH-DENY".to_string(),
                    severity: Severity::Fail,
                    path: rel.display().to_string(),
                    line: 0,
                    message: format!(
                        "path is forbidden by {} `paths.deny`",
                        spec.spec_path.display()
                    ),
                });
            } else if !spec.allow.is_empty() && !spec.allow.is_match(relative) {
                diagnostics.push(Diagnostic {
                    id: "CRATE-PATH-OUTSIDE".to_string(),
                    severity: Severity::Warn,
                    path: rel.display().to_string(),
                    line: 0,
                    message: format!("path is outside {} `paths.allow`", spec.spec_path.display()),
                });
            }

            if let Some(rule) = spec.file_rule_in_scope(relative)
                && rule.is_none()
            {
                diagnostics.push(Diagnostic {
                    id: "CRATE-FILE-UNMAPPED".to_string(),
                    severity: Severity::Warn,
                    path: rel.display().to_string(),
                    line: 0,
                    message: format!(
                        "no [[files]] entry in {} matches this file; add a responsibility entry or move the file",
                        spec.spec_path.display()
                    ),
                });
            }
        }

        dedupe_and_sort(diagnostics)
    }
}

fn dedupe_and_sort(mut diagnostics: Vec<Diagnostic>) -> Vec<Diagnostic> {
    let mut seen = BTreeSet::new();
    diagnostics.retain(|diagnostic| {
        seen.insert((
            diagnostic.id.clone(),
            diagnostic.path.clone(),
            diagnostic.line,
            diagnostic.message.clone(),
        ))
    });
    diagnostics.sort_by(|a, b| {
        a.path
            .cmp(&b.path)
            .then_with(|| a.line.cmp(&b.line))
            .then_with(|| a.id.cmp(&b.id))
    });
    diagnostics
}

fn dir_matches(set: &GlobSet, dir: &Path) -> bool {
    set.is_match(dir) || set.is_match(dir.join("Cargo.toml"))
}

fn compile_glob(pattern: &str) -> Result<GlobSet, String> {
    let glob = GlobBuilder::new(pattern)
        .literal_separator(true)
        .build()
        .map_err(|error| format!("bad glob {pattern:?}: {error}"))?;
    let mut builder = GlobSetBuilder::new();
    builder.add(glob);
    builder
        .build()
        .map_err(|error| format!("bad glob {pattern:?}: {error}"))
}

fn compile_globs(patterns: &[String]) -> Result<GlobSet, String> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let glob = GlobBuilder::new(pattern)
            .literal_separator(true)
            .build()
            .map_err(|error| format!("bad glob {pattern:?}: {error}"))?;
        builder.add(glob);
    }
    builder
        .build()
        .map_err(|error| format!("bad globs: {error}"))
}
