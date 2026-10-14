//! 一个组件函数只有一个 `rsx!`（web-spec R1.3）。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;
use syn::spanned::Spanned;
use syn::visit::Visit;

pub struct OneRsxPerComponent;

impl Rule for OneRsxPerComponent {
    fn id(&self) -> &str {
        "RSX.one-rsx-per-component"
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "一组件函数一 rsx"
    }

    fn doc(&self) -> &str {
        "一个组件函数体内至多一个 `rsx!` 调用（web-spec R1.3）。\
         动态拼装请拆子组件或用 fragment 参数，而不是在同一函数里叠多个 rsx。"
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
            let Some(file) = syn::parse_file(&text).ok() else {
                continue;
            };
            let rel_str = rel.display().to_string();
            walk_items(file.items.iter(), &rel_str, &mut findings);
        }
        findings
    }
}

fn walk_items<'a>(items: impl Iterator<Item = &'a syn::Item>, rel: &str, out: &mut Vec<Finding>) {
    for item in items {
        match item {
            syn::Item::Fn(func) => check_fn(func, rel, out),
            syn::Item::Impl(imp) => {
                for inner in &imp.items {
                    if let syn::ImplItem::Fn(method) = inner {
                        check_fn(method, rel, out);
                    }
                }
            }
            syn::Item::Mod(module) => {
                if let Some(content) = &module.content {
                    walk_items(content.1.iter(), rel, out);
                }
            }
            _ => {}
        }
    }
}

fn check_fn<SynFn: FnLike>(func: &SynFn, rel: &str, out: &mut Vec<Finding>) {
    let mut counter = RsxCounter::default();
    counter.visit_block(func.block());
    if counter.count > 1 {
        out.push(Finding::new(
            "RSX.one-rsx-per-component",
            Severity::Warn,
            rel,
            u32::try_from(counter.line).unwrap_or(u32::MAX),
            format!("组件函数含 {} 个 rsx!，应为 1 个（R1.3）", counter.count),
        ));
    }
}

/// syn 2 里 ItemFn 与 ImplItemFn 的块访问形状一致，用小 trait 抹平。
trait FnLike {
    fn block(&self) -> &syn::Block;
}

impl FnLike for syn::ItemFn {
    fn block(&self) -> &syn::Block {
        &self.block
    }
}

impl FnLike for syn::ImplItemFn {
    fn block(&self) -> &syn::Block {
        &self.block
    }
}

#[derive(Default)]
struct RsxCounter {
    count: usize,
    line: usize,
}

impl<'ast> Visit<'ast> for RsxCounter {
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if mac.path.is_ident("rsx") {
            self.count += 1;
            if self.line == 0 {
                self.line = mac.span().start().line;
            }
        }
        syn::visit::visit_macro(self, mac);
    }
}
