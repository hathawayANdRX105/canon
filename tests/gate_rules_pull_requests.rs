//! `gate::rules::pull_requests` 单元测试 + 真 spec 冒烟。

use std::path::Path;

use serde_yaml::Value as YamlValue;

use canon::gate::rules::pull_requests::check_content;
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
#[test]
fn missing_spec_is_a_loud_gate_setup_fail() {
    // None: no spec on disk at all.
    let findings = check_content("feat: add thing", "## Issue\n", &[], "feat/a", None);
    assert_eq!(findings.len(), 1, "no checks may run without the spec");
    assert_eq!(findings[0].rule_id, "canon.setup");
    assert_eq!(findings[0].severity, Severity::Fail);
    // A null (empty) spec is just as missing — never silently green.
    let findings = check_content(
        "feat: add thing",
        "## Issue\n",
        &[],
        "feat/a",
        Some(&YamlValue::Null),
    );
    assert_eq!(findings.len(), 1, "no checks may run without the spec");
    assert_eq!(findings[0].rule_id, "canon.setup");
}
#[test]
fn ci_and_done_when_modes_drive_severity() {
    // ci_check_mode FAIL promotes the non-conventional title to FAIL;
    // done_when_check_mode WARN demotes the thin Checklist to WARN.
    let cfg = cfg(r#"
ci_check_mode: "FAIL"
done_when_check_mode: "WARN"
done_when_min_checkboxes: 2
done_when_headings: ["Construction plan", "Checklist"]
required_body_headings: ["Issue"]
conventional_commit_types: ["feat", "fix"]
title_must_be_chinese: true
"#);
    let findings = check_content(
        "not a conventional title",
        "## Issue\n\n## Checklist\n\n- [ ] one\n",
        &["bug"],
        "feat/a",
        Some(&cfg),
    );
    let ci = findings
        .iter()
        .find(|f| f.rule_id == "PR-02" && f.severity != Severity::Info)
        .expect("PR-02 finding");
    assert_eq!(ci.severity, Severity::Fail);
    let dw = findings
        .iter()
        .find(|f| f.rule_id == "PR-07")
        .expect("PR-07 finding");
    assert_eq!(dw.severity, Severity::Warn);
}
#[test]
fn garbled_mode_keeps_the_original_severity() {
    // An unparseable mode must not turn a check off or silently pass it.
    let cfg = cfg("ci_check_mode: \"please\"\ndone_when_check_mode: \"\"\n");
    let findings = check_content(
        "not a conventional title",
        "## Checklist\n\n- [ ] one\n",
        &["bug"],
        "feat/a",
        Some(&cfg),
    );
    let ci = findings
        .iter()
        .find(|f| f.rule_id == "PR-02" && f.severity != Severity::Info)
        .expect("PR-02 finding");
    assert_eq!(ci.severity, Severity::Warn);
}
#[test]
fn forbidden_brackets_are_caught_independent_of_cjk() {
    // （ is U+FF08: outside the CJK ideograph block, so the bracket rule
    // must fire on its own.
    let cfg = cfg("title_must_be_chinese: true\nforbidden_brackets_in_title: [\"（\"]\n");
    let findings = check_content(
        "feat: fix（thing",
        "## Issue\n",
        &["bug"],
        "feat/a",
        Some(&cfg),
    );
    assert!(findings.iter().any(|f| {
        f.rule_id == "PR-01" && f.severity == Severity::Fail && f.msg.contains("fullwidth")
    }));
}
#[test]
fn linkage_is_optional_unless_spec_asks_otherwise() {
    // PR-first workflow: a PR with zero `Fixes #` is legitimate (INFO).
    // `fixes_linkage_mode` promotes the missing-linkage finding for repos
    // that still want one issue per PR; multiple Fixes stays WARN.
    let spec = cfg("fixes_keywords: [\"Fixes\"]\n");
    let findings = check_content("feat: x", "", &[], "feat/a", Some(&spec));
    let pr05 = findings
        .iter()
        .find(|f| f.rule_id == "PR-05")
        .expect("PR-05 finding");
    assert_eq!(pr05.severity, Severity::Info, "no linkage is legal");
    let spec = cfg("fixes_keywords: [\"Fixes\"]\nfixes_linkage_mode: \"FAIL\"\n");
    let findings = check_content("feat: x", "", &[], "feat/a", Some(&spec));
    let pr05 = findings
        .iter()
        .find(|f| f.rule_id == "PR-05")
        .expect("PR-05 finding");
    assert_eq!(pr05.severity, Severity::Fail, "spec can require linkage");
    let spec = cfg("fixes_keywords: [\"Fixes\"]\n");
    let findings = check_content(
        "feat: x",
        "Fixes #1\nFixes #2\n",
        &[],
        "feat/a",
        Some(&spec),
    );
    let pr05 = findings
        .iter()
        .find(|f| f.rule_id == "PR-05")
        .expect("PR-05 finding");
    assert_eq!(pr05.severity, Severity::Warn, "one PR closes one issue");
}

// ---- spec 冒烟：拿真 spec 文件跑规则 ----

fn load(name: &str) -> YamlValue {
    let p = repo_spec_path(name);
    serde_yaml::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap()
}

#[test]
fn real_spec_clean_pr_is_not_fail() {
    let cfg = load("github_pull_requests.yaml");
    let body = "\
## What
做这件事的原因是 X
## Why
背景说明
## Construction plan
- [ ] a
- [ ] b
## Delivery record
已完成
## How to test
cargo test
## Checklist
- [ ] a
- [ ] b
";
    let f = check_content("feat: add thing", body, &["feature"], "feat/x", Some(&cfg));
    let fails: Vec<_> = f.iter().filter(|x| x.severity == Severity::Fail).collect();
    assert!(
        fails.is_empty(),
        "clean PR must not FAIL against the real spec: {f:?}"
    );
}
