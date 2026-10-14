//! 类型化问题集与阈值判定（纯函数，可单测）。
//!
//! 语义与 canon `jev_rule.py` 对齐：
//! - noul：答案 = 概率；p ≥ fail → FAIL，p ≥ warn → WARN，否则丢弃（假阳性过滤器）。
//! - score：答案 = 分值（同一对阈值，量纲由问题定义方掌握）。
//! - choice：答案 = 标签；`fail_labels` 各标签概率之和过阈值才算。

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

use check_core::finding::Severity;

/// 单个问题的定义（TOML 参数）。
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestionSpec {
    /// noul | score | choice。
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub instructions: String,
    /// noul：{true, false}；choice：{label: 说明}；score：["档位说明", …]。
    #[serde(default)]
    pub criteria: Value,
    /// 问题级阈值（覆盖 kind 级缺省）。
    #[serde(default)]
    pub fail: Option<f64>,
    #[serde(default)]
    pub warn: Option<f64>,
    /// choice 专用：哪些标签算违规。
    #[serde(default)]
    pub fail_labels: Vec<String>,
}

/// 一条问题的判定结果。
#[derive(Debug, Clone)]
pub struct Verdict {
    pub qid: String,
    pub severity: Severity,
    /// noul/score = 分值；choice = fail_labels 概率和。
    pub value: f64,
}

/// 把问题集编成 wire 形态的 JSON（进 `questions` 字段）。
pub fn to_wire(questions: &BTreeMap<String, QuestionSpec>) -> Value {
    let mut out = serde_json::Map::new();
    for (qid, q) in questions {
        let mut item = serde_json::Map::new();
        item.insert("type".into(), Value::String(q.kind.clone()));
        if !q.instructions.is_empty() {
            item.insert("instructions".into(), Value::String(q.instructions.clone()));
        }
        if !q.criteria.is_null() {
            item.insert("criteria".into(), q.criteria.clone());
        }
        out.insert(qid.clone(), Value::Object(item));
    }
    Value::Object(out)
}

/// 阈值判定：answers（端点返回的 answers 节点）→ 过阈值的判定列表。
pub fn evaluate(
    questions: &BTreeMap<String, QuestionSpec>,
    answers: &Value,
    rule_fail: f64,
    rule_warn: f64,
) -> Vec<Verdict> {
    let mut verdicts = Vec::new();
    for (qid, spec) in questions {
        let Some(ans) = answers.get(qid) else {
            continue;
        };
        let fail = spec.fail.unwrap_or(rule_fail);
        let warn = spec.warn.unwrap_or(rule_warn);
        let Some((value, severity)) = score_one(spec, ans, fail, warn) else {
            continue;
        };
        if severity == Severity::Info {
            continue; // 低于 warn：丢弃（假阳性过滤器，与 jev_rule 同口径）
        }
        verdicts.push(Verdict {
            qid: qid.clone(),
            severity,
            value,
        });
    }
    verdicts
}

/// (分值, 严重度)；低于 warn 返回 Info 由调用方丢弃。
fn score_one(spec: &QuestionSpec, ans: &Value, fail: f64, warn: f64) -> Option<(f64, Severity)> {
    let value = match spec.kind.as_str() {
        "noul" => ans.get("noul")?.as_f64()?,
        "score" => ans.get("score")?.as_f64()?,
        "choice" => {
            let probs = ans.get("probabilities").and_then(Value::as_object)?;
            spec.fail_labels
                .iter()
                .filter_map(|label| probs.get(label).and_then(Value::as_f64))
                .sum()
        }
        _ => return None,
    };
    let severity = if value >= fail {
        Severity::Fail
    } else if value >= warn {
        Severity::Warn
    } else {
        Severity::Info
    };
    Some((value, severity))
}
