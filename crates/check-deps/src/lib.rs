//! 依赖图谱类规则：域边界（cargo metadata 路径依赖）与外部工具翻译。

pub mod dep_edge;
pub mod dep_tag;
pub mod machete;
pub mod metadata;

pub use dep_edge::DepEdgeFactory;

use check_core::rule::Rule;

/// 成员仓 bin 组装用：本域全部静态规则。
pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![Box::new(machete::UnusedDeps), Box::new(dep_tag::GitDepPin)]
}

/// 成员仓 bin 组装用：本域的参数化 kind（checks.toml 实例化）。
pub fn factories() -> Vec<Box<dyn check_core::kinds::KindFactory>> {
    vec![Box::new(dep_edge::DepEdgeFactory)]
}
