//! Rust 源码结构类规则：dead_code allow、process-cmd、stale API、
//! todo 挂号、空模块、测试断言（canon §6 迁移表 rust 栈全量）。
//!
//! 属性类规则（dead_code allow）走 syn AST 遍历——字符串字面量与
//! 注释里的文案不再误报（T10 取证遗留 FP 的修正）；行号取 proc-macro2
//! span-locations。解析失败的文件跳过（语法错误由 clippy 规则兜底）。
//! 生效 severity / hooks 由成员仓 checks.toml 与 canon dispatch 决定。

use check_core::rule::Rule;

/// 扩展名严格等于 .rs 才扫：无扩展名文件（如 `.agent/skills/notes/notes` 脚本）
/// 曾被误扫（spec-problem 双跑对照实测，is_some_and 对 None 误放行）。
pub(crate) fn is_rust_file(rel: &std::path::Path) -> bool {
    rel.extension().is_some_and(|e| e == "rs")
}

mod attrs;
mod dead_code_allow;
mod empty_module;
mod idiom;
mod process_cmd;
mod stale_api;
mod test_no_assert;
mod todo_needs_issue;

/// 成员仓 bin 组装用：本域全部规则。
pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(dead_code_allow::DeadCodeAllow),
        Box::new(process_cmd::ProcessCmd),
        Box::new(stale_api::StaleApi),
        Box::new(todo_needs_issue::TodoNeedsIssue),
        Box::new(empty_module::EmptyModule),
        Box::new(test_no_assert::TestNoAssert),
    ]
}

/// 成员仓 bin 组装用：本域参数化 kind 工厂（rust-idiom）。
pub fn factories() -> Vec<Box<dyn check_core::kinds::KindFactory>> {
    vec![Box::new(idiom::RustIdiomFactory)]
}
