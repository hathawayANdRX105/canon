//! rsx 元素嵌套深度（web-spec R1）：过深嵌套是「同一视图塞进一个组件」的症状，
//! 该拆子组件。参数化 kind——同一实现按仓配置不同深度上限。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;
use dioxus_rsx::{BodyNode, Element};

use crate::parse::parse_rsx_calls;

/// 超过 max_depth 的元素即 finding（深度=沿元素链的层级，根元素为 1）。
/// 缺省 3：div>div>div>div（深度 4）起报，与 mono rsx 嵌套审计口径一致。
pub struct RsxNesting {
    id: String,
    max_depth: usize,
}

impl Rule for RsxNesting {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "rsx 嵌套深度"
    }

    fn doc(&self) -> &str {
        "元素链深度超过上限即 finding（for/if 体内继续计深，组件子树是独立 \
         rsx 不叠加）。上限在 checks.toml 的 [kinds.rsx-nesting.<name>] \
         以 max_depth 调整。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let mut findings = Vec::new();
        for rel in ctx.scope_files() {
            if rel.extension().is_some_and(|e| e != "rs") {
                continue;
            }
            let Ok(text) = ctx.read(rel) else {
                continue;
            };
            let Some(calls) = parse_rsx_calls(&text) else {
                continue;
            };
            let rel_str = rel.display().to_string();
            for call in &calls {
                let mut lines = Vec::new();
                walk_depth(&call.body.roots, 0, self.max_depth, &mut lines);
                for line in lines {
                    findings.push(Finding::new(
                        self.id.as_str(),
                        Severity::Warn,
                        &rel_str,
                        u32::try_from(line).unwrap_or(u32::MAX),
                        format!("rsx 嵌套超过 {} 层，该拆子组件了", self.max_depth),
                    ));
                }
            }
        }
        findings
    }
}

fn walk_depth(nodes: &[BodyNode], depth: usize, max: usize, out: &mut Vec<usize>) {
    for node in nodes {
        match node {
            BodyNode::Element(el) => enter_element(el, depth, max, out),
            BodyNode::ForLoop(looped) => walk_depth(&looped.body.roots, depth, max, out),
            BodyNode::IfChain(chain) => {
                chain.for_each_branch(&mut |body| walk_depth(&body.roots, depth, max, out))
            }
            BodyNode::Component(_) | BodyNode::Text(_) | BodyNode::RawExpr(_) => {}
        }
    }
}

fn enter_element(el: &Element, depth: usize, max: usize, out: &mut Vec<usize>) {
    let next = depth + 1;
    if next > max {
        out.push(el.name.span().start().line);
    }
    walk_depth(&el.children, next, max, out);
}

pub struct RsxNestingFactory;

impl check_core::kinds::KindFactory for RsxNestingFactory {
    fn kind(&self) -> &'static str {
        "rsx-nesting"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: RsxNestingParams =
            serde::Deserialize::deserialize(toml::Value::Table(params.clone()))
                .map_err(|e| format!("bad params: {e}"))?;
        Ok(Box::new(RsxNesting {
            id: format!("RSX-NESTING.{name}"),
            max_depth: parsed.max_depth,
        }))
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RsxNestingParams {
    #[serde(default = "default_max_depth")]
    max_depth: usize,
}

fn default_max_depth() -> usize {
    3
}
