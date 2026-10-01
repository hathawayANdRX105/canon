//! PR-* validation rules.
//!
//! Pure content checks driven by `spec/github_pull_requests.yaml`, including
//! the fork "user:" prefix strip on `head_ref` (PR-08).

use regex::Regex;
use serde_yaml::Value as YamlValue;
use std::collections::HashSet;

use crate::shared::{Finding, Severity};

// ---------------------------------------------------------------------------
// Regexes — compiled once via std::sync::LazyLock (no once_cell dependency).
// ---------------------------------------------------------------------------

fn cjk_re() -> &'static Regex {
    static RE: std::sync::LazyLock<Regex> =
        std::sync::LazyLock::new(|| Regex::new(r"[\u4e00-\u9fff]").unwrap());
    &RE
}

fn heading_re() -> &'static Regex {
    static RE: std::sync::LazyLock<Regex> =
        std::sync::LazyLock::new(|| Regex::new(r"(?m)^#{1,6} ").unwrap());
    &RE
}

fn checkbox_re() -> &'static Regex {
    static RE: std::sync::LazyLock<Regex> =
        std::sync::LazyLock::new(|| Regex::new(r"(?m)^\s*-\s*\[([ xX])\]").unwrap());
    &RE
}

/// Conventional-commit title regex built from the spec's `conventional_commit_types`.
fn conv_commit_re(types: &[String]) -> Regex {
    let alt = types
        .iter()
        .map(|t| regex::escape(t))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(r"^({alt})(\(.+\))?:\s+"))
        .expect("conventional_commit_types must form a valid regex")
}

/// `(?:kw1|kw2) #N` regex built from spec keywords — serves `fixes_keywords`
/// and `linkage_check.plain_text_link_keywords`.
fn link_re(keywords: &[String]) -> Regex {
    let alt = keywords
        .iter()
        .map(|k| regex::escape(k))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(r"(?:{alt})\s+#(\d+)")).expect("keywords must form a valid regex")
}

// ---------------------------------------------------------------------------
// Helpers — direct ports of the Python _functions
// ---------------------------------------------------------------------------

/// True if `s` contains any CJK ideograph (U+4E00–U+9FFF).
fn has_cjk(s: &str) -> bool {
    cjk_re().is_match(s)
}

/// Extract the body text under a `## heading` line, up to the next `## `.
/// Mirrors Python `_section`.
fn section(body: &str, heading: &str) -> String {
    let pattern = format!(r"(?m)^## {}\s*$", regex::escape(heading));
    let re = match Regex::new(&pattern) {
        Ok(r) => r,
        Err(_) => return String::new(),
    };
    let m = match re.find(body) {
        Some(m) => m,
        None => return String::new(),
    };
    let rest = &body[m.end()..];
    let next_re = match Regex::new(r"(?m)^## ") {
        Ok(r) => r,
        Err(_) => return rest.to_string(),
    };
    match next_re.find(rest) {
        Some(n) => rest[..n.start()].to_string(),
        None => rest.to_string(),
    }
}

/// Return all heading texts (leading `#`s stripped) in line order.
/// Mirrors Python `_headings`.
fn headings(body: &str) -> Vec<String> {
    let re = heading_re();
    body.lines()
        .filter(|line| re.is_match(line))
        .map(|line| line.trim().trim_start_matches('#').trim().to_string())
        .collect()
}

/// Extract all `<keyword> #N` issue numbers for the given spec keywords.
fn extract_linked(body: &str, keywords: &[String]) -> Vec<String> {
    link_re(keywords)
        .captures_iter(body)
        .map(|c| c.get(1).unwrap().as_str().to_string())
        .collect()
}

// ---------------------------------------------------------------------------
// Spec-driven values — every check parameter comes from the yaml; nothing
// is baked into code. A missing/unparseable switch keeps the check's
// original severity rather than silently passing or failing.
// ---------------------------------------------------------------------------

/// `<key>: FAIL|WARN|INFO` mode switch. Absent or unparseable ⇒ `default`.
fn check_mode_severity(cfg: Option<&YamlValue>, key: &str, default: Severity) -> Severity {
    crate::shared::cfg_str(cfg, key)
        .and_then(|mode| Severity::parse(&mode))
        .unwrap_or(default)
}

/// `<key>` as a count — accepts `2` or `"2"`. Absent/garbled ⇒ 0, which
/// disables the floor (missing switch ⇒ check off, never a silent default).
fn cfg_usize(cfg: Option<&YamlValue>, key: &str) -> usize {
    crate::shared::cfg_str(cfg, key)
        .and_then(|s| s.parse::<usize>().ok())
        .or_else(|| {
            crate::shared::cfg_get(cfg, key)
                .and_then(|v| v.as_u64())
                .map(|n| n as usize)
        })
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Main entry — the `check_content` port
// ---------------------------------------------------------------------------

/// Pure content validation for a pull request.  No API calls.
///
/// * `title`     — PR title
/// * `body`      — PR body (markdown)
/// * `labels`    — label names on the PR
/// * `head_ref`  — head ref name (may include fork "user:" prefix)
/// * `state`     — "open" or "closed"/"merged"
/// * `cfg`       — parsed `github_pull_requests.yaml`; missing/null ⇒ a single
///                 `canon.setup` FAIL and no checks run
///
/// Returns a `Vec<Finding>` in the same order as the Python version.
pub fn check_content(
    title: &str,
    body: &str,
    labels: &[&str],
    head_ref: &str,
    state: &str,
    draft: bool,
    cfg: Option<&YamlValue>,
) -> Vec<Finding> {
    let mut out = check_content_impl(title, body, labels, head_ref, state, draft, cfg);
    crate::shared::apply_check_allowlist(&mut out, cfg);
    out
}

fn check_content_impl(
    title: &str,
    body: &str,
    labels: &[&str],
    head_ref: &str,
    state: &str,
    draft: bool,
    cfg: Option<&YamlValue>,
) -> Vec<Finding> {
    // No spec ⇒ loud canon.setup FAIL. Never fall back to values baked into
    // code: a repo without its rules pack must not pass green. An empty
    // (null) spec is just as missing as an absent one.
    let cfg = match cfg.filter(|c| !c.is_null()) {
        Some(c) => c,
        None => {
            return vec![crate::shared::missing_cfg_finding(
                "github_pull_requests.yaml",
            )];
        }
    };

    let mut findings: Vec<Finding> = Vec::new();

    // PR-01 title language gate + forbidden fullwidth brackets.
    // `title_must_be_chinese` is the switch; the direction follows the spec's
    // default table (PR-01: FAIL — 标题含 CJK): PR titles are English.
    if crate::shared::cfg_bool(Some(cfg), "title_must_be_chinese").unwrap_or(false) {
        if has_cjk(title) {
            findings.push(Finding::new(
                "PR-01",
                Severity::Fail,
                "title contains CJK (title should be English)",
            ));
        } else {
            findings.push(Finding::new("PR-01", Severity::Info, "title is English"));
        }
        // Fullwidth brackets sit outside U+4E00–U+9FFF, so this is a distinct
        // condition, not a subset of the CJK scan above.
        let mut hits: Vec<char> =
            crate::shared::cfg_str_list(Some(cfg), "forbidden_brackets_in_title")
                .iter()
                .flat_map(|s| s.chars())
                .filter(|b| title.contains(*b))
                .collect();
        hits.sort_unstable();
        hits.dedup();
        if !hits.is_empty() {
            findings.push(Finding::new(
                "PR-01",
                Severity::Fail,
                &format!(
                    "title contains forbidden fullwidth brackets: {}",
                    hits.iter().collect::<String>()
                ),
            ));
        }
    }

    // PR-02 conventional-commit title; severity is the spec's ci_check_mode.
    let conv_types = crate::shared::cfg_str_list(Some(cfg), "conventional_commit_types");
    let conv_severity = check_mode_severity(Some(cfg), "ci_check_mode", Severity::Warn);
    if conv_commit_re(&conv_types).is_match(title) {
        findings.push(Finding::new(
            "PR-02",
            Severity::Info,
            "conventional commit title",
        ));
    } else {
        findings.push(Finding::new(
            "PR-02",
            conv_severity,
            &format!(
                "title not conventional commit (repo template allows natural English): {title}"
            ),
        ));
    }

    // PR-03 body structure headings
    let body_h: HashSet<String> = headings(body).into_iter().collect();
    let required = crate::shared::cfg_str_list(Some(cfg), "required_body_headings");
    for h in &required {
        if body_h.contains(h) {
            findings.push(Finding::new(
                "PR-03",
                Severity::Info,
                &format!("heading present: {h}"),
            ));
        } else {
            findings.push(Finding::new(
                "PR-03",
                Severity::Fail,
                &format!("missing heading: ## {h}"),
            ));
        }
    }

    // PR-07 done-when sections need ≥ done_when_min_checkboxes boxes;
    // severity is the spec's done_when_check_mode.
    let cb_re = checkbox_re();
    let dw_min = cfg_usize(Some(cfg), "done_when_min_checkboxes");
    let dw_severity = check_mode_severity(Some(cfg), "done_when_check_mode", Severity::Fail);
    for h in &crate::shared::cfg_str_list(Some(cfg), "done_when_headings") {
        if body_h.contains(h) {
            let sec = section(body, h);
            let boxes = cb_re.captures_iter(&sec).count();
            if boxes < dw_min {
                findings.push(Finding::new(
                    "PR-07",
                    dw_severity,
                    &format!("{h} 必须至少 {dw_min} 个 checkbox，当前 {boxes} 个"),
                ));
            }
        }
    }

    // PR-04 headings English only + Chinese-prose sections
    let all_headings = headings(body);
    let bad_h: Vec<&String> = all_headings.iter().filter(|h| has_cjk(h)).collect();
    if !bad_h.is_empty() {
        let joined: Vec<String> = bad_h.iter().map(|h| h.to_string()).collect();
        findings.push(Finding::new(
            "PR-04",
            Severity::Fail,
            &format!(
                "headings contain CJK (headings must be English): [{}]",
                joined.join(", ")
            ),
        ));
    } else {
        findings.push(Finding::new(
            "PR-04",
            Severity::Info,
            "headings are English only",
        ));
    }
    for h in &crate::shared::cfg_str_list(Some(cfg), "chinese_prose_sections") {
        let prose = section(body, h);
        if has_cjk(&prose) {
            findings.push(Finding::new(
                "PR-04",
                Severity::Info,
                &format!("{h} section has Chinese prose"),
            ));
        } else {
            findings.push(Finding::new(
                "PR-04",
                Severity::Warn,
                &format!("{h} section has no Chinese prose (template requires Chinese)"),
            ));
        }
    }

    // PR-05 issue linkage — Fixes #N checks
    let fixes = extract_linked(
        body,
        &crate::shared::cfg_str_list(Some(cfg), "fixes_keywords"),
    );
    // dedupe + sort numerically (Python: sorted(set(fixes), key=int))
    let mut fixes_unique: Vec<i32> = fixes
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    fixes_unique.sort();
    let fixes_count = fixes_unique.len();

    if state == "open" && fixes_count > 0 {
        findings.push(Finding::new(
            "PR-05",
            Severity::Warn,
            "open PR already uses Fixes # (may close issue prematurely)",
        ));
    } else {
        findings.push(Finding::new(
            "PR-05",
            Severity::Info,
            "no premature Fixes while open (or PR not open)",
        ));
    }
    if fixes_count == 1 {
        findings.push(Finding::new("PR-05", Severity::Info, "exactly one Fixes #"));
    } else if fixes_count == 0 {
        if draft {
            findings.push(Finding::new(
                "PR-05",
                Severity::Info,
                "draft PR, Fixes may appear at merge authorization",
            ));
        } else {
            findings.push(Finding::new(
                "PR-05",
                Severity::Warn,
                "no Fixes # yet (needs one primary issue before merge)",
            ));
        }
    } else {
        findings.push(Finding::new(
            "PR-05",
            Severity::Warn,
            &format!("multiple Fixes # ({fixes_count}): one PR should close one issue"),
        ));
    }
    if fixes_count <= 1 {
        findings.push(Finding::new("PR-05", Severity::Info, "one primary issue"));
    } else {
        findings.push(Finding::new(
            "PR-05",
            Severity::Warn,
            "one PR should close one primary issue",
        ));
    }

    // PR-10 plain-text Part of / Related links. Keywords and severity come
    // from the spec's linkage_check map.
    let linkage = crate::shared::cfg_get(Some(cfg), "linkage_check");
    let text_links = extract_linked(
        body,
        &crate::shared::cfg_str_list(linkage, "plain_text_link_keywords"),
    );
    if !text_links.is_empty() {
        findings.push(Finding::new(
            "PR-10",
            check_mode_severity(linkage, "plain_text_links_severity", Severity::Info),
            &format!(
                "Part of/Related #({}) 是纯文本，不产生 GitHub 关联；epic 关联通过 Fixes 的 sub-issue 层级或 UI development 面板",
                text_links.join(", ")
            ),
        ));
    } else {
        findings.push(Finding::new(
            "PR-10",
            Severity::Info,
            "no plain-text Part of/Related links",
        ));
    }

    // PR-06 type label + keyword suggestions (appears twice in Python — once
    // before P-11 and once before P-31; both emit identical findings.  We
    // replicate the first occurrence here.)
    let type_labels = crate::shared::cfg_str_list(Some(cfg), "type_labels_cfg");
    let label_set: HashSet<&str> = labels.iter().copied().collect();
    if type_labels.iter().any(|l| label_set.contains(l.as_str())) {
        findings.push(Finding::new("PR-06", Severity::Info, "type label present"));
    } else {
        findings.push(Finding::new(
            "PR-06",
            Severity::Fail,
            "no type label (expected one of the type set)",
        ));
    }
    let kw_map = crate::shared::cfg_str_map(Some(cfg), "keyword_label_suggestions");
    {
        let haystack = format!("{title}\n{body}").to_lowercase();
        let mut missing: Vec<String> = Vec::new();
        for (keyword, suggested) in &kw_map {
            if haystack.contains(&keyword.to_lowercase()) && !label_set.contains(suggested.as_str())
            {
                missing.push(suggested.clone());
            }
        }
        if !missing.is_empty() {
            // dedupe + sort (Python: sorted(set(missing)))
            let deduped: Vec<String> = {
                let s: HashSet<&str> = missing.iter().map(|s| s.as_str()).collect();
                let mut v: Vec<String> = s.into_iter().map(String::from).collect();
                v.sort();
                v
            };
            findings.push(Finding::new(
                "PR-06",
                Severity::Warn,
                &format!(
                    "based on content keywords, consider also labeling: {}",
                    deduped.join(" ")
                ),
            ));
        } else {
            findings.push(Finding::new(
                "PR-06",
                Severity::Info,
                "content keywords align with assigned labels",
            ));
        }
    }

    // PR-08 branch name — strip fork "user:" prefix before prefix check.
    // This is the fix for the real bug: a forkPR's head_ref is "user:branch",
    // and we must check the *branch* part, not "user:branch".
    let allowed = crate::shared::cfg_str_list(Some(cfg), "allowed_branch_prefixes");
    let branch = if head_ref.contains(':') {
        // Python: head_ref.rsplit(":", 1)[-1] — last segment after the final ":"
        head_ref.rsplit(':').next().unwrap_or(head_ref)
    } else {
        head_ref
    };
    if branch.is_empty() || !allowed.iter().any(|p| branch.starts_with(p.as_str())) {
        findings.push(Finding::new(
            "PR-08",
            Severity::Fail,
            &format!(
                "branch name not allowed: {branch} (allowed prefixes: {:?})",
                allowed
            ),
        ));
    } else {
        findings.push(Finding::new(
            "PR-08",
            Severity::Info,
            &format!("branch name OK: {branch} (prefixes: {:?})", allowed),
        ));
    }

    // PR-06 duplicate block (Python emits it a second time before P-31).
    // We replicate faithfully.
    if type_labels.iter().any(|l| label_set.contains(l.as_str())) {
        findings.push(Finding::new("PR-06", Severity::Info, "type label present"));
    } else {
        findings.push(Finding::new(
            "PR-06",
            Severity::Fail,
            "no type label (expected one of the type set)",
        ));
    }
    {
        let haystack = format!("{title}\n{body}").to_lowercase();
        let mut missing: Vec<String> = Vec::new();
        for (keyword, suggested) in &kw_map {
            if haystack.contains(&keyword.to_lowercase()) && !label_set.contains(suggested.as_str())
            {
                missing.push(suggested.clone());
            }
        }
        if !missing.is_empty() {
            let deduped: Vec<String> = {
                let s: HashSet<&str> = missing.iter().map(|s| s.as_str()).collect();
                let mut v: Vec<String> = s.into_iter().map(String::from).collect();
                v.sort();
                v
            };
            findings.push(Finding::new(
                "PR-06",
                Severity::Warn,
                &format!(
                    "based on content keywords, consider also labeling: {}",
                    deduped.join(" ")
                ),
            ));
        } else {
            findings.push(Finding::new(
                "PR-06",
                Severity::Info,
                "content keywords align with assigned labels",
            ));
        }
    }
    // Apply user-provided severity overrides (e.g. PR-05 demoted to WARN).
    crate::shared::apply_severity_overrides(&mut findings, Some(cfg));

    findings
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(yaml: &str) -> YamlValue {
        serde_yaml::from_str(yaml).expect("valid inline spec")
    }

    #[test]
    fn missing_spec_is_a_loud_gate_setup_fail() {
        // None: no spec on disk at all.
        let findings = check_content(
            "feat: add thing",
            "## Issue\n",
            &[],
            "feat/a",
            "open",
            false,
            None,
        );
        assert_eq!(findings.len(), 1, "no checks may run without the spec");
        assert_eq!(findings[0].rule_id, "canon.setup");
        assert_eq!(findings[0].severity, Severity::Fail);

        // A null (empty) spec is just as missing — never silently green.
        let findings = check_content(
            "feat: add thing",
            "## Issue\n",
            &[],
            "feat/a",
            "open",
            false,
            Some(&YamlValue::Null),
        );
        assert_eq!(findings.len(), 1, "no checks may run without the spec");
        assert_eq!(findings[0].rule_id, "canon.setup");
    }

    #[test]
    fn ci_and_done_when_modes_drive_severity() {
        // ci_check_mode FAIL promotes the non-conventional title to FAIL;
        // done_when_check_mode WARN demotes the thin Checklist to WARN.
        let cfg = cfg(r#"
ci_check_mode: "FAIL"
done_when_check_mode: "WARN"
done_when_min_checkboxes: 2
done_when_headings: ["Construction plan", "Checklist"]
required_body_headings: ["Issue"]
conventional_commit_types: ["feat", "fix"]
title_must_be_chinese: true
"#);
        let findings = check_content(
            "not a conventional title",
            "## Issue\n\n## Checklist\n\n- [ ] one\n",
            &["bug"],
            "feat/a",
            "open",
            false,
            Some(&cfg),
        );
        let ci = findings
            .iter()
            .find(|f| f.rule_id == "PR-02" && f.severity != Severity::Info)
            .expect("PR-02 finding");
        assert_eq!(ci.severity, Severity::Fail);
        let dw = findings
            .iter()
            .find(|f| f.rule_id == "PR-07")
            .expect("PR-07 finding");
        assert_eq!(dw.severity, Severity::Warn);
    }

    #[test]
    fn garbled_mode_keeps_the_original_severity() {
        // An unparseable mode must not turn a check off or silently pass it.
        let cfg = cfg("ci_check_mode: \"please\"\ndone_when_check_mode: \"\"\n");
        let findings = check_content(
            "not a conventional title",
            "## Checklist\n\n- [ ] one\n",
            &["bug"],
            "feat/a",
            "open",
            false,
            Some(&cfg),
        );
        let ci = findings
            .iter()
            .find(|f| f.rule_id == "PR-02" && f.severity != Severity::Info)
            .expect("PR-02 finding");
        assert_eq!(ci.severity, Severity::Warn);
    }

    #[test]
    fn forbidden_brackets_are_caught_independent_of_cjk() {
        // （ is U+FF08: outside the CJK ideograph block, so the bracket rule
        // must fire on its own.
        let cfg = cfg("title_must_be_chinese: true\nforbidden_brackets_in_title: [\"（\"]\n");
        let findings = check_content(
            "feat: fix（thing",
            "## Issue\n",
            &["bug"],
            "feat/a",
            "open",
            false,
            Some(&cfg),
        );
        assert!(findings.iter().any(|f| {
            f.rule_id == "PR-01" && f.severity == Severity::Fail && f.msg.contains("fullwidth")
        }));
    }
}

#[cfg(test)]
mod spec_smoke {
    use super::*;

    fn load(name: &str) -> YamlValue {
        let p = crate::rules::repo_spec_path(name);
        serde_yaml::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap()
    }

    #[test]
    fn real_spec_clean_pr_is_not_fail() {
        let cfg = load("github_pull_requests.yaml");
        let body = "\
## Issue
Fixes #1
## What
做这件事的原因是 X
## Why
背景说明
## Construction plan
- [ ] a
- [ ] b
## Delivery record
已完成
## How to test
cargo test
## Checklist
- [ ] a
- [ ] b
";
        let f = check_content(
            "feat: add thing",
            body,
            &["feature"],
            "feat/x",
            "open",
            false,
            Some(&cfg),
        );
        let fails: Vec<_> = f.iter().filter(|x| x.severity == Severity::Fail).collect();
        assert!(
            fails.is_empty(),
            "clean PR must not FAIL against the real spec: {f:?}"
        );
    }
}
