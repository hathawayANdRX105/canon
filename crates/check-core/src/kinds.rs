//! 参数化规则种类（D9）：同一 Rust 实现，checks.toml 多实例。
//! TOML 只做参数，不做逻辑——复杂语义写专用 Rust 规则（域 crate），不塞配置。

use std::path::Path;

use serde::Deserialize;

use crate::context::Ctx;
use crate::finding::{Class, Finding, Severity};
use crate::rule::Rule;

/// kind 工厂：把 checks.toml 里的参数表实例化成规则。
/// 参数键由各实现的 serde 类型裁决（deny_unknown_fields），未知键实例化即报错。
pub trait KindFactory: Send + Sync {
    fn kind(&self) -> &'static str;

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String>;
}

/// 内置 kind；其余 kind 随所属域 crate（T1–T8）落地。
pub fn builtin_factories() -> Vec<Box<dyn KindFactory>> {
    vec![
        Box::new(PathForbiddenFactory),
        Box::new(TextPatternFactory),
        Box::new(NamePatternFactory),
    ]
}

// ---------------------------------------------------------------------------
// path-forbidden：命中的跟踪路径存在即违规（全仓不变量，line=0）。

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PathForbiddenParams {
    /// 相对仓库根的 glob；`*` 不跨目录分隔符，`**` 跨。
    paths: Vec<String>,
    message: String,
}

struct PathForbiddenRule {
    id: String,
    patterns: globset::GlobSet,
    message: String,
}

impl Rule for PathForbiddenRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "禁止出现的路径存在"
    }

    fn doc(&self) -> &str {
        "跟踪文件命中 paths glob 即违规；全仓不变量，无视变更 scope。\
         参数见 checks.toml 的 [kinds.path-forbidden.<name>]。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Fail
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        ctx.tracked_files()
            .iter()
            .filter(|rel| self.patterns.is_match(rel.as_path()))
            .map(|rel| {
                Finding::new(
                    self.id.as_str(),
                    Severity::Fail,
                    rel.display().to_string(),
                    0,
                    self.message.clone(),
                )
            })
            .collect()
    }
}

struct PathForbiddenFactory;

impl KindFactory for PathForbiddenFactory {
    fn kind(&self) -> &'static str {
        "path-forbidden"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: PathForbiddenParams =
            PathForbiddenParams::deserialize(toml::Value::Table(params.clone()))
                .map_err(|e| format!("bad params: {e}"))?;
        let mut builder = globset::GlobSet::builder();
        for pattern in &parsed.paths {
            let glob = globset::GlobBuilder::new(pattern)
                .literal_separator(true)
                .build()
                .map_err(|e| format!("bad glob {pattern:?}: {e}"))?;
            builder.add(glob);
        }
        let patterns = builder.build().map_err(|e| format!("bad globs: {e}"))?;
        Ok(Box::new(PathForbiddenRule {
            id: format!("PATH-FORBIDDEN.{name}"),
            patterns,
            message: parsed.message,
        }))
    }
}

// ---------------------------------------------------------------------------
// text-pattern：scope 文件逐行匹配拒绝正则，命中即行级 finding。

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TextPatternParams {
    #[serde(default)]
    paths_include: Vec<String>,
    #[serde(default)]
    paths_exclude: Vec<String>,
    /// 逐行匹配；一行只报一次（首个命中的模式）。
    #[serde(default)]
    patterns_deny: Vec<String>,
    message: String,
}

struct TextPatternRule {
    id: String,
    include: globset::GlobSet,
    exclude: globset::GlobSet,
    patterns: Vec<regex::Regex>,
    message: String,
}

impl Rule for TextPatternRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "文本模式拒绝"
    }

    fn doc(&self) -> &str {
        "scope 文件逐行匹配 patterns_deny，命中即行级 finding。\
         参数见 checks.toml 的 [kinds.text-pattern.<name>]。"
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let mut findings = Vec::new();
        for rel in ctx.scope_files() {
            if !self.applies(rel) {
                continue;
            }
            findings.extend(self.scan(rel, ctx));
        }
        findings
    }
}

impl TextPatternRule {
    fn applies(&self, rel: &Path) -> bool {
        (self.include.is_empty() || self.include.is_match(rel)) && !self.exclude.is_match(rel)
    }

    fn scan(&self, rel: &Path, ctx: &Ctx) -> Vec<Finding> {
        let Ok(text) = ctx.read(rel) else {
            return Vec::new();
        };
        let mut findings = Vec::new();
        for (idx, line) in text.lines().enumerate() {
            if self.patterns.iter().any(|re| re.is_match(line)) {
                findings.push(Finding::new(
                    self.id.as_str(),
                    Severity::Warn,
                    rel.display().to_string(),
                    u32::try_from(idx + 1).unwrap_or(u32::MAX),
                    format!("{}: {}", self.message, line.trim()),
                ));
            }
        }
        findings
    }
}

struct TextPatternFactory;

impl KindFactory for TextPatternFactory {
    fn kind(&self) -> &'static str {
        "text-pattern"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: TextPatternParams =
            TextPatternParams::deserialize(toml::Value::Table(params.clone()))
                .map_err(|e| format!("bad params: {e}"))?;
        let include = compile_globs(&parsed.paths_include)?;
        let exclude = compile_globs(&parsed.paths_exclude)?;
        let mut patterns = Vec::new();
        for source in &parsed.patterns_deny {
            let re = regex::Regex::new(source).map_err(|e| format!("bad regex {source:?}: {e}"))?;
            patterns.push(re);
        }
        Ok(Box::new(TextPatternRule {
            id: format!("TEXT-PATTERN.{name}"),
            include,
            exclude,
            patterns,
            message: parsed.message,
        }))
    }
}

fn compile_globs(patterns: &[String]) -> Result<globset::GlobSet, String> {
    let mut builder = globset::GlobSet::builder();
    for pattern in patterns {
        let glob = globset::GlobBuilder::new(pattern)
            .literal_separator(true)
            .build()
            .map_err(|e| format!("bad glob {pattern:?}: {e}"))?;
        builder.add(glob);
    }
    builder.build().map_err(|e| format!("bad globs: {e}"))
}

// ---------------------------------------------------------------------------
// name-pattern：跟踪文件里，命中 paths_include 的**文件名**必须匹配 pattern
// （全仓不变量，line=0）。用于文件命名规范（snake_case、*_test.rs 等）。

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NamePatternParams {
    /// 参与命名的跟踪文件 glob（相对仓库根）。
    paths_include: Vec<String>,
    #[serde(default)]
    paths_exclude: Vec<String>,
    /// 文件名（不含目录）必须匹配的正则。
    pattern: String,
    message: String,
}

struct NamePatternRule {
    id: String,
    include: globset::GlobSet,
    exclude: globset::GlobSet,
    pattern: regex::Regex,
    message: String,
}

impl Rule for NamePatternRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "文件命名规范"
    }

    fn doc(&self) -> &str {
        "跟踪文件命中 paths_include（且不在 paths_exclude）时，文件名必须匹配 \
         pattern 正则，否则 finding；全仓不变量。参数见 checks.toml 的 \
         [kinds.name-pattern.<name>]。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Fail
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        ctx.tracked_files()
            .iter()
            .filter(|rel| self.applies(rel))
            .filter_map(|rel| self.check_name(rel))
            .collect()
    }
}

impl NamePatternRule {
    fn applies(&self, rel: &Path) -> bool {
        self.include.is_match(rel) && !self.exclude.is_match(rel)
    }

    fn check_name(&self, rel: &Path) -> Option<Finding> {
        let name = rel.file_name()?.to_str()?;
        if self.pattern.is_match(name) {
            return None;
        }
        Some(Finding::new(
            self.id.as_str(),
            Severity::Fail,
            rel.display().to_string(),
            0,
            format!(
                "{}（文件名 {name} 不匹配 {}）",
                self.message,
                self.pattern.as_str()
            ),
        ))
    }
}

struct NamePatternFactory;

impl KindFactory for NamePatternFactory {
    fn kind(&self) -> &'static str {
        "name-pattern"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: NamePatternParams =
            NamePatternParams::deserialize(toml::Value::Table(params.clone()))
                .map_err(|e| format!("bad params: {e}"))?;
        let include = compile_globs(&parsed.paths_include)?;
        let exclude = compile_globs(&parsed.paths_exclude)?;
        let pattern = regex::Regex::new(&parsed.pattern)
            .map_err(|e| format!("bad regex {:?}: {e}", parsed.pattern))?;
        Ok(Box::new(NamePatternRule {
            id: format!("NAME-PATTERN.{name}"),
            include,
            exclude,
            pattern,
            message: parsed.message,
        }))
    }
}
