//! 前端资产类规则：CSS 结构不变量（首切片 = theme token 镜像）。
//!
//! 只做「块级不变量」这类确定性判定；颜色语义、样式归属等需要理解意图的
//! 判断留给语义层（jev/laya，端点可用时）。解析器是行级块扫描——与既有
//! `dioxus_css_mirror` shell harness 同口径（列首选择器 + 列首 `}` 收块）。

pub mod css_blocks;
pub mod var_mirror;

/// 成员仓 bin 组装用：本域静态规则（当前无）。
pub fn rules() -> Vec<Box<dyn check_core::rule::Rule>> {
    Vec::new()
}

/// 成员仓 bin 组装用：本域参数化 kind（checks.toml 实例化）。
pub fn factories() -> Vec<Box<dyn check_core::kinds::KindFactory>> {
    vec![Box::new(var_mirror::CssVarMirrorFactory)]
}
