//! gh interception gate — GT-01..GT-07
//!
//! When `gate` is installed as `~/.local/bin/gh`, it intercepts
//! `gh issue create/close` and `gh pr create/merge`, validates via rules,
//! and passes through everything else to the real gh.

use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use regex::Regex;
use serde_yaml::Value as YamlValue;

use crate::rules::{issues, pull_requests};
use crate::shared::{Finding, Severity};

const LOG_DIR: &str = ".local/share/gh-gate";
const LOG_FILE: &str = "gate.log";

fn timestamp() -> String {
    let d = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = d.as_secs();
    // crude ISO-ish: seconds since epoch is enough for log ordering
    format!("{}", secs)
}

/// Find the real gh binary in PATH, skipping our own executable.
///
/// gate is deployed as BOTH `~/.local/bin/gate` and `~/.local/bin/gh`
/// (argv[0]==gh interception). When gate-as-gh runs `--version` it passes
/// through to the real gh, so version sniffing cannot distinguish them —
/// skip the gate install dir (`~/.local/bin`) plus same-file candidates.
pub fn find_real_gh() -> String {
    let self_path = env::current_exe().ok();
    let self_resolved = self_path.as_ref().and_then(|p| p.canonicalize().ok());
    let gate_dir =
        env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".local").join("bin"));

    if let Some(path) = env::var_os("PATH") {
        for dir in env::split_paths(&path) {
            let candidate = dir.join("gh");
            if candidate.is_file() {
                if let Ok(resolved) = candidate.canonicalize()
                    && let Some(s) = &self_resolved
                    && resolved == *s
                {
                    continue;
                }
                // gate's install dir — this is the intercept binary, not the
                // real gh. The real gh lives elsewhere on PATH or in fallbacks.
                if gate_dir.as_ref().is_some_and(|gd| dir == *gd) {
                    continue;
                }
                // Sanity: must report a gh version (covers gate-as-gh copies
                // outside ~/.local/bin, e.g. during tests).
                if let Ok(out) = Command::new(&candidate).arg("--version").output() {
                    let ver = String::from_utf8_lossy(&out.stdout);
                    if out.status.success() && ver.trim_start().starts_with("gh version") {
                        return candidate.to_string_lossy().to_string();
                    }
                }
            }
        }
    }
    for fallback in ["/usr/bin/gh", "/usr/local/bin/gh", "/bin/gh"] {
        if Path::new(fallback).is_file() {
            return fallback.to_string();
        }
    }
    "gh".to_string()
}

/// Run the real gh binary with args, capturing stdout+stderr. Returns (rc, stdout, stderr).
pub fn run_gh(args: &[String], _input: Option<&str>) -> (i32, String, String) {
    let gh = find_real_gh();
    let mut cmd = Command::new(&gh);
    cmd.args(args);
    match cmd.output() {
        Ok(out) => (
            out.status.code().unwrap_or(1),
            String::from_utf8_lossy(&out.stdout).to_string(),
            String::from_utf8_lossy(&out.stderr).to_string(),
        ),
        Err(e) => (1, String::new(), format!("failed to run gh: {e}")),
    }
}

/// Final pass-through: spawn the real gh with inherited stdio and wait.
///
/// 之前用 Command::output() 捕获 stdout/stderr,交互命令(gh auth login 的 device-flow
/// 等码、browse、任何 TUI)会因 stdin 被 PIPE 吞掉而挂死/静默——实测 auth login 卡 30s
/// 无输出。放行路径必须原样继承终端。
pub fn passthrough(args: &[String]) -> i32 {
    let gh = find_real_gh();
    match Command::new(&gh)
        .args(args)
        .spawn()
        .and_then(|mut c| c.wait())
    {
        Ok(status) => status.code().unwrap_or(1),
        Err(e) => {
            eprintln!("failed to run gh: {e}");
            1
        }
    }
}

fn read_body_file(path: &str) -> Result<String, String> {
    if path == "-" {
        return Err(
            "--body-file - is not supported; pass --body or a readable file path".to_string(),
        );
    }
    fs::read_to_string(path).map_err(|e| format!("failed to read --body-file '{path}': {e}"))
}

/// Extract (title, body, labels, head, parent) from gh args. Mirrors Python `_extract`.
pub fn extract(args: &[String]) -> Result<(String, String, Vec<String>, String, String), String> {
    let mut title = String::new();
    let mut body = String::new();
    let mut head = String::new();
    let mut parent = String::new();
    let mut labels: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        let next = args.get(i + 1).map(|s| s.as_str());
        match a.as_str() {
            "--title" | "-t" => {
                if let Some(v) = next {
                    title = v.to_string();
                    i += 1;
                }
            }
            "--body" | "-b" => {
                if let Some(v) = next {
                    body = v.to_string();
                    i += 1;
                }
            }
            "--body-file" => {
                if let Some(v) = next {
                    body = read_body_file(v)?;
                    i += 1;
                }
            }
            "--label" | "-l" => {
                if let Some(v) = next {
                    labels.extend(v.split(',').map(|s| s.to_string()));
                    i += 1;
                }
            }
            "--head" | "-H" => {
                if let Some(v) = next {
                    head = v.to_string();
                    i += 1;
                }
            }
            "--parent" | "-P" => {
                if let Some(v) = next {
                    parent = v.trim_start_matches('#').to_string();
                    i += 1;
                }
            }
            _ => {
                if let Some(stripped) = a.strip_prefix("--title=") {
                    title = stripped.to_string();
                } else if let Some(stripped) = a.strip_prefix("--body=") {
                    body = stripped.to_string();
                } else if let Some(stripped) = a.strip_prefix("--body-file=") {
                    body = read_body_file(stripped)?;
                } else if let Some(stripped) = a.strip_prefix("--label=") {
                    labels.extend(stripped.split(',').map(|s| s.to_string()));
                } else if let Some(stripped) = a.strip_prefix("--head=") {
                    head = stripped.to_string();
                } else if let Some(stripped) = a.strip_prefix("--parent=") {
                    parent = stripped.trim_start_matches('#').to_string();
                }
            }
        }
        i += 1;
    }
    Ok((title, body, labels, head, parent))
}

/// Strip gate-only flags (--parent) from args before passing to real gh.
pub fn gh_args(args: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut skip = false;
    for a in args {
        if skip {
            skip = false;
            continue;
        }
        if a == "--parent" || a == "-P" || a == "--repo" || a == "-R" {
            skip = true;
            continue;
        }
        if a.starts_with("--parent=") || a.starts_with("--repo=") {
            continue;
        }
        out.push(a.clone());
    }
    out
}

/// Extract the `--repo X` / `-R X` / `--repo=X` value from gh args, if any.
fn arg_repo(args: &[String]) -> Option<String> {
    let mut i = 0;
    while i < args.len() {
        if (args[i] == "--repo" || args[i] == "-R") && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
        if let Some(v) = args[i].strip_prefix("--repo=") {
            return Some(v.to_string());
        }
        i += 1;
    }
    None
}

/// Append a line to ~/.local/share/gh-gate/gate.log.
pub fn log(action: &str, target: &str, result: &str, detail: &str) {
    let home = match env::var_os("HOME") {
        Some(h) => PathBuf::from(h),
        None => return,
    };
    let dir = home.join(LOG_DIR);
    let file = dir.join(LOG_FILE);
    if let Ok(()) = fs::create_dir_all(&dir)
        && let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&file)
    {
        let _ = writeln!(
            f,
            "{} | {action} | {target} | {result} | {detail}",
            timestamp()
        );
    }
}

/// Derive repo from `git remote get-url origin`.
pub fn derive_repo() -> String {
    crate::tools::git::derive_repo().unwrap_or_default()
}

/// Check all checkboxes in a body are ticked. Returns (all_ticked, unticked_items).
pub fn check_all_checkboxes(body: &str) -> (bool, Vec<String>) {
    let unticked_re = Regex::new(r"(?m)^\s*-\s*\[\s\]\s*(.+)").unwrap();
    let mut unticked = Vec::new();
    for cap in unticked_re.captures_iter(body) {
        unticked.push(cap.get(1).unwrap().as_str().trim().to_string());
    }
    (unticked.is_empty(), unticked)
}

fn extract_fixes(body: &str) -> Vec<String> {
    let re = Regex::new(r"(?i)(?:Fixes|Closes|Resolves)\s+#(\d+)").unwrap();
    re.captures_iter(body)
        .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

fn is_epic(labels: &[String]) -> bool {
    labels.iter().any(|l| l.eq_ignore_ascii_case("epic"))
}

/// GT-06 pure decision: given an issue's labels and the list of currently-open
/// sub-issue numbers under it, return the blocker list (open sub-issues) when the
/// close/merge must be denied, or `None` when allowed (not an epic, or epic with
/// all subs closed). Pure — testable without real gh.
fn gt06_open_sub_block(labels: &[String], open_subs: &[String]) -> Option<Vec<String>> {
    if !is_epic(labels) {
        return None;
    }
    if open_subs.is_empty() {
        return None;
    }
    Some(open_subs.to_vec())
}

/// GT-04 pure decision: only the Done when section gates close — Implementation
/// Order progress boxes (epic) and other lists must not block. Returns the
/// unticked items that block, empty when close is allowed.
fn gt04_unticked_done_when(body: &str, cfg: Option<&YamlValue>) -> Vec<String> {
    let done = crate::rules::issues::done_when_section(body, cfg);
    let (ok, unticked) = check_all_checkboxes(&done);
    if ok { Vec::new() } else { unticked }
}

/// GT-04b pure decision: epics are exempt (their completion signal is GT-06
/// all-subs-closed; PRs link to sub-issues, not the epic). Non-epic issues
/// without a linked PR only WARN (demoted from FAIL) — legitimate
/// non-PR closes (duplicate/obsolete) stay possible.
fn gt04b_warn_no_link(labels: &[String], has_linked_pr: bool) -> bool {
    !is_epic(labels) && !has_linked_pr
}

/// GT-05 pure decision: unticked Done when boxes of a Fixes target that block
/// merge. Epic targets return empty — IS-11 forbids Done when on epics and
/// GT-06 already guards epic completion via open sub-issues.
fn gt05_unticked_issue_boxes(
    labels: &[String],
    issue_body: &str,
    cfg: Option<&YamlValue>,
) -> Vec<String> {
    if is_epic(labels) {
        return Vec::new();
    }
    let done = crate::rules::issues::done_when_section(issue_body, cfg);
    let (ok, unticked) = check_all_checkboxes(&done);
    if ok { Vec::new() } else { unticked }
}

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
fn run_done_when_judge(
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

/// Heuristic for the GT-01 hint: title mentions epic or the body carries an
/// Implementation Order section → the issue looks like an epic created
/// without the `epic` label.
fn looks_like_epic(title: &str, body: &str) -> bool {
    title.to_lowercase().contains("epic") || body.contains("## Implementation Order")
}

/// Query the GitHub sub_issues endpoint and return the numbers whose state is
/// `open`. Returns `Err` on any API failure (so the caller can BLOCK rather
/// than silently allow — failing closed on an unverifiable epic-close check
/// is the safe direction). `Ok(vec![])` only on a genuine "no open subs".
///
/// Single API call: the sub_issues response already carries each sub-issue's
/// `.state`, so the jq filter selects open ones directly (no N+1 per-sub query).
fn query_open_subs(repo: &str, num: &str) -> Result<Vec<String>, String> {
    if num.is_empty() || !num.chars().all(|c| c.is_ascii_digit()) {
        return Err(format!("非法 issue 号: {num}"));
    }
    let (rc, subs_json, err) = run_gh(
        &[
            "api".to_string(),
            format!("repos/{repo}/issues/{num}/sub_issues"),
            "--jq".to_string(),
            r#".[] | select(.state == "open") | .number"#.to_string(),
        ],
        None,
    );
    if rc != 0 {
        return Err(format!("sub_issues 查询失败 (rc={rc}): {}", err.trim()));
    }
    if subs_json.trim().is_empty() {
        return Ok(vec![]); // truly no open sub-issues
    }
    let mut open = Vec::new();
    for sn in subs_json.split_whitespace() {
        match sn.parse::<u32>() {
            Ok(_) => open.push(sn.to_string()),
            Err(_) => {
                let shown = crate::shared::truncate_utf8(sn, 20);
                return Err(format!("sub_issues 响应含非法编号: '{shown}'"));
            }
        }
    }
    Ok(open)
}

// ---------------------------------------------------------------------------
// Interceptions
// ---------------------------------------------------------------------------

/// GT-01 + GT-03: issue create
pub fn intercept_issue_create(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--disable-check") {
        log("ISSUE_CREATE", "?", "BYPASS", "--disable-check");
        println!("⚠ 闸门: --disable-check 跳过校验（已记入 gate.log；仅本次调用生效）");
        let clean: Vec<String> = args
            .iter()
            .filter(|a| *a != "--disable-check")
            .cloned()
            .collect();
        let mut full = vec!["issue".to_string(), "create".to_string()];
        full.extend(clean);
        return passthrough(&full);
    }

    let (title, body, labels, _, parent) = match extract(args) {
        Ok(parts) => parts,
        Err(error) => {
            println!("闸门: {error}");
            log("ISSUE_CREATE", "?", "REJECT", &error);
            return 1;
        }
    };
    let repo = derive_repo();
    let mode = if is_epic(&labels) { "parent" } else { "sub" };
    let labels_str: Vec<&str> = labels.iter().map(String::as_str).collect();
    let cfg = crate::shared::load_spec_yaml("github_issues.yaml");
    let mut findings =
        issues::check_content(&title, &body, &labels_str, mode, "open", cfg.as_ref());
    // Apply global severity overrides
    crate::shared::apply_global_overrides(&mut findings);
    let fails: Vec<&Finding> = findings
        .iter()
        .filter(|f| f.severity == Severity::Fail)
        .collect();
    for f in &findings {
        if f.severity <= Severity::Warn {
            println!("{}\t{}", f.severity.as_str(), f.msg);
        }
    }
    if !fails.is_empty() {
        if mode == "sub" && looks_like_epic(&title, &body) {
            println!("提示: 该 issue 看起来是 epic（标题含 epic 或正文有 Implementation Order）。");
            println!("  epic 不需要 Done when，请带 --label epic 重新创建。");
        }
        println!("闸门: 校验 FAIL，拒绝创建。修正后重试。");
        log(
            "ISSUE_CREATE",
            crate::shared::truncate_utf8(&title, 40),
            "REJECT",
            &format!("FAIL={}", fails.len()),
        );
        return 1;
    }

    println!("闸门: 检查通过，执行 gh ...");
    let clean = gh_args(args);
    let mut full = vec!["issue".to_string(), "create".to_string()];
    full.extend(clean);
    let (rc, out, err) = run_gh(&full, None);
    if !out.is_empty() {
        print!("{out}");
    }
    if !err.is_empty() {
        eprint!("{err}");
    }
    if rc != 0 {
        return rc;
    }

    let url = out.trim().to_string();
    // `sub_issue_must_link_parent` off ⇒ the spec opts this repo out of the
    // addSubIssue enforcement entirely.
    let must_link =
        crate::shared::cfg_bool(cfg.as_ref(), "sub_issue_must_link_parent").unwrap_or(false);
    if url.starts_with("https://github.com/")
        && url.contains("/issues/")
        && !is_epic(&labels)
        && must_link
        && !parent.is_empty()
        && parent != "0"
    {
        // IS-09/Linkage gate: body text like "Parent: #N" is already
        // rejected by check_content above.  Here we verify the REAL
        // addSubIssue mutation actually mounted the sub-issue.
        if !auto_link_sub(&url, &repo, &parent) {
            // Issue is ALREADY created — do not return 1 or the user retries
            // and duplicates it. Warn loudly and let them fix linkage manually.
            println!("\n⚠ 闸门: issue 已创建 ({url})，但自动挂载到 parent #{parent} 失败。");
            println!(
                "  issue 未回滚。请运行: gh api repos/{repo}/issues/{parent}/sub_issues -X POST -F sub_issue_id=<id>"
            );
            log(
                "ISSUE_CREATE",
                crate::shared::truncate_utf8(&title, 40),
                "WARN",
                "created but auto_link failed",
            );
            return 2; // 部分成功：issue 已创建但挂载失败，非 0 以区分全成功
        }
        if let Some(sub_num) = extract_num(&url, "/issues/")
            && !verify_mount(&repo, &sub_num, &parent)
        {
            // Retry once: GitHub sub_issues list may lag the mutation.
            std::thread::sleep(std::time::Duration::from_millis(800));
            if !verify_mount(&repo, &sub_num, &parent) {
                println!(
                    "\n⚠ 闸门: issue 已创建 ({url})，但挂载验证失败（eventual consistency 重试后仍未出现）。"
                );
                println!(
                    "  issue 未回滚。请运行: gh api repos/{repo}/issues/{parent}/sub_issues -X POST -F sub_issue_id=<id>"
                );
                log(
                    "ISSUE_CREATE",
                    crate::shared::truncate_utf8(&title, 40),
                    "WARN",
                    "created but mount verify failed",
                );
                return 2; // 部分成功
            }
        }
    }
    0
}

/// Load `dispatch.yaml` for gate-block severity overrides.
fn load_dispatch_cfg() -> Option<YamlValue> {
    crate::tools::git::find_githooks_dir()
        .map(|d| d.join("spec/dispatch.yaml"))
        .and_then(|p| crate::shared::load_yaml(p.to_str().unwrap_or("")).ok())
        .filter(|v| !v.is_null())
}

/// Policy switch: `cfg.<key>` defaults to enabled (true) when absent.
fn gate_switch(cfg: Option<&YamlValue>, key: &str) -> bool {
    crate::shared::cfg_bool(cfg, key).unwrap_or(true)
}

/// Final decision for a gh gate block: dispatch-level + global severity
/// overrides, print, log; Some(1) = blocked, None = proceed.
/// Fail-closed data errors never reach here — they hard-return 1 above.
fn gate_block(
    action: &str,
    target: &str,
    findings: &mut Vec<Finding>,
    dispatch: Option<&YamlValue>,
) -> Option<i32> {
    crate::shared::apply_severity_overrides(findings, dispatch);
    crate::shared::apply_global_overrides(findings);
    let fails: Vec<&Finding> = findings
        .iter()
        .filter(|f| f.severity == Severity::Fail)
        .collect();
    for f in findings.iter().filter(|f| f.severity == Severity::Warn) {
        println!("闸门 WARN [{}]: {}", f.rule_id, f.msg);
    }
    if fails.is_empty() {
        return None;
    }
    for f in &fails {
        println!("闸门 [{}]: {}", f.rule_id, f.msg);
    }
    log(
        action,
        target,
        "REJECT",
        &format!(
            "blocked: {}",
            fails
                .iter()
                .map(|f| f.rule_id.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ),
    );
    Some(1)
}

/// GT-COMMENT + GT-04 + GT-04b + GT-06: issue close
///
/// Policy blocks are Findings — severity overridable via dispatch.yaml /
/// global severity_overrides.yaml, and switchable off via github_issues.yaml
/// (`close_requires_comment` / `epic_sub_issue_gate` / `close_done_when_gate`).
/// Fail-closed data errors (JSON parse / sub-issue query failure) still
/// hard-block: refusing on uncertainty is a safety property, not policy.
pub fn intercept_issue_close(args: &[String]) -> i32 {
    let issue_cfg = crate::shared::load_spec_yaml("github_issues.yaml");
    if issue_cfg.is_none() {
        println!("闸门 WARN: 缺少 .githooks/spec/github_issues.yaml，Done when 关闭闸门失效");
    }

    let mut findings: Vec<Finding> = Vec::new();

    // GT-COMMENT: close requires --comment (switch: close_requires_comment)
    let has_comment = args.iter().any(|a| a.starts_with("--comment") || a == "-c");
    if !has_comment && gate_switch(issue_cfg.as_ref(), "close_requires_comment") {
        findings.push(Finding::new(
            "GT-COMMENT",
            Severity::Fail,
            "gh issue close 必须带 --comment 说明关闭原因，例如：gh issue close <N> --comment \"Agent 🤖 - Note: 原因说明\"",
        ));
    }

    let issue_num = args
        .iter()
        .find(|a| a.chars().all(|c| c.is_ascii_digit()))
        .cloned();
    let repo = arg_repo(args).unwrap_or_else(derive_repo);
    if let (Some(num), false) = (&issue_num, repo.is_empty()) {
        let (rc, data, _) = run_gh(
            &[
                "api".to_string(),
                format!("repos/{repo}/issues/{num}"),
                "--jq".to_string(),
                "{body, labels: [.labels[].name], state}".to_string(),
            ],
            None,
        );
        if rc == 0 && !data.trim().is_empty() {
            let parsed: serde_json::Value = match serde_json::from_str(&data) {
                Ok(v) => v,
                Err(e) => {
                    println!("闸门: #{num} issue 数据解析失败，为安全起见拒绝关闭: {e}");
                    log(
                        "ISSUE_CLOSE",
                        &format!("#{num}"),
                        "REJECT",
                        "issue JSON parse failed (fail-closed)",
                    );
                    return 1;
                }
            };
            let body = parsed.get("body").and_then(|b| b.as_str()).unwrap_or("");
            let labels: Vec<String> = parsed
                .get("labels")
                .and_then(|l| l.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            // GT-06 (switch: epic_sub_issue_gate): epic close with open
            // sub-issues must be blocked. Query failure stays fail-closed.
            if is_epic(&labels) && gate_switch(issue_cfg.as_ref(), "epic_sub_issue_gate") {
                match query_open_subs(&repo, num) {
                    Ok(open_subs) => {
                        if let Some(block) = gt06_open_sub_block(&labels, &open_subs) {
                            findings.push(Finding::new(
                                "GT-06",
                                Severity::Fail,
                                &format!(
                                    "#{num} 是 epic，但有 sub-issue 未关闭: #{}",
                                    block.join(", #")
                                ),
                            ));
                        }
                    }
                    Err(e) => {
                        println!(
                            "闸门: 无法确认 epic #{num} 的 sub-issues，为安全起见拒绝关闭: {e}"
                        );
                        log(
                            "ISSUE_CLOSE",
                            &format!("#{num}"),
                            "REJECT",
                            &format!("sub query failed (fail-closed): {e}"),
                        );
                        return 1;
                    }
                }
            }
            // GT-04 (switch: close_done_when_gate): only Done when
            // checkboxes gate close (Implementation Order progress boxes and
            // other lists must not block). Heading name from github_issues.yaml.
            if gate_switch(issue_cfg.as_ref(), "close_done_when_gate") {
                let unticked = gt04_unticked_done_when(body, issue_cfg.as_ref());
                if !unticked.is_empty() {
                    println!(
                        "闸门: #{num} Done when 有 checkbox 未全部勾选，未勾 {} 项：",
                        unticked.len()
                    );
                    for item in unticked.iter().take(5) {
                        println!("  - [ ] {item}");
                    }
                    findings.push(Finding::new(
                        "GT-04",
                        Severity::Fail,
                        &format!("#{num} Done when 有 {} 个 checkbox 未勾选", unticked.len()),
                    ));
                }
            }

            // DWJ (switch: done_when_judge.enabled): model judge on each
            // done-when item against real evidence. Policy layer — every
            // degradation path is INFO; GT-04 + tools stay the backstop.
            if dwj_enabled(issue_cfg.as_ref()) {
                findings.extend(run_done_when_judge(
                    &repo,
                    num,
                    body,
                    args,
                    issue_cfg.as_ref(),
                ));
            }

            // GT-04b: epic exempt; non-epic without linked PR → WARN only.
            if !is_epic(&labels) {
                let (rc4, tl, _) = run_gh(&[
                    "api".to_string(),
                    format!("repos/{repo}/issues/{num}/timeline"),
                    "--jq".to_string(),
                    "[.[] | select(.event == \"cross-referenced\" and .source.issue.pull_request != null) | .source.issue.number]".to_string(),
                ], None);
                if rc4 == 0 {
                    let linked: serde_json::Value =
                        serde_json::from_str(&tl).unwrap_or(serde_json::Value::Array(vec![]));
                    let has_link = linked.as_array().map(|a| !a.is_empty()).unwrap_or(false);
                    if gt04b_warn_no_link(&labels, has_link) {
                        println!(
                            "闸门 WARN: #{num} 无 PR 关联（无 PR Fixes/Closes 它）。不阻塞关闭，请确认是合法关闭（duplicate/废弃/won't do）。"
                        );
                        log("ISSUE_CLOSE", &format!("#{num}"), "WARN", "no linked PR");
                    }
                }
            }
        }
    }

    let dispatch = load_dispatch_cfg();
    if gate_block(
        "ISSUE_CLOSE",
        &format!("#{}", issue_num.as_deref().unwrap_or_default()),
        &mut findings,
        dispatch.as_ref(),
    )
    .is_some()
    {
        return 1;
    }

    let mut full = vec!["issue".to_string(), "close".to_string()];
    full.extend(gh_args(args));
    let (rc, out, err) = run_gh(&full, None);
    if !out.is_empty() {
        print!("{out}");
    }
    if !err.is_empty() {
        eprint!("{err}");
    }
    if rc == 0 {
        log(
            "ISSUE_CLOSE",
            &format!("#{}", issue_num.unwrap_or_default()),
            "CLOSED",
            "",
        );
    }
    rc
}

/// GT-02: pr create
pub fn intercept_pr_create(args: &[String]) -> i32 {
    let (title, body, labels, head, _) = match extract(args) {
        Ok(parts) => parts,
        Err(error) => {
            println!("闸门: {error}");
            log("PR_CREATE", "?", "REJECT", &error);
            return 1;
        }
    };
    let labels_str: Vec<&str> = labels.iter().map(String::as_str).collect();

    let cfg = crate::tools::git::find_githooks_dir()
        .and_then(|d| {
            crate::shared::load_yaml(
                d.join("spec/github_pull_requests.yaml")
                    .to_str()
                    .unwrap_or(""),
            )
            .ok()
        })
        .unwrap_or(serde_yaml::Value::Null);

    let mut findings =
        pull_requests::check_content(&title, &body, &labels_str, &head, "open", false, Some(&cfg));
    // Apply global severity overrides from severity_overrides.yaml
    crate::shared::apply_global_overrides(&mut findings);
    let fails: Vec<&Finding> = findings
        .iter()
        .filter(|f| f.severity == Severity::Fail)
        .collect();
    for f in &findings {
        if f.severity <= Severity::Warn {
            println!("{}\t{}", f.severity.as_str(), f.msg);
        }
    }
    if !fails.is_empty() {
        println!("闸门: 校验 FAIL，拒绝创建。修正后重试。");
        log(
            "PR_CREATE",
            crate::shared::truncate_utf8(&title, 40),
            "REJECT",
            &format!("FAIL={}", fails.len()),
        );
        return 1;
    }

    println!("闸门: 检查通过，执行 gh ...");
    let mut full = vec!["pr".to_string(), "create".to_string()];
    full.extend(args.iter().cloned());
    let (rc, out, err) = run_gh(&full, None);
    if !out.is_empty() {
        print!("{out}");
    }
    if !err.is_empty() {
        eprint!("{err}");
    }
    if rc != 0 {
        return rc;
    }
    let url = out.trim().to_string();
    if url.starts_with("https://github.com/")
        && url.contains("/pull/")
        && let Some(num) = extract_num(&url, "/pull/")
    {
        log(
            "PR_CREATE",
            &format!("PR #{num}"),
            "CREATED",
            crate::shared::truncate_utf8(&title, 40),
        );
    }
    0
}

/// GT-BODY + GT-CHK + GT-05 + GT-06 + CM-01/02 + GT-07: pr merge
///
/// Policy blocks are Findings — severities overridable via dispatch.yaml /
/// global severity_overrides.yaml, and the blocks themselves switchable via
/// github_pull_requests.yaml (`merge_requires_body` / `merge_checkbox_gate` /
/// `merge_title_gate`) and github_issues.yaml (`merge_fixes_gate` /
/// `epic_sub_issue_gate`). Fail-closed data errors still hard-block:
/// refusing on uncertainty is a safety property, not policy.
pub fn intercept_pr_merge(args: &[String]) -> i32 {
    let pr_cfg = crate::shared::load_spec_yaml("github_pull_requests.yaml");
    let issue_cfg = crate::shared::load_spec_yaml("github_issues.yaml");

    let mut findings: Vec<Finding> = Vec::new();

    // GT-BODY (switch: merge_requires_body)
    let has_body = args.iter().any(|a| a.starts_with("--body") || a == "-b");
    if !has_body && gate_switch(pr_cfg.as_ref(), "merge_requires_body") {
        findings.push(Finding::new(
            "GT-BODY",
            Severity::Fail,
            "gh pr merge 必须带 --body 说明合并原因，例如：gh pr merge <N> --squash --body \"Agent 🤖 - Merge: 原因说明\"",
        ));
    }

    let pr_num = args
        .iter()
        .find(|a| a.chars().all(|c| c.is_ascii_digit()))
        .cloned();
    let repo = derive_repo();
    if let (Some(num), false) = (&pr_num, repo.is_empty()) {
        let (rc, body, _) = run_gh(
            &[
                "api".to_string(),
                format!("repos/{repo}/pulls/{num}"),
                "--jq".to_string(),
                ".body".to_string(),
            ],
            None,
        );
        if rc == 0 && !body.trim().is_empty() {
            // GT-CHK (switch: merge_checkbox_gate)
            if gate_switch(pr_cfg.as_ref(), "merge_checkbox_gate") {
                let (all_ticked, unticked) = check_all_checkboxes(body.trim());
                if !all_ticked {
                    println!(
                        "闸门: PR #{num} 有 checkbox 未全部勾选，未勾 {} 项：",
                        unticked.len()
                    );
                    for item in unticked.iter().take(5) {
                        println!("  - [ ] {item}");
                    }
                    findings.push(Finding::new(
                        "GT-CHK",
                        Severity::Fail,
                        &format!("PR #{num} 有 {} 个 checkbox 未勾选", unticked.len()),
                    ));
                }
            }

            let fixes = extract_fixes(body.trim());
            for fn_ in fixes {
                // One call: fetch issue body + labels in a single jq object.
                let (rc2, issue_data, _) = run_gh(
                    &[
                        "api".to_string(),
                        format!("repos/{repo}/issues/{fn_}"),
                        "--jq".to_string(),
                        "{body, labels: [.labels[].name]}".to_string(),
                    ],
                    None,
                );
                if rc2 != 0 || issue_data.trim().is_empty() {
                    // Fail-closed: a failed issue fetch must NOT silently skip
                    // the checkbox / GT-06 epic checks below.
                    println!("闸门: 关联 issue #{fn_} 数据查询失败，为安全起见拒绝合并 (rc={rc2})");
                    log(
                        "PR_MERGE",
                        &format!("PR #{num}"),
                        "REJECT",
                        &format!("issue #{fn_} fetch failed (fail-closed, rc={rc2})"),
                    );
                    return 1;
                }
                let parsed: serde_json::Value = match serde_json::from_str(&issue_data) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("闸门: 关联 issue #{fn_} 数据解析失败，为安全起见拒绝合并: {e}");
                        log(
                            "PR_MERGE",
                            &format!("PR #{num}"),
                            "REJECT",
                            &format!("issue #{fn_} JSON parse failed (fail-closed)"),
                        );
                        return 1;
                    }
                };
                let issue_body = parsed.get("body").and_then(|b| b.as_str()).unwrap_or("");
                let labels: Vec<String> = parsed
                    .get("labels")
                    .and_then(|l| l.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                // GT-05 (switch: merge_fixes_gate): epic Fixes targets are
                // exempt from the checkbox check — IS-11 forbids Done when on
                // epics, and GT-06 below guards epic completion via open subs.
                if gate_switch(issue_cfg.as_ref(), "merge_fixes_gate") {
                    let unticked =
                        gt05_unticked_issue_boxes(&labels, issue_body, issue_cfg.as_ref());
                    if !unticked.is_empty() {
                        println!(
                            "闸门: PR #{num} 关联 issue #{fn_} Done when 有 checkbox 未全部勾选，未勾 {} 项：",
                            unticked.len()
                        );
                        findings.push(Finding::new(
                            "GT-05",
                            Severity::Fail,
                            &format!(
                                "PR #{num} 关联 issue #{fn_} Done when 有 {} 个 checkbox 未勾选",
                                unticked.len()
                            ),
                        ));
                    }
                }

                // GT-06 (switch: epic_sub_issue_gate): only epic targets need
                // the open-sub check. Query failure stays fail-closed.
                if is_epic(&labels) && gate_switch(issue_cfg.as_ref(), "epic_sub_issue_gate") {
                    match query_open_subs(&repo, &fn_) {
                        Ok(open_subs) => {
                            if let Some(block) = gt06_open_sub_block(&labels, &open_subs) {
                                findings.push(Finding::new(
                                    "GT-06",
                                    Severity::Fail,
                                    &format!(
                                        "合并会关闭 epic #{fn_}，但存在 open sub-issue #{}",
                                        block.join(", #")
                                    ),
                                ));
                            }
                        }
                        Err(e) => {
                            println!(
                                "闸门: 无法确认 epic #{fn_} 的 sub-issues，为安全起见拒绝合并: {e}"
                            );
                            log(
                                "PR_MERGE",
                                &format!("PR #{num}"),
                                "REJECT",
                                &format!("epic #{fn_} sub query failed (fail-closed): {e}"),
                            );
                            return 1;
                        }
                    }
                }
            }

            // squash title conventional commit (CM-01/CM-02, switch: merge_title_gate)
            let merge_title = extract_merge_title(args, &repo, num);
            if !merge_title.is_empty() && gate_switch(pr_cfg.as_ref(), "merge_title_gate") {
                let conv = Regex::new(r"^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\(.+\))?!?:\s+\S+").unwrap();
                if !conv.is_match(&merge_title) {
                    findings.push(Finding::new(
                        "CM-01",
                        Severity::Fail,
                        &format!("merge 标题非 conventional commit 格式: '{merge_title}'"),
                    ));
                }
                let cjk = Regex::new(r"[\u4e00-\u9fff]").unwrap();
                if cjk.is_match(&merge_title) {
                    findings.push(Finding::new(
                        "CM-02",
                        Severity::Fail,
                        &format!("merge 标题含 CJK（应为英文）: '{merge_title}'"),
                    ));
                }
            }
        }
    }

    let dispatch = load_dispatch_cfg();
    if gate_block(
        "PR_MERGE",
        &format!("PR #{}", pr_num.as_deref().unwrap_or_default()),
        &mut findings,
        dispatch.as_ref(),
    )
    .is_some()
    {
        return 1;
    }

    let merge_reason = extract_merge_body(args);
    let mut full = vec!["pr".to_string(), "merge".to_string()];
    full.extend(args.iter().cloned());
    let (rc, out, err) = run_gh(&full, None);
    if !out.is_empty() {
        print!("{out}");
    }
    if !err.is_empty() {
        eprint!("{err}");
    }
    if rc != 0 {
        log(
            "PR_MERGE",
            &format!("PR #{}", pr_num.unwrap_or_default()),
            "FAIL",
            crate::shared::truncate_utf8(&err, 80),
        );
        return rc;
    }

    // GT-07: delete local branch + post-merge comment
    if let (Some(num), false) = (&pr_num, repo.is_empty()) {
        let (rc4, head_ref, _) = run_gh(
            &[
                "api".to_string(),
                format!("repos/{repo}/pulls/{num}"),
                "--jq".to_string(),
                ".head.ref".to_string(),
            ],
            None,
        );
        if rc4 == 0 {
            let head = head_ref.trim().to_string();
            if !head.is_empty() && !["main", "master", "develop"].contains(&head.as_str()) {
                let _ = Command::new("git")
                    .arg("branch")
                    .arg("-d")
                    .arg(&head)
                    .output();
                println!("提示: 本地分支 '{head}' 已删除。远程删除执行:");
                println!("  git push origin --delete {head}");
            }
        }
        if !merge_reason.is_empty() {
            let (rc2, _, err2) = run_gh(
                &[
                    "pr".to_string(),
                    "comment".to_string(),
                    num.clone(),
                    "--body".to_string(),
                    merge_reason.clone(),
                ],
                None,
            );
            if rc2 == 0 {
                println!("INFO\tPR #{num} 合并留言已发布");
            } else {
                println!("WARN\tPR #{num} 合并留言失败: {}", err2.trim());
            }
        }
        log(
            "PR_MERGE",
            &format!("PR #{num}"),
            "MERGED",
            crate::shared::truncate_utf8(&merge_reason, 80),
        );
    }
    rc
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn extract_num(url: &str, marker: &str) -> Option<String> {
    let seg = url.split(marker).nth(1)?.split('/').next()?;
    // 只接受非空纯数字段（防止 URL 片段注入 API 路径）。
    if !seg.is_empty() && seg.chars().all(|c| c.is_ascii_digit()) {
        Some(seg.to_string())
    } else {
        None
    }
}

fn extract_merge_title(args: &[String], repo: &str, pr_num: &str) -> String {
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--title" && i + 1 < args.len() {
            return args[i + 1].clone();
        }
        if let Some(v) = args[i].strip_prefix("--title=") {
            return v.to_string();
        }
        i += 1;
    }
    let (rc, title, _) = run_gh(
        &[
            "api".to_string(),
            format!("repos/{repo}/pulls/{pr_num}"),
            "--jq".to_string(),
            ".title".to_string(),
        ],
        None,
    );
    if rc == 0 {
        return title.trim().to_string();
    }
    String::new()
}

fn extract_merge_body(args: &[String]) -> String {
    let mut i = 0;
    while i < args.len() {
        if (args[i] == "--body" || args[i] == "-b") && i + 1 < args.len() {
            return args[i + 1].clone();
        }
        if let Some(v) = args[i].strip_prefix("--body=") {
            return v.to_string();
        }
        i += 1;
    }
    String::new()
}

/// Attempt to mount sub-issue to parent via addSubIssue API.
/// Returns true on success, false on failure or no-op.
fn auto_link_sub(url: &str, repo: &str, parent_arg: &str) -> bool {
    let sub_num = match extract_num(url, "/issues/") {
        Some(n) => n,
        None => return false,
    };
    if parent_arg.is_empty() || parent_arg == "0" {
        return false;
    }
    if !parent_arg.chars().all(|c| c.is_ascii_digit()) {
        return false; // 防路径注入：parent 必须是纯数字 issue 号
    }
    let (_, sub_id_raw, _) = run_gh(
        &[
            "api".to_string(),
            format!("repos/{repo}/issues/{sub_num}"),
            "--jq".to_string(),
            ".id".to_string(),
        ],
        None,
    );
    let sub_id = sub_id_raw.trim().to_string();
    if sub_id.is_empty() {
        return false;
    }
    let (rc2, out2, _) = run_gh(
        &[
            "api".to_string(),
            format!("repos/{repo}/issues/{parent_arg}/sub_issues"),
            "-X".to_string(),
            "POST".to_string(),
            "-F".to_string(),
            format!("sub_issue_id={sub_id}"),
        ],
        None,
    );
    if rc2 == 0 {
        println!("INFO\t#{sub_num} 已挂载到 parent #{parent_arg}");
        true
    } else {
        println!(
            "FAIL\t挂载 #{sub_num} → parent #{parent_arg}: {}",
            out2.trim()
        );
        false
    }
}

/// Verify sub-issue is mounted to parent by checking the parent's
/// sub_issues list.  Pure parse — testable without real gh.
fn is_mounted(sub_issues_output: &str, sub_num: &str) -> bool {
    sub_issues_output.lines().any(|line| line.trim() == sub_num)
}

/// Verify mount after auto_link: query parent's sub_issues and confirm
/// the new sub-issue number is present.  Returns true if mounted or
/// not applicable (epic / no parent).
fn verify_mount(repo: &str, sub_num: &str, parent: &str) -> bool {
    if parent.is_empty() || !parent.chars().all(|c| c.is_ascii_digit()) {
        return false; // 防路径注入：parent 必须是纯数字 issue 号
    }
    let (rc, out, _) = run_gh(
        &[
            "api".to_string(),
            format!("repos/{repo}/issues/{parent}/sub_issues"),
            "--jq".to_string(),
            ".[].number".to_string(),
        ],
        None,
    );
    if rc != 0 {
        return false;
    }
    is_mounted(&out, sub_num)
}

/// Main dispatch: `gate` installed as `~/.local/bin/gh`.
pub fn dispatch(args: &[String]) -> i32 {
    if args.is_empty() {
        return passthrough(&[]);
    }
    let cmd = &args[0];
    let Some(subcmd) = args.get(1) else {
        return passthrough(args);
    };
    // Intercept handlers expect ONLY the subcommand's arguments (they re-prefix
    // "issue <sub>" themselves when passing through). Forward args[2..].
    let rest = &args[2..];
    match (cmd.as_str(), subcmd.as_str()) {
        ("issue", "create") => intercept_issue_create(rest),
        ("issue", "close") => intercept_issue_close(rest),
        ("pr", "create") => intercept_pr_create(rest),
        ("pr", "merge") => intercept_pr_merge(rest),
        _ => passthrough(args),
    }
}

// ===========================================================================
// Tests — #203 real linkage enforcement
// ===========================================================================

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
