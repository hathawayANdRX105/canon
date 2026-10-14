//! Rust 源码结构类规则：tests 布置、tests/ 内 cfg(test)、dead_code allow。
//!
//! 这三条是文本级不变量，逐行扫描即精确（行号即 finding 位置、无解析失败盲区）；
//! AST 级结构规则（命名语义、unsafe 不变量、rsx 结构）在 check-dioxus / 后续批次。
//! 生效 severity / hooks 由成员仓 checks.toml 与 canon dispatch 决定。

use check_core::rule::Rule;

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
