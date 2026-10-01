//! MCP server surface: JSON-RPC 2.0 over stdio, newline-delimited.
//!
//! Hand-rolled rather than pulled from the Rust SDK on purpose — the wire
//! layer stays dep-free even with the flow tool family; the flow tools
//! themselves live in `crate::flow::tools`.
//!
//! Wire shapes match what `omenic`'s own MCP client speaks
//! (`crates/mcp`, protocol revision 2025-06-18), so both consumers are covered.
//!
//! Spec tools are shells over the engine; flow tools are shells over the
//! flow Store. No rule/task logic lives here.
//!
//! `preflight` runs the same `checklist_*.yaml` set the git hook would run and
//! reports FAIL only by default, so an agent learns about a violation while it
//! can still fix it rather than when the hook rejects the commit.

use serde_json::{Value, json};

use crate::catalog::{self, Rule};
use crate::engine::{self, SlaLevel};
use crate::shared::{self, Severity};

pub const PROTOCOL_VERSION: &str = "2025-06-18";

/// Handle one inbound line. `None` for notifications (no `id`) — those get no reply.
pub fn handle_line(line: &str) -> Option<String> {
    let req: Value = serde_json::from_str(line.trim()).ok()?;
    let method = req.get("method").and_then(Value::as_str).unwrap_or("");
    let params = req.get("params").cloned().unwrap_or(json!({}));
    // A notification carries no id and must not be answered.
    let id = req.get("id").cloned()?;

    Some(match dispatch(method, &params) {
        Ok(v) => json!({"jsonrpc": "2.0", "id": id, "result": v}).to_string(),
        Err((code, msg)) => {
            json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": msg}}).to_string()
        }
    })
}

type RpcResult = Result<Value, (i64, String)>;

fn dispatch(method: &str, params: &Value) -> RpcResult {
    match method {
        "initialize" => Ok(json!({
            "protocolVersion": PROTOCOL_VERSION,
            "capabilities": {"tools": {}},
            "serverInfo": {"name": "canon", "version": env!("CARGO_PKG_VERSION")},
        })),
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({"tools": tool_defs()})),
        "tools/call" => call_tool(params),
        other => Err((-32601, format!("method not found: {other}"))),
    }
}

fn tool_defs() -> Vec<Value> {
    let mut tools: Vec<Value> = vec![
        json!({
            "name": "spec_catalog",
            "description": "List every rule this repo enforces: id, effective severity, sla, hooks, and why it exists. Call this before writing code to learn what will be checked.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "severity": {"type": "string", "enum": ["FAIL", "WARN", "INFO"],
                                 "description": "Only return rules at this severity"},
                    "sla": {"type": "string", "enum": ["l1", "l2", "l3"],
                            "description": "Only return rules at this tier or below"}
                },
                "required": []
            }
        }),
        json!({
            "name": "spec_explain",
            "description": "Full text of one rule: what it forbids, how it detects it, which hook enforces it, and why. Call this before fixing a finding.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "rule_id": {"type": "string", "description": "Rule id from spec_catalog, e.g. 'ccn' or 'IS-04'"}
                },
                "required": ["rule_id"]
            }
        }),
        json!({
            "name": "preflight",
            "description": "Run the repo's checks now instead of waiting for the git hook to reject a commit. Scope to a branch diff, a commit, one or more paths, or a directory via `target`; narrow to a rule family via `focus`. Returns FAIL findings only by default, each with the rule's rationale inline.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "sla": {"type": "string", "enum": ["l1", "l2", "l3"],
                            "description": "Tier ceiling; default l1"},
                    "include_warn": {"type": "boolean",
                                     "description": "Also return WARN findings; default false"},
                    "focus": {"type": "string",
                              "description": "Comma-separated rule families: test, refactor, security, style, size, docs, lint — or bare rule substrings like 'antislop'. Omit for everything."},
                    "target": {
                        "type": "object",
                        "description": "What to check. Omit for the staged working tree.",
                        "properties": {
                            "scope": {"type": "string", "enum": ["staged", "unstaged", "branch", "merge"],
                                      "description": "staged = git diff --cached (default); unstaged = working tree vs HEAD; branch = merge-base..HEAD; merge = GATE_BASE range"},
                            "rev": {"type": "string", "description": "Explicit git range, e.g. 'main..HEAD' or 'HEAD~3'"},
                            "commit": {"type": "string", "description": "One commit sha — checks that commit's own diff (<sha>^..<sha>)"},
                            "branch": {"type": "string", "description": "Branch name — checks origin/main...<branch>"},
                            "paths": {"type": "array", "items": {"type": "string"},
                                      "description": "Files or directories to restrict to, e.g. ['src/rules/', 'Cargo.toml']. A directory expands to its git-tracked files."}
                        },
                        "required": []
                    }
                },
                "required": []
            }
        }),
    ];
    tools.extend(crate::flow::tools::tool_defs());
    tools
}

fn arg_str(params: &Value, key: &str) -> Option<String> {
    params.get(key).and_then(Value::as_str).map(str::to_string)
}

fn arg_bool(params: &Value, key: &str) -> bool {
    params.get(key).and_then(Value::as_bool).unwrap_or(false)
}

/// No spec tree means every check would silently pass — say so instead.
fn spec_dir() -> Result<std::path::PathBuf, (i64, String)> {
    engine::spec_dir().ok_or((
        -32000,
        "no .githooks/spec found — run `canon init` in this repo".into(),
    ))
}

fn call_tool(params: &Value) -> RpcResult {
    let name = arg_str(params, "name").unwrap_or_default();
    let args = params.get("arguments").cloned().unwrap_or(json!({}));
    let text = match name.as_str() {
        "spec_catalog" => catalog_tool(&args)?,
        "spec_explain" => explain_tool(&args)?,
        "preflight" => preflight_tool(&args)?,
        "board_view" | "task_get" | "project_list" | "journal" | "project_create"
        | "project_update" | "task_create" | "task_claim" | "task_transition" | "step_add"
        | "step_mark" | "event_note" | "spec_list" | "spec_run" | "spec_bind" | "template_list" => {
            crate::flow::tools::call(&name, &args)?
        }
        other => return Err((-32602, format!("unknown tool: {other}"))),
    };
    Ok(json!({"content": [{"type": "text", "text": text}], "isError": false}))
}

fn rule_json(r: &Rule) -> Value {
    json!({
        "rule_id": r.id,
        "severity": r.severity.as_str(),
        "sla": r.sla,
        "hooks": r.hooks,
        "mode": r.mode,
        "source": r.source,
        "why": r.why.lines().next().unwrap_or(""),
    })
}

fn sla_rank(s: &str) -> u8 {
    match s {
        "l1" => 1,
        "l2" => 2,
        _ => 3,
    }
}

fn pretty(v: Value) -> String {
    serde_json::to_string_pretty(&v).unwrap_or_default()
}

fn catalog_tool(args: &Value) -> Result<String, (i64, String)> {
    let dir = spec_dir()?;
    let want_sev = arg_str(args, "severity").and_then(|s| Severity::parse(&s));
    let want_sla = arg_str(args, "sla");
    let rules: Vec<Value> = catalog::load(&dir)
        .into_iter()
        .filter(|r| want_sev.is_none_or(|s| r.severity == s))
        .filter(|r| {
            want_sla
                .as_ref()
                .is_none_or(|s| sla_rank(s) <= sla_rank(r.sla.as_deref().unwrap_or("l3")))
        })
        .map(|r| rule_json(&r))
        .collect();
    Ok(pretty(json!({
        "spec_dir": dir.display().to_string(),
        "count": rules.len(),
        "rules": rules,
    })))
}

fn explain_tool(args: &Value) -> Result<String, (i64, String)> {
    let id = arg_str(args, "rule_id").ok_or((-32602, "rule_id is required".to_string()))?;
    let dir = spec_dir()?;
    let rule = catalog::load(&dir)
        .into_iter()
        .find(|r| r.id.eq_ignore_ascii_case(&id))
        .ok_or((
            -32602,
            format!("no rule named {id}; call spec_catalog first"),
        ))?;
    Ok(pretty(json!({
        "rule_id": rule.id,
        "severity": rule.severity.as_str(),
        "sla": rule.sla,
        "hooks": rule.hooks,
        "mode": rule.mode,
        "source": rule.source,
        "why": rule.why,
        "next": "Fix the code, then re-run preflight. Do not weaken the rule, edit .githooks/spec/, or add an allow/ignore to get past it."
    })))
}

/// Turn the `target` object into an engine [`engine::Target`].
///
/// `commit`, `rev`, and `branch` are mutually exclusive on purpose — silently
/// preferring one would make a caller believe it checked the other.
///
/// Scope keys are read from `target` or, absent that, from the top level. The
/// lenient fallback is deliberate: a caller that guesses `paths` instead of
/// `target.paths` must get the scope it asked for. Ignoring it would return a
/// confident report about the *staged tree* while the caller believes it
/// reviewed a commit.
fn resolve_target(args: &Value) -> Result<engine::Target, (i64, String)> {
    let t = args.get("target").cloned().unwrap_or_else(|| args.clone());
    let paths: Vec<String> = t
        .get("paths")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    let has_paths = !paths.is_empty();

    let revs: Vec<&str> = ["rev", "commit", "branch"]
        .iter()
        .filter_map(|k| t.get(*k).and_then(Value::as_str))
        .filter(|s| !s.is_empty())
        .collect();
    if revs.len() > 1 {
        return Err((
            -32602,
            "target: use only one of rev / commit / branch".to_string(),
        ));
    }
    if let Some(rev) = revs.first() {
        let range = match t.get("commit").and_then(Value::as_str) {
            Some(sha) => format!("{sha}^..{sha}"),
            None => match t.get("branch").and_then(Value::as_str) {
                // A branch is "what this branch added", which needs both ends of
                // the range; passing the bare name would silently degrade to
                // `git diff <branch>` (working tree vs branch) instead.
                Some(b) => format!("{}...{b}", engine::base_ref()),
                None => (*rev).to_string(),
            },
        };
        // A range plus a path list is a legitimate combination — one commit's
        // diff restricted to two files — so narrow instead of erroring.
        if has_paths {
            Ok(engine::Target::Files {
                paths,
                base: Some(range),
            })
        } else {
            Ok(engine::Target::Rev(range))
        }
    } else if has_paths {
        Ok(engine::Target::Files { paths, base: None })
    } else {
        Ok(match arg_str(&t, "scope").as_deref() {
            Some("unstaged") => engine::Target::Hook(engine::HookScope::PrePush),
            Some("branch") | Some("merge") => engine::Target::Hook(engine::HookScope::Merge),
            _ => engine::Target::Hook(engine::HookScope::PreCommit),
        })
    }
}

fn preflight_tool(args: &Value) -> Result<String, (i64, String)> {
    let sla = arg_str(args, "sla").unwrap_or_else(|| "l1".into());
    let include_warn = arg_bool(args, "include_warn");
    let focus = arg_str(args, "focus").unwrap_or_default();
    let dir = spec_dir()?;
    let rules = catalog::load(&dir);
    let by_id: std::collections::HashMap<String, Rule> = rules
        .iter()
        .map(|r| (r.id.to_lowercase(), r.clone()))
        .collect();
    // Only `checklist_*.yaml` is runnable by name. The `code_*` / `cleanup_*`
    // topic files go through their own dispatchers, and the IS/PR/RV/CM ids are
    // gh-wrapper rules — handing those to run_targeted just prints "unknown
    // checklist" once per id.
    let ceiling = sla_rank(&sla);
    let names: Vec<String> = rules
        .iter()
        .filter(|r| r.source.contains("checklist_"))
        .filter(|r| r.sla.as_deref().is_some_and(|s| sla_rank(s) <= ceiling))
        .filter(|r| focus.is_empty() || catalog::focus_matches(&focus, &r.id))
        .map(|r| r.id.clone())
        .collect();
    if names.is_empty() {
        return Ok(pretty(json!({
            "sla": sla,
            "focus": focus,
            "would_block": false,
            "blocking": 0,
            "findings": [],
            "note": "No rule matched this sla/focus combination — nothing ran, which is not the same as passing.",
        })));
    }
    let target = resolve_target(args)?;
    let mut findings = engine::run_targeted(target.clone(), &names, SlaLevel::parse(&sla));
    shared::apply_global_overrides(&mut findings);

    let kept: Vec<Value> = findings
        .into_iter()
        .filter(|f| f.severity == Severity::Fail || include_warn)
        .map(|f| {
            let why = by_id
                .get(&f.rule_id.to_lowercase())
                .and_then(|r| r.why.lines().next())
                .unwrap_or("");
            let mut v = f.to_json();
            if let Some(o) = v.as_object_mut() {
                o.insert("why".into(), json!(why));
            }
            v
        })
        .collect();

    let blocking = kept
        .iter()
        .filter(|f| f.get("severity").and_then(Value::as_str) == Some("FAIL"))
        .count();
    let scoped = !matches!(target, engine::Target::Hook(_));
    Ok(pretty(json!({
        "sla": sla,
        "focus": focus,
        "target": target.describe(),
        "rules_run": names.len(),
        "would_block": blocking > 0,
        "blocking": blocking,
        "findings": kept,
        "note": if scoped {
            "Scoped run. grep-mode rules (mode: grep) run their own repo-wide scan and ignore the target; treat their findings as whole-repo, not scoped."
        } else {
            "Runs every declared rule at this tier, merge-only ones included. WARN is hidden unless include_warn."
        }
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(tools.len(), 19);
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
}
