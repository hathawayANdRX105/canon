//! Rust 源码结构类规则：tests 布置、tests/ 内 cfg(test)、dead_code allow。
//!
//! 属性类规则（cfg(test)/dead_code allow）走 syn AST 遍历——字符串字面量与
//! 注释里的文案不再误报（T10 取证遗留 FP 的修正）；行号取 proc-macro2
//! span-locations。解析失败的文件跳过（语法错误由 clippy 规则兜底）。
//! 生效 severity / hooks 由成员仓 checks.toml 与 canon dispatch 决定。

use check_core::rule::Rule;

mod attrs;
mod cfg_test_in_tests;
mod dead_code_allow;
mod test_file_in_src;

/// 成员仓 bin 组装用：本域全部规则。
pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(test_file_in_src::TestFileInSrc),
        Box::new(cfg_test_in_tests::CfgTestInTests),
        Box::new(dead_code_allow::DeadCodeAllow),
    ]
}
