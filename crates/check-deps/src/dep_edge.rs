//! 依赖边界：workspace 内 normal path 依赖边的 glob deny 清单（D9 dep-edge kind）。
//!
//! 参数（deny_unknown_fields）：
//!   edges_deny = [{ from = "<源包目录 glob>", to = "<目标包目录 glob>", message = "可选" }]
//! from 命中依赖方包目录、to 命中被依赖方包目录才报处。glob 对 Cargo.toml
//! 相对目录匹配，`**` 跨层、可带尾 `/**`。预编译在 instantiate，运行期零重解析。

use std::path::Path;

use globset::GlobSet;
use serde::Deserialize;

use crate::metadata;

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

pub struct DepEdgeFactory;

impl check_core::kinds::KindFactory for DepEdgeFactory {
    fn kind(&self) -> &'static str {
        "dep-edge"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: DepEdgeParams = Deserialize::deserialize(toml::Value::Table(params.clone()))
            .map_err(|e| format!("bad params: {e}"))?;
        let mut deny = Vec::new();
        for edge in &parsed.edges_deny {
            deny.push(EdgeDenyCompiled {
                from: compile(&edge.from)?,
                to: compile(&edge.to)?,
                message: edge.message().to_string(),
            });
        }
        Ok(Box::new(DepEdgeRule {
            id: format!("DEP-EDGE.{name}"),
            deny,
        }))
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DepEdgeParams {
    edges_deny: Vec<EdgeDeny>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EdgeDeny {
    from: String,
    to: String,
    #[serde(default)]
    message: Option<String>,
}

impl EdgeDeny {
    fn message(&self) -> String {
        self.message
            .clone()
            .unwrap_or_else(|| format!("依赖边界违规：{} 不得依赖 {}", self.from, self.to))
    }
}

fn compile(pattern: &str) -> Result<GlobSet, String> {
    let glob = globset::GlobBuilder::new(pattern)
        .literal_separator(true)
        .build()
        .map_err(|e| format!("bad glob {pattern:?}: {e}"))?;
    let mut builder = globset::GlobSet::builder();
    builder.add(glob);
    builder
        .build()
        .map_err(|e| format!("bad glob {pattern:?}: {e}"))
}

/// 一条 deny 边编译后的匹配器。
struct EdgeDenyCompiled {
    from: GlobSet,
    to: GlobSet,
    message: String,
}

impl EdgeDenyCompiled {
    /// 目录口径：glob 命中「目录自身」或「目录下任一文件」都算。
    /// globset 对尾 `/**` 不匹配目录本身（实测），故双探。
    fn matches_dir(&self, set: &GlobSet, dir: &str) -> bool {
        set.is_match(Path::new(dir)) || set.is_match(Path::new(dir).join("Cargo.toml"))
    }

    fn hit(&self, src_dir: &str, dst_dir: &str) -> bool {
        self.matches_dir(&self.from, src_dir) && self.matches_dir(&self.to, dst_dir)
    }
}

struct DepEdgeRule {
    id: String,
    deny: Vec<EdgeDenyCompiled>,
}

impl Rule for DepEdgeRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "依赖边界"
    }

    fn doc(&self) -> &str {
        "workspace 内 normal path 依赖边命中 edges_deny（from、to 都按包目录 \
         glob 匹配）即 finding；dev/build 依赖不参与。参数见 checks.toml 的 \
         [kinds.dep-edge.<name>]。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Fail
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let Some(graph) = metadata::load(ctx.root()) else {
            return Vec::new();
        };
        let mut findings = Vec::new();
        for (package, dep) in &graph.edges {
            let (Some(src_dir), Some(dst_dir)) =
                (graph.package_dirs.get(package), graph.package_dirs.get(dep))
            else {
                continue;
            };
            for edge in &self.deny {
                if !edge.hit(src_dir, dst_dir) {
                    continue;
                }
                findings.push(Finding::new(
                    self.id.as_str(),
                    Severity::Fail,
                    format!("{src_dir}/Cargo.toml"),
                    0,
                    format!("{}（实际边：{package} -> {dep}）", edge.message),
                ));
            }
        }
        findings
    }
}
