//! Integration tests — fail-closed spec loading + capability selection.
//!
//! Runs against the real lib API: a broken checklist yaml must produce a
//! hard `gate.setup` FAIL (not a silent drop), and the `checks:` /
//! `fail_severity` config knobs must shape findings.

use gate::shared::{Finding, Severity, apply_check_allowlist, apply_family_severity};

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
    let mut findings = vec![find("CL-01"), find("CL-02"), find("RV-05")];
    let cfg = serde_yaml::from_str("checks: [CL, RV-05]").unwrap();
    apply_check_allowlist(&mut findings, Some(&cfg));
    // "CL" prefix keeps CL-01/CL-02; RV-05 kept by exact match; nothing else
    assert_eq!(findings.len(), 3);
    let cfg = serde_yaml::from_str("checks: [CL-01, RV-05]").unwrap();
    let mut findings = vec![find("CL-01"), find("CL-02"), find("RV-05")];
    apply_check_allowlist(&mut findings, Some(&cfg));
    let ids: Vec<_> = findings.iter().map(|f| f.rule_id.as_str()).collect();
    assert_eq!(ids, vec!["CL-01", "RV-05"]);
}

#[test]
fn checks_key_absent_keeps_everything() {
    let mut findings = vec![find("CL-01"), find("RV-05")];
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
