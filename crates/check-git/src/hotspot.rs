//! git 历史信号类规则：热点、变更耦合、缺陷历史（Crime Scene 学派的 advisory 实现）。
//!
//! 全部 `Class::Advisory`——统计信号没有对错，只做可疑信息，不做拦截；
//! 成员仓在 checks.toml 以 kind 实例化（D9：TOML 只做参数），并可用
//! `[rules."GIT.<id>"] scope_include/exclude` 收口生成物等非纪律面。
//! git 不可用 / 非 git 仓 → 情报缺失静默（advisory 不产伪 finding）。

use serde::Deserialize;

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

use crate::history;

// ---------------------------------------------------------------------------
// git-hotspot：变更频率 × 文件行数 的可疑热点（优先级提示，不是对错判定）。

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HotspotParams {
    /// 统计窗口（git 相对时间，如 "6 months ago"）。
    #[serde(default = "default_since")]
    pub since: String,
    /// 变更次数下限（低于此不参评）。
    #[serde(default = "default_min_churn")]
    pub min_churn: usize,
    /// 最多报几条（按分数降序）。
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_since() -> String {
    "6 months ago".into()
}

fn default_min_churn() -> usize {
    5
}

fn default_limit() -> usize {
    10
}

pub struct HotspotFactory;

impl check_core::kinds::KindFactory for HotspotFactory {
    fn kind(&self) -> &'static str {
        "git-hotspot"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: HotspotParams = Deserialize::deserialize(toml::Value::Table(params.clone()))
            .map_err(|e| format!("bad params: {e}"))?;
        Ok(Box::new(Hotspot {
            id: format!("GIT.hotspot.{name}"),
            params: parsed,
        }))
    }
}

pub struct Hotspot {
    id: String,
    params: HotspotParams,
}

impl Rule for Hotspot {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Advisory
    }

    fn title(&self) -> &str {
        "变更热点（churn × 规模）"
    }

    fn doc(&self) -> &str {
        "git log 统计窗口内每文件的变更次数 × 文件行数得热度分，\
         churn ≥ min_churn 的文件按分降序报 limit 条。热点≠错误：\
         含义是「改动最频繁且体量大」——排整改优先级（all 口径）、\
         提示「你正在改的就是热点」（changed 口径）。scope 过滤走 \
         checks.toml 覆盖链（scope_exclude 收口生成物）。\
         参数见 checks.toml 的 [kinds.git-hotspot.<name>]。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        if !ctx.root().join(".git").exists() {
            return Vec::new();
        }
        let Some(commits) = history::load(ctx.root(), &self.params.since) else {
            return Vec::new();
        };
        let churn = history::churn(&commits);
        let in_scope = crate::in_scope_paths(ctx);
        let mut scored: Vec<(usize, u64, &String)> = in_scope
            .iter()
            .filter_map(|rel| {
                let churn = *churn.get(rel)?;
                if churn < self.params.min_churn {
                    return None;
                }
                let loc = ctx
                    .read(std::path::Path::new(rel))
                    .map(|t| t.lines().count())
                    .unwrap_or(0) as u64;
                if loc == 0 {
                    return None;
                }
                Some((churn, churn as u64 * loc, rel))
            })
            .collect();
        scored.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.cmp(b.2)));
        scored
            .iter()
            .take(self.params.limit)
            .map(|(churn, score, rel)| {
                Finding::new(
                    self.id.as_str(),
                    Severity::Warn,
                    (*rel).clone(),
                    0,
                    format!("变更热点：{churn} 次变更 × {score} 分（整改优先级参考，非错误）"),
                )
            })
            .collect()
    }
}
