//! 合并前清理 #[allow(dead_code)]：压制信号不是修问题。
//! 对应 canon rust_no_dead_code_allow；同行带 `//` 理由的形态放行。
//!
//! syn attribute 口径（AST 遍历）：只认真实属性节点——字符串字面量、注释里的
//! `#[allow(dead_code)]` 文案不再误报（T10 取证遗留 FP 的修正）。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

use crate::attrs;

pub struct DeadCodeAllow;

impl Rule for DeadCodeAllow {
    fn id(&self) -> &str {
        "RUST.dead-code-allow"
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "清理 #[allow(dead_code)]"
    }

    fn doc(&self) -> &str {
        "真实 `#[allow(dead_code)]` 属性即 finding（AST 判定，`clippy::dead_code` \
         等跨路径不误伤）；确需保留的，属性同行 `//` 写明理由放行\
         （口径与 canon rust_no_dead_code_allow 一致）。多行属性按结束行判定豁免。\
         解析失败的文件跳过——语法错误由 clippy 规则兜底。"
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
            let Some(text) = ctx.read(rel).ok() else {
                continue;
            };
            let Some(file) = attrs::parse(&text) else {
                continue;
            };
            for hit in attrs::scan(&file, &attrs::is_allow_dead_code) {
                if hit.trailing_comment(&text) {
                    continue;
                }
                findings.push(Finding::new(
                    self.id(),
                    Severity::Warn,
                    rel.display().to_string(),
                    u32::try_from(hit.line).unwrap_or(u32::MAX),
                    format!("合并前清理 #[allow(dead_code)]：{}", hit.source.trim()),
                ));
            }
        }
        findings
    }
}
