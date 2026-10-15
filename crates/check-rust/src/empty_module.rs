//! 空模块文件：去掉注释/空行后 ≤2 行且无任何实现 → 提示合并到上层 mod。
//! 对应 canon rust_no_empty_module；文件级文本口径（与旧 shell 一致）。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

pub struct EmptyModule;

impl Rule for EmptyModule {
    fn id(&self) -> &str {
        "RUST.empty-module"
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "微型空文件"
    }

    fn doc(&self) -> &str {
        ".rs 文件去掉注释与空行后 ≤2 行、且不含 fn/struct/impl/trait/enum/\
         macro_rules! 实现即 WARN——文件拆得太碎（只剩 re-export 甚至全空）\
         增加认知负担。文件级口径（< 100 字节快速预筛 + 内容判定）。"
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
            if text.len() >= 100 {
                continue;
            }
            let effective = text
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && !l.starts_with("//"))
                .count();
            if effective > 2 {
                continue;
            }
            let has_impl = ["fn ", "struct ", "impl ", "trait ", "enum ", "macro_rules!"]
                .iter()
                .any(|kw| text.contains(kw));
            if has_impl {
                continue;
            }
            findings.push(Finding::new(
                self.id(),
                Severity::Warn,
                rel.display().to_string(),
                0,
                format!("微型空文件（有效 {effective} 行），合并到上层 mod"),
            ));
        }
        findings
    }
}
