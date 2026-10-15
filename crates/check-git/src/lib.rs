//! git 历史信号类规则：热点、变更耦合、缺陷历史（Crime Scene 学派）。
//!
//! 全部 Class::Advisory——统计信号只做可疑信息，不做拦截（用户拍板）；
//! 成员仓 checks.toml 以 kind 实例化（D9），scope/severity 走覆盖链。
//! 依赖零新增：git 经 std::process::Command（与 check-deps 的 cargo 调用同款）。

use check_core::context::Ctx;

pub mod coupling;
pub mod defects;
pub mod history;
pub mod hotspot;

/// 跟踪文件的相对路径集（历史口径过滤：被删文件自然剔除）。
pub(crate) fn tracked_rel_paths(ctx: &Ctx) -> Vec<String> {
    ctx.tracked_files()
        .iter()
        .map(|p| p.display().to_string())
        .collect()
}

/// 成员仓 bin 组装用：本域的参数化 kind（checks.toml 实例化）。
pub fn factories() -> Vec<Box<dyn check_core::kinds::KindFactory>> {
    vec![
        Box::new(hotspot::HotspotFactory),
        Box::new(coupling::CouplingFactory),
        Box::new(defects::DefectsFactory),
    ]
}
