//! check-semantic 测试：阈值判定（纯函数）、端点不可达的 notChecked 语义。
//!
//! 全部测试不依赖 live 端点（用必然拒绝连接的 127.0.0.1:1），CI 可直接跑。

use check_core::config::{ChecksConfig, SemanticEndpoint};
use check_core::context::Ctx;
use check_core::finding::Severity;
use check_core::registry::{Registry, RunOptions};
use check_semantic::client::Ask;
use check_semantic::questions::{QuestionSpec, evaluate};
use serde_json::json;
use std::collections::BTreeMap;
use std::path::Path;

/// 必然拒绝连接的端点（无人监听；CI 与本地行为一致）。
const DEAD_ENDPOINT: &str = "http://127.0.0.1:1";

fn spec(kind: &str, fail: Option<f64>, warn: Option<f64>, fail_labels: &[&str]) -> QuestionSpec {
    QuestionSpec {
        kind: kind.to_string(),
        instructions: "q".to_string(),
        criteria: json!(null),
        fail,
        warn,
        fail_labels: fail_labels.iter().map(|s| s.to_string()).collect(),
    }
}

fn questions(items: &[(&str, QuestionSpec)]) -> BTreeMap<String, QuestionSpec> {
    items
        .iter()
        .map(|(id, q)| (id.to_string(), q.clone()))
        .collect()
}

#[test]
fn noul_thresholds_fail_warn_drop() {
    let qs = questions(&[
        ("block", spec("noul", None, None, &[])),
        ("drop", spec("noul", None, None, &[])),
    ]);
    let answers = json!({
        "block": {"type": "noul", "noul": 0.92},
        "drop": {"type": "noul", "noul": 0.10},
    });
    let verdicts = evaluate(&qs, &answers, 0.85, 0.6);
    assert_eq!(verdicts.len(), 1, "{verdicts:?}");
    assert_eq!(verdicts[0].qid, "block");
    assert_eq!(verdicts[0].severity, Severity::Fail);
    assert!((verdicts[0].value - 0.92).abs() < 1e-9);
}

#[test]
fn noul_between_thresholds_is_warn() {
    let qs = questions(&[("mid", spec("noul", None, None, &[]))]);
    let answers = json!({"mid": {"type": "noul", "noul": 0.7}});
    let verdicts = evaluate(&qs, &answers, 0.85, 0.6);
    assert_eq!(verdicts.len(), 1);
    assert_eq!(verdicts[0].severity, Severity::Warn);
}

#[test]
fn choice_uses_fail_labels_probability_sum() {
    let qs = questions(&[("bucket", spec("choice", None, None, &["bad", "awful"]))]);
    let answers = json!({
        "bucket": {"type": "choice", "choice": "bad", "probabilities": {"bad": 0.5, "awful": 0.4, "ok": 0.1}},
    });
    let verdicts = evaluate(&qs, &answers, 0.85, 0.6);
    assert_eq!(verdicts.len(), 1);
    assert_eq!(verdicts[0].severity, Severity::Fail);
    assert!((verdicts[0].value - 0.9).abs() < 1e-9, "{:?}", verdicts[0]);
}

#[test]
fn score_thresholds_apply_question_level_override() {
    let qs = questions(&[("grade", spec("score", Some(2.0), Some(1.0), &[]))]);
    let answers = json!({"grade": {"type": "score", "score": 2.5}});
    let verdicts = evaluate(&qs, &answers, 0.85, 0.6);
    assert_eq!(verdicts.len(), 1);
    assert_eq!(verdicts[0].severity, Severity::Fail);
}

#[test]
fn client_reports_unreachable_endpoint() {
    let err = Ask::new(DEAD_ENDPOINT, "state", json!({}))
        .timeout(3)
        .send()
        .expect_err("dead endpoint must fail");
    assert!(err.contains("unreachable") || err.contains("http"), "{err}");
}

/// 端到端：kind 实例 + 死端点 → notChecked（不伪装 finding；agent 口径 rc=10）。
#[test]
fn semantic_kind_reports_not_checked_when_endpoint_dead() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("sample.rs");
    std::fs::write(&file, "fn f() {}\n").unwrap();

    let cfg: ChecksConfig = toml::from_str(
        r#"
[kinds.semantic-question.demo]
intent = "Sample rule body for the probe."
paths_include = ["**/*.rs"]
message = "语义"

[kinds.semantic-question.demo.questions.q1]
type = "noul"
instructions = "Is the sample sound?"
fail = 0.85
warn = 0.6
"#,
    )
    .expect("valid checks toml");

    let (registry, errors) = Registry::build(Vec::new(), &check_semantic::factories(), &cfg);
    assert!(errors.is_empty(), "{errors:?}");

    let endpoint = SemanticEndpoint {
        endpoint: DEAD_ENDPOINT.to_string(),
        model: "jev-latest".to_string(),
        max_len: 8192,
    };
    let ctx = Ctx::new(dir.path(), vec![Path::new("sample.rs").to_path_buf()], None);
    let report = registry.run(
        &ctx,
        &RunOptions {
            classes: None,
            semantic_endpoint: Some(&endpoint),
            overrides: None,
        },
    );

    assert!(report.findings.is_empty(), "不得伪装 finding：{report:?}");
    assert_eq!(report.not_checked.len(), 1, "{report:?}");
    assert!(
        report.not_checked[0].reason.contains("判定失败"),
        "{}",
        report.not_checked[0].reason
    );
    assert_eq!(report.exit_code(), 10, "agent 口径：有 notChecked → 10");
}
