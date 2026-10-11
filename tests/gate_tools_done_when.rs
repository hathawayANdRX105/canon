//! `gate::tools::done_when` 单元测试：DWJ 开关与 judge 状态构造。

use serde_yaml::Value as YamlValue;

use canon::gate::shared::Severity;
use canon::gate::tools::done_when::{
    build_judge_state, close_comment, done_when_item_texts, dwj_enabled, parse_judge_findings,
};

fn cfg(yaml: &str) -> Option<YamlValue> {
    serde_yaml::from_str(yaml).ok()
}
const DW_CFG: &str = "heading_names:\n  done_when: \"Done when\"\n";
#[test]
fn dwj_switch_is_opt_in() {
    assert!(!dwj_enabled(None));
    assert!(!dwj_enabled(cfg("other: 1\n").as_ref()));
    // Missing done_when_judge map or flag → off.
    assert!(!dwj_enabled(
        cfg("done_when_judge:\n  command: python3\n").as_ref()
    ));
    let on = cfg("done_when_judge:\n  enabled: true\n  command: python3\n");
    assert!(dwj_enabled(on.as_ref()));
}
#[test]
fn done_when_items_extract_ticked_and_unticked() {
    let body = "## Done when\n\n- [x] 接口编译通过\n- [x] 测试全绿\n\n## 备注\n\n- [ ] 不扫这里\n";
    let items = done_when_item_texts(body, cfg(DW_CFG).as_ref());
    assert_eq!(items, vec!["接口编译通过", "测试全绿"]);
}
#[test]
fn judge_state_carries_items_and_evidence() {
    let state = build_judge_state("42", &["a".into(), "b".into()], "diff --git ...");
    let v: serde_json::Value = serde_json::from_str(&state).unwrap();
    assert_eq!(v["issue"], "42");
    assert_eq!(v["done_when_items"].as_array().unwrap().len(), 2);
    assert_eq!(v["evidence"], "diff --git ...");
}
#[test]
fn judge_findings_parse_severities_and_extras() {
    let stdout = r#"[{"id":"REVIEW-ITEM_0","severity":"FAIL","path":"","line":0,
            "message":"item_0 p(issue)=0.92","tier":"jev","confidence":"0.92"},
            {"id":"REVIEW-ITEM_1","severity":"INFO","message":"ok","tier":"none"}]"#;
    let findings = parse_judge_findings(stdout);
    assert_eq!(findings.len(), 2);
    let f0 = &findings[0];
    assert_eq!(f0.rule_id, "REVIEW-ITEM_0");
    assert_eq!(f0.severity, Severity::Fail);
    assert_eq!(f0.extra.get("confidence").map(String::as_str), Some("0.92"));
    assert_eq!(f0.extra.get("tier").map(String::as_str), Some("jev"));
}
#[test]
fn judge_degradation_is_info_never_block() {
    // Not JSON / empty / no array → SKIPPED INFO.
    for bad in ["", "not json", "{\"id\":1}"] {
        let f = parse_judge_findings(bad);
        assert_eq!(f.len(), 1, "input: {bad}");
        assert_eq!(f[0].rule_id, "DWJ-SKIPPED");
        assert_eq!(f[0].severity, Severity::Info);
    }
}
#[test]
fn close_comment_reads_both_flag_forms() {
    let args: Vec<String> = ["issue", "close", "7", "--comment", "done ci green"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(close_comment(&args), "done ci green");
}
