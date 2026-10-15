//! 废弃/不推荐 Rust API 检测：uninitialized / try! / Error::description / ONCE_INIT / BorrowError。
//! 对应 canon stale_api；文本口径（旧行为），跳过注释行（旧版不跳，改进）。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

const STALE_PATS: &[&str] = &[
    "std::mem::uninitialized",
    "try!(",
    "Error::description()",
    "std::sync::ONCE_INIT",
    "std::cell::BorrowError",
];

pub struct StaleApi;

impl Rule for StaleApi {
    fn id(&self) -> &str {
        "RUST.stale-api"
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "废弃 Rust API"
    }

    fn doc(&self) -> &str {
        "命中废弃/不推荐 API 即 WARN：std::mem::uninitialized（未定义行为）、\
         try!（已被 ? 取代）、Error::description()（缺具体信息）、\
         std::sync::ONCE_INIT（已被 Once::new() 取代）、std::cell::BorrowError\
         （已被 BorrowMutError 语义覆盖）。注释行不误伤。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let mut findings = Vec::new();
        for rel in ctx.scope_files() {
            if !crate::is_rust_file(rel) {
                continue;
            }
            let Some(text) = ctx.read(rel).ok() else {
                continue;
            };
            for (idx, line) in text.lines().enumerate() {
                let trimmed = line.trim_start();
                if trimmed.starts_with("//") {
                    continue;
                }
                if let Some(pat) = STALE_PATS.iter().find(|p| line.contains(*p)) {
                    findings.push(Finding::new(
                        self.id(),
                        Severity::Warn,
                        rel.display().to_string(),
                        u32::try_from(idx + 1).unwrap_or(u32::MAX),
                        format!("废弃 API（{pat}）：{}", line.trim()),
                    ));
                }
            }
        }
        findings
    }
}
