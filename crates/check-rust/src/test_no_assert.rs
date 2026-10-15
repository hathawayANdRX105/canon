//! 测试函数必须验证行为：`#[test]` 函数体内没有任何断言/验证类 token 即 WARN。
//! 对应 canon rust_test_no_assert；旧实现是 awk 启发式（不能处理嵌套 mod tests），
//! 本版走 syn（visit #[test] item，嵌套 mod 正确处理）。
//!
//! 口径 = 确定性召回 + 保守豁免词（verify/mock/spy 等以其他方式验证的形态）；
//! 精细的「是否真验证行为」判定留语义层（check-semantic question）。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;
use syn::spanned::Spanned;
use syn::visit::Visit;

/// 断言/验证类 token：命中任一即视为「有验证」。
const VERIFY_TOKENS: &[&str] = &[
    "assert!",
    "assert_eq!",
    "assert_ne!",
    "assert_matches!",
    "should_panic",
    ".expect(",
    "verify",
    "mock",
    "snapshot",
    "unwrap_or_else",
    ".unwrap()",
];

pub struct TestNoAssert;

impl Rule for TestNoAssert {
    fn id(&self) -> &str {
        "RUST.test-no-assert"
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "测试函数必须含断言"
    }

    fn doc(&self) -> &str {
        "`#[test]` 函数体内没有任何 assert/assert_eq/assert_ne/should_panic/expect/\
         verify/mock/snapshot 类 token 即 WARN——「跑过就算」的空测试。\
         syn 口径：嵌套 mod tests 内的 #[test] 一并覆盖（旧 awk 版的已知盲区）。\
         保守豁免词近似旧 jev 判决半（spy/verify 风格幸存）；精判留语义层。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let mut findings = Vec::new();
        for rel in ctx.scope_files() {
            if !is_test_file(rel) {
                continue;
            }
            let Some(text) = ctx.read(rel).ok() else {
                continue;
            };
            let Some(file) = syn::parse_file(&text).ok() else {
                continue;
            };
            let mut v = TestVisitor {
                source: &text,
                hits: Vec::new(),
            };
            v.visit_file(&file);
            for (line, name) in v.hits {
                findings.push(Finding::new(
                    self.id(),
                    Severity::Warn,
                    rel.display().to_string(),
                    u32::try_from(line).unwrap_or(u32::MAX),
                    format!("测试 {name} 无任何断言/验证——跑过不等于测过"),
                ));
            }
        }
        findings
    }
}

fn is_test_file(rel: &std::path::Path) -> bool {
    if rel.extension().is_some_and(|e| e != "rs") {
        return false;
    }
    let path = rel.to_string_lossy();
    let name = rel.file_name().and_then(|n| n.to_str()).unwrap_or_default();
    path.contains("tests/") || name.ends_with("_test.rs") || name.starts_with("test_")
}

struct TestVisitor<'a> {
    source: &'a str,
    hits: Vec<(usize, String)>,
}

impl<'ast> Visit<'ast> for TestVisitor<'ast> {
    fn visit_item_fn(&mut self, func: &'ast syn::ItemFn) {
        let is_test = func.attrs.iter().any(|a| a.path().is_ident("test"));
        if is_test {
            let start = func.span().start();
            let end = func.span().end();
            // 函数体文本（span 行区间内逐行截取；列精确到 body 大括号之间太脆，行级足够）
            let body: String = self
                .source
                .lines()
                .skip(start.line.saturating_sub(1))
                .take(end.line - start.line + 1)
                .collect::<Vec<_>>()
                .join("\n");
            let has_verification = VERIFY_TOKENS.iter().any(|t| body.contains(t));
            if !has_verification {
                let name = func.sig.ident.to_string();
                self.hits.push((start.line, name));
            }
        }
        syn::visit::visit_item_fn(self, func);
    }
}
