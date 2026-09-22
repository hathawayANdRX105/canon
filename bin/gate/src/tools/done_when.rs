//! DWJ: Done-when judge — external model hook at issue close.
//!
//! After the GT-04 mechanical checkbox gate passes, every Done when item is
//! judged against real evidence by a yaml-configured harness running the
//! same three-tier degradation as review_chain (jev -> small LLM -> none).
//! Policy layer, never fail-closed: every infra failure degrades to INFO so
//! GT-04 + tool checklists remain the backstop.

use std::io::Write;
use std::process::Command;

use crate::tools::gh_wrap::run_gh;
use regex::Regex;
use serde_yaml::Value as YamlValue;

use crate::shared::{Finding, Severity};

// ---------------------------------------------------------------------------
// DWJ: Done-when judge — external model hook at issue close
// ---------------------------------------------------------------------------

/// Master switch: `github_issues.yaml` → `done_when_judge.enabled`.
/// Absent map or absent flag → disabled (opt-in mechanism).
pub fn dwj_enabled(cfg: Option<&YamlValue>) -> bool {
    cfg.and_then(|c| c.get("done_when_judge"))
        .and_then(|d| d.get("enabled"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

/// Extract every checkbox item text (ticked or not) from the Done when
/// section. GT-04 already hard-requires them ticked; the judge re-checks
/// the *content* against real evidence.
fn done_when_item_texts(body: &str, cfg: Option<&YamlValue>) -> Vec<String> {
    let done = crate::rules::issues::done_when_section(body, cfg);
    let re = Regex::new(r"(?m)^\s*-\s*\[[ xX]\]\s*(.+)").unwrap();
    re.captures_iter(&done)
        .filter_map(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
        .filter(|s| !s.is_empty())
        .collect()
}

/// Build the judge payload: acceptance items + evidence (linked PR diff,
/// else the close comment). Pure — IO done by the caller.
pub fn build_judge_state(num: &str, items: &[String], evidence: &str) -> String {
    let evidence = if evidence.len() > 12000 {
        crate::shared::truncate_utf8(evidence, 12000).to_string()
    } else {
        evidence.to_string()
    };
    serde_json::json!({
        "issue": num,
        "done_when_items": items,
        "evidence": evidence,
    })
    .to_string()
}

/// Parse harness stdout (findings JSON array) into Findings. Any structural
/// failure (not JSON / not an array / missing fields) degrades to a single
/// INFO — the judge is a policy layer, never a fail-closed safety layer.
pub fn parse_judge_findings(stdout: &str) -> Vec<Finding> {
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        return judge_skipped("empty judge output");
    }
    let parsed: serde_json::Value = match serde_json::from_str(trimmed) {
        Ok(v) => v,
        Err(_) => return judge_skipped("judge output not JSON"),
    };
    let Some(arr) = parsed.as_array() else {
        return judge_skipped("judge output not an array");
    };
    let mut out = Vec::new();
    for f in arr {
        let Some(id) = f.get("id").and_then(|v| v.as_str()) else {
            continue;
        };
        let sev = f
            .get("severity")
            .and_then(|v| v.as_str())
            .and_then(Severity::parse)
            .unwrap_or(Severity::Info);
        let msg = f.get("message").and_then(|v| v.as_str()).unwrap_or("");
        let mut finding = Finding::new(id, sev, msg);
        if let Some(tier) = f.get("tier").and_then(|v| v.as_str()) {
            finding = finding.with_extra("tier", tier);
        }
        if let Some(conf) = f.get("confidence").and_then(|v| v.as_str()) {
            finding = finding.with_extra("confidence", conf);
        }
        out.push(finding);
    }
    if out.is_empty() {
        return judge_skipped("judge produced no findings");
    }
    out
}

fn judge_skipped(reason: &str) -> Vec<Finding> {
    vec![Finding::new(
        "DWJ-SKIPPED",
        Severity::Info,
        &format!("Done-when judge skipped: {reason}；机械门(GT-04)与工具检查仍生效"),
    )]
}

/// First linked PR's raw diff, else empty. Best-effort: used as judge
/// evidence, never as a gate by itself.
fn linked_pr_diff(repo: &str, num: &str) -> String {
    let (rc, tl, _) = run_gh(
        &[
            "api".to_string(),
            format!("repos/{repo}/issues/{num}/timeline"),
            "--jq".to_string(),
            "[.[] | select(.event == \"cross-referenced\" and .source.issue.pull_request != null) | .source.issue.number] | first // empty".to_string(),
        ],
        None,
    );
    if rc != 0 {
        return String::new();
    }
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&tl) else {
        return String::new();
    };
    let Some(pr) = v.as_u64().or_else(|| {
        v.as_array()
            .and_then(|a| a.first())
            .and_then(|x| x.as_u64())
    }) else {
        return String::new();
    };
    let (rc2, diff, _) = run_gh(&[
        "api".to_string(),
        format!("repos/{repo}/pulls/{pr}"),
        "-H".to_string(),
        "Accept: application/vnd.github.diff".to_string(),
    ],
    None,
    );
    if rc2 == 0 { diff } else { String::new() }
}

/// Close comment text from the intercept args, else empty.
fn close_comment(args: &[String]) -> String {
    let mut out = String::new();
    for (i, a) in args.iter().enumerate() {
        if a == "--comment" || a == "-c" {
            if let Some(v) = args.get(i + 1) {
                out.push_str(v);
            }
        } else if let Some(rest) = a.strip_prefix("--comment=") {
            out.push_str(rest);
        }
    }
    out
}

/// Run the yaml-configured judge harness with a timeout; INFRA failure
/// (missing binary / spawn error / timeout) → SKIPPED INFO, never a block.
fn run_judge_command(command: &str, args: &[String], state: &str, timeout_secs: u64) -> String {
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();
    let cmd = command.to_string();
    let argv: Vec<String> = args.to_vec();
    let payload = state.to_string();
    std::thread::spawn(move || {
        let child = Command::new(&cmd)
            .args(&argv)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn();
        let result = match child {
            Ok(mut c) => {
                if let Some(mut stdin) = c.stdin.take() {
                    let _ = stdin.write_all(payload.as_bytes());
                }
                c.wait_with_output()
            }
            Err(e) => Err(e),
        };
        let _ = tx.send(result);
    });
    match rx.recv_timeout(std::time::Duration::from_secs(timeout_secs)) {
        Ok(Ok(o)) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Ok(Err(_)) => String::new(),
        Err(_) => String::new(),
    }
}

/// DWJ orchestration: build state, run the harness, parse findings. Every
/// degradation path returns INFO so GT-04 + tool checklists remain the
/// backstop.
pub fn run_done_when_judge(
    repo: &str,
    num: &str,
    body: &str,
    close_args: &[String],
    cfg: Option<&YamlValue>,
) -> Vec<Finding> {
    let Some(dj) = cfg.and_then(|c| c.get("done_when_judge")) else {
        return Vec::new();
    };
    let command = dj.get("command").and_then(|v| v.as_str()).unwrap_or("");
    if command.is_empty() {
        return judge_skipped("done_when_judge.command empty");
    }
    let args: Vec<String> = dj
        .get("args")
        .and_then(|v| v.as_sequence())
        .map(|s| {
            s.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let timeout = dj
        .get("timeout_secs")
        .and_then(|v| v.as_u64())
        .unwrap_or(180);

    let items = done_when_item_texts(body, cfg);
    if items.is_empty() {
        return vec![Finding::new(
            "DWJ-INFO",
            Severity::Info,
            "Done when 无 checkbox 项，judge n/a",
        )];
    }
    let evidence = {
        let diff = linked_pr_diff(repo, num);
        if !diff.is_empty() {
            diff
        } else {
            close_comment(close_args)
        }
    };
    let state = build_judge_state(num, &items, &evidence);
    // Harness args reference `.githooks/spec/...` — run from the repo root.
    let mut cmd_args = args;
    if let Some(root) = crate::tools::git::git_root() {
        let resolved: Vec<String> = cmd_args
            .iter()
            .map(|a| {
                if a.starts_with(".githooks/") {
                    root.join(a).to_string_lossy().to_string()
                } else {
                    a.clone()
                }
            })
            .collect();
        cmd_args = resolved;
    }
    println!("--- done_when judge (jev -> 小模型 -> 跳过) ---");
    let out = run_judge_command(command, &cmd_args, &state, timeout);
    let mut findings = parse_judge_findings(&out);
    crate::shared::apply_global_overrides(&mut findings);
    findings
}


#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(yaml: &str) -> Option<YamlValue> {
        serde_yaml::from_str(yaml).ok()
    }

    const DW_CFG: &str = "heading_names:\n  done_when: \"Done when\"\n";

    #[test]
    fn dwj_switch_is_opt_in() {
        assert!(!dwj_enabled(None));
        assert!(!dwj_enabled(cfg("other: 1\n").as_ref()));
        // Missing done_when_judge map or flag → off.
        assert!(!dwj_enabled(cfg("done_when_judge:\n  command: python3\n").as_ref()));
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
}
