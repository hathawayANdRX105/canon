//! Rule inventory over a deployed spec tree.
//!
//! Two spec shapes live under `.githooks/spec/`: `checklist_*.yaml` (rules that
//! describe themselves — `sla`, `fail_severity`, `hooks`, `mode`) and the
//! `github/*.yaml` policy files (rule-id → severity maps, no self-description).
//! Both flatten into one [`Rule`] list so an agent can ask "what does this repo
//! enforce, and how do I fix it" without parsing yaml itself.
//!
//! The `why` text is the leading `#` block of each file — the handbook's
//! "为什么 / 检测" prose. serde_yaml drops comments, so it has to be read off
//! the raw file head; that prose is the whole reason `spec_explain` is worth
//! having, since canon's own output only carries `rule_id + msg`.

use std::path::Path;

use serde_yaml::Value as YamlValue;

use crate::shared::Severity;

/// One enforceable rule, whichever spec file declared it.
#[derive(Debug, Clone)]
pub struct Rule {
    /// `checklist_rust_no_dead_code_allow`, or a github rule id like `IS-04`.
    pub id: String,
    /// Path relative to the spec root, for tracing a rule back to its source.
    pub source: String,
    /// Effective severity after any override — this is what decides blocking.
    pub severity: Severity,
    /// `l1` / `l2` / `l3`; absent for the github policy rules.
    pub sla: Option<String>,
    /// Hooks the rule is registered on (`pre-commit`, `pre-push`, `merge`).
    pub hooks: Vec<String>,
    /// `grep` / `diff` / `file` — how the harness reads the tree.
    pub mode: Option<String>,
    /// Leading comment block: why the rule exists and what it looks for.
    pub why: String,
}

/// Leading `#` comment lines, up to the first non-comment line. Comment
/// markers and surrounding blank padding come off; the prose stays verbatim.
fn why_from_head(raw: &str) -> String {
    let mut lines: Vec<&str> = Vec::new();
    for line in raw.lines() {
        let t = line.trim_start();
        match t.strip_prefix('#') {
            Some(rest) => lines.push(rest.trim()),
            None if t.is_empty() => {
                // A blank line ends the header only once prose has started;
                // leading blanks are just yaml indentation noise.
                if !lines.is_empty() {
                    break;
                }
            }
            None => break,
        }
    }
    while lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }
    lines.join("\n")
}

fn checklist_id(path: &Path) -> String {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    name.trim_start_matches("checklist_")
        .trim_end_matches(".yaml")
        .to_string()
}

/// Which dispatch topic owns a spec file, or `None` when the file declares its
/// own `hooks:` and needs no routing.
fn topic_of(source: &str) -> Option<String> {
    let (dir, name) = source.split_once('/')?;
    if name.starts_with("checklist_") {
        // `checklist` is a cross-directory topic: every checklist_*.yaml anywhere.
        return Some("checklist".into());
    }
    if dir == "github" {
        return Some(format!("github/{}", name.trim_end_matches(".yaml")));
    }
    matches!(dir, "code" | "cleanup" | "workspace" | "dioxus").then(|| dir.to_string())
}

fn hooks_from_dispatch(dispatch: Option<&YamlValue>, source: &str) -> Vec<String> {
    let Some(topic) = topic_of(source) else {
        return vec![];
    };
    let mut out = vec![];
    for hook in ["pre-commit", "pre-push", "merge"] {
        let Some(list) = dispatch
            .and_then(|d| d.get(hook))
            .and_then(|v| v.as_sequence())
        else {
            continue;
        };
        if list.iter().any(|t| t.as_str() == Some(&topic)) {
            out.push(hook.to_string());
        }
    }
    out
}

/// `severity_overrides.yaml` carries the full rule registry as a comment block
/// (`#   IS-04: FAIL — Done when 缺少 checkbox`). Only the `severity_overrides:`
/// mapping below it is machine-read by the gate binary, and that mapping is
/// usually empty — the comment is the only complete statement of what exists,
/// rules with no active override (IS-00..03, PR-01/02, RV-07, WS-*, CL-*).
fn parse_registry(raw: &str) -> Vec<Rule> {
    let mut out = vec![];
    for line in raw.lines() {
        let Some(rest) = line.trim_start().strip_prefix('#') else {
            continue;
        };
        let rest = rest.trim();
        // `PR-01: FAIL — 标题含 CJK`
        let Some((head, desc)) = rest.split_once("—").or_else(|| rest.split_once('-')) else {
            continue;
        };
        let head = head.trim();
        let Some((id, sev)) = head.split_once(':') else {
            continue;
        };
        let id = id.trim();
        if !id
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-')
            || !id.contains('-')
        {
            continue;
        }
        let Some(severity) = Severity::parse(sev.trim()) else {
            continue;
        };
        out.push(Rule {
            id: id.to_string(),
            severity,
            sla: None,
            hooks: enforcement_point(id)
                .iter()
                .map(|s| s.to_string())
                .collect(),
            mode: None,
            source: "severity_overrides.yaml#registry".into(),
            why: desc.trim().to_string(),
        });
    }
    out
}

/// Registry rules are not git-hook rules — they fire from the `gh` wrapper
/// `canon init` installs as `~/.local/bin/gh`, or from the commit-msg hook.
fn enforcement_point(id: &str) -> &'static [&'static str] {
    match id.split('-').next().unwrap_or_default() {
        "CM" => &["commit-msg"],
        "IS" => &["gh issue create", "gh issue close"],
        "PR" => &["gh pr create", "gh pr merge"],
        "RV" => &["gh pr merge"],
        "WS" | "CL" => &["merge"],
        _ => &[],
    }
}

/// Every rule this repo enforces: the checklist/topic files, the github policy
/// overrides, and the full registry from the overrides file's comment block.
pub fn load(spec_dir: &Path) -> Vec<Rule> {
    let dispatch = read_yaml(&spec_dir.join("dispatch.yaml"));
    let mut out: Vec<Rule> = vec![];

    for sub in [
        "quality",
        "code",
        "cleanup",
        "workspace",
        "github",
        "dioxus",
    ] {
        let dir = spec_dir.join(sub);
        if !dir.is_dir() {
            continue;
        }
        let mut files: Vec<_> = std::fs::read_dir(&dir)
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "yaml"))
            .collect();
        files.sort();
        for f in files {
            if sub == "github" {
                out.extend(github_rules(&f, spec_dir));
            } else if let Some(mut r) = checklist_rule(&f, sub) {
                // Topic-driven files (code_*, cleanup_*, workspace_*) carry no
                // `hooks:` of their own; dispatch.yaml is the routing table.
                if r.hooks.is_empty() {
                    r.hooks = hooks_from_dispatch(dispatch.as_ref(), &r.source);
                }
                out.push(r);
            }
        }
    }

    if let Ok(raw) = std::fs::read_to_string(spec_dir.join("severity_overrides.yaml")) {
        out.extend(parse_registry(&raw));
    }
    // A registry id and a github `severity_overrides` entry can name the same
    // rule; keep the richer record (the registry carries the description) and
    // fall back to the other side's fields rather than dropping either.
    out.sort_by(|a, b| a.id.cmp(&b.id));
    let mut merged: Vec<Rule> = Vec::with_capacity(out.len());
    for r in out {
        match merged.iter_mut().find(|m| m.id == r.id) {
            Some(m) => {
                if m.why.is_empty() {
                    m.why = r.why;
                }
                if m.hooks.is_empty() {
                    m.hooks = r.hooks;
                }
            }
            None => merged.push(r),
        }
    }
    merged
}

/// Named groupings for `preflight(focus=...)`, matched as substrings against
/// the rule id. A free token that is not a preset name is used as a substring
/// itself, so `focus: "antislop"` works without a table entry and the table
/// never becomes a second rule registry to keep in sync.
pub const FOCUS_PRESETS: &[(&str, &[&str])] = &[
    ("test", &["test", "assert"]),
    (
        "refactor",
        &[
            "ccn",
            "duplication",
            "file_size",
            "crg_impact",
            "no_dead_code",
            "no_empty_module",
            "oversize",
        ],
    ),
    (
        "security",
        &["hardcoded_secret", "api_endpoint_security", "secret"],
    ),
    ("style", &["clippy", "code_", "slop", "antislop"]),
    ("size", &["file_size", "ccn", "oversize"]),
    // Doc-hygiene is a merge-time `cleanup_*` topic rule, not a checklist, so it
    // is unreachable from preflight. Point `docs` at the comment-quality rules
    // that are, rather than shipping a preset that matches nothing.
    ("docs", &["doc", "slop_comment", "antislop"]),
    ("lint", &["clippy", "code_", "ccn"]),
];

/// True when `rule_id` falls under `focus`. `focus` is a comma-separated list
/// of preset names or bare substrings; empty focus matches everything.
pub fn focus_matches(focus: &str, rule_id: &str) -> bool {
    let id = rule_id.to_ascii_lowercase();
    focus
        .split(',')
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .any(|token| {
            let token = token.to_ascii_lowercase();
            match FOCUS_PRESETS.iter().find(|(n, _)| *n == token) {
                Some((_, terms)) => terms.iter().any(|t| id.contains(t)),
                None => id.contains(&token),
            }
        })
}

fn read_yaml(path: &Path) -> Option<YamlValue> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_yaml::from_str::<YamlValue>(&s).ok())
}

fn checklist_rule(path: &Path, sub: &str) -> Option<Rule> {
    let raw = std::fs::read_to_string(path).ok()?;
    let cfg = serde_yaml::from_str::<YamlValue>(&raw).ok()?;
    // `enabled: false` is an opt-out, not a rule the agent should be warned
    // about; a missing key means the file is a topic grouping (code_*.yaml,
    // cleanup_*.yaml), not a single enforceable rule.
    if cfg.get("enabled").and_then(|v| v.as_bool()) != Some(true) {
        return None;
    }
    Some(Rule {
        id: checklist_id(path),
        source: format!("{sub}/{}", path.file_name()?.to_string_lossy()),
        severity: cfg
            .get("fail_severity")
            .and_then(|v| v.as_str())
            .and_then(Severity::parse)
            .unwrap_or(Severity::Warn),
        sla: cfg.get("sla").and_then(|v| v.as_str()).map(str::to_string),
        hooks: crate::shared::cfg_str_list(Some(&cfg), "hooks"),
        mode: cfg.get("mode").and_then(|v| v.as_str()).map(str::to_string),
        why: why_from_head(&raw),
    })
}

/// `severity_overrides` in a github policy file is the only machine-readable
/// statement of those rules' severity — the checks themselves live in Rust
/// (`rules/issues.rs` and friends), not in yaml.
fn github_rules(path: &Path, spec_dir: &Path) -> Vec<Rule> {
    let Some(cfg) = read_yaml(path) else {
        return vec![];
    };
    let Some(over) = cfg.get("severity_overrides").and_then(|v| v.as_mapping()) else {
        return vec![];
    };
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let mut out: Vec<Rule> = over
        .iter()
        .filter_map(|(k, v)| {
            Some(Rule {
                id: k.as_str()?.to_string(),
                severity: v.as_str().and_then(Severity::parse)?,
                source: format!("github/{name}"),
                sla: None,
                hooks: Vec::new(),
                mode: None,
                why: String::new(),
            })
        })
        .collect();
    // Root-level overrides (CM-01..03) apply to commit-msg, which has no yaml of
    // its own — surface them alongside the github ids.
    out.extend(root_overrides(spec_dir));
    out
}

fn root_overrides(spec_dir: &Path) -> Vec<Rule> {
    let Some(cfg) = read_yaml(&spec_dir.join("severity_overrides.yaml")) else {
        return vec![];
    };
    let Some(over) = cfg.get("severity_overrides").and_then(|v| v.as_mapping()) else {
        return vec![];
    };
    over.iter()
        .filter_map(|(k, v)| {
            Some(Rule {
                id: k.as_str()?.to_string(),
                severity: v.as_str().and_then(Severity::parse)?,
                source: "severity_overrides.yaml".to_string(),
                sla: None,
                hooks: vec!["commit-msg".to_string()],
                mode: None,
                why: String::new(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn why_strips_markers_and_keeps_prose() {
        let raw = "# 为什么: 防烂注释\n# 检测: 匹配 AI 味\n\n# 另一段\nenabled: true\n";
        assert_eq!(why_from_head(raw), "为什么: 防烂注释\n检测: 匹配 AI 味");
    }

    #[test]
    fn why_empty_when_file_starts_with_key() {
        assert_eq!(why_from_head("enabled: true\n# 尾部注释\n"), "");
    }

    #[test]
    fn checklist_id_strips_affixes() {
        assert_eq!(checklist_id(Path::new("checklist_ccn.yaml")), "ccn");
        assert_eq!(
            checklist_id(Path::new("rust_no_dead_code_allow.yaml")),
            "rust_no_dead_code_allow"
        );
    }
}
