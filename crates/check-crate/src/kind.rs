//! `crate-placement` rule kind: turns the catalog diagnostics into canon
//! findings. Configuration lives in `checks.toml`; project specs live in the
//! repository (`crates/<package>/placement.toml` by default).

use serde::Deserialize;

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, NotChecked, RuleOutcome, Severity};
use check_core::kinds::KindFactory;
use check_core::rule::Rule;

use crate::catalog::{Catalog, DiagnoseOptions, LoadConfig};

pub struct CratePlacementFactory;

impl KindFactory for CratePlacementFactory {
    fn kind(&self) -> &'static str {
        "crate-placement"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: Params = Deserialize::deserialize(toml::Value::Table(params.clone()))
            .map_err(|error| format!("bad params: {error}"))?;
        Ok(Box::new(CratePlacementRule {
            id: format!("CRATE-PLACEMENT.{name}"),
            config: LoadConfig {
                spec_file: parsed.spec_file,
                members_include: parsed.members_include,
                members_exclude: parsed.members_exclude,
            },
            require_spec: parsed.require_spec,
            allow_cjk: parsed.allow_cjk,
        }))
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Params {
    #[serde(default = "default_spec_file")]
    spec_file: String,
    #[serde(default = "default_members_include")]
    members_include: Vec<String>,
    #[serde(default)]
    members_exclude: Vec<String>,
    #[serde(default)]
    require_spec: bool,
    #[serde(default)]
    allow_cjk: bool,
}

fn default_spec_file() -> String {
    "placement.toml".to_string()
}

fn default_members_include() -> Vec<String> {
    vec!["crates/**".to_string(), "bin/**".to_string()]
}

struct CratePlacementRule {
    id: String,
    config: LoadConfig,
    require_spec: bool,
    allow_cjk: bool,
}

impl Rule for CratePlacementRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "crate placement contract"
    }

    fn doc(&self) -> &str {
        "Loads per-package placement specs (default `placement.toml`), maps changed files \
         to workspace packages via `cargo metadata`, and reports spec validation problems, \
         path violations, and source files without a [[files]] responsibility. \
         Params under [kinds.crate-placement.<name>]: spec_file, members_include, \
         members_exclude, require_spec, allow_cjk."
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        self.run_outcome(ctx).findings
    }

    fn run_outcome(&self, ctx: &Ctx) -> RuleOutcome {
        let catalog = match Catalog::load(ctx.root(), &self.config) {
            Ok(catalog) => catalog,
            Err(error) => {
                return RuleOutcome {
                    findings: Vec::new(),
                    not_checked: vec![NotChecked {
                        id: self.id.clone(),
                        reason: error.to_string(),
                    }],
                };
            }
        };
        let changed = ctx.scope_files().to_vec();
        let options = DiagnoseOptions {
            require_spec: self.require_spec,
            all: false,
            allow_cjk: self.allow_cjk,
        };
        let findings = catalog
            .diagnose(&changed, &options)
            .into_iter()
            .map(|diagnostic| {
                Finding::new(
                    self.id.clone(),
                    diagnostic.severity,
                    diagnostic.path,
                    diagnostic.line,
                    format!("[{}] {}", diagnostic.id, diagnostic.message),
                )
            })
            .collect();
        RuleOutcome::findings(findings)
    }
}
