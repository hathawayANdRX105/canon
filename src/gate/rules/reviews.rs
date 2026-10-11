//! PR review comment validation.
//!
//! Implements `run`: validates PR review comments against
//! `.githooks/spec/github_reviews.yaml`. Every check parameter (formats,
//! word lists, thresholds, severities) is read from the spec — nothing is
//! baked into code, and a missing switch turns its check off rather than
//! inventing a default that could let a broken spec pass green.
//!
//! All RV-* rules: RV-01 (checkbox forbidden), RV-02 (allowed reply words),
//! RV-03 (reply detail), RV-04 (inline review format), RV-06 (inline findings
//! have reply). The CRG-comment rules (RV-05, the CRG half of RV-04) went away
//! with the `code-review-graph` and `open-code-review` binaries.

use regex::Regex;
use serde_yaml::Value as YamlValue;

use crate::gate::shared::{Finding, Severity};

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
fn reply_formats(cfg: &YamlValue) -> &YamlValue {
    cfg.get("reply_formats").unwrap_or(&YamlValue::Null)
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
    crate::gate::shared::apply_check_allowlist(&mut out, Some(cfg));
    out
}

fn run_impl(comment_bodies: &[String], cfg: &YamlValue) -> Vec<Finding> {
    // An empty (null) spec is just as missing as an absent one: never
    // silently green.
    if cfg.is_null() {
        return vec![crate::gate::shared::missing_cfg_finding(
            "github_reviews.yaml",
        )];
    }

    let mut findings = Vec::new();
    let bodies: Vec<&str> = comment_bodies.iter().map(|s| s.as_str()).collect();

    let comment = review_format(cfg, "review_comment");
    let inline = review_format(cfg, "inline_review");
    let replies_fmt = reply_formats(cfg);

    let reply_words = crate::gate::shared::cfg_str_list(Some(comment), "allowed_reply_words");
    let intent_required =
        crate::gate::shared::cfg_bool(Some(replies_fmt), "required_intent_word").unwrap_or(false);
    let colon_after =
        crate::gate::shared::cfg_bool(Some(replies_fmt), "colon_after").unwrap_or(false);
    // Accepts `5` or `"5"`; 0 ⇒ no detail floor (inert without a threshold).
    let min_detail = crate::gate::shared::cfg_str(Some(replies_fmt), "reply_min_detail")
        .and_then(|s| s.parse::<usize>().ok())
        .or_else(|| {
            crate::gate::shared::cfg_get(Some(replies_fmt), "reply_min_detail")
                .and_then(|v| v.as_u64())
                .map(|n| n as usize)
        })
        .unwrap_or(0);

    let inline_prefix = crate::gate::shared::cfg_str(Some(inline), "prefix").unwrap_or_default();
    let inline_levels = crate::gate::shared::cfg_str_list(Some(inline), "allowed_inline_levels");
    let inline_chinese =
        crate::gate::shared::cfg_bool(Some(inline), "content_must_be_chinese").unwrap_or(false);

    // ---- P-22 checkbox forbidden in reviews (RV-01) ----
    let checkbox_banned = crate::gate::shared::cfg_bool(Some(comment), "checkbox_forbidden")
        .unwrap_or(false)
        || crate::gate::shared::cfg_bool(Some(inline), "checkbox_forbidden").unwrap_or(false);
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

    // ---- P-35 inline review prefix format (RV-04) ----
    for body in &bodies {
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

    // Apply the spec's severity_overrides (e.g. RV-01 demoted to WARN).
    crate::gate::shared::apply_severity_overrides(&mut findings, Some(cfg));

    findings
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
