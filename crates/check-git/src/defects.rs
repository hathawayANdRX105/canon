//! 缺陷历史：subject 命中修复关键词的提交按文件聚合——「这个文件历史上修过 N 次 bug」。
//! SZZ-lite（只挖 commit message，不做行级 blame 追溯）：research 显示完整 SZZ
//! ~25% 误链，行级判定属语义层；本规则只出「改动时重点回归」的背景信息。

use serde::Deserialize;

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

use crate::history::{self, DEFAULT_FIX_KEYWORDS};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DefectsParams {
    /// 统计窗口（git 相对时间）。
    #[serde(default = "default_since")]
    pub since: String,
    /// 修复提交数下限。
    #[serde(default = "default_min_fixes")]
    pub min_fixes: usize,
    /// 最多报几条（按次数降序）。
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_since() -> String {
    "12 months ago".into()
}

fn default_min_fixes() -> usize {
    3
}

fn default_limit() -> usize {
    10
}

pub struct DefectsFactory;

impl check_core::kinds::KindFactory for DefectsFactory {
    fn kind(&self) -> &'static str {
        "git-defects"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: DefectsParams = Deserialize::deserialize(toml::Value::Table(params.clone()))
            .map_err(|e| format!("bad params: {e}"))?;
        Ok(Box::new(Defects {
            id: format!("GIT.defects.{name}"),
            params: parsed,
        }))
    }
}

pub struct Defects {
    id: String,
    params: DefectsParams,
}

impl Rule for Defects {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Advisory
    }

    fn title(&self) -> &str {
        "缺陷历史（修复提交密度）"
    }

    fn doc(&self) -> &str {
        "git log 统计窗口内 subject 命中修复关键词（fix/bug/revert/修复/回滚…）的\
         提交按文件聚合，≥ min_fixes 按次数降序报 limit 条。含义：该文件历史上\
         缺陷密度高，再改动时重点回归。SZZ-lite：只看 message，不做行级追溯\
         （完整 SZZ ~25% 误链，行级判定留语义层）。\
         参数见 checks.toml 的 [kinds.git-defects.<name>]。"
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
        let counts = history::defect_counts(&commits, DEFAULT_FIX_KEYWORDS);
        let in_scope = crate::in_scope_paths(ctx);
        let mut hits: Vec<(&String, usize)> = in_scope
            .iter()
            .filter_map(|rel| Some((rel, *counts.get(rel)?)))
            .filter(|(_, n)| *n >= self.params.min_fixes)
            .collect();
        hits.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        hits.iter()
            .take(self.params.limit)
            .map(|(rel, n)| {
                Finding::new(
                    self.id.as_str(),
                    Severity::Warn,
                    (*rel).clone(),
                    0,
                    format!("缺陷历史：窗口内 {n} 次修复提交——再改动时重点回归"),
                )
            })
            .collect()
    }
}
