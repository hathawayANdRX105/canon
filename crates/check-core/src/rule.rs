//! 规则 trait：机器目录（meta）与执行（run）两个面。

use crate::context::Ctx;
use crate::finding::{Class, Finding, RuleOutcome, Severity};

pub trait Rule: Send + Sync {
    /// 规则 id，形如 `TEXT-PATTERN.no-dbg`（kind 实例）或 `RUST.process-cmd`。
    fn id(&self) -> &str;

    fn class(&self) -> Class;

    /// 一行标题（list 输出）。
    fn title(&self) -> &str;

    /// 规则条文；`explain` 的输出，semantic 类同时作为进模型的 intent。
    fn doc(&self) -> &str;

    /// checks.toml 未覆盖时的建议严重度（文档性质，生效值走覆盖链）。
    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    /// 对当前口径的文件集执行检查。文件遍历口径由规则自行决定：
    /// 逐文件规则用 [`Ctx::scope_files`]，全仓不变量用 [`Ctx::tracked_files`]。
    fn run(&self, ctx: &Ctx) -> Vec<Finding>;

    /// 与 [`Rule::run`] 同义，但可额外报告「无法判定的子项」（如语义端点
    /// 不可达 → notChecked）。引擎走本方法；默认实现只返回 findings。
    fn run_outcome(&self, ctx: &Ctx) -> RuleOutcome {
        RuleOutcome::findings(self.run(ctx))
    }
}
