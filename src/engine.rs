//! Checklist rule engine: the only executable detection surface is here, and
//! it contains **no detection logic** — every rule is a `checklist_*.yaml`
//! under `.githooks/spec/` that pipes a payload (diff / changed files /
//! nothing) to an external harness command and parses finding JSON from
//! stdout. Severity is max(yaml `fail_severity`, harness-reported) so a FAIL
//! from the harness always blocks.
//!
//! Protocol doc: `specs/docs/CHECKLIST_SPEC.md` in the canon repo.

use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use serde::Deserialize;

use crate::shared::{Finding, Severity, load_yaml, truncate_utf8};
use crate::tools::git;

/// Which canon entrypoint invoked us; controls the diff scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookScope {
    PreCommit,
    PrePush,
    Merge,
}

impl HookScope {
    fn as_str(self) -> &'static str {
        match self {
            HookScope::PreCommit => "pre-commit",
            HookScope::PrePush => "pre-push",
            HookScope::Merge => "merge",
        }
    }

    fn matches_yaml(self, yaml_hook: &str) -> bool {
        yaml_hook == self.as_str()
    }
}

/// SLA tier for a checklist check.
/// L1 = structural (zero token, milliseconds).
/// L2 = semantic (lightweight, seconds).
/// L3 = LLM-based (on-demand, minutes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum SlaLevel {
    #[default]
    L1,
    L2,
    L3,
}

impl SlaLevel {
    pub fn parse(s: &str) -> SlaLevel {
        match s.to_lowercase().as_str() {
            "l2" => SlaLevel::L2,
            "l3" => SlaLevel::L3,
            _ => SlaLevel::L1,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
struct RawSpec {
    enabled: Option<bool>,
    hooks: Vec<String>,
    #[serde(default)]
    r#match: MatchSpec,
    mode: Option<String>,
    harness: HarnessSpec,
    timeout: Option<u64>,
    optional: Option<bool>,
    fail_severity: Option<String>,
    sla: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
struct MatchSpec {
    paths_include: Vec<String>,
    paths_exclude: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
struct HarnessSpec {
    command: String,
    args: Vec<String>,
}

#[derive(Debug)]
pub struct ChecklistSpec {
    name: String,
    enabled: bool,
    hooks: Vec<String>,
    include: Vec<String>,
    exclude: Vec<String>,
    mode: Mode,
    command: String,
    args: Vec<String>,
    timeout_secs: u64,
    optional: bool,
    base_severity: Severity,
    sla: SlaLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Diff,
    File,
    /// Static check: harness receives empty stdin and runs whatever
    /// grep/find/ripgrep/etc. it wants. Findings carry their own
    /// path/line via the JSON output.
    Grep,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum HarnessFinding {
    Single(FindingJson),
    Many(Vec<FindingJson>),
}

#[derive(Debug, Deserialize)]
struct FindingJson {
    id: String,
    severity: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    line: Option<u32>,
    message: String,
    /// Catch-all extra fields (score, confidence, evidence, ...) from L3
    /// review agents; exposed by `check --json`.
    #[serde(default, flatten)]
    extra: std::collections::BTreeMap<String, serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Spec loading
// ---------------------------------------------------------------------------

fn load_spec(path: &std::path::Path) -> Result<ChecklistSpec, String> {
    let v = match load_yaml(path.to_str().unwrap_or("")) {
        Ok(v) => v,
        Err(e) => {
            // Fail-closed: a broken spec must not silently drop the rule —
            // the error is surfaced as a hard canon.setup finding at run time.
            return Err(format!("yaml parse fail {}: {e}", path.display()));
        }
    };
    let raw: RawSpec = match serde_yaml::from_value(v) {
        Ok(r) => r,
        Err(e) => {
            // `deny_unknown_fields`: a typo in any key drops the whole spec.
            // That used to be a silent no-op; now it is a loud error.
            return Err(format!("spec deserialize fail {}: {e}", path.display()));
        }
    };
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .strip_prefix("checklist_")
        .unwrap_or("unknown")
        .to_string();
    let mode = match raw.mode.as_deref().unwrap_or("diff") {
        "file" => Mode::File,
        "grep" => Mode::Grep,
        _ => Mode::Diff,
    };
    let base_severity = raw
        .fail_severity
        .as_deref()
        .and_then(Severity::parse)
        .unwrap_or(Severity::Warn);
    Ok(ChecklistSpec {
        name,
        enabled: raw.enabled.unwrap_or(true),
        // No default hooks: an explicit `hooks:` list is the single routing
        // source (kymido's dispatch.yaml topic router is gone by design).
        hooks: raw.hooks,
        include: raw.r#match.paths_include,
        exclude: raw.r#match.paths_exclude,
        mode,
        command: raw.harness.command,
        args: raw.harness.args,
        timeout_secs: raw.timeout.unwrap_or(60),
        optional: raw.optional.unwrap_or(true),
        base_severity,
        sla: raw.sla.as_deref().map(SlaLevel::parse).unwrap_or_default(),
    })
}

pub fn find_specs(spec_dir: &std::path::Path) -> Vec<(PathBuf, Result<ChecklistSpec, String>)> {
    fn walk(dir: &std::path::Path, out: &mut Vec<PathBuf>, depth: u8) {
        if depth > 3 {
            return;
        }
        for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out, depth + 1);
            } else if path.extension().and_then(|s| s.to_str()) == Some("yaml")
                && path
                    .file_name()
                    .is_some_and(|n| n.to_string_lossy().starts_with("checklist_"))
            {
                out.push(path);
            }
        }
    }
    let mut paths = Vec::new();
    walk(spec_dir, &mut paths, 0);
    // Stable order: file name.
    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    paths
        .into_iter()
        .map(|path| {
            let spec = load_spec(&path);
            (path, spec)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Diff / file plumbing
// ---------------------------------------------------------------------------

/// Merge base ref: `GATE_BASE` env override, else `origin/main...HEAD`.
/// ponytail: env var is enough until a repo needs named bases from config.
fn merge_base() -> String {
    std::env::var("GATE_BASE").unwrap_or_else(|_| "origin/main...HEAD".to_string())
}

/// The base rev alone (no `...HEAD`), for building `<base>...<branch>`.
pub fn base_ref() -> String {
    std::env::var("GATE_BASE")
        .ok()
        .map(|v| v.split("...").next().unwrap_or(&v).to_string())
        .unwrap_or_else(|| "origin/main".to_string())
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    /// The hook's own view: staged, HEAD-relative, or merge-base.
    Hook(HookScope),
    /// An explicit git revision range or single commit, e.g. `main..HEAD`.
    Rev(String),
    /// Paths or directories, optionally relative to a base rev. A directory
    /// expands to its git-tracked files. `base` is what makes "this commit's
    /// changes to these two files" expressible; without one the scope is the
    /// files as they are now (a plain file review).
    Files {
        paths: Vec<String>,
        base: Option<String>,
    },
}

impl From<HookScope> for Target {
    fn from(h: HookScope) -> Self {
        Target::Hook(h)
    }
}

impl Target {
    /// Human-readable label for tool output, so a finding can say what it saw.
    pub fn describe(&self) -> String {
        match self {
            Target::Hook(h) => h.as_str().to_string(),
            Target::Rev(r) => format!("rev {r}"),
            Target::Files { paths, base } => match base {
                Some(b) => format!("{} path(s) since {b}", paths.len()),
                None => format!("{} path(s)", paths.len()),
            },
        }
    }
}

/// `Mode::Grep` harnesses run their own repo-wide scan, so a rev/path target
/// cannot narrow them. Callers surface this rather than implying a scoped run
/// was exhaustive.
pub fn is_scopeable(spec: &ChecklistSpec) -> bool {
    spec.mode != Mode::Grep
}

fn diff_args(target: &Target) -> Vec<String> {
    match target {
        Target::Hook(HookScope::PreCommit) => {
            vec![
                "diff".into(),
                "--cached".into(),
                "--unified=3".into(),
                "--no-color".into(),
            ]
        }
        Target::Hook(HookScope::PrePush) => {
            vec![
                "diff".into(),
                "HEAD".into(),
                "--unified=3".into(),
                "--no-color".into(),
            ]
        }
        Target::Hook(HookScope::Merge) => vec![
            "diff".into(),
            merge_base(),
            "--unified=3".into(),
            "--no-color".into(),
        ],
        Target::Rev(r) => vec![
            "diff".into(),
            r.clone(),
            "--unified=3".into(),
            "--no-color".into(),
        ],
        // Paths go after `--` so they are read as a pathspec list rather than
        // a rev — free correctness, and it keeps a file named like a branch
        // from being read as one.
        Target::Files { paths, base } => {
            let mut v = vec!["diff".into()];
            v.extend(base.clone().into_iter());
            v.extend(["--unified=3".into(), "--no-color".into(), "--".into()]);
            v.extend(paths.iter().cloned());
            v
        }
    }
}

fn capture_diff(target: &Target) -> Option<String> {
    let out = Command::new("git").args(diff_args(target)).output().ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

/// Files a `Mode::File` / `Mode::Diff` check should look at.
///
/// Without a base rev the paths resolve through `git ls-files`, so a directory
/// expands to its tracked contents and the result stays gitignore-aware — the
/// same property that keeps grep harnesses from scanning reference trees.
fn changed_files(target: &Target) -> Vec<String> {
    let args: Vec<String> = match target {
        Target::Hook(HookScope::PreCommit) => {
            vec![
                "diff".into(),
                "--cached".into(),
                "--name-only".into(),
                "--no-color".into(),
            ]
        }
        Target::Hook(HookScope::PrePush) => {
            vec![
                "diff".into(),
                "HEAD".into(),
                "--name-only".into(),
                "--no-color".into(),
            ]
        }
        Target::Hook(HookScope::Merge) => vec![
            "diff".into(),
            merge_base(),
            "--name-only".into(),
            "--no-color".into(),
        ],
        Target::Rev(r) => vec![
            "diff".into(),
            r.clone(),
            "--name-only".into(),
            "--no-color".into(),
        ],
        Target::Files { paths, base } => {
            let mut v = match base {
                // "what changed in these paths since base"
                Some(b) => vec![
                    "diff".into(),
                    b.clone(),
                    "--name-only".into(),
                    "--no-color".into(),
                    "--".into(),
                ],
                // "these files as they are now"
                None => vec!["ls-files".into(), "--".into()],
            };
            v.extend(paths.iter().cloned());
            v
        }
    };
    let Ok(out) = Command::new("git").args(&args).output() else {
        return vec![];
    };
    if !out.status.success() {
        return vec![];
    }
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn matches_include(rel: &str, include: &str) -> bool {
    // Loose match is fine: the harness sees the full file content.
    let pat = include.strip_prefix("**/").unwrap_or(include);
    if let Some(ext) = pat.strip_prefix("*.") {
        return rel.ends_with(&format!(".{ext}"));
    }
    if let Some(suffix) = pat.strip_prefix('*') {
        return rel.ends_with(suffix);
    }
    rel == pat || rel.contains(&format!("/{pat}"))
}

fn file_matches(spec: &ChecklistSpec, rel: &str) -> bool {
    if !spec.exclude.iter().all(|x| !rel.contains(x)) {
        return false;
    }
    if spec.include.is_empty() {
        return true;
    }
    spec.include.iter().any(|p| matches_include(rel, p))
}

fn has_match(spec: &ChecklistSpec, target: &Target) -> bool {
    let files = changed_files(target);
    files.iter().any(|f| file_matches(spec, f))
}

// ---------------------------------------------------------------------------
// Harness invocation
// ---------------------------------------------------------------------------

fn run_harness(spec: &ChecklistSpec, stdin_payload: &[u8]) -> (i32, String) {
    // Capture stdout/stderr into temp files, not pipes. A full-repo audit
    // harness emits far more than a pipe buffer holds (measured: 87 KB vs the
    // 64 KB default) and the engine only drains stdout after the child exits —
    // with pipes that ordering deadlocks the child on write until its timeout
    // kills it (the 15-minute merge runs). Files never block, and a timed-out
    // harness still yields partial output for diagnostics.
    let mut out_capture = match CaptureFile::create("stdout") {
        Ok(c) => c,
        Err(_) => return (127, String::new()),
    };
    let mut err_capture = match CaptureFile::create("stderr") {
        Ok(c) => c,
        Err(_) => return (127, String::new()),
    };
    let mut cmd = Command::new(&spec.command);
    cmd.args(&spec.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::from(out_capture.take_handle()))
        .stderr(Stdio::from(err_capture.take_handle()));
    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(_) => return (127, String::new()),
    };
    // Feed stdin from a thread: a harness that never reads must not block
    // the timeout loop below on a full pipe.
    let payload = stdin_payload.to_vec();
    let mut sin = child.stdin.take();
    std::thread::spawn(move || {
        if let Some(w) = &mut sin {
            let _ = w.write_all(&payload);
        }
    });
    let (rc, _piped) = wait_with_timeout(child, spec.timeout_secs);
    let mut combined = out_capture.read();
    combined.push_str(&err_capture.read());
    (rc, combined)
}

/// One temp file a harness writes into; read back after the child exits and
/// removed on drop. Unique per harness so parallel runs never share a path.
struct CaptureFile {
    path: PathBuf,
    handle: Option<std::fs::File>,
}

impl CaptureFile {
    fn create(tag: &str) -> std::io::Result<Self> {
        static SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let seq = SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("canon-harness-{}-{seq}.{tag}", std::process::id()));
        let file = std::fs::File::create(&path)?;
        Ok(Self {
            path,
            handle: Some(file),
        })
    }

    /// Hand the write handle to the child's stdio. Taken exactly once.
    fn take_handle(&mut self) -> std::fs::File {
        self.handle
            .take()
            .expect("CaptureFile handle taken exactly once")
    }

    fn read(&self) -> String {
        std::fs::read_to_string(&self.path).unwrap_or_default()
    }
}

impl Drop for CaptureFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// Poll until the child exits or the deadline passes; kill on timeout.
/// Timeout returns rc=2 (per CHECKLIST_SPEC: harness failed → WARN skip).
fn wait_with_timeout(mut child: Child, timeout_secs: u64) -> (i32, String) {
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let code = status.code().unwrap_or(-1);
                let mut combined = String::new();
                if let Some(mut out) = child.stdout.take() {
                    let _ = out.read_to_string(&mut combined);
                }
                if let Some(mut err) = child.stderr.take() {
                    let _ = err.read_to_string(&mut combined);
                }
                return (code, combined);
            }
            Ok(None) if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                return (2, String::new());
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(50)),
            Err(_) => return (1, String::new()),
        }
    }
}

fn merge_severity(base: Severity, reported: Severity) -> Severity {
    // Severity order: Fail < Warn < Info. Max-wins means smaller ordinal.
    std::cmp::min(base, reported)
}

fn findings_from_stdout(spec: &ChecklistSpec, stdout: &str) -> Vec<Finding> {
    let trimmed = stdout.trim();
    if trimmed.is_empty() || trimmed == "[]" {
        return vec![];
    }
    // Try as a bare array first; if that fails, try a single object.
    let parsed: Result<HarnessFinding, _> = serde_json::from_str(trimmed);
    let items: Vec<FindingJson> = match parsed {
        Ok(HarnessFinding::Many(v)) => v,
        Ok(HarnessFinding::Single(s)) => vec![s],
        Err(_) => {
            // Maybe harness wrapped output — try the last JSON array on a line.
            if let Some(start) = trimmed.rfind('[')
                && let Some(end) = trimmed.rfind(']')
                && end > start
                && let Ok(HarnessFinding::Many(v)) = serde_json::from_str(&trimmed[start..=end])
            {
                return convert(spec, v);
            }
            eprintln!(
                "checklist.{}: harness stdout not valid JSON; first 80 chars: {}",
                spec.name,
                truncate_utf8(trimmed, 80)
            );
            return vec![Finding::new(
                &format!("checklist.{}.CK-01", spec.name),
                Severity::Warn,
                "harness output not valid JSON; check skipped",
            )];
        }
    };
    convert(spec, items)
}

fn convert(spec: &ChecklistSpec, items: Vec<FindingJson>) -> Vec<Finding> {
    items
        .into_iter()
        .filter_map(|raw| {
            let reported = Severity::parse(&raw.severity)?;
            let severity = merge_severity(spec.base_severity, reported);
            let id = format!("checklist.{}.{}", spec.name, raw.id);
            let path_prefix = raw
                .path
                .as_deref()
                .filter(|p| !p.is_empty())
                .map(|p| format!("{p}: "))
                .unwrap_or_default();
            let line_suffix = raw.line.map(|l| format!(" (L{l})")).unwrap_or_default();
            let msg = format!("{path_prefix}{}{line_suffix}", raw.message);
            let mut f = Finding::new(&id, severity, &msg);
            if let Some(l) = raw.line {
                f = f.with_line(l);
            }
            // Forward harness-provided extra fields (score, confidence,
            // evidence) so --json mode can expose them to dev agents.
            for (k, v) in raw.extra {
                let s = match v {
                    serde_json::Value::String(s) => s,
                    other => other.to_string(),
                };
                f = f.with_extra(&k, s);
            }
            Some(f)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Per-spec execution
// ---------------------------------------------------------------------------

fn run_one(spec: &ChecklistSpec, target: &Target, ignore_hooks: bool) -> Vec<Finding> {
    if !spec.enabled {
        return vec![Finding::new(
            &format!("checklist.{}", spec.name),
            Severity::Info,
            "disabled in config",
        )];
    }
    if !ignore_hooks {
        // A hook target honours `hooks:` routing; an explicit rev/path target
        // is already a deliberate ask, so the hook filter must not veto it.
        let routed = match target {
            Target::Hook(h) => spec.hooks.iter().any(|x| h.matches_yaml(x)),
            _ => true,
        };
        if !routed {
            return vec![]; // not in this hook's scope — silent skip
        }
    }
    if spec.mode != Mode::Grep && !has_match(spec, target) {
        return vec![Finding::new(
            &format!("checklist.{}", spec.name),
            Severity::Info,
            "no matching files in diff",
        )];
    }

    let stdin_payload: Vec<u8> = match spec.mode {
        Mode::Diff => capture_diff(target).unwrap_or_default().into_bytes(),
        Mode::File => {
            // Concatenate all matching changed files; harness gets a clear
            // separator so it can attribute findings back to a file.
            let root = git::git_root().unwrap_or_else(|| PathBuf::from("."));
            let mut buf = String::new();
            for rel in changed_files(target) {
                if !file_matches(spec, &rel) {
                    continue;
                }
                let path = root.join(&rel);
                if let Ok(content) = std::fs::read_to_string(&path) {
                    buf.push_str(&format!("\n===== FILE: {rel} =====\n"));
                    buf.push_str(&content);
                }
            }
            buf.into_bytes()
        }
        Mode::Grep => Vec::new(), // harness runs static checks itself
    };

    if stdin_payload.is_empty() && spec.mode != Mode::Grep {
        return vec![Finding::new(
            &format!("checklist.{}", spec.name),
            Severity::Info,
            "empty diff; nothing to check",
        )];
    }

    let (rc, output) = run_harness(spec, &stdin_payload);
    if rc == 127 || output.to_lowercase().contains("no such file or directory") {
        let sev = if spec.optional {
            Severity::Warn
        } else {
            Severity::Fail
        };
        return vec![Finding::new(
            &format!("checklist.{}", spec.name),
            sev,
            &format!("harness not installed: {} (skipped)", spec.command),
        )];
    }
    if rc != 0 && rc != 2 {
        // Unexpected non-zero — surface as WARN with exit code.
        return vec![Finding::new(
            &format!("checklist.{}", spec.name),
            Severity::Warn,
            &format!("harness exited {rc}: {}", truncate_utf8(&output, 200)),
        )];
    }
    if rc == 2 {
        return vec![Finding::new(
            &format!("checklist.{}", spec.name),
            Severity::Warn,
            &format!("harness internal error: {}", truncate_utf8(&output, 200)),
        )];
    }

    findings_from_stdout(spec, &output)
}

/// Spec dir: `.githooks/spec` found by walking up from cwd.
pub fn spec_dir() -> Option<PathBuf> {
    git::find_githooks_dir().map(|g| g.join("spec"))
}

// ---------------------------------------------------------------------------
// Parallel execution
// ---------------------------------------------------------------------------

/// How many checklist harnesses may run at once. Harnesses are external
/// processes (cargo, python, grep) that mostly *wait*, so a handful in
/// parallel turns the serial wall-clock sum into roughly the slowest single
/// rule. `CANON_CHECK_PARALLELISM` overrides; `1` restores the serial path.
const DEFAULT_CHECK_PARALLELISM: usize = 4;
const MAX_CHECK_PARALLELISM: usize = 16;

fn check_parallelism(jobs: usize) -> usize {
    let requested = std::env::var("CANON_CHECK_PARALLELISM")
        .ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .unwrap_or(DEFAULT_CHECK_PARALLELISM);
    requested.clamp(1, MAX_CHECK_PARALLELISM).min(jobs.max(1))
}

/// Map `f` over `jobs` with at most [`check_parallelism`] workers, returning
/// per-job output **in input order** — findings stay byte-identical to the
/// serial run no matter which worker finishes first.
fn parallel_map<T, F>(jobs: &[T], f: F) -> Vec<Vec<Finding>>
where
    T: Sync,
    F: Fn(&T) -> Vec<Finding> + Sync,
{
    if jobs.is_empty() {
        return Vec::new();
    }
    let workers = check_parallelism(jobs.len());
    if workers <= 1 {
        return jobs.iter().map(&f).collect();
    }
    let next = std::sync::atomic::AtomicUsize::new(0);
    let slots: std::sync::Mutex<Vec<Option<Vec<Finding>>>> =
        std::sync::Mutex::new((0..jobs.len()).map(|_| None).collect());
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let Some(job) = jobs.get(i) else { break };
                    let out = f(job);
                    if let Ok(mut guard) = slots.lock() {
                        guard[i] = Some(out);
                    }
                }
            });
        }
    });
    slots
        .into_inner()
        .expect("parallel slots mutex")
        .into_iter()
        .map(|slot| slot.unwrap_or_default())
        .collect()
}

/// One `checklist_*.yaml` file: its path plus the parsed spec (or the parse
/// error, kept so the run can fail closed instead of dropping the rule).
type SpecEntry = (PathBuf, Result<ChecklistSpec, String>);

/// Run specs through [`parallel_map`], preserving file order. `keep` selects
/// which entries execute (index-based so callers can pre-filter without
/// cloning specs). A broken spec stays a fail-closed setup finding on its own
/// slot, same as serial.
fn run_specs(
    specs: &[SpecEntry],
    keep: &dyn Fn(usize) -> bool,
    target: &Target,
    ignore_hooks: bool,
) -> Vec<Finding> {
    let jobs: Vec<&SpecEntry> = specs
        .iter()
        .enumerate()
        .filter(|(i, _)| keep(*i))
        .map(|(_, e)| e)
        .collect();
    let outs = parallel_map(&jobs, |(path, spec)| match spec {
        Ok(s) => {
            eprintln!("--- checklist: {} ---", s.name);
            run_one(s, target, ignore_hooks)
        }
        Err(e) => vec![Finding::new(
            "canon.setup",
            Severity::Fail,
            &format!(
                "checklist {} broken, rule disabled: {e} — fix the yaml",
                path.file_name().unwrap_or_default().to_string_lossy()
            ),
        )],
    });
    outs.into_iter().flatten().collect()
}

/// Run all `checklist_*.yaml` matching the scope. Findings are aggregated
/// across every spec; caller applies overrides, prints, and maps to exit code.
///
/// No rules found is a loud FAIL — the handbook's #1 portability pain was a
/// wrong path silently scanning 0 files and passing green.
pub fn run_all(scope: HookScope) -> Vec<Finding> {
    let Some(dir) = spec_dir() else {
        return vec![Finding::new(
            "canon.setup",
            Severity::Fail,
            "no .githooks/ found — run `canon init` in the repo root",
        )];
    };
    let specs = find_specs(&dir);
    if specs.is_empty() {
        return vec![Finding::new(
            "canon.setup",
            Severity::Fail,
            &format!(
                "no checklist_*.yaml under {} — seed a rules pack (`canon init`) or fix the path",
                dir.display()
            ),
        )];
    }
    run_specs(&specs, &|_| true, &Target::Hook(scope), false)
}

/// Run rules against an explicit [`Target`] — a rev range, a commit, or a set
/// of paths — instead of the working tree. Same broken-spec and empty-pack
/// fail-closed handling as [`run_all`], so a scoped run can never pass green by
/// scanning nothing.
pub fn run_targeted(target: Target, names: &[String], max_sla: SlaLevel) -> Vec<Finding> {
    let Some(dir) = spec_dir() else {
        return vec![Finding::new(
            "canon.setup",
            Severity::Fail,
            "no .githooks/ found — run `canon init` in the repo root",
        )];
    };
    let specs = find_specs(&dir);
    if specs.is_empty() {
        return vec![Finding::new(
            "canon.setup",
            Severity::Fail,
            &format!(
                "no checklist_*.yaml under {} — seed a rules pack (`canon init`) or fix the path",
                dir.display()
            ),
        )];
    }
    run_specs(
        &specs,
        &|i| match &specs[i].1 {
            Ok(s) => (names.is_empty() || names.contains(&s.name)) && s.sla <= max_sla,
            // Broken specs surface even when the filter excludes them:
            // fail-closed beats quiet.
            Err(_) => true,
        },
        &target,
        true,
    )
}

/// Display name of a checklist spec from its file stem
/// (`checklist_ccn.yaml` → `ccn`); `None` for non-conforming names.
fn spec_stem_name(path: &PathBuf) -> Option<String> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| s.strip_prefix("checklist_"))
        .map(str::to_string)
}

/// `canon check [names...]` — manual run on Merge scope regardless of the
/// `hooks:` filter. No names → list what is available. SLA filter applies.
pub fn run_named(names: &[String], max_sla: SlaLevel) -> Vec<Finding> {
    let Some(dir) = spec_dir() else {
        // Fail-closed on the manual path too: no spec = hard FAIL, not a
        // quiet ALL PASS.
        return vec![Finding::new(
            "canon.setup",
            Severity::Fail,
            "no .githooks/ found — run `canon init` in the repo root",
        )];
    };
    run_named_in(&dir, names, max_sla)
}

/// Same as [`run_named`] against an explicit spec directory — flow's
/// `spec_run` tool checks *other* repos this way (its caller chdirs into the
/// target repo root first; harness commands are cwd-anchored and the flow
/// layer serializes those runs behind a lock).
pub fn run_named_in(
    spec_dir: &std::path::Path,
    names: &[String],
    max_sla: SlaLevel,
) -> Vec<Finding> {
    let findings = Vec::new();
    let specs = find_specs(spec_dir);
    if specs.is_empty() {
        return vec![Finding::new(
            "canon.setup",
            Severity::Fail,
            &format!(
                "no checklist_*.yaml under {} — seed a rules pack (`canon init`) or fix the path",
                spec_dir.display()
            ),
        )];
    }
    if names.is_empty() {
        for (path, s) in &specs {
            match s {
                Ok(s) if s.sla <= max_sla => eprintln!("{}", s.name),
                Ok(_) => {}
                Err(e) => eprintln!(
                    "{} (broken spec: {e})",
                    spec_stem_name(path).unwrap_or_else(|| "?".to_string())
                ),
            }
        }
        return findings;
    }
    let available = specs
        .iter()
        .filter_map(|(_, s)| s.as_ref().ok().map(|s| s.name.as_str()))
        .collect::<Vec<_>>()
        .join(", ");
    let jobs: Vec<(&String, Option<&SpecEntry>)> = names
        .iter()
        .map(|name| {
            // A broken spec is a setup failure, not an unknown name: match the
            // requested name against broken specs' file stems first.
            let broken = specs
                .iter()
                .find(|(p, s)| s.is_err() && spec_stem_name(p).as_deref() == Some(name.as_str()));
            let found = broken.or_else(|| {
                specs
                    .iter()
                    .find(|(_, s)| s.as_ref().ok().is_some_and(|s| &s.name == name))
            });
            (name, found)
        })
        .collect();
    let outs = parallel_map(&jobs, |(name, entry)| match entry {
        Some((_path, Ok(spec))) => {
            eprintln!("--- checklist: {} ---", spec.name);
            run_one(spec, &Target::Hook(HookScope::Merge), true)
        }
        Some((path, Err(e))) => vec![Finding::new(
            "canon.setup",
            Severity::Fail,
            &format!(
                "checklist {} broken: {e} — fix the yaml",
                path.file_name().unwrap_or_default().to_string_lossy()
            ),
        )],
        None => {
            eprintln!("unknown checklist: {name} (available: {available})");
            vec![]
        }
    });
    outs.into_iter().flatten().collect()
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_map_preserves_input_order_under_completion_skew() {
        // Reversed input + descending sleeps: a naive unordered collect would
        // emit the fastest job first. Findings must stay in input order.
        let jobs: Vec<usize> = (0..12).rev().collect();
        let out = parallel_map(&jobs, |i| {
            std::thread::sleep(Duration::from_millis((12 - i) as u64 * 4));
            vec![Finding::new(&format!("job-{i}"), Severity::Info, "")]
        });
        let got: Vec<String> = out.into_iter().flatten().map(|f| f.rule_id).collect();
        let want: Vec<String> = jobs.iter().map(|i| format!("job-{i}")).collect();
        assert_eq!(
            got, want,
            "findings must stay in input order regardless of completion order"
        );
    }

    #[test]
    fn parallel_map_observes_more_than_one_worker() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let live = AtomicUsize::new(0);
        let peak = AtomicUsize::new(0);
        let jobs: Vec<usize> = (0..8).collect();
        parallel_map(&jobs, |_| {
            let now_live = live.fetch_add(1, Ordering::SeqCst) + 1;
            peak.fetch_max(now_live, Ordering::SeqCst);
            std::thread::sleep(Duration::from_millis(80));
            live.fetch_sub(1, Ordering::SeqCst);
            vec![]
        });
        assert!(
            peak.load(Ordering::SeqCst) > 1,
            "checklist harnesses must overlap when parallelism > 1"
        );
    }

    #[test]
    fn check_parallelism_stays_bounded() {
        assert_eq!(check_parallelism(0), 1);
        assert_eq!(check_parallelism(1), 1);
        // Env is not mutated in tests (edition 2024 set_var is unsafe), so
        // this asserts the invariants rather than a specific default.
        for jobs in [2usize, 3, 9, 64] {
            let n = check_parallelism(jobs);
            assert!(
                (1..=jobs).contains(&n) && n <= MAX_CHECK_PARALLELISM,
                "parallelism {n} out of bounds for {jobs} jobs"
            );
        }
    }

    #[test]
    fn run_harness_captures_output_larger_than_pipe_buffer() {
        // 70 KB > the 64 KiB pipe buffer. With piped stdout that the engine
        // only drains after exit, this deadlocked until the harness timeout
        // (the 15-minute merge runs); with file capture it must exit cleanly
        // and keep every byte.
        let s = spec(
            "harness: {command: python3, args: [\"-c\", \"import sys; sys.stdout.write('a'*70000)\"]}\ntimeout: 20",
        );
        let (rc, out) = run_harness(&s, b"");
        assert_eq!(rc, 0, "harness should exit cleanly, rc={rc}, out={out:?}");
        assert!(
            out.len() >= 70_000,
            "harness output must not be truncated: {} bytes",
            out.len()
        );
    }

    fn spec(yaml: &str) -> ChecklistSpec {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("checklist_t.yaml");
        std::fs::write(&path, yaml).unwrap();
        load_spec(&path).unwrap()
    }

    #[test]
    fn spec_parses_defaults() {
        let s = spec("harness: {command: sh, args: []}");

        assert_eq!(s.mode, Mode::Diff);
        assert_eq!(s.base_severity, Severity::Warn);
        assert_eq!(s.sla, SlaLevel::L1);
        assert!(s.optional);
        assert!(
            s.hooks.is_empty(),
            "no implicit hooks — yaml is the only router"
        );
    }
    #[test]
    fn broken_spec_is_loud_not_silent() {
        // A typo'd key used to drop the whole spec silently (fail-open).
        // It must now surface as an Err so run_all can hard-block on it.
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("checklist_bad.yaml");
        std::fs::write(&path, "harness: {command: sh}\nbad_key: 1\n").unwrap();
        let specs = find_specs(dir.path());
        assert_eq!(specs.len(), 1);
        assert!(
            specs[0].1.is_err(),
            "typo'd key must fail loudly, not silently skip"
        );
        assert!(specs[0].1.as_ref().unwrap_err().contains("deserialize"));
    }

    #[test]
    fn spec_parses_overrides() {
        let s = spec(
            "mode: grep\nfail_severity: FAIL\nsla: l3\noptional: false\nhooks: [merge]\ntimeout: 5",
        );
        assert_eq!(s.mode, Mode::Grep);
        assert_eq!(s.base_severity, Severity::Fail);
        assert_eq!(s.sla, SlaLevel::L3);
        assert!(!s.optional);
        assert_eq!(s.hooks, vec!["merge".to_string()]);
        assert_eq!(s.timeout_secs, 5);
    }

    #[test]
    fn finding_json_roundtrip() {
        let spec = spec("harness: {command: sh, args: []}\nfail_severity: FAIL");
        let out = findings_from_stdout(
            &spec,
            r#"[{"id":"HS-01","severity":"WARN","path":"a.py","line":3,"message":"secret","confidence":0.9}]"#,
        );
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].rule_id, "checklist.t.HS-01");
        // max(yaml base, harness report): FAIL base + WARN report → FAIL blocks
        assert_eq!(out[0].severity, Severity::Fail);
        assert_eq!(out[0].line_hint, Some(3));
        assert_eq!(
            out[0].extra.get("confidence").map(String::as_str),
            Some("0.9")
        );
    }
}
