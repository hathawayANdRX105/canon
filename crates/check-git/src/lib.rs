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

/// scope 内文件的相对路径集（checks.toml 覆盖链过滤后；被删文件自然剔除）。
/// all 口径 = 全部跟踪文件；changed 口径 = 本次改动文件。
pub(crate) fn in_scope_paths(ctx: &Ctx) -> Vec<String> {
    ctx.scope_files()
        .iter()
        .map(|p| p.display().to_string())
        .collect()
}

/// 同上的集合形态（耦合对「任一侧命中」判定用）。
pub(crate) fn in_scope_paths_set(ctx: &Ctx) -> std::collections::BTreeSet<String> {
    in_scope_paths(ctx).into_iter().collect()
}

/// 成员仓 bin 组装用：本域的参数化 kind（checks.toml 实例化）。
pub fn factories() -> Vec<Box<dyn check_core::kinds::KindFactory>> {
    vec![
        Box::new(hotspot::HotspotFactory),
        Box::new(coupling::CouplingFactory),
        Box::new(defects::DefectsFactory),
    ]
}
