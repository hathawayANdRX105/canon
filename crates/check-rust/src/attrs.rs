//! syn 属性扫描助手：AST 遍历替代文本行匹配。
//!
//! 字符串字面量、注释、doc-comment 里的 `#[allow(dead_code)]` / `#[cfg(test)]`
//! 文案不会成为属性节点，天然免疫误报（T10 取证遗留 FP 的修正）。
//! 行号取 proc-macro2 span-locations（syn 2 + span-locations）。

use syn::File;
use syn::spanned::Spanned;
use syn::visit::Visit;

/// 一次属性命中：1-based 行号 + 属性源码区间。
pub struct AttrHit {
    /// 属性起始行（1-based）。
    pub line: usize,
    /// 属性结束行（1-based；多行属性豁免判定用）。
    end_line: usize,
    /// 属性结束列（0-based 字节偏移；同行尾注豁免判定用）。
    end_col: usize,
    /// 属性归一化源文本（token 流，用于 finding 消息）。
    pub source: String,
}

impl AttrHit {
    /// 属性结束位置同一行是否带尾注 `//`（canon 口径的保留理由豁免）。
    pub fn trailing_comment(&self, source: &str) -> bool {
        let Some(line) = source.lines().nth(self.end_line.saturating_sub(1)) else {
            return false;
        };
        line.chars()
            .skip(self.end_col)
            .collect::<String>()
            .contains("//")
    }
}

/// 解析 Rust 源码；语法错误返回 None（调用方跳过，clippy 兜底）。
pub fn parse(source: &str) -> Option<File> {
    syn::parse_file(source).ok()
}

/// 收集满足 `pred` 的属性命中（含嵌套 item 的属性；宏体内的 token 不解析）。
pub fn scan(file: &File, pred: &dyn Fn(&syn::Attribute) -> bool) -> Vec<AttrHit> {
    let mut visitor = AttrVisitor {
        hits: Vec::new(),
        pred,
    };
    visitor.visit_file(file);
    visitor.hits
}

/// `#[allow(...)]` 且嵌套项含裸路径 `dead_code`（多字段变体命中，
/// `clippy::dead_code` 等跨路径不命中——与旧文本口径一致）。
pub fn is_allow_dead_code(attr: &syn::Attribute) -> bool {
    attr.path().is_ident("allow") && nested_paths(attr).iter().any(|p| p.is_ident("dead_code"))
}

/// `#[cfg(test)]`（精确路径；`cfg_attr` / `cfg(all(..))` 不命中——旧口径亦然）。
pub fn is_cfg_test(attr: &syn::Attribute) -> bool {
    attr.path().is_ident("cfg") && nested_paths(attr).iter().any(|p| p.is_ident("test"))
}

/// 解析属性括号内的顶层项路径（`allow(dead_code, foo)` → `[dead_code, foo]`）。
fn nested_paths(attr: &syn::Attribute) -> Vec<syn::Path> {
    let Ok(nested) = attr.parse_args_with(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
    ) else {
        return Vec::new();
    };
    nested
        .iter()
        .filter(|m| matches!(m, syn::Meta::Path(_)))
        .map(syn::Meta::path)
        .cloned()
        .collect()
}

struct AttrVisitor<'a> {
    hits: Vec<AttrHit>,
    pred: &'a dyn Fn(&syn::Attribute) -> bool,
}

impl<'ast> Visit<'ast> for AttrVisitor<'ast> {
    fn visit_attribute(&mut self, attr: &'ast syn::Attribute) {
        if (self.pred)(attr) {
            let start = attr.span().start();
            let end = attr.span().end();
            self.hits.push(AttrHit {
                line: start.line,
                end_line: end.line,
                end_col: end.column,
                source: quote::ToTokens::to_token_stream(attr).to_string(),
            });
        }
        // 属性不嵌套属性，无需递归。
    }
}

// ---------------------------------------------------------------------------
// 字符串字面量区间：文本类规则过滤「字面量里的文案」用。

/// 一个字符串字面量的行/列覆盖区间（1-based 行、0-based 列）。
pub struct StrLitSpan {
    start_line: usize,
    start_col: usize,
    end_line: usize,
    end_col: usize,
}

impl StrLitSpan {
    /// (line, col) 是否落在字面量区间内。
    pub fn contains(&self, line: usize, col: usize) -> bool {
        let after_start =
            line > self.start_line || (line == self.start_line && col >= self.start_col);
        let before_end = line < self.end_line || (line == self.end_line && col <= self.end_col);
        after_start && before_end
    }
}

/// 收集文件内全部字符串字面量的区间（visit LitStr；多行/原始串均覆盖）。
pub fn str_lit_spans(file: &File) -> Vec<StrLitSpan> {
    let mut v = LitVisitor {
        spans: Vec::new(),
        _marker: Default::default(),
    };
    v.visit_file(file);
    v.spans
}

struct LitVisitor<'ast> {
    spans: Vec<StrLitSpan>,
    _marker: std::marker::PhantomData<&'ast ()>,
}

impl<'ast> Visit<'ast> for LitVisitor<'ast> {
    fn visit_lit_str(&mut self, lit: &'ast syn::LitStr) {
        let start = lit.span().start();
        let end = lit.span().end();
        self.spans.push(StrLitSpan {
            start_line: start.line,
            start_col: start.column,
            end_line: end.line,
            end_col: end.column,
        });
    }
}
