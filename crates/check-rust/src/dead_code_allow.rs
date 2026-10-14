//! 合并前清理 #[allow(dead_code)]：压制信号不是修问题。
//! 对应 canon rust_no_dead_code_allow；同行带 `//` 理由的形态放行。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

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
        "出现 `#[allow(dead_code)]` 即 finding；确需保留的，同行 `//` 写明理由放行\
         （口径与 canon rust_no_dead_code_allow 一致）。多行属性按起始行判定。"
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
            for (idx, line) in text.lines().enumerate() {
                let Some(after) = line.split(']').next_back() else {
                    continue;
                };
                if line.contains("#[allow(dead_code") && !after.contains("//") {
                    findings.push(Finding::new(
                        self.id(),
                        Severity::Warn,
                        rel.display().to_string(),
                        u32::try_from(idx + 1).unwrap_or(u32::MAX),
                        format!("合并前清理 #[allow(dead_code)]：{}", line.trim()),
                    ));
                }
            }
        }
        findings
    }
}
