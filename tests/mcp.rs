//! End-to-end MCP contract against canon's real spec tree.
//!
//! The unit tests in `src/mcp.rs` pin the JSON-RPC framing; these pin what a
//! client actually gets back — that the catalog is populated from the deployed
//! spec, that `spec_explain` returns the rationale prose, and that `preflight`
//! honours its FAIL-only default.

use canon::mcp;

fn call(line: &str) -> serde_json::Value {
    let out = mcp::handle_line(line).expect("tool call must reply");
    serde_json::from_str(&out).unwrap()
}

fn tool_text(v: &serde_json::Value) -> serde_json::Value {
    let t = &v["result"]["content"][0]["text"];
    serde_json::from_str(t.as_str().unwrap()).unwrap()
}

fn catalog(args: &str) -> serde_json::Value {
    let line = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"spec_catalog","arguments":{args}}}}}"#
    );
    tool_text(&call(&line))
}

#[test]
fn catalog_populates_from_deployed_spec_tree() {
    let d = catalog("{}");
    let n = d["count"].as_u64().expect("count");
    assert!(n > 30, "expected the real rule pack, got {n}");

    let ids: Vec<&str> = d["rules"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["rule_id"].as_str().unwrap())
        .collect();
    // A checklist rule, a gh-wrapper rule, and a commit-msg rule all have to
    // show up — they come from three different files.
    for want in ["ccn", "IS-15", "CM-01"] {
        assert!(ids.contains(&want), "{want} missing from catalog");
    }
}

#[test]
fn catalog_severity_filter_returns_only_that_severity() {
    let d = catalog(r#"{"severity":"FAIL"}"#);
    let rules = d["rules"].as_array().unwrap();
    assert!(!rules.is_empty());
    assert!(
        rules.iter().all(|r| r["severity"] == "FAIL"),
        "severity filter leaked a non-FAIL rule"
    );
}

#[test]
fn catalog_resolves_hooks_through_dispatch_for_topic_rules() {
    let d = catalog("{}");
    let code = d["rules"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["rule_id"] == "code_rust")
        .expect("code_rust is a declared rule");
    // code_*.yaml declares no `hooks:`; dispatch.yaml is what routes it.
    let hooks: Vec<&str> = code["hooks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|h| h.as_str().unwrap())
        .collect();
    assert!(
        hooks.contains(&"pre-commit") && hooks.contains(&"pre-push"),
        "dispatch routing not applied: {hooks:?}"
    );
}

#[test]
fn explain_returns_the_rationale_prose() {
    let line = r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"spec_explain","arguments":{"rule_id":"ccn"}}}"#;
    let d = tool_text(&call(line));
    let why = d["why"].as_str().unwrap();
    assert!(
        why.len() > 40,
        "explain must carry the handbook prose, got {why:?}"
    );
    assert!(
        why.contains("复杂度") || why.contains("ccn"),
        "prose looks wrong: {why}"
    );
    assert!(
        d["next"].as_str().unwrap().contains("Do not weaken"),
        "explain must tell the agent not to route around the rule"
    );
}

#[test]
fn explain_rejects_an_unknown_rule_id() {
    let line = r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"spec_explain","arguments":{"rule_id":"nope"}}}"#;
    let v = call(line);
    assert_eq!(v["error"]["code"], -32602);
}

#[test]
fn preflight_hides_warn_by_default() {
    let line = r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"preflight","arguments":{"sla":"l1"}}}"#;
    let d = tool_text(&call(line));
    let sevs: Vec<&str> = d["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["severity"].as_str().unwrap())
        .collect();
    assert!(
        sevs.iter().all(|s| *s == "FAIL"),
        "WARN must be hidden unless include_warn: {sevs:?}"
    );
    assert!(
        d["would_block"].as_bool().unwrap() == !d["findings"].as_array().unwrap().is_empty(),
        "would_block must agree with the finding list"
    );
}

#[test]
fn preflight_reports_include_warn_when_asked() {
    let line = r#"{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"preflight","arguments":{"sla":"l1","include_warn":true}}}"#;
    let d = tool_text(&call(line));
    // The l1 pack is all-WARN or all-FAIL by construction; the point here is
    // that the flag widens the set rather than being silently ignored.
    let base = tool_text(&call(
        r#"{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"preflight","arguments":{"sla":"l1"}}}"#,
    ));
    assert!(d["findings"].as_array().unwrap().len() >= base["findings"].as_array().unwrap().len());
}

// ── target scoping ───────────────────────────────────────────────

/// Empty `extra` must not leave a dangling comma in the JSON body.
fn preflight(extra: &str) -> serde_json::Value {
    let extra = if extra.is_empty() {
        String::new()
    } else {
        format!(",{extra}")
    };
    let line = format!(
        r#"{{"jsonrpc":"2.0","id":9,"method":"tools/call","params":{{"name":"preflight","arguments":{{"sla":"l1","include_warn":true{extra}}}}}}}"#
    );
    tool_text(&call(&line))
}

#[test]
fn single_path_target_is_reported_as_scoped() {
    let d = preflight(r#""paths":["src/catalog.rs"]"#);
    assert_eq!(d["target"], "1 path(s)");
    assert!(
        d["note"].as_str().unwrap().contains("Scoped run"),
        "a path target must warn that grep rules ignore it"
    );
}

#[test]
fn directory_target_is_accepted() {
    // A directory is a valid pathspec; it expands to its tracked files.
    let d = preflight(r#""paths":["src/rules/"]"#);
    assert!(d["target"].as_str().unwrap().contains("path(s)"));
}

#[test]
fn commit_becomes_a_single_commit_range() {
    let d = preflight(r#""commit":"HEAD""#);
    assert_eq!(d["target"], "rev HEAD^..HEAD");
}

#[test]
fn branch_becomes_a_two_sided_base_range() {
    // A bare branch name would degrade to `git diff <branch>` (working tree vs
    // branch) — a different question from "what did this branch add".
    let d = preflight(r#""branch":"HEAD""#);
    let t = d["target"].as_str().unwrap();
    assert!(t.contains("..."), "branch must build a base range, got {t}");
}

#[test]
fn commit_plus_paths_narrows_rather_than_erroring() {
    let d = preflight(r#""target":{"commit":"HEAD","paths":["src/mcp.rs"]}"#);
    assert_eq!(d["target"], "1 path(s) since HEAD^..HEAD");
}

#[test]
fn conflicting_rev_selectors_are_rejected() {
    let line = r#"{"jsonrpc":"2.0","id":9,"method":"tools/call","params":{"name":"preflight","arguments":{"target":{"rev":"HEAD","commit":"HEAD"}}}}"#;
    let v = call(line);
    assert_eq!(v["error"]["code"], -32602);
}

#[test]
fn focus_narrows_the_rule_set() {
    let all = preflight("");
    let refactor = preflight(r#""focus":"refactor""#);
    let (Some(a), Some(r)) = (all["rules_run"].as_u64(), refactor["rules_run"].as_u64()) else {
        panic!("rules_run must be present");
    };
    assert!(
        r < a,
        "focus must run fewer rules than the full set ({r} vs {a})"
    );
}

#[test]
fn a_preset_name_resolves_to_its_rule_group() {
    let preset = preflight(r#""focus":"test""#);
    assert!(
        preset["rules_run"].as_u64().unwrap() >= 1,
        "the test preset must match at least one rule"
    );
}

#[test]
fn a_free_substring_resolves_without_a_preset() {
    let substring = preflight(r#""focus":"ccn""#);
    assert_eq!(
        substring["rules_run"], 1,
        "a bare substring should match exactly the ccn rule"
    );
}

#[test]
fn unmatched_focus_says_nothing_ran() {
    let d = preflight(r#""focus":"no_such_rule_xyz""#);
    assert!(
        d["note"]
            .as_str()
            .unwrap()
            .contains("not the same as passing"),
        "an empty focus result must not read as a pass"
    );
}
