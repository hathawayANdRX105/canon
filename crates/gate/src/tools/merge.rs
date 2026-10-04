//! merge hook — validate PR, reviews, cleanup before squash-merge.
//!
//! Port of `.githooks/hooks/merge`. Uses the already-ported Rust rules
//! (pull_requests, reviews) for GitHub validation, plus the Rust cleanup
//! module for branch cleanup.
//!
//! Usage: `canon merge <owner/repo> <pr_number> [--dry-run]`

use crate::engine;
use crate::shared::{
    Finding, Severity, apply_global_overrides, exit_code, gh_api, gh_api_paginate, load_yaml,
    print_findings,
};
use crate::tools::{cleanup, git, workspace};
/// `canon merge <owner/repo> <pr_number> [--dry-run]` — pre-merge validation.
pub fn run(args: &[String]) -> i32 {
    let mut positional = Vec::new();
    let mut dry_run = false;
    for arg in args {
        match arg.as_str() {
            "--dry-run" => dry_run = true,
            _ if !arg.starts_with("--") => positional.push(arg.clone()),
            _ => {}
        }
    }

    if positional.len() < 2 {
        eprintln!("Usage: gate merge <owner/repo> <pr_number> [--dry-run]");
        return 2;
    }

    let repo = &positional[0];
    let pr_num: u32 = match positional[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("invalid PR number: {}", positional[1]);
            return 2;
        }
    };

    let githooks =
        git::find_githooks_dir().unwrap_or_else(|| std::path::PathBuf::from(".githooks"));
    let spec_dir = githooks.join("spec");
    let dispatch_path = spec_dir.join("dispatch.yaml");
    let cfg = load_yaml(dispatch_path.to_str().unwrap_or("")).ok();

    let mut findings = Vec::new();

    println!("== Merge #{} ({}) ==", pr_num, repo);
    if dry_run {
        println!("[DRY-RUN]");
    }

    // Run topics from dispatch.yaml merge section — no silent defaults.
    let topics: Vec<String> = match &cfg {
        Some(c) => c
            .get("merge")
            .and_then(|v| v.as_sequence())
            .map(|seq| {
                seq.iter()
                    .filter_map(|t| t.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
        None => vec![],
    };
    if cfg.is_none() {
        findings.push(crate::shared::missing_cfg_finding("dispatch.yaml"));
    }

    for topic in &topics {
        println!("--- {} ---", topic);
        match topic.as_str() {
            "workspace" => findings.extend(workspace::run_workspace(".")),
            "github/pull_requests" => findings.extend(run_pr_rules(repo, pr_num)),
            "github/reviews" => findings.extend(run_review_rules(repo, pr_num, &spec_dir)),
            "cleanup" => {
                findings.extend(cleanup::run(dry_run));
                findings.extend(crate::tools::tests_check::run());
                findings.extend(crate::tools::docs_hygiene::run());
            }
            "checklist" => findings.extend(engine::run_all(engine::HookScope::Merge)),
            other => eprintln!("unknown merge topic: {}", other),
        }
    }

    // dispatch.yaml `severity_overrides:` first, then global severity_overrides.yaml.
    crate::shared::apply_severity_overrides(&mut findings, cfg.as_ref());
    apply_global_overrides(&mut findings);
    print_findings(&findings);
    let rc = exit_code(&findings);
    if rc == 0 && !dry_run {
        // Extract Fixes from PR body
        if let Ok(pr) = gh_api(&format!("repos/{}/pulls/{}", repo, pr_num), None) {
            let body = pr.get("body").and_then(|b| b.as_str()).unwrap_or("");
            let fixes = extract_fixes(body);
            if !fixes.is_empty() {
                print!("Fixes: #{}", fixes.join(", #"));
                println!();
            }
            if let Some(head) = pr
                .get("head")
                .and_then(|h| h.get("ref"))
                .and_then(|r| r.as_str())
            {
                println!("Branch: {}", head);
            }
        }
    }

    rc
}

fn run_pr_rules(repo: &str, pr_num: u32) -> Vec<Finding> {
    let cfg = crate::shared::load_spec_yaml("github_pull_requests.yaml");
    match gh_api(&format!("repos/{}/pulls/{}", repo, pr_num), None) {
        Ok(pr) => crate::rules::pull_requests::check_content(
            pr.get("title").and_then(|t| t.as_str()).unwrap_or(""),
            pr.get("body").and_then(|b| b.as_str()).unwrap_or(""),
            &labels_from(&pr),
            pr.get("head")
                .and_then(|h| h.get("ref"))
                .and_then(|r| r.as_str())
                .unwrap_or(""),
            cfg.as_ref(),
        ),
        Err(e) => vec![Finding::new(
            "PR",
            Severity::Fail,
            &format!("could not fetch PR #{}: {}", pr_num, e),
        )],
    }
}

fn run_review_rules(repo: &str, pr_num: u32, spec_dir: &std::path::Path) -> Vec<Finding> {
    let review_cfg = crate::shared::find_spec_file(&spec_dir, "github_reviews.yaml")
        .and_then(|path| load_yaml(path.to_str().unwrap_or("")).ok());

    // Collect all review + issue comments
    let mut bodies = Vec::new();

    if let Ok(comments) = gh_api_paginate(&format!("repos/{}/pulls/{}/comments", repo, pr_num), 100)
    {
        for c in &comments {
            if let Some(body) = c.get("body").and_then(|b| b.as_str()) {
                bodies.push(body.to_string());
            }
        }
    }
    if let Ok(comments) = gh_api(&format!("repos/{}/issues/{}/comments", repo, pr_num), None)
        && let Some(arr) = comments.as_array()
    {
        for c in arr {
            if let Some(body) = c.get("body").and_then(|b| b.as_str()) {
                bodies.push(body.to_string());
            }
        }
    }

    match &review_cfg {
        Some(cfg) => crate::rules::reviews::run(&bodies, cfg),
        None => vec![Finding::new(
            "RV",
            Severity::Warn,
            "github_reviews.yaml not found, review checks skipped",
        )],
    }
}

fn extract_fixes(body: &str) -> Vec<String> {
    let re = regex::Regex::new(r"(?:Fixes|Closes|Resolves)\s+#(\d+)").unwrap();
    re.captures_iter(body)
        .map(|c| c.get(1).unwrap().as_str().to_string())
        .collect()
}

fn labels_from(pr: &serde_json::Value) -> Vec<&str> {
    pr.get("labels")
        .and_then(|l| l.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|l| l.get("name").and_then(|n| n.as_str()))
                .collect()
        })
        .unwrap_or_default()
}

// ===========================================================================
// Tests
// ===========================================================================
