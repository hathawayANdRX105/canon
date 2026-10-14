//! rsx 源的解析入口：文本 → syn File → 全部 `rsx!` CallBody。

use dioxus_rsx::CallBody;
use syn::visit::Visit;

/// 解析失败的文件返回 None——容忍既有债，不产出伪 finding 也不中断其余文件。
pub fn parse_rsx_calls(text: &str) -> Option<Vec<CallBody>> {
    let file = syn::parse_file(text).ok()?;
    let mut visitor = RsxVisitor { calls: Vec::new() };
    visitor.visit_file(&file);
    Some(visitor.calls)
}

struct RsxVisitor {
    calls: Vec<CallBody>,
}

impl<'ast> Visit<'ast> for RsxVisitor {
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if mac.path.is_ident("rsx")
            && let Ok(body) = syn::parse2::<CallBody>(mac.tokens.clone())
        {
            self.calls.push(body);
        }
        syn::visit::visit_macro(self, mac);
    }
}
