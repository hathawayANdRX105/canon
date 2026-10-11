//! `gate::rules::reviews` 单元测试 + 真 spec 冒烟。

use std::path::Path;

use serde_yaml::Value as YamlValue;

use canon::gate::rules::reviews::run;
use canon::gate::shared::Severity;

/// 仓根 `specs/` 就是本 crate 的 CARGO_MANIFEST_DIR 下的目录。
fn repo_spec_path(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("specs")
        .join("github")
        .join(name)
}

fn cfg(yaml: &str) -> YamlValue {
    serde_yaml::from_str(yaml).expect("valid inline spec")
}
fn full_spec() -> YamlValue {
    cfg(r#"
review_formats:
  review_comment:
    checkbox_forbidden: true
    allowed_reply_words: ["Fix", "Block", "Resolve", "Note", "Withdraw", "Supersede"]
  inline_review:
    prefix: "Agent 🤖 - Inline Review"
    allowed_inline_levels: ["P0", "P1", "P2", "P3"]
    content_must_be_chinese: true
    checkbox_forbidden: true
reply_formats:
  required_intent_word: true
  colon_after: true
  reply_min_detail: 5
"#)
}
#[test]
fn missing_spec_is_a_loud_gate_setup_fail() {
    // A null (empty) spec is just as missing as an absent one.
    let findings = run(&[], &YamlValue::Null);
    assert_eq!(findings.len(), 1, "no checks may run without the spec");
    assert_eq!(findings[0].rule_id, "canon.setup");
    assert_eq!(findings[0].severity, Severity::Fail);
}
#[test]
fn switches_off_skip_their_checks() {
    // No format sections at all: RV-01 is off, and the inline
    // extractor finds nothing, so no RV-04 may fire either.
    let findings = run(&["- [ ] leftover checkbox".to_string()], &cfg("{}"));
    assert!(findings.iter().all(|f| f.rule_id != "RV-01"));
    assert!(findings.iter().all(|f| f.severity != Severity::Fail));
}
#[test]
fn inline_level_and_chinese_are_gated_by_spec() {
    let body = "Agent 🤖 - Inline Review P9: english only content".to_string();
    let findings = run(&[body], &full_spec());
    // P9 is not in allowed_inline_levels, and the content has no CJK.
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "RV-04" && f.severity == Severity::Fail && f.msg.contains("P9"))
    );
    assert!(findings.iter().any(|f| f.rule_id == "RV-04"
        && f.severity == Severity::Fail
        && f.msg.contains("Chinese")));
}
#[test]
fn reply_word_and_detail_come_from_spec() {
    let body = "Agent 🤖 - Nope: 太短".to_string();
    let findings = run(&[body], &full_spec());
    // "Nope" is not an allowed intent word; and the reason is shorter
    // than reply_min_detail.
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "RV-02" && f.severity == Severity::Warn)
    );
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "RV-03" && f.severity == Severity::Warn)
    );
}

// ---- spec 冒烟：拿真 spec 文件跑规则 ----

fn load(name: &str) -> YamlValue {
    let p = repo_spec_path(name);
    serde_yaml::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap()
}

#[test]
fn real_spec_clean_review_is_not_fail() {
    let cfg = load("github_reviews.yaml");
    let inline = "Agent 🤖 - Inline Review P2: 这里补一个空指针检查\n".to_string();
    let reply = "Agent 🤖 - Fix: 已按建议修复，补充了测试".to_string();
    let f = run(&[inline, reply], &cfg);
    let fails: Vec<_> = f.iter().filter(|x| x.severity == Severity::Fail).collect();
    assert!(
        fails.is_empty(),
        "clean review must not FAIL against the real spec: {f:?}"
    );
}

#[test]
fn real_spec_bad_review_fails() {
    let cfg = load("github_reviews.yaml");
    // P9 is outside allowed_inline_levels and the prose is English only —
    // both are RV-04 contract violations the real spec declares.
    let bad = "Agent 🤖 - Inline Review P9: english only content\n".to_string();
    let f = run(&[bad], &cfg);
    assert!(
        f.iter()
            .any(|x| x.rule_id == "RV-04" && x.severity == Severity::Fail),
        "{f:?}"
    );
}
