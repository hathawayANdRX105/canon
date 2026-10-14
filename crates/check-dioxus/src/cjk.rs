//! rsx 元素体内不得出现中文文本（web-spec H3：页面文案走 i18n / 文案表）。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;
use dioxus_rsx::BodyNode;
use quote::ToTokens;

use crate::parse::parse_rsx_calls;
use crate::walk::span_line;

pub struct RsxCjkText;

impl Rule for RsxCjkText {
    fn id(&self) -> &str {
        "RSX.cjk-text"
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "rsx 元素体内无中文"
    }

    fn doc(&self) -> &str {
        "`rsx!` 的文本节点不得含中文（H3：用户可见文案走 i18n / 文案表，\
         面向人的调试注释不受限）。AST 判定只看文本节点，注释与属性值不误伤。"
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
                walk_nodes(&call.body.roots, &rel_str, &mut findings);
            }
        }
        findings
    }
}

fn walk_nodes(nodes: &[BodyNode], rel: &str, out: &mut Vec<Finding>) {
    for node in nodes {
        walk_node(node, rel, out);
    }
}

fn walk_node(node: &BodyNode, rel: &str, out: &mut Vec<Finding>) {
    match node {
        BodyNode::Text(text) => {
            let rendered = text.to_token_stream().to_string();
            if has_cjk(&rendered) {
                out.push(Finding::new(
                    "RSX.cjk-text",
                    Severity::Warn,
                    rel,
                    span_line(text.input.span()),
                    format!("元素体内出现中文文本：{}", rendered.trim()),
                ));
            }
        }
        BodyNode::Element(el) => walk_nodes(&el.children, rel, out),
        // 组件内联子模板块也是写在本文件的 rsx，一并扫
        BodyNode::Component(component) => walk_nodes(&component.children.roots, rel, out),
        BodyNode::ForLoop(looped) => walk_nodes(&looped.body.roots, rel, out),
        BodyNode::IfChain(chain) => {
            chain.for_each_branch(&mut |body| walk_nodes(&body.roots, rel, out));
        }
        BodyNode::RawExpr(_) => {}
    }
}

fn has_cjk(text: &str) -> bool {
    text.chars().any(is_cjk)
}

fn is_cjk(c: char) -> bool {
    matches!(c as u32, 0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0xF900..=0xFAFF)
}
