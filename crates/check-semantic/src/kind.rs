//! `semantic-question` kind：意图条文 + 类型化问题集；逐文件调用决策端点。
//!
//! 端点/网络失败 → `notChecked`（声明「没查什么」），绝不伪装 finding、也不假绿。
//! 问题集与阈值语义见 [`crate::questions`]；wire 协议见 [`crate::client`]。

use std::collections::BTreeMap;
use std::path::Path;

use globset::GlobSet;
use serde::Deserialize;

use crate::client::{Ask, resolve_key};
use crate::questions::{QuestionSpec, Verdict, evaluate, to_wire};

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, NotChecked, RuleOutcome, Severity};
use check_core::rule::Rule;

pub struct SemanticQuestionFactory;

impl check_core::kinds::KindFactory for SemanticQuestionFactory {
    fn kind(&self) -> &'static str {
        "semantic-question"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: SemanticParams = Deserialize::deserialize(toml::Value::Table(params.clone()))
            .map_err(|e| format!("bad params: {e}"))?;
        if parsed.questions.is_empty() {
            return Err("questions 不能为空（至少一个问句）".to_string());
        }
        let include = compile_globs(&parsed.paths_include)?;
        let exclude = compile_globs(&parsed.paths_exclude)?;
        Ok(Box::new(SemanticQuestionRule {
            id: format!("SEMANTIC.{name}"),
            intent: parsed.intent,
            include,
            exclude,
            context_files: parsed.context_files,
            questions: parsed.questions,
            fail: parsed.fail,
            warn: parsed.warn,
            max_state_chars: parsed.max_state_chars,
            message: parsed.message,
        }))
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SemanticParams {
    /// 规则条文——原样进 state 的 RULE INTENT（写成给新同事看的条文）。
    intent: String,
    #[serde(default)]
    paths_include: Vec<String>,
    #[serde(default)]
    paths_exclude: Vec<String>,
    /// 仓内规范文档（相对仓库根），内联进 state（每个截 8k 字符）。
    #[serde(default)]
    context_files: Vec<String>,
    /// qid → 问题定义。
    questions: BTreeMap<String, QuestionSpec>,
    /// 规则级阈值缺省（问题级 fail/warn 可覆盖）。
    #[serde(default = "default_fail")]
    fail: f64,
    #[serde(default = "default_warn")]
    warn: f64,
    /// 单文件 state 上限（字符；超出截断并标注）。
    #[serde(default = "default_state_cap")]
    max_state_chars: usize,
    #[serde(default)]
    message: String,
}

fn default_fail() -> f64 {
    0.85
}

fn default_warn() -> f64 {
    0.6
}

fn default_state_cap() -> usize {
    16_000
}

fn compile_globs(patterns: &[String]) -> Result<GlobSet, String> {
    let mut builder = globset::GlobSet::builder();
    for pattern in patterns {
        let glob = globset::GlobBuilder::new(pattern)
            .literal_separator(true)
            .build()
            .map_err(|e| format!("bad glob {pattern:?}: {e}"))?;
        builder.add(glob);
    }
    builder.build().map_err(|e| format!("bad globs: {e}"))
}

struct SemanticQuestionRule {
    id: String,
    intent: String,
    include: GlobSet,
    exclude: GlobSet,
    context_files: Vec<String>,
    questions: BTreeMap<String, QuestionSpec>,
    fail: f64,
    warn: f64,
    max_state_chars: usize,
    message: String,
}

impl Rule for SemanticQuestionRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Semantic
    }

    fn title(&self) -> &str {
        "语义判决"
    }

    fn doc(&self) -> &str {
        "意图条文 + 类型化问题集（noul/choice/score），逐文件调用决策端点；\
         过阈值的判定出 finding，低于 warn 丢弃；端点不可达 → notChecked。\
         问题集见 checks.toml 的 [kinds.semantic-question.<name>.questions.<qid>]。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    /// 语义类统一走 run_outcome（run 留空；引擎不直接调用）。
    fn run(&self, _ctx: &Ctx) -> Vec<Finding> {
        Vec::new()
    }

    fn run_outcome(&self, ctx: &Ctx) -> RuleOutcome {
        let mut outcome = RuleOutcome::default();
        let Some(endpoint) = ctx.semantic() else {
            outcome.not_checked.push(NotChecked {
                id: self.id.clone(),
                reason: "semantic endpoint not configured".to_string(),
            });
            return outcome;
        };
        let targets: Vec<&std::path::PathBuf> = ctx
            .scope_files()
            .iter()
            .filter(|rel| self.applies(rel.as_path()))
            .collect();
        if targets.is_empty() {
            return outcome;
        }
        let key = resolve_key();
        let wire = to_wire(&self.questions);
        let mut failures = 0usize;
        let mut first_error = String::new();
        for rel in targets {
            let Some(state) = self.build_state(rel, ctx) else {
                continue;
            };
            let ask = Ask::new(endpoint.endpoint.clone(), state, wire.clone())
                .key(key.clone())
                .model(endpoint.model.clone());
            match ask.send() {
                Ok(answers) => {
                    for verdict in evaluate(&self.questions, &answers, self.fail, self.warn) {
                        outcome.findings.push(self.finding(rel, &verdict));
                    }
                }
                Err(error) => {
                    failures += 1;
                    if first_error.is_empty() {
                        first_error = format!("{}: {error}", rel.display());
                    }
                }
            }
        }
        if failures > 0 {
            outcome.not_checked.push(NotChecked {
                id: self.id.clone(),
                reason: format!("{failures} 个文件判定失败（端点/网络）：{first_error}"),
            });
        }
        outcome
    }
}

impl SemanticQuestionRule {
    fn applies(&self, rel: &Path) -> bool {
        (self.include.is_empty() || self.include.is_match(rel)) && !self.exclude.is_match(rel)
    }

    /// 一条 state：RULE INTENT + 上下文文档 + 文件全文（截断）。
    fn build_state(&self, rel: &Path, ctx: &Ctx) -> Option<String> {
        let content = ctx.read(rel).ok()?;
        let mut parts = vec![format!("RULE INTENT:\n{}", self.intent)];
        for cf in &self.context_files {
            let body = ctx
                .read(Path::new(cf))
                .unwrap_or_else(|_| "<missing>".to_string());
            let clipped: String = body.chars().take(8_000).collect();
            parts.push(format!("PROJECT SPEC CONTEXT ({cf}):\n{clipped}"));
        }
        let body: String = if content.chars().count() > self.max_state_chars {
            let clipped: String = content.chars().take(self.max_state_chars).collect();
            format!("{clipped}\n…[truncated]")
        } else {
            content
        };
        parts.push(format!("FILE CONTENT ({}):\n{body}", rel.display()));
        Some(parts.join("\n\n"))
    }

    fn finding(&self, rel: &Path, verdict: &Verdict) -> Finding {
        let lead = if self.message.is_empty() {
            "语义判决"
        } else {
            self.message.as_str()
        };
        let mut finding = Finding::new(
            self.id.as_str(),
            verdict.severity,
            rel.display().to_string(),
            0,
            format!(
                "{lead} [{}] p={:.2}（阈值 fail≥{:.2} / warn≥{:.2}）",
                verdict.qid, verdict.value, self.fail, self.warn
            ),
        );
        if let Some(q) = self.questions.get(&verdict.qid)
            && !q.instructions.is_empty()
        {
            finding = finding.with_suggestion(q.instructions.clone());
        }
        finding
    }
}
