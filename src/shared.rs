//! Shared helpers for the canon validators.
//!
//! Centralizes the primitives every validator needs: a GitHub API client that
//! tolerates flaky networks, a Finding contract flowing through rule checks,
//! and a YAML loader.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;

use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

/// All short-lived network failures seen in this repo's CI history.
/// Non-matching errors (4xx, permission denied, malformed request) propagate
/// immediately — retrying them would just burn the budget.
pub const TRANSIENT_PATTERNS: &[&str] = &[
    "EOF",
    "unexpected EOF",
    "connection reset",
    "Connection reset",
    "Connection closed",
    "connection refused",
    "broken pipe",
    "TLS handshake timeout",
    "dial tcp",
    "i/o timeout",
    "net/http: timeout",
    "transport is closing",
    "500 Internal Server Error",
    "502 Bad Gateway",
    "503 Service Unavailable",
    "504 Gateway Timeout",
];

pub const MAX_RETRIES: u32 = 8;
pub const INITIAL_BACKOFF_SECONDS: u64 = 3;

/// Ordered so any `Fail` dominates `Warn`, which dominates `Info`.
/// IntEnum semantics: `Fail=10`, `Warn=20`, `Info=30`; lower sorts first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Severity {
    Fail = 10,
    Warn = 20,
    Info = 30,
}

impl Severity {
    pub fn parse(s: &str) -> Option<Severity> {
        match s.to_ascii_uppercase().as_str() {
            "FAIL" => Some(Severity::Fail),
            "WARN" => Some(Severity::Warn),
            "INFO" => Some(Severity::Info),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Severity::Fail => "FAIL",
            Severity::Warn => "WARN",
            Severity::Info => "INFO",
        }
    }
}

/// A single rule result.
/// `rule_id` is the stable identifier surfaced in CLI output (e.g. "P-30").
/// `line_hint` is optional because most checks operate on whole documents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub rule_id: String,
    pub severity: Severity,
    pub msg: String,
    pub line_hint: Option<u32>,
    /// Extra structured fields (score, confidence, category, evidence, ...)
    /// 来自 yaml harness 输出, 提供给 --json 模式给开发 agent 解析.
    /// 文本模式忽略.
    pub extra: BTreeMap<String, String>,
}

impl Finding {
    /// `{rule_id:<6} {SEVERITY}[\t{msg}]` with an optional ` L{line}` suffix
    /// on the prefix — matches the Python `Finding.format()` exactly.
    pub fn format(&self) -> String {
        let mut prefix = format!("{:<6} {}", self.rule_id, self.severity.as_str());
        if let Some(line) = self.line_hint {
            prefix.push_str(&format!(" L{}", line));
        }
        format!("{}\t{}", prefix, self.msg)
    }

    pub fn new(rule_id: &str, severity: Severity, msg: &str) -> Self {
        Self {
            rule_id: rule_id.to_string(),
            severity,
            msg: msg.to_string(),
            line_hint: None,
            extra: BTreeMap::new(),
        }
    }

    pub fn with_line(mut self, line: u32) -> Self {
        self.line_hint = Some(line);
        self
    }

    /// Insert a structured extra field (score, confidence, evidence, ...).
    /// Used by harness mode: grep / mode: diff yaml to carry review-agent
    /// signals beyond the default severity/msg/line schema. --json mode
    /// exposes these to the calling dev agent.
    pub fn with_extra(mut self, key: &str, value: impl Into<String>) -> Self {
        self.extra.insert(key.to_string(), value.into());
        self
    }

    /// Serialize to JSON Value, including all extra fields as a flat object.
    /// Used by canon check --json mode to emit machine-readable output for
    /// dev agents that consume score / confidence / evidence.
    pub fn to_json(&self) -> JsonValue {
        let mut obj = serde_json::Map::new();
        obj.insert(
            "rule_id".to_string(),
            JsonValue::String(self.rule_id.clone()),
        );
        obj.insert(
            "severity".to_string(),
            JsonValue::String(self.severity.as_str().to_string()),
        );
        obj.insert("msg".to_string(), JsonValue::String(self.msg.clone()));
        if let Some(line) = self.line_hint {
            obj.insert("line".to_string(), JsonValue::Number(line.into()));
        }
        for (k, v) in &self.extra {
            obj.insert(k.clone(), JsonValue::String(v.clone()));
        }
        JsonValue::Object(obj)
    }
}

/// Return a "has failure" bool: true if any `Fail`.
/// The Python `aggregate_result` returns an exit code (1/0); callers needing
/// an exit code can use `aggregate_result(findings) as i32` via exit_code().
pub fn aggregate_result(findings: &[Finding]) -> bool {
    findings.iter().any(|f| f.severity == Severity::Fail)
}

/// Process exit code: 1 if any `Fail`, else 0 — the direct port of
/// `aggregate_result`.
pub fn exit_code(findings: &[Finding]) -> i32 {
    if aggregate_result(findings) { 1 } else { 0 }
}

/// Apply user-provided severity overrides to a list of findings.
/// Loads overrides from `.githooks/spec/severity_overrides.yaml` in the repo root.
pub fn apply_global_overrides(findings: &mut [Finding]) {
    let overrides = match load_severity_overrides() {
        Some(cfg) => cfg,
        None => return,
    };

    let map = match overrides
        .get("severity_overrides")
        .and_then(|c| c.as_mapping())
    {
        Some(m) => m,
        None => return,
    };
    for finding in findings.iter_mut() {
        if let Some(sev_val) = map.get(YamlValue::String(finding.rule_id.to_string()))
            && let Some(sev_str) = sev_val.as_str()
        {
            let new_sev = match sev_str.to_uppercase().as_str() {
                "FAIL" => Severity::Fail,
                "WARN" => Severity::Warn,
                "INFO" => Severity::Info,
                _ => continue,
            };
            if finding.severity != Severity::Info {
                finding.severity = new_sev;
            }
        }
    }
}
pub fn apply_severity_overrides(findings: &mut [Finding], cfg: Option<&YamlValue>) {
    let map = match cfg
        .and_then(|c| c.get("severity_overrides"))
        .and_then(|c| c.as_mapping())
    {
        Some(m) => m,
        None => return,
    };
    for finding in findings.iter_mut() {
        if let Some(sev_val) = map.get(YamlValue::String(finding.rule_id.to_string()))
            && let Some(sev_str) = sev_val.as_str()
        {
            let new_sev = match sev_str.to_uppercase().as_str() {
                "FAIL" => Severity::Fail,
                "WARN" => Severity::Warn,
                "INFO" => Severity::Info,
                _ => continue,
            };
            if finding.severity != Severity::Info {
                finding.severity = new_sev;
            }
        }
    }
}

/// Load severity_overrides from `.githooks/spec/severity_overrides.yaml`.
fn load_severity_overrides() -> Option<YamlValue> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        let candidate = dir.join(".githooks");
        if candidate.is_dir() {
            let path = candidate.join("spec/severity_overrides.yaml");
            if let Ok(text) = fs::read_to_string(&path)
                && let Ok(cfg) = serde_yaml::from_str(&text)
            {
                return Some(cfg);
            }
            return None;
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// Print FAIL/WARN findings, plus INFO that carries extra (score/confidence).
/// Other INFO is suppressed unless nothing actionable fired.
/// All output goes to **stderr** — matches the Python which writes every line
/// to `sys.stderr`.
pub fn print_findings(findings: &[Finding]) {
    use std::io::Write;
    let stderr = std::io::stderr();
    let mut out = stderr.lock();

    let actionable: Vec<&Finding> = findings
        .iter()
        .filter(|f| f.severity <= Severity::Warn || !f.extra.is_empty())
        .collect();
    if !actionable.is_empty() {
        let mut sorted = actionable.clone();
        sorted.sort_by(|a, b| {
            (a.severity, a.rule_id.as_str(), a.line_hint.unwrap_or(0)).cmp(&(
                b.severity,
                b.rule_id.as_str(),
                b.line_hint.unwrap_or(0),
            ))
        });
        for finding in &sorted {
            let _ = writeln!(out, "{}", finding.format());
        }
        let passed = findings
            .iter()
            .filter(|f| f.severity == Severity::Info)
            .count();
        if passed > 0 {
            let _ = writeln!(out, "({} checks passed)", passed);
        }
    } else {
        let _ = writeln!(out, "({} checks passed)", findings.len());
    }
    let _ = writeln!(
        out,
        "RESULT: {}",
        if aggregate_result(findings) {
            "FAIL"
        } else {
            "ALL PASS"
        }
    );
}

// ---------------------------------------------------------------------------
// Transient error detection
// ---------------------------------------------------------------------------

/// True if `message` contains any known transient-error substring.
pub fn is_transient(message: &str) -> bool {
    TRANSIENT_PATTERNS.iter().any(|pat| message.contains(pat))
}

// ---------------------------------------------------------------------------
// GitHub API client
// ---------------------------------------------------------------------------

/// Run `gh api <args...>`, returning (exit_code, combined_stripped_output).
/// All output is captured; combined stdout+stderr is trimmed.
fn run_gh(args: &[&str]) -> (i32, String) {
    let mut cmd = Command::new("gh");
    cmd.arg("api").args(args);
    let output = match cmd.output() {
        Ok(o) => o,
        Err(e) => return (127, e.to_string()),
    };
    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    (
        output.status.code().unwrap_or(127),
        combined.trim().to_string(),
    )
}

/// Run `gh api` with up to `MAX_RETRIES` attempts; backoff = `3s * attempt`.
/// Non-transient errors raise immediately; transient ones retry.
fn run_gh_with_retry(args: &[&str]) -> Result<String, String> {
    let mut last_msg = String::new();
    for attempt in 1..=MAX_RETRIES {
        let (rc, out) = run_gh(args);
        if rc == 0 {
            return Ok(out);
        }
        last_msg = out.clone();
        if !is_transient(&out) {
            return Err(format!("gh api hard failure ({}): {}", rc, out));
        }
        // Backoff: 3s * attempt.
        thread::sleep(Duration::from_secs(
            INITIAL_BACKOFF_SECONDS * attempt as u64,
        ));
    }
    Err(format!(
        "gh api exhausted {} retries: {}",
        MAX_RETRIES, last_msg
    ))
}

/// GET a GitHub REST endpoint, returning parsed JSON (or `Null` on empty).
/// `params` become `-F key=value` flags.
pub fn gh_api(path: &str, params: Option<&BTreeMap<&str, &str>>) -> Result<JsonValue, String> {
    let mut args: Vec<String> = vec![path.to_string()];
    if let Some(p) = params {
        for (k, v) in p {
            args.push("-F".to_string());
            args.push(format!("{}={}", k, v));
        }
    }
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let raw = run_gh_with_retry(&arg_refs)?;
    if raw.is_empty() {
        return Ok(JsonValue::Null);
    }
    serde_json::from_str(&raw).map_err(|e| format!("json decode: {}", e))
}

/// Iterate a list endpoint by explicit cursor pagination.
/// Uses `page=N&per_page=N`; stops when a page returns fewer than `page_size`.
pub fn gh_api_paginate(path: &str, page_size: u32) -> Result<Vec<JsonValue>, String> {
    let mut results = Vec::new();
    let mut page = 1u32;
    loop {
        let sep = if path.contains('?') { '&' } else { '?' };
        let paged = format!("{}{}page={}&per_page={}", path, sep, page, page_size);
        let raw = run_gh_with_retry(&[&paged])?;
        if raw.is_empty() {
            break;
        }
        let items: Vec<JsonValue> =
            serde_json::from_str(&raw).map_err(|e| format!("json decode: {}", e))?;
        if items.is_empty() {
            break;
        }
        let len = items.len() as u32;
        results.extend(items);
        if len < page_size {
            break;
        }
        page += 1;
    }
    Ok(results)
}

// ---------------------------------------------------------------------------
// YAML loader
// ---------------------------------------------------------------------------

/// Load a YAML file as a `serde_yaml::Value`; empty/whitespace file → `Null`
/// (YAML null; callers treat `Null` as an empty map, mirroring Python's
/// `data or {}`). Returns an error if parsing fails or the file is missing.
pub fn load_yaml(path: &str) -> Result<YamlValue, serde_yaml::Error> {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            return Err(<serde_yaml::Error as serde::de::Error>::custom(
                e.to_string(),
            ));
        }
    };
    if text.trim().is_empty() {
        return Ok(YamlValue::Null);
    }
    serde_yaml::from_str(&text)
}

// ---------------------------------------------------------------------------
// Spec cfg accessors — rule families read their yaml; no code defaults.
// ---------------------------------------------------------------------------

/// Resolve a spec file by name, searching `spec_dir` recursively (depth ≤ 3).
/// Yaml files may live flat or grouped in theme subdirs (`quality/`, `code/`,
/// `github/`, …) — the layout is free and the loaders never hardcode it.
pub fn find_spec_file(spec_dir: &Path, filename: &str) -> Option<PathBuf> {
    fn walk(dir: &Path, filename: &str, depth: u8, out: &mut Vec<PathBuf>) {
        if depth > 3 {
            return;
        }
        for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, filename, depth + 1, out);
            } else if path.file_name().is_some_and(|n| n == filename) {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(spec_dir, filename, 0, &mut out);
    out.sort();
    out.into_iter().next()
}

/// Load `<repo>/.githooks/spec/<file>` walking up from cwd. `None` when the
/// file is missing — rule entries turn that into a loud `canon.setup` finding
/// (no silent defaults: a repo without its spec must not pass green).
pub fn load_spec_yaml(file: &str) -> Option<YamlValue> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        if dir.join(".githooks").is_dir() {
            let spec_dir = dir.join(".githooks/spec");
            return find_spec_file(&spec_dir, file)
                .and_then(|path| load_yaml(path.to_str()?).ok())
                .filter(|v| !v.is_null());
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// The standard loud finding for a missing/empty spec yaml.
pub fn missing_cfg_finding(file: &str) -> Finding {
    Finding::new(
        "canon.setup",
        Severity::Fail,
        &format!("missing .githooks/spec/{file} — run `canon init` or restore the rules pack"),
    )
}

/// `cfg.<key>` as a list of strings; empty when cfg/key missing.
pub fn cfg_str_list(cfg: Option<&YamlValue>, key: &str) -> Vec<String> {
    cfg.and_then(|c| c.get(key))
        .and_then(|v| v.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// `cfg.<key>` as bool; `None` when absent or not a bool.
pub fn cfg_bool(cfg: Option<&YamlValue>, key: &str) -> Option<bool> {
    cfg.and_then(|c| c.get(key)).and_then(|v| v.as_bool())
}

/// `cfg.<key>` raw value; `None` when absent.
pub fn cfg_get<'a>(cfg: Option<&'a YamlValue>, key: &str) -> Option<&'a YamlValue> {
    cfg.and_then(|c| c.get(key))
}

/// `cfg.<key>` as mapping of string→string (e.g. `keyword_label_suggestions`).
pub fn cfg_str_map(cfg: Option<&YamlValue>, key: &str) -> Vec<(String, String)> {
    cfg.and_then(|c| c.get(key))
        .and_then(|v| v.as_mapping())
        .map(|m| {
            m.iter()
                .filter_map(|(k, v)| Some((k.as_str()?.to_string(), v.as_str()?.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

/// `cfg.<key>` as mapping of string→string with fallback to code default —
/// ONLY for heading-name lookups where a missing name makes checks inert.
pub fn cfg_str(cfg: Option<&YamlValue>, key: &str) -> Option<String> {
    cfg.and_then(|c| c.get(key))
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

// ---------------------------------------------------------------------------
// Capability selection: `checks:` allowlist + family `fail_severity`
// ---------------------------------------------------------------------------

/// `cfg.fail_severity` as a Severity; absent key keeps the historic WARN.
pub fn cfg_fail_severity(cfg: &YamlValue) -> Severity {
    cfg.get("fail_severity")
        .and_then(|v| v.as_str())
        .and_then(Severity::parse)
        .unwrap_or(Severity::Warn)
}

/// Remap WARN-class findings to `cfg.fail_severity`. INFO/FAIL are untouched
/// (INFO can never be promoted — same one-way rule as severity overrides).
pub fn apply_family_severity(findings: &mut Vec<Finding>, cfg: Option<&YamlValue>) {
    let Some(cfg) = cfg.filter(|c| !c.is_null()) else {
        return;
    };
    let sev = cfg_fail_severity(cfg);
    if sev == Severity::Warn {
        return;
    }
    for f in findings.iter_mut() {
        if f.severity == Severity::Warn {
            f.severity = sev;
        }
    }
}

/// `checks:` capability allowlist — when the family yaml lists a `checks:`
/// sequence, only findings whose rule_id matches an entry (exact or prefix,
/// e.g. `CL` disables every CL-* check) are retained. Omitted key = all
/// capabilities enabled (backward compatible).
pub fn apply_check_allowlist(findings: &mut Vec<Finding>, cfg: Option<&YamlValue>) {
    let Some(cfg) = cfg.filter(|c| !c.is_null()) else {
        return;
    };
    let list: Vec<String> = cfg
        .get("checks")
        .and_then(|v| v.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    if list.is_empty() {
        return;
    }
    let keep = |f: &Finding| {
        list.iter()
            .any(|c| c == &f.rule_id || f.rule_id.starts_with(c.as_str()))
    };
    findings.retain(keep);
}

// ---------------------------------------------------------------------------
// External command runner
// ---------------------------------------------------------------------------

/// Run an external command, returning (exit_code, combined_stripped_output).
pub fn run_external(cmd: &[&str], cwd: Option<&str>) -> Result<(i32, String), String> {
    if cmd.is_empty() {
        return Err("empty command".to_string());
    }
    let mut builder = Command::new(cmd[0]);
    builder.args(&cmd[1..]);
    if let Some(dir) = cwd {
        builder.current_dir(dir);
    }
    let output = builder
        .output()
        .map_err(|e| format!("spawn {:?}: {}", cmd[0], e))?;
    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok((
        output.status.code().unwrap_or(127),
        combined.trim().to_string(),
    ))
}

/// Truncate a string to at most `max` bytes without panicking on a multi-byte
/// UTF-8 boundary (CJK chars are multi-byte). Returns `s` unchanged if short.
pub fn truncate_utf8(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

// ===========================================================================
// Tests
// ===========================================================================
