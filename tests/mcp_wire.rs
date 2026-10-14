//! `mcp` 线协议测试：JSON-RPC 分帧、错误码、tools/list schema。

use serde_json::Value;

use canon::gate::catalog::Rule;
use canon::gate::shared::Severity;
use canon::mcp::{PROTOCOL_VERSION, handle_line, rule_json};

#[test]
fn notification_gets_no_reply() {
    assert!(handle_line(r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#).is_none());
}
#[test]
fn initialize_reports_protocol_and_tools() {
    let out = handle_line(
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18"}}"#,
    )
    .expect("initialize must reply");
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["result"]["protocolVersion"], PROTOCOL_VERSION);
    assert!(v["result"]["capabilities"]["tools"].is_object());
}
#[test]
fn tools_list_exposes_all_tools_with_schemas() {
    let out = handle_line(r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#).unwrap();
    let v: Value = serde_json::from_str(&out).unwrap();
    let tools = v["result"]["tools"].as_array().unwrap();
    let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(names, ["spec_catalog", "spec_explain", "preflight"]);
    assert!(tools.iter().all(|t| t["inputSchema"]["type"] == "object"));
}
#[test]
fn unknown_method_is_json_rpc_error_not_a_crash() {
    let out = handle_line(r#"{"jsonrpc":"2.0","id":3,"method":"nope"}"#).unwrap();
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["error"]["code"], -32601);
}
#[test]
fn why_is_only_the_first_line_in_catalog_rows() {
    let r = Rule {
        id: "x".into(),
        source: "s".into(),
        severity: Severity::Fail,
        sla: Some("l1".into()),
        hooks: vec!["merge".into()],
        mode: Some("grep".into()),
        why: "第一行\n第二行".into(),
    };
    assert_eq!(rule_json(&r)["why"], "第一行");
}
