//! 共享的 rsx 树遍历与判键助手。
//!
//! `BodyNode::key()` 在 dioxus-rsx 里是 pub(crate)，外部判 key 需自看属性名。

use dioxus_rsx::{Attribute, AttributeName, BodyNode};

/// 元素/组件属性里是否带 `key`。
pub fn has_key(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| attr_is_key(&a.name))
}

/// 该节点本身是否携带 key（元素属性 / 组件属性）。
pub fn has_key_node(node: &BodyNode) -> bool {
    match node {
        BodyNode::Element(el) => has_key(&el.raw_attributes) || has_key(&el.merged_attributes),
        BodyNode::Component(component) => has_key(&component.fields),
        BodyNode::Text(_) | BodyNode::RawExpr(_) | BodyNode::ForLoop(_) | BodyNode::IfChain(_) => {
            false
        }
    }
}

fn attr_is_key(name: &AttributeName) -> bool {
    match name {
        AttributeName::BuiltIn(ident) => ident == "key",
        AttributeName::Custom(lit) => lit.value() == "key",
        AttributeName::Spread(_) => false,
    }
}

/// 行号助手：proc-macro2 span-locations（本 crate 显式启用）。
pub fn span_line(span: proc_macro2::Span) -> u32 {
    u32::try_from(span.start().line).unwrap_or(u32::MAX)
}
