//! Dioxus rsx 结构规则：一组件一 rsx、元素嵌套深度、元素体内中文、for 循环 key。
//!
//! 与旧 dioxus_web.py（括号计数近似）不同，本 crate 直接解析 `rsx!` 的 AST
//! （dioxus-rsx 0.7）：元素/组件/文本/for/if 全部是类型化节点，判定精确到行。
//! 解析失败的文件跳过（容忍既有债，不产出伪 finding）；生效 severity / hooks
//! 由成员仓 checks.toml 与 canon dispatch 决定。

use check_core::rule::Rule;

mod cjk;
mod list_key;
mod nesting;
mod one_rsx;
mod parse;
mod walk;

pub use nesting::RsxNestingFactory;

use check_core::kinds::KindFactory;

/// 成员仓 bin 组装用：本域全部静态规则。
pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(one_rsx::OneRsxPerComponent),
        Box::new(cjk::RsxCjkText),
        Box::new(list_key::RsxListKey),
    ]
}

/// 成员仓 bin 组装用：本域的参数化 kind（checks.toml 实例化）。
pub fn factories() -> Vec<Box<dyn KindFactory>> {
    vec![Box::new(RsxNestingFactory)]
}
