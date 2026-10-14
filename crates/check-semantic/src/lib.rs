//! 语义类规则：jev / laya 决策端点 + 类型化问题 DSL + 阈值判定。
//!
//! 原则（plan D5）：端点未配置 / 不可达 → `notChecked`（报告显式声明「没查什么」），
//! **绝不伪装 finding，也绝不假绿**。判定链与 canon `jev_rule` 同口径：
//! 候选由确定性检查出，语义层只判「意思」；低于 warn 的判决直接丢弃。

pub mod client;
pub mod kind;
pub mod questions;

/// 成员仓 bin 组装用：本域静态规则（当前无——语义规则一律是参数化实例）。
pub fn rules() -> Vec<Box<dyn check_core::rule::Rule>> {
    Vec::new()
}

/// 成员仓 bin 组装用：本域参数化 kind。
pub fn factories() -> Vec<Box<dyn check_core::kinds::KindFactory>> {
    vec![Box::new(kind::SemanticQuestionFactory)]
}
