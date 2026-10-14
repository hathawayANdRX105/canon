//! `gate::shared` 单元测试：override 合并层与 Finding 格式化。

use std::fs;

use serde_yaml::Value as YamlValue;

use canon::gate::shared::{
    Finding, Severity, apply_global_overrides_in, apply_severity_overrides, deep_merge,
    find_spec_file, load_yaml, merge_custom_overlay,
};

fn yaml(text: &str) -> YamlValue {
    serde_yaml::from_str(text).unwrap()
}
#[test]
fn deep_merge_mappings_recurse_scalars_and_lists_replace() {
    let base = yaml("a: {x: 1, y: 2}\nb: [1, 2]\nc: old\nd: keep\n");
    let overlay = yaml("a: {y: 3, z: 4}\nb: [9]\nc: new\n");
    let merged = deep_merge(&base, &overlay);
    // nested mapping merges key by key
    let a = merged.get("a").unwrap();
    assert_eq!(a.get("x").unwrap().as_u64(), Some(1));
    // overlay value wins for a shared key
    assert_eq!(a.get("y").unwrap().as_u64(), Some(3));
    assert_eq!(a.get("z").unwrap().as_u64(), Some(4));
    // list replaces wholesale
    assert_eq!(merged.get("b").unwrap().as_sequence().unwrap().len(), 1);
    // scalar replaces
    assert_eq!(merged.get("c").unwrap().as_str(), Some("new"));
    // base-only keys survive
    assert_eq!(merged.get("d").unwrap().as_str(), Some("keep"));
}
#[test]
fn deep_merge_type_mismatch_overlay_wins() {
    let merged = deep_merge(&yaml("a: 1"), &yaml("a: {x: 1}"));
    assert!(merged.get("a").unwrap().is_mapping());
    let merged = deep_merge(&yaml("a: {x: 1}"), &yaml("a: 2"));
    assert_eq!(merged.get("a").unwrap().as_u64(), Some(2));
}
#[test]
fn find_spec_file_ignores_custom_namespace() {
    let dir = tempfile::tempdir().unwrap();
    let spec = dir.path().join("spec");
    fs::create_dir_all(spec.join("custom")).unwrap();
    fs::create_dir_all(spec.join("github")).unwrap();
    fs::write(spec.join("github/x.yaml"), "a: 1").unwrap();
    fs::write(spec.join("custom/x.yaml"), "a: 2").unwrap();
    // the non-custom copy is the base hit even though `custom/…` would
    // sort before `github/…`
    let hit = find_spec_file(&spec, "x.yaml").unwrap();
    assert!(hit.starts_with(spec.join("github").as_path()));
    // a custom-only file is not a base source at all
    fs::write(spec.join("custom/only.yaml"), "a: 2").unwrap();
    assert_eq!(find_spec_file(&spec, "only.yaml"), None);
}
#[test]
fn custom_overlay_missing_is_noop() {
    let dir = tempfile::tempdir().unwrap();
    let spec = dir.path().join("spec");
    fs::create_dir_all(&spec).unwrap();
    fs::write(spec.join("fam.yaml"), "a: 1\nb: [1, 2]\n").unwrap();
    let base = load_yaml(spec.join("fam.yaml").to_str().unwrap()).unwrap();
    assert_eq!(merge_custom_overlay(&base, &spec, "fam.yaml"), base);
}
#[test]
fn custom_overlay_empty_or_broken_keeps_base() {
    let dir = tempfile::tempdir().unwrap();
    let spec = dir.path().join("spec");
    fs::create_dir_all(spec.join("custom")).unwrap();
    fs::write(spec.join("fam.yaml"), "a: {x: 1}\nb: [1, 2]\nc: keep\n").unwrap();
    let base = load_yaml(spec.join("fam.yaml").to_str().unwrap()).unwrap();
    // empty overlay = no-op
    fs::write(spec.join("custom/fam.yaml"), "").unwrap();
    assert_eq!(merge_custom_overlay(&base, &spec, "fam.yaml"), base);
    // broken overlay = canon base kept
    fs::write(spec.join("custom/fam.yaml"), "a: [unterminated\n").unwrap();
    assert_eq!(merge_custom_overlay(&base, &spec, "fam.yaml"), base);
}
#[test]
fn custom_severity_layer_wins_over_root_layer() {
    let dir = tempfile::tempdir().unwrap();
    let spec = dir.path().join("spec");
    fs::create_dir_all(spec.join("custom")).unwrap();
    fs::write(
        spec.join("severity_overrides.yaml"),
        "severity_overrides:\n  CM-01: WARN\n  PR-06: INFO\n",
    )
    .unwrap();
    fs::write(
        spec.join("custom/severity_overrides.yaml"),
        "severity_overrides:\n  CM-01: FAIL\n",
    )
    .unwrap();
    let mut findings = vec![
        Finding::new("CM-01", Severity::Fail, "x"),
        Finding::new("PR-06", Severity::Fail, "x"),
        Finding::new("IS-14", Severity::Warn, "x"),
    ];
    apply_global_overrides_in(&mut findings, &spec);
    // custom layer is the last word: the root WARN is overridden back
    assert_eq!(findings[0].severity, Severity::Fail);
    // rules absent from custom keep the root value
    assert_eq!(findings[1].severity, Severity::Info);
    // rules in neither layer stay put
    assert_eq!(findings[2].severity, Severity::Warn);
}
#[test]
fn custom_severity_layer_missing_is_silent_noop() {
    let dir = tempfile::tempdir().unwrap();
    let spec = dir.path().join("spec");
    fs::create_dir_all(&spec).unwrap();
    fs::write(
        spec.join("severity_overrides.yaml"),
        "severity_overrides:\n  PR-06: INFO\n",
    )
    .unwrap();
    let mut findings = vec![Finding::new("PR-06", Severity::Fail, "x")];
    apply_global_overrides_in(&mut findings, &spec);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].severity, Severity::Info);
}
#[test]
fn broken_custom_severity_layer_adds_loud_warn_finding() {
    let dir = tempfile::tempdir().unwrap();
    let spec = dir.path().join("spec");
    fs::create_dir_all(spec.join("custom")).unwrap();
    fs::write(
        spec.join("severity_overrides.yaml"),
        "severity_overrides:\n  PR-06: INFO\n",
    )
    .unwrap();
    fs::write(
        spec.join("custom/severity_overrides.yaml"),
        "severity_overrides: [oops\n",
    )
    .unwrap();
    let mut findings = vec![Finding::new("PR-06", Severity::Fail, "x")];
    apply_global_overrides_in(&mut findings, &spec);
    // root layer still applied; broken custom layer ignored but reported
    assert_eq!(findings[0].severity, Severity::Info);
    assert_eq!(findings.len(), 2);
    let loud = &findings[1];
    assert_eq!(loud.rule_id, "canon.setup");
    assert_eq!(loud.severity, Severity::Warn);
    assert!(
        loud.msg.contains("custom/severity_overrides.yaml"),
        "{}",
        loud.msg
    );
}
#[test]
fn no_override_files_at_all_is_silent_noop() {
    let dir = tempfile::tempdir().unwrap();
    let mut findings = vec![Finding::new("CM-01", Severity::Fail, "x")];
    apply_global_overrides_in(&mut findings, dir.path());
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].severity, Severity::Fail);
}
#[test]
fn severity_overrides_never_touch_info_findings() {
    let mut findings = vec![
        Finding::new("PR-05", Severity::Info, "x"),
        Finding::new("PR-06", Severity::Warn, "x"),
    ];
    apply_severity_overrides(
        &mut findings,
        Some(&yaml("severity_overrides:\n  PR-05: FAIL\n  PR-06: INFO\n")),
    );
    // one-way rule: a passing check stays passing
    assert_eq!(findings[0].severity, Severity::Info);
    assert_eq!(findings[1].severity, Severity::Info);
}
