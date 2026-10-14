//! for 循环体内必须渲染带 key 的元素/组件（dioxus 列表 diff 的正确性前提）。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;
use dioxus_rsx::{BodyNode, ElementName};

use crate::parse::parse_rsx_calls;
use crate::walk::span_line;

pub struct RsxListKey;

impl Rule for RsxListKey {
    fn id(&self) -> &str {
        "RSX.list-key"
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "for 循环体带 key"
    }

    fn doc(&self) -> &str {
        "`for x in list {}` 的体内应只渲染一个节点，且该节点必须带 `key`\
         （元素属性或组件属性）。无 key 的列表项会让 Dioxus 的 diff 退化为\
         按序对账，插删时错位复用状态。"
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

const ID: &str = "RSX.list-key";

fn walk_nodes(nodes: &[BodyNode], rel: &str, out: &mut Vec<Finding>) {
    for node in nodes {
        walk_node(node, rel, out);
    }
}

fn walk_node(node: &BodyNode, rel: &str, out: &mut Vec<Finding>) {
    match node {
        BodyNode::ForLoop(looped) => {
            check_for_body(looped, rel, out);
            walk_nodes(&looped.body.roots, rel, out);
        }
        BodyNode::Element(el) => walk_nodes(&el.children, rel, out),
        BodyNode::Component(component) => walk_nodes(&component.children.roots, rel, out),
        BodyNode::IfChain(chain) => {
            chain.for_each_branch(&mut |body| walk_nodes(&body.roots, rel, out));
        }
        BodyNode::Text(_) | BodyNode::RawExpr(_) => {}
    }
}

fn check_for_body(looped: &dioxus_rsx::ForLoop, rel: &str, out: &mut Vec<Finding>) {
    let line = span_line(looped.for_token.span);
    if looped.body.roots.len() != 1 {
        out.push(Finding::new(
            ID,
            Severity::Warn,
            rel,
            line,
            format!(
                "for 体内应只渲染一个节点，当前 {} 个",
                looped.body.roots.len()
            ),
        ));
        return;
    }
    if !crate::walk::has_key_node(looped.body.roots.first().unwrap()) {
        out.push(Finding::new(
            ID,
            Severity::Warn,
            rel,
            line,
            format!(
                "for 体内 {} 缺 key 属性",
                node_label(looped.body.roots.first().unwrap())
            ),
        ));
    }
}

fn node_label(node: &BodyNode) -> String {
    match node {
        BodyNode::Element(el) => match &el.name {
            ElementName::Ident(ident) => format!("元素 <{ident}>"),
            other => format!("元素 <{other:?}>"),
        },
        BodyNode::Component(component) => {
            let last = component.name.segments.last();
            match last {
                Some(segment) => format!("组件 {}", segment.ident),
                None => "组件".to_string(),
            }
        }
        BodyNode::Text(_) => "文本节点".to_string(),
        BodyNode::RawExpr(_) => "表达式节点".to_string(),
        BodyNode::ForLoop(_) => "嵌套 for".to_string(),
        BodyNode::IfChain(_) => "if/else".to_string(),
    }
}
