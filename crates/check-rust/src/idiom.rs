//! `rust-idiom` 参数化 kind：按 Rust 关键词/语法（syn AST）匹配惯用法与反模式。
//!
//! 每个检测器只做**语法级**匹配，不做类型推断（D9：复杂语义写专用 Rust 规则，
//! TOML 只做参数与装配）。检测器集合对应 Rust Book / rust-unofficial/patterns /
//! clippy lint 的惯用法，但实现是 syn 语法近似——误报由 agent 按 canon 处置纪律
//! 书面驳回（WARN 与 FAIL 同等对待）。语法错误的文件跳过（clippy 兜底）。

use std::collections::HashSet;
use std::path::Path;

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::kinds::KindFactory;
use check_core::rule::Rule;
use quote::ToTokens;
use serde::Deserialize;
use syn::spanned::Spanned;
use syn::visit::{
    Visit, visit_expr, visit_expr_method_call, visit_impl_item_fn, visit_item_fn, visit_item_mod,
    visit_local, visit_trait_item_fn,
};
use syn::{
    BinOp, Block, Expr, ExprBinary, ExprForLoop, ExprIf, ExprMatch, ExprMethodCall, ExprWhile, Lit,
    Local, Pat, ReturnType, Stmt, Type,
};

/// 全部检测器 id（checks.toml `idioms` 参数取值集合）。
pub const ALL_IDIOMS: &[&str] = &[
    "if-else-chain",
    "collapsible-if",
    "nested-if-let",
    "manual-let-else",
    "needless-return",
    "manual-question-mark",
    "unwrap-in-fallible",
    "hashmap-entry",
    "while-let-on-iterator",
    "vec-push-loop",
    "bool-comparison",
    "redundant-pattern-matching",
];

// ---------------------------------------------------------------------------
// 工厂与参数

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RustIdiomParams {
    #[serde(default)]
    paths_include: Vec<String>,
    #[serde(default)]
    paths_exclude: Vec<String>,
    /// 启用的检测器 id（见 ALL_IDIOMS）；空 = 全部。
    #[serde(default)]
    idioms: Vec<String>,
    /// if-else-chain 报出的最短连续 `else if` 数（链长 = 分支数 - 1）。
    #[serde(default = "default_min_chain_len")]
    min_chain_len: usize,
}

fn default_min_chain_len() -> usize {
    3
}

/// 成员仓 bin 组装用：本域参数化 kind 工厂。
pub struct RustIdiomFactory;

impl KindFactory for RustIdiomFactory {
    fn kind(&self) -> &'static str {
        "rust-idiom"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: RustIdiomParams =
            RustIdiomParams::deserialize(toml::Value::Table(params.clone()))
                .map_err(|e| format!("bad params: {e}"))?;
        if parsed.min_chain_len < 2 {
            return Err(format!(
                "min_chain_len 必须 ≥ 2，实得 {}",
                parsed.min_chain_len
            ));
        }
        let enabled: Vec<&'static str> = if parsed.idioms.is_empty() {
            ALL_IDIOMS.to_vec()
        } else {
            let mut v = Vec::new();
            for id in &parsed.idioms {
                let Some(&known) = ALL_IDIOMS.iter().find(|x| **x == id.as_str()) else {
                    return Err(format!(
                        "unknown idiom {id:?}（可选：{}）",
                        ALL_IDIOMS.join(", ")
                    ));
                };
                v.push(known);
            }
            v
        };
        let include = compile_globs(&parsed.paths_include)?;
        let exclude = compile_globs(&parsed.paths_exclude)?;
        Ok(Box::new(RustIdiomRule {
            id: format!("RUST-IDIOM.{name}"),
            include,
            exclude,
            enabled,
            min_chain_len: parsed.min_chain_len,
        }))
    }
}

fn compile_globs(patterns: &[String]) -> Result<globset::GlobSet, String> {
    let mut builder = globset::GlobSet::builder();
    for pattern in patterns {
        let glob = globset::GlobBuilder::new(pattern)
            .literal_separator(true)
            .build()
            .map_err(|e| format!("bad glob {pattern:?}: {e}"))?;
        builder.add(glob);
    }
    builder.build().map_err(|e| format!("bad globs: {e}"))
}

// ---------------------------------------------------------------------------
// 规则

struct RustIdiomRule {
    id: String,
    include: globset::GlobSet,
    exclude: globset::GlobSet,
    enabled: Vec<&'static str>,
    min_chain_len: usize,
}

impl RustIdiomRule {
    fn applies(&self, rel: &Path) -> bool {
        (self.include.is_empty() || self.include.is_match(rel)) && !self.exclude.is_match(rel)
    }
}

impl Rule for RustIdiomRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "Rust 惯用法与反模式（syn 关键词/语法匹配）"
    }

    fn doc(&self) -> &str {
        "Rust 惯用法与反模式审查（syn 语法匹配，无类型推断；WARN，agent 必须逐条处置）。检测器：\n\
         - if-else-chain：连续 else if ≥ min_chain_len 改用 match+enum（Rust Book ch6）\n\
         - collapsible-if：if a { if b {} } 合并为 if a && b（clippy collapsible_if）\n\
         - nested-if-let：if let 嵌套 if let 改用 let-else/组合模式\n\
         - manual-let-else：手写 if let+else 返回改用 let-else（clippy manual_let_else）\n\
         - needless-return：函数尾 return 表达式可省略（clippy needless_return）\n\
         - manual-question-mark：match/if 手动传播错误改用 ?（Book ch9；clippy question_mark）\n\
         - unwrap-in-fallible：Result/Option 函数内 unwrap/expect 改用 ?（clippy unwrap_used）\n\
         - hashmap-entry：contains_key+get_mut/insert 改用 Entry API（clippy map_entry）\n\
         - while-let-on-iterator：while let x=iter.next() 改用 for（clippy while_let_on_iterator）\n\
         - vec-push-loop：for 循环只做 push 改用迭代器 collect\n\
         - bool-comparison：x==true/false 简化（clippy bool_comparison）\n\
         - redundant-pattern-matching：match 手写 bool 改用 is_ok/is_err/is_some/is_none（clippy redundant_pattern_matching）\n\
         参数见 checks.toml [kinds.rust-idiom.<name>]（paths_include / paths_exclude / idioms / min_chain_len）。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let enabled: HashSet<&'static str> = self.enabled.iter().copied().collect();
        let mut findings = Vec::new();
        for rel in ctx.scope_files() {
            if !crate::is_rust_file(rel) || !self.applies(rel) {
                continue;
            }
            let Ok(text) = ctx.read(rel) else {
                continue;
            };
            let Ok(file) = syn::parse_file(&text) else {
                continue; // 语法错误跳过（由 clippy 兜底）
            };
            let mut visitor = IdiomVisitor {
                rel,
                id: &self.id,
                enabled: &enabled,
                min_chain_len: self.min_chain_len,
                findings: Vec::new(),
                cfg_test_depth: 0,
                fallible_depth: 0,
                covered_chain_heads: HashSet::new(),
            };
            visitor.visit_file(&file);
            findings.extend(visitor.findings);
        }
        findings
    }
}

// ---------------------------------------------------------------------------
// syn 访问器

struct IdiomVisitor<'a> {
    rel: &'a Path,
    id: &'a str,
    enabled: &'a HashSet<&'static str>,
    min_chain_len: usize,
    findings: Vec<Finding>,
    /// `#[cfg(test)] mod` 嵌套深度：测试代码不做 unwrap 审查。
    cfg_test_depth: usize,
    /// 返回 Result/Option 的函数嵌套深度：>0 时 unwrap/expect 可改用 `?`。
    fallible_depth: usize,
    /// 已作为 if/else 链头报告过的 if 起始位置（防子链重复报告）。
    covered_chain_heads: HashSet<(usize, usize)>,
}

impl<'ast, 'a> IdiomVisitor<'a> {
    fn on(&self, detector: &str) -> bool {
        self.enabled.contains(detector)
    }

    fn push(
        &mut self,
        line: usize,
        detector: &str,
        message: impl Into<String>,
        suggestion: impl Into<String>,
    ) {
        self.findings.push(
            Finding::new(
                self.id,
                Severity::Warn,
                self.rel.display().to_string(),
                u32::try_from(line).unwrap_or(u32::MAX),
                format!("[{}] {}", detector, message.into()),
            )
            .with_suggestion(suggestion),
        );
    }

    // ---- if 家族 ----

    fn check_if(&mut self, e: &'ast ExprIf) {
        if self.on("if-else-chain") {
            let key = (e.span().start().line, e.span().start().column);
            if !self.covered_chain_heads.contains(&key) {
                self.check_if_else_chain(e);
            }
        }
        if self.on("collapsible-if") {
            self.check_collapsible_if(e);
        }
        if self.on("nested-if-let") {
            self.check_nested_if_let(e);
        }
        if self.on("manual-question-mark") {
            self.check_is_err_early_return(e);
        }
        if self.on("hashmap-entry") {
            self.check_hashmap_entry(e);
        }
    }

    fn check_if_else_chain(&mut self, e: &'ast ExprIf) {
        let mut chain = vec![e];
        let mut cur = e;
        while let Some((_, else_expr)) = cur.else_branch.as_ref() {
            let Expr::If(next) = else_expr.as_ref() else {
                break;
            };
            chain.push(next);
            cur = next;
        }
        // min_chain_len = 连续 else if 数；链长（分支数）= else if 数 + 1
        if chain.len() - 1 < self.min_chain_len {
            return;
        }
        for node in &chain {
            let s = node.span().start();
            self.covered_chain_heads.insert((s.line, s.column));
        }
        let line = e.span().start().line;
        self.push(
            line,
            "if-else-chain",
            format!("if/else 链共 {} 个分支，建议改用 match + enum 表达", chain.len()),
            "用 match 替代 if/else 链：match value { Variant::A => .., Variant::B => .., _ => .. }。参考 https://doc.rust-lang.org/book/ch06-02-match.html",
        );
    }

    fn check_collapsible_if(&mut self, e: &'ast ExprIf) {
        if e.else_branch.is_some() || is_let_cond(&e.cond) {
            return;
        }
        let Some(inner) = single_expr_of_block(&e.then_branch) else {
            return;
        };
        let Expr::If(inner_if) = inner else {
            return;
        };
        if inner_if.else_branch.is_some() || is_let_cond(&inner_if.cond) {
            return;
        }
        let line = e.span().start().line;
        self.push(
            line,
            "collapsible-if",
            "嵌套 if 可合并条件，减少一层嵌套",
            "合并为 if a && b { .. }。参考 clippy::collapsible_if（style）。",
        );
    }

    fn check_nested_if_let(&mut self, e: &'ast ExprIf) {
        if e.else_branch.is_some() || !is_let_cond(&e.cond) {
            return;
        }
        let Some(inner) = single_expr_of_block(&e.then_branch) else {
            return;
        };
        let Expr::If(inner_if) = inner else {
            return;
        };
        if inner_if.else_branch.is_some() || !is_let_cond(&inner_if.cond) {
            return;
        }
        let line = e.span().start().line;
        self.push(
            line,
            "nested-if-let",
            "if let 直接嵌套 if let，可改用 let-else 或组合模式",
            "用 let-else 或组合模式：let (Some(a), Some(b)) = (a, b) else { return };",
        );
    }

    fn check_is_err_early_return(&mut self, e: &'ast ExprIf) {
        let Expr::MethodCall(mc) = e.cond.as_ref() else {
            return;
        };
        if mc.method != "is_err" && mc.method != "is_none" {
            return;
        }
        let Some(ret) = single_expr_of_block(&e.then_branch) else {
            return;
        };
        let Expr::Return(ret_expr) = ret else {
            return;
        };
        let Some(ret_val) = &ret_expr.expr else {
            return;
        };
        let line = e.span().start().line;
        if mc.method == "is_err" {
            // if x.is_err() { return x; }（或 x.as_ref() 变体）→ x?
            let recv = normalize_tt(&mc.receiver.to_token_stream());
            let ret = normalize_tt(&ret_val.to_token_stream());
            if !recv.is_empty()
                && !ret.is_empty()
                && (recv == ret || recv.contains(&ret) || ret.contains(&recv))
            {
                self.push(
                    line,
                    "manual-question-mark",
                    "手写 is_err + return 传播错误，可用 ? 替代",
                    "改写：x?;。参考 https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
                );
            }
        } else if is_path_ident(ret_val, "None") {
            // if opt.is_none() { return None; } → opt?;
            self.push(
                line,
                "manual-question-mark",
                "手写 is_none + return None，可用 ? 替代",
                "改写：opt?;。参考 https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
            );
        }
    }

    fn check_hashmap_entry(&mut self, e: &'ast ExprIf) {
        let Expr::MethodCall(mc) = e.cond.as_ref() else {
            return;
        };
        if mc.method != "contains_key" {
            return;
        }
        let recv = normalize_tt(&mc.receiver.to_token_stream());
        if recv.is_empty() {
            return;
        }
        let then_has_get_mut = block_contains_method(&e.then_branch, "get_mut", &recv);
        let else_has_insert = e
            .else_branch
            .as_ref()
            .is_some_and(|(_, els)| expr_contains_method(els, "insert", &recv));
        if !then_has_get_mut && !else_has_insert {
            return;
        }
        let line = e.span().start().line;
        self.push(
            line,
            "hashmap-entry",
            "contains_key + get_mut/insert 非原子且冗余，改用 Entry API",
            "用 Entry API：match map.entry(k) { Occupied(mut e) => *e.get_mut() += 1, Vacant(e) => { e.insert(v); } }。参考 https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#method.entry",
        );
    }

    // ---- match 家族 ----

    fn check_match(&mut self, e: &'ast ExprMatch) {
        if self.on("manual-question-mark") {
            self.check_try_match(e);
        }
        if self.on("redundant-pattern-matching") {
            self.check_redundant_bool_match(e);
        }
    }

    fn check_try_match(&mut self, e: &'ast ExprMatch) {
        if e.arms.len() != 2 {
            return;
        }
        let (a, b) = (&e.arms[0], &e.arms[1]);
        if a.guard.is_some() || b.guard.is_some() {
            return;
        }
        let line = e.span().start().line;
        // Result：Ok(v) => v, Err(e) => return Err(e[.into()])
        if let Some(v) = ctor_binding(&a.pat, "Ok")
            && is_path_ident(&a.body, &v.to_string())
            && let Some(er) = ctor_binding(&b.pat, "Err")
            && returns_ctor_with_binding(&b.body, "Err", er)
        {
            self.push(
                line,
                "manual-question-mark",
                "match 手动传播错误，可用 ? 替代",
                "改写：let v = x?;。参考 https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
            );
            return;
        }
        if let Some(v) = ctor_binding(&b.pat, "Ok")
            && is_path_ident(&b.body, &v.to_string())
            && let Some(er) = ctor_binding(&a.pat, "Err")
            && returns_ctor_with_binding(&a.body, "Err", er)
        {
            self.push(
                line,
                "manual-question-mark",
                "match 手动传播错误，可用 ? 替代",
                "改写：let v = x?;。参考 https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
            );
            return;
        }
        // Option：Some(v) => v, None => return None
        if let Some(v) = ctor_binding(&a.pat, "Some")
            && is_path_ident(&a.body, &v.to_string())
            && pat_is_none(&b.pat)
            && returns_none(&b.body)
        {
            self.push(
                line,
                "manual-question-mark",
                "match 手动处理 Option 空值，可用 ? 替代",
                "改写：let v = x?;。参考 https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
            );
        }
    }

    fn check_redundant_bool_match(&mut self, e: &'ast ExprMatch) {
        if e.arms.len() != 2 {
            return;
        }
        let (a, b) = (&e.arms[0], &e.arms[1]);
        if a.guard.is_some() || b.guard.is_some() {
            return;
        }
        let (Some(v1), Some(v2)) = (bool_lit(&a.body), bool_lit(&b.body)) else {
            return;
        };
        let n1 = variant_name(&a.pat);
        let n2 = variant_name(&b.pat);
        let method = match (n1.as_deref(), n2.as_deref(), v1, v2) {
            (Some("Ok"), Some("Err"), true, false) | (Some("Err"), Some("Ok"), false, true) => {
                Some("is_ok")
            }
            (Some("Ok"), Some("Err"), false, true) | (Some("Err"), Some("Ok"), true, false) => {
                Some("is_err")
            }
            (Some("Some"), Some("None"), true, false)
            | (Some("None"), Some("Some"), false, true) => Some("is_some"),
            (Some("Some"), Some("None"), false, true)
            | (Some("None"), Some("Some"), true, false) => Some("is_none"),
            // 通配符形式：match x { Some(_) => true, _ => false }
            (Some("Ok"), None, true, false) => Some("is_ok"),
            (Some("Some"), None, true, false) => Some("is_some"),
            (None, Some("Ok"), false, true) => Some("is_ok"),
            (None, Some("Some"), false, true) => Some("is_some"),
            _ => None,
        };
        let Some(m) = method else {
            return;
        };
        let line = e.span().start().line;
        self.push(
            line,
            "redundant-pattern-matching",
            format!("match 手写布尔结果，可用 {m}() 替代"),
            format!("改写：x.{m}()。参考 clippy::redundant_pattern_matching（complexity）。"),
        );
    }

    // ---- 其他表达式 ----

    fn check_method_call(&mut self, mc: &'ast ExprMethodCall) {
        if self.on("unwrap-in-fallible")
            && self.fallible_depth > 0
            && self.cfg_test_depth == 0
            && (mc.method == "unwrap" || mc.method == "expect")
        {
            let line = mc.span().start().line;
            self.push(
                line,
                "unwrap-in-fallible",
                format!("返回 Result/Option 的函数里使用 .{}()，失败会 panic", mc.method),
                "用 ? 传播错误：let v = x?;（若此处确实不可能失败，可保留并书面说明）。参考 https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
            );
        }
    }

    fn check_while(&mut self, e: &'ast ExprWhile) {
        if !self.on("while-let-on-iterator") {
            return;
        }
        let Expr::Let(let_expr) = e.cond.as_ref() else {
            return;
        };
        let Expr::MethodCall(mc) = &*let_expr.expr else {
            return;
        };
        if mc.method != "next" {
            return;
        }
        let line = e.span().start().line;
        self.push(
            line,
            "while-let-on-iterator",
            "while let Some(x) = iter.next() 可改用 for 循环",
            "改写：for x in iter { .. }。参考 clippy::while_let_on_iterator（style）。",
        );
    }

    fn check_for_loop(&mut self, e: &'ast ExprForLoop) {
        if !self.on("vec-push-loop") {
            return;
        }
        if e.body.stmts.len() != 1 {
            return;
        }
        let Stmt::Expr(Expr::MethodCall(mc), Some(_)) = &e.body.stmts[0] else {
            return;
        };
        if mc.method != "push" {
            return;
        }
        let line = e.span().start().line;
        self.push(
            line,
            "vec-push-loop",
            "for 循环只做 push，可改用迭代器 + collect",
            "改写：let v: Vec<_> = source.iter().map(|x| ..).collect();",
        );
    }

    fn check_binary(&mut self, e: &'ast ExprBinary) {
        if !self.on("bool-comparison") {
            return;
        }
        let value = match (&*e.left, &*e.right) {
            (Expr::Lit(l), _) => match &l.lit {
                Lit::Bool(b) => b.value,
                _ => return,
            },
            (_, Expr::Lit(r)) => match &r.lit {
                Lit::Bool(b) => b.value,
                _ => return,
            },
            _ => return,
        };
        let line = e.span().start().line;
        match (&e.op, value) {
            (BinOp::Eq(_), true) => self.push(
                line,
                "bool-comparison",
                "x == true 可直接用 x",
                "改写：if x { .. }",
            ),
            (BinOp::Eq(_), false) => self.push(
                line,
                "bool-comparison",
                "x == false 可用 !x",
                "改写：if !x { .. }",
            ),
            (BinOp::Ne(_), true) => self.push(
                line,
                "bool-comparison",
                "x != true 可用 !x",
                "改写：if !x { .. }",
            ),
            (BinOp::Ne(_), false) => self.push(
                line,
                "bool-comparison",
                "x != false 可用 x",
                "改写：if x { .. }",
            ),
            _ => {}
        }
    }

    // ---- 函数体 ----

    fn check_needless_return(&mut self, block: &'ast Block) {
        let Some(last) = block.stmts.last() else {
            return;
        };
        // 尾表达式（无分号）是块值，不是可删除的 return 语句
        if matches!(last, Stmt::Expr(_, None)) {
            return;
        }
        let Stmt::Expr(Expr::Return(ret), Some(_)) = last else {
            return;
        };
        if ret.expr.is_none() {
            return;
        }
        let line = ret.span().start().line;
        self.push(
            line,
            "needless-return",
            "函数末尾 return 表达式可省略",
            "删除 return，让表达式成为尾表达式。参考 clippy::needless_return（style）。",
        );
    }

    fn enter_fallible(
        &mut self,
        attrs: &[syn::Attribute],
        output: &ReturnType,
        visit: impl FnOnce(&mut Self),
    ) {
        if !is_test_fn(attrs) && self.cfg_test_depth == 0 && returns_result_or_option(output) {
            self.fallible_depth += 1;
            visit(self);
            self.fallible_depth -= 1;
        } else {
            visit(self);
        }
    }
}

impl<'ast> Visit<'ast> for IdiomVisitor<'_> {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        match expr {
            Expr::If(e) => self.check_if(e),
            Expr::Match(e) => self.check_match(e),
            Expr::MethodCall(e) => self.check_method_call(e),
            Expr::While(e) => self.check_while(e),
            Expr::ForLoop(e) => self.check_for_loop(e),
            Expr::Binary(e) => self.check_binary(e),
            Expr::Closure(e) => {
                if self.on("needless-return")
                    && let Expr::Block(b) = &*e.body
                {
                    self.check_needless_return(&b.block);
                }
            }
            _ => {}
        }
        visit_expr(self, expr);
    }

    fn visit_local(&mut self, node: &'ast Local) {
        if self.on("manual-let-else") {
            self.check_manual_let_else(node);
        }
        visit_local(self, node);
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        if self.on("needless-return") {
            self.check_needless_return(&node.block);
        }
        if self.on("unwrap-in-fallible") {
            self.enter_fallible(&node.attrs, &node.sig.output, |v| visit_item_fn(v, node));
        } else {
            visit_item_fn(self, node);
        }
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        if self.on("needless-return") {
            self.check_needless_return(&node.block);
        }
        if self.on("unwrap-in-fallible") {
            self.enter_fallible(&node.attrs, &node.sig.output, |v| {
                visit_impl_item_fn(v, node)
            });
        } else {
            visit_impl_item_fn(self, node);
        }
    }

    fn visit_trait_item_fn(&mut self, node: &'ast syn::TraitItemFn) {
        if self.on("needless-return")
            && let Some(block) = &node.default
        {
            self.check_needless_return(block);
        }
        if self.on("unwrap-in-fallible") {
            self.enter_fallible(&node.attrs, &node.sig.output, |v| {
                visit_trait_item_fn(v, node)
            });
        } else {
            visit_trait_item_fn(self, node);
        }
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        let is_cfg_test = node.attrs.iter().any(crate::attrs::is_cfg_test);
        if is_cfg_test {
            self.cfg_test_depth += 1;
        }
        visit_item_mod(self, node);
        if is_cfg_test {
            self.cfg_test_depth -= 1;
        }
    }
}

impl<'a> IdiomVisitor<'a> {
    /// 手写 `let v = if let Some(v) = w { v } else { return };` → let-else。
    fn check_manual_let_else(&mut self, local: &Local) {
        let Some(init) = &local.init else {
            return;
        };
        let Expr::If(if_expr) = &*init.expr else {
            return;
        };
        let Some((_, else_expr)) = &if_expr.else_branch else {
            return;
        };
        if !is_diverging(else_expr) {
            return;
        }
        let Some(then_expr) = single_expr_of_block(&if_expr.then_branch) else {
            return;
        };
        let Expr::Path(p) = then_expr else {
            return;
        };
        let Some(ident) = p.path.get_ident() else {
            return;
        };
        let Pat::Ident(pi) = &local.pat else {
            return;
        };
        if pi.ident != *ident {
            return;
        }
        let line = local.span().start().line;
        let pat_tt = local.pat.to_token_stream();
        let init_tt = init.expr.to_token_stream();
        let else_tt = else_expr.to_token_stream();
        self.push(
            line,
            "manual-let-else",
            "手写 if let + else 返回，可改用 let-else",
            format!("改写：let {pat_tt} = {init_tt} else {{ {else_tt} }};。参考 clippy::manual_let_else（pedantic）。"),
        );
    }
}

// ---------------------------------------------------------------------------
// 语法助手

/// `if let` 条件（`while let` 的 cond 同样是 `Expr::Let`）。
fn is_let_cond(expr: &Expr) -> bool {
    matches!(expr, Expr::Let(_))
}

/// 块内唯一的表达式：尾表达式（无分号）或唯一一条表达式语句。
fn single_expr_of_block(block: &Block) -> Option<&Expr> {
    if block.stmts.len() != 1 {
        return None;
    }
    match &block.stmts[0] {
        Stmt::Expr(expr, _) => Some(expr),
        _ => None,
    }
}

/// 发散表达式（let-else 的 else 分支形态）：return / break / continue，
/// 或只含一条发散语句的块（`else { return .. }`）。
fn is_diverging(expr: &Expr) -> bool {
    match expr {
        Expr::Return(_) | Expr::Break(_) | Expr::Continue(_) => true,
        Expr::Block(b) => single_expr_of_block(&b.block).is_some_and(is_diverging),
        _ => false,
    }
}

fn is_path_ident(expr: &Expr, ident: &str) -> bool {
    matches!(expr, Expr::Path(p) if p.path.get_ident().is_some_and(|i| i == ident))
}

fn bool_lit(expr: &Expr) -> Option<bool> {
    match expr {
        Expr::Lit(l) => match &l.lit {
            Lit::Bool(b) => Some(b.value),
            _ => None,
        },
        _ => None,
    }
}

/// 元组结构体模式 `Ok(v)` / `Some(v)` / `Err(e)` 中的绑定标识符。
fn ctor_binding<'a>(pat: &'a Pat, ctor: &str) -> Option<&'a syn::Ident> {
    let syn::Pat::TupleStruct(pts) = pat else {
        return None;
    };
    if pts.path.get_ident().is_none_or(|i| i != ctor) || pts.elems.len() != 1 {
        return None;
    }
    let syn::Pat::Ident(pi) = &pts.elems[0] else {
        return None;
    };
    (pi.subpat.is_none()).then_some(&pi.ident)
}

/// `return Err(e)` / `return Err(e.into())`。
fn returns_ctor_with_binding(expr: &Expr, ctor: &str, ident: &syn::Ident) -> bool {
    let Expr::Return(ret) = expr else {
        return false;
    };
    let Some(ret_expr) = &ret.expr else {
        return false;
    };
    let Expr::Call(call) = &**ret_expr else {
        return false;
    };
    let Expr::Path(callee) = &*call.func else {
        return false;
    };
    if callee.path.get_ident().is_none_or(|i| i != ctor) || call.args.len() != 1 {
        return false;
    }
    let arg = call.args.first().expect("len checked");
    if is_path_ident(arg, &ident.to_string()) {
        return true;
    }
    // e.into()
    matches!(
        arg,
        Expr::MethodCall(mc) if mc.method == "into" && is_path_ident(&mc.receiver, &ident.to_string())
    )
}

fn pat_is_none(pat: &Pat) -> bool {
    matches!(pat, Pat::Path(pp) if pp.path.get_ident().is_some_and(|i| i == "None"))
}

fn returns_none(expr: &Expr) -> bool {
    matches!(expr, Expr::Return(ret) if ret.expr.as_deref().is_some_and(|e| is_path_ident(e, "None")))
}

/// 模式对应的变体名（`Ok`/`Err`/`Some`/`None`），通配符为 None。
fn variant_name(pat: &Pat) -> Option<String> {
    match pat {
        Pat::TupleStruct(pts) => pts.path.get_ident().map(|i| i.to_string()),
        Pat::Path(pp) => pp.path.get_ident().map(|i| i.to_string()),
        _ => None,
    }
}

fn is_test_fn(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| a.path().is_ident("test"))
}

/// 返回类型是否为 `Result<..>` / `Option<..>`（只看最后一个路径段）。
fn returns_result_or_option(output: &ReturnType) -> bool {
    let ReturnType::Type(_, ty) = output else {
        return false;
    };
    let Type::Path(tp) = &**ty else {
        return false;
    };
    tp.path
        .segments
        .last()
        .is_some_and(|s| s.ident == "Result" || s.ident == "Option")
}

/// 归一化 token 文本（去空白），用于方法 receiver 的粗略相等比较。
fn normalize_tt(ts: &proc_macro2::TokenStream) -> String {
    ts.to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

/// 表达式树里是否存在 `receiver.method(...)`（receiver 文本粗略相等）。
fn expr_contains_method(expr: &Expr, method: &str, recv: &str) -> bool {
    let mut f = MethodFinder {
        method,
        recv,
        found: false,
    };
    visit_expr(&mut f, expr);
    f.found
}

fn block_contains_method(block: &Block, method: &str, recv: &str) -> bool {
    let mut f = MethodFinder {
        method,
        recv,
        found: false,
    };
    syn::visit::visit_block(&mut f, block);
    f.found
}

struct MethodFinder<'a> {
    method: &'a str,
    recv: &'a str,
    found: bool,
}

impl<'ast> Visit<'ast> for MethodFinder<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        let r = normalize_tt(&node.receiver.to_token_stream());
        if node.method == self.method
            && !r.is_empty()
            && (r == self.recv || self.recv.contains(&r) || r.contains(self.recv))
        {
            self.found = true;
        }
        visit_expr_method_call(self, node);
    }
}
