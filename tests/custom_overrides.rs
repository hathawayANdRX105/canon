//! Integration tests — the project-local override layer (`spec/custom/`).
//!
//! `canon-sync push` overwrites every canon-owned spec file on each deploy, so
//! per-repo tuning written there vanishes. The `custom/` namespace is the
//! surviving channel, and these tests pin its contract at the lib API:
//! - severity: root `severity_overrides.yaml` first, then
//!   `custom/severity_overrides.yaml` (last word wins); missing = no-op,
//!   broken = one loud `canon.setup` WARN;
//! - family yamls: `custom/<same-name>` deep-merges over the canon file
//!   (mappings recursively, lists/scalars replace per key) and is never a
//!   base source on its own.

use std::fs;
use std::path::Path;

use canon::gate::shared::{
    Finding, Severity, apply_global_overrides_in, apply_severity_overrides, load_spec_yaml_in,
};

fn root_overrides(text: &str, spec: &Path) {
    fs::write(spec.join("severity_overrides.yaml"), text).unwrap();
}

fn custom_overrides(text: &str, spec: &Path) {
    fs::create_dir_all(spec.join("custom")).unwrap();
    fs::write(spec.join("custom/severity_overrides.yaml"), text).unwrap();
}

#[test]
fn custom_severity_layer_wins_over_root_layer() {
    let root = tempfile::tempdir().unwrap();
    let spec = root.path().join("spec");
    fs::create_dir_all(&spec).unwrap();
    root_overrides("severity_overrides:\n  CM-01: WARN\n  PR-06: INFO\n", &spec);
    custom_overrides("severity_overrides:\n  CM-01: FAIL\n", &spec);

    let mut findings = vec![
        Finding::new("CM-01", Severity::Fail, "x"),
        Finding::new("PR-06", Severity::Fail, "x"),
        Finding::new("IS-14", Severity::Warn, "x"),
    ];
    apply_global_overrides_in(&mut findings, &spec);
    // same-named rule: the custom layer is the last word over the root WARN
    assert_eq!(findings[0].severity, Severity::Fail);
    // root-only rules still apply
    assert_eq!(findings[1].severity, Severity::Info);
    // rules in neither layer stay put, and no setup finding was added
    assert_eq!(findings[2].severity, Severity::Warn);
    assert_eq!(findings.len(), 3);
}

#[test]
fn custom_layer_missing_is_silent_noop() {
    let root = tempfile::tempdir().unwrap();
    let spec = root.path().join("spec");
    fs::create_dir_all(&spec).unwrap();
    root_overrides("severity_overrides:\n  PR-06: INFO\n", &spec);

    let mut findings = vec![Finding::new("PR-06", Severity::Fail, "x")];
    apply_global_overrides_in(&mut findings, &spec);
    assert_eq!(findings.len(), 1, "root layer applies; no findings added");
    assert_eq!(findings[0].severity, Severity::Info);
}

#[test]
fn broken_custom_layer_adds_single_loud_warn() {
    let root = tempfile::tempdir().unwrap();
    let spec = root.path().join("spec");
    fs::create_dir_all(&spec).unwrap();
    root_overrides("severity_overrides:\n  PR-06: INFO\n", &spec);
    custom_overrides("severity_overrides: [broken\n", &spec);

    let mut findings = vec![Finding::new("PR-06", Severity::Fail, "x")];
    apply_global_overrides_in(&mut findings, &spec);
    // root layer still applied; broken custom layer ignored but reported once
    assert_eq!(findings.len(), 2);
    assert_eq!(findings[0].severity, Severity::Info);
    assert_eq!(findings[1].rule_id, "canon.setup");
    assert_eq!(findings[1].severity, Severity::Warn);
}

#[test]
fn info_findings_survive_every_override_layer() {
    let root = tempfile::tempdir().unwrap();
    let spec = root.path().join("spec");
    fs::create_dir_all(&spec).unwrap();
    root_overrides("severity_overrides:\n  CM-01: FAIL\n", &spec);
    custom_overrides("severity_overrides:\n  CM-01: WARN\n", &spec);

    let mut findings = vec![Finding::new("CM-01", Severity::Info, "x")];
    apply_global_overrides_in(&mut findings, &spec);
    assert_eq!(
        findings[0].severity,
        Severity::Info,
        "INFO is a one-way rule"
    );

    // same rule for the family/dispatch section
    let mut findings = vec![Finding::new("CM-01", Severity::Info, "x")];
    apply_severity_overrides(
        &mut findings,
        Some(&serde_yaml::from_str("severity_overrides:\n  CM-01: FAIL\n").unwrap()),
    );
    assert_eq!(findings[0].severity, Severity::Info);
}

#[test]
fn custom_family_yaml_deep_merges_over_canon_base() {
    let root = tempfile::tempdir().unwrap();
    let spec = root.path().join("spec");
    fs::create_dir_all(spec.join("custom")).unwrap();
    fs::create_dir_all(spec.join("github")).unwrap();
    fs::write(
        spec.join("github/github_pull_requests.yaml"),
        "fixes_linkage_mode: WARN\n\
         type_labels_cfg: [bug, feature]\n\
         linkage: {plain_severity: INFO, extra: 1}\n\
         keep: me\n",
    )
    .unwrap();
    fs::write(
        spec.join("custom/github_pull_requests.yaml"),
        "fixes_linkage_mode: FAIL\n\
         type_labels_cfg: [bug]\n\
         linkage: {extra: 2, epic_severity: WARN}\n",
    )
    .unwrap();

    let cfg = load_spec_yaml_in(&spec, "github_pull_requests.yaml")
        .expect("canon base must load with the overlay on top");
    // scalar replaces wholesale
    assert_eq!(
        cfg.get("fixes_linkage_mode").unwrap().as_str(),
        Some("FAIL")
    );
    // list replaces wholesale
    assert_eq!(
        cfg.get("type_labels_cfg")
            .unwrap()
            .as_sequence()
            .unwrap()
            .len(),
        1
    );
    // mapping merges recursively: base key kept, overlay key added, shared key overridden
    let linkage = cfg.get("linkage").unwrap();
    assert_eq!(
        linkage.get("plain_severity").unwrap().as_str(),
        Some("INFO")
    );
    assert_eq!(linkage.get("extra").unwrap().as_u64(), Some(2));
    assert_eq!(linkage.get("epic_severity").unwrap().as_str(), Some("WARN"));
    // base-only top-level key survives
    assert_eq!(cfg.get("keep").unwrap().as_str(), Some("me"));
}

#[test]
fn custom_only_family_yaml_provides_cfg_without_base() {
    let root = tempfile::tempdir().unwrap();
    let spec = root.path().join("spec");
    fs::create_dir_all(spec.join("custom")).unwrap();
    fs::write(spec.join("custom/github_issues.yaml"), "enabled: false\n").unwrap();
    // no base anywhere: the overlay stands in for the whole cfg
    let cfg = load_spec_yaml_in(&spec, "github_issues.yaml")
        .expect("a custom-only cfg must load, not fail closed");
    assert_eq!(cfg.get("enabled").unwrap().as_bool(), Some(false));
}

#[test]
fn broken_custom_family_overlay_keeps_canon_base() {
    let root = tempfile::tempdir().unwrap();
    let spec = root.path().join("spec");
    fs::create_dir_all(spec.join("custom")).unwrap();
    fs::write(spec.join("github_issues.yaml"), "enabled: true\n").unwrap();
    fs::write(spec.join("custom/github_issues.yaml"), "enabled: [broken\n").unwrap();

    let cfg = load_spec_yaml_in(&spec, "github_issues.yaml")
        .expect("a broken project-local overlay must not take down the canon base");
    assert_eq!(cfg.get("enabled").unwrap().as_bool(), Some(true));
}
