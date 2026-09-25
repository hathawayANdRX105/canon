//! PR review comment validation.
//!
//! Implements `run`: validates PR review comments against
//! `.githooks/spec/github_reviews.yaml`. Every check parameter (formats,
//! word lists, thresholds, severities) is read from the spec — nothing is
//! baked into code, and a missing switch turns its check off rather than
//! inventing a default that could let a broken spec pass green.
//!
//! All RV-* rules: RV-01 (checkbox forbidden), RV-02 (allowed reply words),
//! RV-03 (reply detail), RV-04 (CRG/inline review format), RV-05 (CRG Review
//! exists), RV-06 (inline findings have reply).

use regex::Regex;
use serde_yaml::Value as YamlValue;

use crate::shared::{Finding, Severity};

// ---------------------------------------------------------------------------
// Spec access
// ---------------------------------------------------------------------------

/// `review_formats.<name>` section; Null when the spec omits it.
fn review_format<'a>(cfg: &'a YamlValue, name: &str) -> &'a YamlValue {
    cfg.get("review_formats")
        .and_then(|rf| rf.get(name))
        .unwrap_or(&YamlValue::Null)
}

/// `reply_formats` section; Null when the spec omits it.
fn reply_formats<'a>(cfg: &'a YamlValue) -> &'a YamlValue {
    cfg.get("reply_formats").unwrap_or(&YamlValue::Null)
}

/// `H2` → `##`. Garbled or absent level keeps `H2` (the format's base line).
fn heading_marker(level: Option<&str>) -> String {
    let n = level
        .and_then(|l| l.trim_start_matches('H').parse::<usize>().ok())
        .unwrap_or(2);
    "#".repeat(n.max(1))
}

fn has_cjk(s: &str) -> bool {
    s.chars().any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c))
}

/// Extract `Agent 🤖 - Fix: <reason>` / `Block:` / ... reply entries.
/// Any English intent word is extracted; RV-02 decides whether the word is
/// allowed, so that check can actually fire (an extractor built from the
/// whitelist it validates against would make RV-02 dead code).
/// `colon_after` requires whitespace after the colon.
fn extract_replies(body: &str, colon_after: bool) -> Vec<(&str, String)> {
    let sep = if colon_after { "\\s+" } else { "\\s*" };
    let re =
        Regex::new(&format!("Agent 🤖 - ([A-Za-z]+):{sep}(.+)")).expect("reply pattern is static");
    re.captures_iter(body)
        .map(|m| {
            let typ = m.get(1).unwrap().as_str();
            let reason = m.get(2).unwrap().as_str().trim().to_string();
            (typ, reason)
        })
        .collect()
}

/// `Agent 🤖 - Inline Review P2: <content>` entries tied to path+line.
/// `level` is `"unspecified"` when the P-level is missing.
fn extract_inline_reviews<'a>(body: &'a str, prefix: &'a str) -> Vec<(&'a str, String)> {
    if prefix.is_empty() {
        return Vec::new();
    }
    // The level is captured loosely: an optional non-colon token before the
    // `:`. Alternating the *allowed* list here would make the disallowed-level
    // branch of RV-04 unreachable (a P9 would simply not match).
    let re = Regex::new(&format!(
        r"(?m){}(?:\s+([^:\n]+))?:\s*(.+)",
        regex::escape(prefix)
    ))
    .expect("inline prefix must form a valid regex");
    re.captures_iter(body)
        .map(|m| {
            let level = m.get(1).map(|g| g.as_str().trim()).unwrap_or("unspecified");
            let content = m.get(2).unwrap().as_str().trim().to_string();
            (level, content)
        })
        .collect()
}

/// Extract `<marker> Agent 🤖 - CRG Review: <title>` entries (case-insensitive).
fn extract_crg_reviews(body: &str, marker: &str, text: &str) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }
    let re = Regex::new(&format!(
        r"(?mi)^{} {}:?\s*(.+)",
        marker,
        regex::escape(text)
    ))
    .expect("CRG heading spec must form a valid regex");
    re.captures_iter(body)
        .map(|m| m.get(1).unwrap().as_str().trim().to_string())
        .collect()
}

/// Headings at the CRG level that are not CRG Review headings — i.e.
/// sub-sections sitting at the wrong level.
fn stray_subheadings(body: &str, marker: &str, crg_text: &str) -> usize {
    if crg_text.is_empty() {
        return 0;
    }
    let lvl = Regex::new(&format!(r"(?m)^{} .*", marker)).expect("valid heading level");
    let crg = Regex::new(&format!(r"(?mi)^{} {}: ", marker, regex::escape(crg_text)))
        .expect("valid CRG heading");
    lvl.find_iter(body)
        .filter(|m| !crg.is_match(m.as_str()))
        .count()
}

// ---------------------------------------------------------------------------
// rule function
// ---------------------------------------------------------------------------

/// Validate review comment bodies against `github_reviews.yaml`.
///
/// Port of Python `reviews.run(comments, cfg)`. `comment_bodies` is the list
/// of non-empty comment body strings (Python normalizes `comments` dicts to
/// `bodies` first — callers pass the pre-extracted bodies).
pub fn run(comment_bodies: &[String], cfg: &YamlValue) -> Vec<Finding> {
    let mut out = run_impl(comment_bodies, cfg);
    crate::shared::apply_check_allowlist(&mut out, Some(cfg));
    out
}

fn run_impl(comment_bodies: &[String], cfg: &YamlValue) -> Vec<Finding> {
    // An empty (null) spec is just as missing as an absent one: never
    // silently green.
    if cfg.is_null() {
        return vec![crate::shared::missing_cfg_finding("github_reviews.yaml")];
    }

    let mut findings = Vec::new();
    let bodies: Vec<&str> = comment_bodies.iter().map(|s| s.as_str()).collect();

    let crg = review_format(cfg, "crg_review");
    let inline = review_format(cfg, "inline_review");
    let replies_fmt = reply_formats(cfg);

    let reply_words = crate::shared::cfg_str_list(Some(crg), "allowed_reply_words");
    let intent_required =
        crate::shared::cfg_bool(Some(replies_fmt), "required_intent_word").unwrap_or(false);
    let colon_after = crate::shared::cfg_bool(Some(replies_fmt), "colon_after").unwrap_or(false);
    // Accepts `5` or `"5"`; 0 ⇒ no detail floor (inert without a threshold).
    let min_detail = crate::shared::cfg_str(Some(replies_fmt), "reply_min_detail")
        .and_then(|s| s.parse::<usize>().ok())
        .or_else(|| {
            crate::shared::cfg_get(Some(replies_fmt), "reply_min_detail")
                .and_then(|v| v.as_u64())
                .map(|n| n as usize)
        })
        .unwrap_or(0);

    let crg_marker =
        heading_marker(crate::shared::cfg_str(Some(crg), "heading_level_crg").as_deref());
    let sub_marker =
        heading_marker(crate::shared::cfg_str(Some(crg), "heading_level_crg_sub").as_deref());
    let crg_text = crate::shared::cfg_str(Some(crg), "heading_text_crg").unwrap_or_default();
    let crg_chinese =
        crate::shared::cfg_bool(Some(crg), "content_must_be_chinese").unwrap_or(false);

    let inline_prefix = crate::shared::cfg_str(Some(inline), "prefix").unwrap_or_default();
    let inline_levels = crate::shared::cfg_str_list(Some(inline), "allowed_inline_levels");
    let inline_chinese =
        crate::shared::cfg_bool(Some(inline), "content_must_be_chinese").unwrap_or(false);

    // ---- P-22 checkbox forbidden in reviews (RV-01) ----
    let checkbox_banned = crate::shared::cfg_bool(Some(crg), "checkbox_forbidden").unwrap_or(false)
        || crate::shared::cfg_bool(Some(inline), "checkbox_forbidden").unwrap_or(false);
    if checkbox_banned {
        let checkbox_re = Regex::new(r"-\s*\[[ xX]\]").unwrap();
        let found = bodies.iter().any(|b| checkbox_re.is_match(b));
        findings.push(Finding::new(
            "RV-01",
            if found {
                Severity::Fail
            } else {
                Severity::Info
            },
            if found {
                "review comment contains checkbox (- [x] / - [ ])"
            } else {
                "no checkboxes in review comments"
            },
        ));
    }

    // ---- P-35 review prefix format (RV-04) ----
    for body in &bodies {
        let crgs = extract_crg_reviews(body, &crg_marker, &crg_text);
        for title in &crgs {
            if has_cjk(title) {
                findings.push(Finding::new(
                    "RV-04",
                    Severity::Fail,
                    &format!("CRG Review title contains CJK: {title}"),
                ));
            } else {
                findings.push(Finding::new(
                    "RV-04",
                    Severity::Info,
                    &format!("CRG Review title is English: {title}"),
                ));
            }
        }
        // The title stays English; the prose underneath must be Chinese.
        if crg_chinese && !crgs.is_empty() && !has_cjk(body) {
            findings.push(Finding::new(
                "RV-04",
                Severity::Fail,
                "CRG Review content must be Chinese (the title stays English)",
            ));
        }
        if !crgs.is_empty() {
            let stray = stray_subheadings(body, &crg_marker, &crg_text);
            if stray > 0 {
                findings.push(Finding::new(
                    "RV-04",
                    Severity::Fail,
                    &format!(
                        "CRG Review sub-sections must be at {sub_marker}, found {stray} at {crg_marker}"
                    ),
                ));
            }
        }

        for ir in extract_inline_reviews(body, &inline_prefix) {
            let level = ir.0;
            if level != "unspecified" && !inline_levels.iter().any(|l| l == level) {
                findings.push(Finding::new(
                    "RV-04",
                    Severity::Fail,
                    &format!("Inline Review level '{level}' not in allowed {inline_levels:?}"),
                ));
            } else {
                findings.push(Finding::new(
                    "RV-04",
                    Severity::Info,
                    &format!("Inline Review prefix OK: level={level}"),
                ));
            }
            if inline_chinese && !has_cjk(&ir.1) {
                findings.push(Finding::new(
                    "RV-04",
                    Severity::Fail,
                    &format!("Inline Review content must be Chinese: {}", ir.1),
                ));
            }
        }
    }

    // ---- P-24 / P-25 reply threads (RV-02, RV-03) ----
    let mut all_replies: Vec<(&str, String)> = Vec::new();
    for body in &bodies {
        all_replies.extend(extract_replies(body, colon_after));
    }

    if all_replies.is_empty() {
        findings.push(Finding::new("RV-02", Severity::Info, "no replies to check"));
        findings.push(Finding::new("RV-03", Severity::Info, "no replies to check"));
    } else {
        if intent_required {
            let bad: Vec<&str> = all_replies
                .iter()
                .filter(|r| !reply_words.iter().any(|w| w == r.0))
                .map(|r| r.0)
                .collect();
            if !bad.is_empty() {
                findings.push(Finding::new(
                    "RV-02",
                    Severity::Warn,
                    &format!("some replies use disallowed words: {bad:?}"),
                ));
            } else {
                findings.push(Finding::new(
                    "RV-02",
                    Severity::Info,
                    &format!("all {} reply(ies) use allowed words", all_replies.len()),
                ));
            }
        }

        let short: usize = all_replies
            .iter()
            .filter(|r| r.1.chars().count() < min_detail)
            .count();
        if short > 0 {
            findings.push(Finding::new(
                "RV-03",
                Severity::Warn,
                &format!(
                    "{short}/{} replies lack sufficient detail",
                    all_replies.len()
                ),
            ));
        } else {
            findings.push(Finding::new(
                "RV-03",
                Severity::Info,
                &format!("all {} replies have sufficient detail", all_replies.len()),
            ));
        }
    }

    // ---- P-36 CRG Review exists (RV-05) ----
    // Only required when the spec defines what a CRG review looks like;
    // without a heading format the presence requirement would fail every
    // PR instead of being switched off.
    if !crg_text.is_empty() {
        let has_crg = bodies
            .iter()
            .any(|b| !extract_crg_reviews(b, &crg_marker, &crg_text).is_empty());
        if has_crg {
            findings.push(Finding::new(
                "RV-05",
                Severity::Info,
                "CRG Review present in PR conversation",
            ));
        } else {
            findings.push(Finding::new(
                "RV-05",
                Severity::Fail,
                "no CRG Review comment in PR conversation",
            ));
        }
    }

    // ---- P-37 inline findings have reply (RV-06) ----
    let inline_count: usize = bodies
        .iter()
        .map(|b| extract_inline_reviews(b, &inline_prefix).len())
        .sum();
    if inline_count == 0 {
        findings.push(Finding::new(
            "RV-06",
            Severity::Info,
            "no inline findings to resolve",
        ));
    } else if all_replies.len() >= inline_count {
        findings.push(Finding::new(
            "RV-06",
            Severity::Info,
            &format!(
                "all {} inline finding(s) have reply ({} replies)",
                inline_count,
                all_replies.len()
            ),
        ));
    } else {
        findings.push(Finding::new(
            "RV-06",
            Severity::Fail,
            &format!(
                "{}/{} inline findings have reply — every inline finding MUST have an Agent 🤖 - {} reply",
                all_replies.len(),
                inline_count,
                reply_words.join("/")
            ),
        ));
    }

    // Apply the spec's severity_overrides (e.g. RV-05 demoted to WARN).
    crate::shared::apply_severity_overrides(&mut findings, Some(cfg));

    findings
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(yaml: &str) -> YamlValue {
        serde_yaml::from_str(yaml).expect("valid inline spec")
    }

    fn full_spec() -> YamlValue {
        cfg(r#"
review_formats:
  crg_review:
    heading_level_crg: "H2"
    heading_level_crg_sub: "H3"
    heading_text_crg: "Agent 🤖 - CRG Review"
    content_must_be_chinese: true
    checkbox_forbidden: true
    allowed_reply_words: ["Fix", "Block", "Resolve", "Note", "Withdraw", "Supersede"]
  inline_review:
    prefix: "Agent 🤖 - Inline Review"
    allowed_inline_levels: ["P0", "P1", "P2", "P3"]
    content_must_be_chinese: true
    checkbox_forbidden: true
reply_formats:
  required_intent_word: true
  colon_after: true
  reply_min_detail: 5
"#)
    }

    #[test]
    fn missing_spec_is_a_loud_gate_setup_fail() {
        // A null (empty) spec is just as missing as an absent one.
        let findings = run(&[], &YamlValue::Null);
        assert_eq!(findings.len(), 1, "no checks may run without the spec");
        assert_eq!(findings[0].rule_id, "gate.setup");
        assert_eq!(findings[0].severity, Severity::Fail);
    }

    #[test]
    fn switches_off_skip_their_checks() {
        // No format sections at all: RV-01 is off, and the CRG/inline
        // extractors find nothing, so no RV-04 may fire either.
        let findings = run(&["- [ ] leftover checkbox".to_string()], &cfg("{}"));
        assert!(findings.iter().all(|f| f.rule_id != "RV-01"));
        assert!(findings.iter().all(|f| f.severity != Severity::Fail));
    }

    #[test]
    fn crg_english_title_plus_chinese_prose_is_clean() {
        let body =
            "## Agent 🤖 - CRG Review: fix the bug\n\n这是中文正文。\n\n### 细节\n\n更多中文。\n"
                .to_string();
        let findings = run(&[body], &full_spec());
        assert!(
            findings.iter().all(|f| f.severity != Severity::Fail),
            "expected a clean review, got {findings:?}"
        );
    }

    #[test]
    fn crg_title_cjk_or_english_prose_fails() {
        let cjk_title = "## Agent 🤖 - CRG Review: 修复问题\n\n中文正文。\n".to_string();
        assert!(
            run(&[cjk_title], &full_spec())
                .iter()
                .any(|f| f.rule_id == "RV-04" && f.severity == Severity::Fail)
        );

        let english_prose =
            "## Agent 🤖 - CRG Review: fix the bug\n\nEnglish prose only.\n".to_string();
        assert!(
            run(&[english_prose], &full_spec())
                .iter()
                .any(|f| f.rule_id == "RV-04" && f.severity == Severity::Fail)
        );
    }

    #[test]
    fn crg_subsection_at_wrong_level_fails() {
        // A `##` sub-section instead of the required `###`.
        let body = "## Agent 🤖 - CRG Review: fix the bug\n\n## 细节\n\n中文。\n".to_string();
        assert!(
            run(&[body], &full_spec())
                .iter()
                .any(|f| f.rule_id == "RV-04" && f.severity == Severity::Fail)
        );
    }

    #[test]
    fn inline_level_and_chinese_are_gated_by_spec() {
        let body = "Agent 🤖 - Inline Review P9: english only content".to_string();
        let findings = run(&[body], &full_spec());
        // P9 is not in allowed_inline_levels, and the content has no CJK.
        assert!(
            findings.iter().any(|f| f.rule_id == "RV-04"
                && f.severity == Severity::Fail
                && f.msg.contains("P9"))
        );
        assert!(findings.iter().any(|f| f.rule_id == "RV-04"
            && f.severity == Severity::Fail
            && f.msg.contains("Chinese")));
    }

    #[test]
    fn reply_word_and_detail_come_from_spec() {
        let body = "Agent 🤖 - Nope: 太短".to_string();
        let findings = run(&[body], &full_spec());
        // "Nope" is not an allowed intent word; and the reason is shorter
        // than reply_min_detail.
        assert!(
            findings
                .iter()
                .any(|f| f.rule_id == "RV-02" && f.severity == Severity::Warn)
        );
        assert!(
            findings
                .iter()
                .any(|f| f.rule_id == "RV-03" && f.severity == Severity::Warn)
        );
    }
}

#[cfg(test)]
mod spec_smoke {
    use super::*;
    use std::path::Path;

    fn load(name: &str) -> YamlValue {
        let p = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("specs")
            .join("github")
            .join(name);
        serde_yaml::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
    }

    #[test]
    fn real_spec_clean_review_is_not_fail() {
        let cfg = load("github_reviews.yaml");
        let crg =
            "## Agent 🤖 - CRG Review: fix the thing\n\n这是中文正文。\n\n### 细节\n\n更多中文。\n"
                .to_string();
        let reply = "Agent 🤖 - Fix: 已按建议修复，补充了测试".to_string();
        let f = run(&[crg, reply], &cfg);
        let fails: Vec<_> = f.iter().filter(|x| x.severity == Severity::Fail).collect();
        assert!(
            fails.is_empty(),
            "clean review must not FAIL against the real spec: {f:?}"
        );
    }

    #[test]
    fn real_spec_bad_review_fails() {
        let cfg = load("github_reviews.yaml");
        let bad = "## Agent 🤖 - CRG Review: 修复问题\n\n英文 only.\n\n## stray\n".to_string();
        let f = run(&[bad], &cfg);
        assert!(
            f.iter()
                .any(|x| x.rule_id == "RV-04" && x.severity == Severity::Fail),
            "{f:?}"
        );
    }
}
