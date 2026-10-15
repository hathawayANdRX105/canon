//! tests/ 目录是集成测试 crate，天然只在测试编译——里面再写 #[cfg(test)] 是多余包装。
//! 对应 canon rust_no_cfg_test_in_tests_dir，改为 Rust 实现。
//!
//! syn attribute 口径（AST 遍历）：只认真实 `#[cfg(test)]` 属性——测试 fixture
//! 字符串里的文案不再误报。src/ 里的 #[cfg(test)] 是正当单测形态，路径过滤排除。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

use crate::attrs;

pub struct CfgTestInTests;

impl Rule for CfgTestInTests {
    fn id(&self) -> &str {
        "RUST.cfg-test-in-tests"
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "tests/ 内不写 #[cfg(test)]"
    }

    fn doc(&self) -> &str {
        "同层 tests/ 目录下的 .rs 是集成测试 crate，本就只在测试编译；\
         真实 `#[cfg(test)]` 属性（AST 判定）属多余包装。注意：src/ 里的 \
         #[cfg(test)] 是正当单测形态，本规则不碰。解析失败的文件跳过——\
         语法错误由 clippy 规则兜底。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let mut findings = Vec::new();
        for rel in ctx.scope_files() {
            if !in_tests_dir(rel) {
                continue;
            }
            let Some(text) = ctx.read(rel).ok() else {
                continue;
            };
            let Some(file) = attrs::parse(&text) else {
                continue;
            };
            for hit in attrs::scan(&file, &attrs::is_cfg_test) {
                findings.push(Finding::new(
                    self.id(),
                    Severity::Warn,
                    rel.display().to_string(),
                    u32::try_from(hit.line).unwrap_or(u32::MAX),
                    "tests/ 内不需要 #[cfg(test)]——集成测试 crate 本就只在测试编译",
                ));
            }
        }
        findings
    }
}

fn in_tests_dir(rel: &std::path::Path) -> bool {
    rel.extension().is_some_and(|e| e == "rs")
        && rel
            .components()
            .filter_map(|c| c.as_os_str().to_str())
            .any(|c| c == "tests")
}
