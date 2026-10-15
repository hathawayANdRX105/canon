//! 时序耦合：同提交共现 ≥ min 次的文件对 →「隐性契约 / 该合并 / 改一必改二」提示。
//! 对应 code-maat 的 coupling summation；Tornhill：coupling 是架构侵蚀的先行信号。

use serde::Deserialize;

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

use crate::history;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CouplingParams {
    /// 统计窗口（git 相对时间）。
    #[serde(default = "default_since")]
    pub since: String,
    /// 共现次数下限。
    #[serde(default = "default_min_cochanges")]
    pub min_cochanges: usize,
    /// 最多报几条（按共现次数降序）。
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_since() -> String {
    "6 months ago".into()
}

fn default_min_cochanges() -> usize {
    4
}

fn default_limit() -> usize {
    10
}

pub struct CouplingFactory;

impl check_core::kinds::KindFactory for CouplingFactory {
    fn kind(&self) -> &'static str {
        "git-coupling"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: CouplingParams = Deserialize::deserialize(toml::Value::Table(params.clone()))
            .map_err(|e| format!("bad params: {e}"))?;
        Ok(Box::new(Coupling {
            id: format!("GIT.coupling.{name}"),
            params: parsed,
        }))
    }
}

pub struct Coupling {
    id: String,
    params: CouplingParams,
}

impl Rule for Coupling {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Advisory
    }

    fn title(&self) -> &str {
        "时序耦合（改一必改二）"
    }

    fn doc(&self) -> &str {
        "git log 统计窗口内同提交共现 ≥ min_cochanges 的文件对，按次数降序报 \
         limit 条。含义：两个模块总是一起改——要么隐性契约该显式化，要么该合并，\
         要么改动在跨层扩散。提示性信号，非错误。\
         参数见 checks.toml 的 [kinds.git-coupling.<name>]。"
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
        // 共现统计/排序统一走 history::coupling（避免与历史模块双实现）
        let hits = history::coupling(&commits, self.params.min_cochanges);
        hits.iter()
            .take(self.params.limit)
            .map(|((a, b), n)| {
                Finding::new(
                    self.id.as_str(),
                    Severity::Warn,
                    a.clone(),
                    0,
                    format!("时序耦合：与 {b} 近 {n} 次同提交变更——隐性契约？该合并？"),
                )
            })
            .collect()
    }
}
