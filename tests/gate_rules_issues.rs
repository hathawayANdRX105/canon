//! `gate::rules::issues` 单元测试：issue 正文 / 标题 / Done when 段校验。

use serde_yaml::Value as YamlValue;

use canon::gate::rules::issues::{check_content, done_when_section};
use canon::gate::shared::Severity;

fn cfg(yaml: &str) -> YamlValue {
    serde_yaml::from_str(yaml).expect("valid inline spec")
}
#[test]
fn missing_spec_is_a_loud_gate_setup_fail() {
    // None: no spec on disk at all.
    let findings = check_content("标题", "正文", &[], "sub", "open", None);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "canon.setup");
    assert_eq!(findings[0].severity, Severity::Fail);
    // A null (empty) spec is just as missing — never silently green.
    let findings = check_content("标题", "正文", &[], "sub", "open", Some(&YamlValue::Null));
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "canon.setup");
}
#[test]
fn switches_off_skip_their_checks() {
    let cfg = cfg("title_must_be_chinese: false\ngarbled_content_check: false\n");
    // English title + literal \n in body: both would FAIL if the
    // switches invented an "on" default. Nothing else in this spec can
    // FAIL either (every list is empty, every heading name absent).
    let findings = check_content(
        "english title",
        "## Goal\n\n正文 body\\nwith literal backslash-n",
        &[],
        "sub",
        "open",
        Some(&cfg),
    );
    assert!(findings.iter().all(|f| f.rule_id != "IS-05"));
    assert!(findings.iter().all(|f| f.severity != Severity::Fail));
}
#[test]
fn spec_values_drive_the_checks() {
    let cfg = cfg(r#"
title_must_be_chinese: true
forbidden_keywords:
  - NOPE
required_headings:
  - Goal
heading_names:
  done_when: "Acceptance"
  suspected_areas: "Impact"
"#);
    // Custom heading names from the spec; "NOPE" is only forbidden
    // because the spec says so.
    let body = "## Goal\n\n正文 NOPE\n\n## Acceptance\n\n- [ ] 完成\n\n## Impact\n\nsrc/foo.rs\n";
    let findings = check_content("english title", body, &[], "sub", "open", Some(&cfg));
    // IS-05 is on and the title has no CJK.
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "IS-05" && f.severity == Severity::Fail)
    );
    // IS-16 keyword comes from the spec list, not from code.
    assert!(
        findings
            .iter()
            .any(|f| f.rule_id == "IS-16" && f.severity == Severity::Fail)
    );
    // Custom Done when heading is honoured (checkbox present ⇒ no IS-04 FAIL).
    assert!(
        findings
            .iter()
            .all(|f| f.rule_id != "IS-04" || f.severity != Severity::Fail)
    );
    // IS-01 is satisfied by the only required heading.
    assert!(
        findings
            .iter()
            .all(|f| f.rule_id != "IS-01" || f.severity != Severity::Fail)
    );
}
#[test]
fn done_when_section_reads_heading_name_from_spec() {
    let cfg = cfg(r#"heading_names: {done_when: "Acceptance"}"#);
    let body = "## Acceptance\n\n- [ ] done\n";
    // `section` keeps the newline that ends the heading line.
    assert_eq!(done_when_section(body, Some(&cfg)), "\n- [ ] done\n");
    // No spec ⇒ no heading name ⇒ inert (empty), never a bogus match.
    assert_eq!(done_when_section(body, None), "");
}
