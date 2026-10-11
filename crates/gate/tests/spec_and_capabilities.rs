//! Integration tests — fail-closed spec loading + capability selection.
//!
//! Runs against the real lib API: a broken checklist yaml must produce a
//! hard `canon.setup` FAIL (not a silent drop), and the `checks:` /
//! `fail_severity` config knobs must shape findings.

use gate::shared::{Finding, Severity, apply_check_allowlist, apply_family_severity};

#[test]
fn catalog_load_covers_every_topic_directory() {
    // Regression: the sub-directory list was hardcoded, so dioxus/ rules were
    // invisible to spec_catalog/spec_explain while the engine still ran them.
    // Any directory holding an `enabled: true` checklist must be visible.
    let dir = tempfile::tempdir().unwrap();
    for sub in ["quality", "dioxus", "some_new_topic"] {
        std::fs::create_dir_all(dir.path().join(sub)).unwrap();
        std::fs::write(
            dir.path()
                .join(sub)
                .join(format!("checklist_{sub}_thing.yaml")),
            "enabled: true\nfail_severity: WARN\nhooks: [merge]\nharness: {command: sh, args: []}\n",
        )
        .unwrap();
    }
    // A directory without rules must contribute nothing, not blow up.
    std::fs::create_dir_all(dir.path().join("docs")).unwrap();
    std::fs::write(dir.path().join("docs/notes.yaml"), "title: just docs\n").unwrap();

    let rules = gate::catalog::load(dir.path());
    let sources: Vec<_> = rules.iter().map(|r| r.source.as_str()).collect();
    assert_eq!(
        sources.len(),
        3,
        "every topic dir must be catalogued, got {sources:?}"
    );
    assert!(
        sources.iter().any(|s| s.starts_with("dioxus/")),
        "{sources:?}"
    );
    assert!(
        sources.iter().any(|s| s.starts_with("some_new_topic/")),
        "a newly added topic dir must not be silently skipped: {sources:?}"
    );
}

#[test]
fn broken_checklist_spec_blocks_instead_of_silently_dropping() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("checklist_bad.yaml"),
        "harness: {command: sh}\ntypo_key: 1\n",
    )
    .unwrap();
    let specs = gate::engine::find_specs(dir.path());
    assert_eq!(specs.len(), 1);
    let err = specs[0]
        .1
        .as_ref()
        .expect_err("typo'd spec key must fail loudly");
    assert!(err.contains("deserialize"), "{err}");
}

#[test]
fn valid_checklist_spec_parses() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("checklist_good.yaml"),
        "mode: grep\nfail_severity: WARN\nharness: {command: sh, args: []}",
    )
    .unwrap();
    let specs = gate::engine::find_specs(dir.path());
    assert!(specs[0].1.is_ok());
}

fn find(rule: &str) -> Finding {
    Finding::new(rule, Severity::Warn, "msg")
}

#[test]
fn checks_allowlist_keeps_only_listed_capabilities() {
    let mut findings = vec![find("CL-01"), find("CL-02"), find("RV-06")];
    let cfg = serde_yaml::from_str("checks: [CL, RV-06]").unwrap();
    apply_check_allowlist(&mut findings, Some(&cfg));
    // "CL" prefix keeps CL-01/CL-02; RV-06 kept by exact match; nothing else
    assert_eq!(findings.len(), 3);
    let cfg = serde_yaml::from_str("checks: [CL-01, RV-06]").unwrap();
    let mut findings = vec![find("CL-01"), find("CL-02"), find("RV-06")];
    apply_check_allowlist(&mut findings, Some(&cfg));
    let ids: Vec<_> = findings.iter().map(|f| f.rule_id.as_str()).collect();
    assert_eq!(ids, vec!["CL-01", "RV-06"]);
}

#[test]
fn checks_key_absent_keeps_everything() {
    let mut findings = vec![find("CL-01"), find("RV-06")];
    apply_check_allowlist(&mut findings, None);
    apply_check_allowlist(&mut findings, Some(&serde_yaml::Value::Null));
    assert_eq!(findings.len(), 2);
}

#[test]
fn fail_severity_remaps_warn_but_never_promotes_info() {
    let mut findings = vec![
        Finding::new("CL-01", Severity::Warn, "w"),
        Finding::new("CL-02", Severity::Info, "i"),
        Finding::new("CL-03", Severity::Fail, "f"),
    ];
    let cfg = serde_yaml::from_str("fail_severity: FAIL").unwrap();
    apply_family_severity(&mut findings, Some(&cfg));
    assert_eq!(
        findings[0].severity,
        Severity::Fail,
        "WARN -> configured FAIL"
    );
    assert_eq!(
        findings[1].severity,
        Severity::Info,
        "INFO is never promoted"
    );
    assert_eq!(findings[2].severity, Severity::Fail, "FAIL stays FAIL");
}
