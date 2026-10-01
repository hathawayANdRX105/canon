//! pre-push hook — workspace + code checks.
//!
//! Port of `.githooks/hooks/pre-push`. Runs the topics listed in
//! `dispatch.yaml` under `pre-push`. Uses native Rust validators.

use crate::engine;
use crate::shared::{apply_global_overrides, exit_code, print_findings};
use crate::tools::{code, git, workspace};

/// `canon pre-push` — runs dispatched workspace + code topics.
pub fn run() -> i32 {
    let githooks_root =
        git::find_githooks_dir().unwrap_or_else(|| std::path::PathBuf::from(".githooks"));
    let spec_dir = githooks_root.join("spec");
    let dispatch_path = spec_dir.join("dispatch.yaml");
    let cfg = crate::shared::load_yaml(dispatch_path.to_str().unwrap_or("")).ok();

    let mut findings = Vec::new();
    // No silent defaults: a missing dispatch means the repo's hook setup is
    // incomplete → loud canon.setup finding (same as pre-commit / merge).
    let topics: Vec<String> = match &cfg {
        Some(c) => c
            .get("pre-push")
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
        let topic_findings = match topic.as_str() {
            "workspace" => workspace::run_workspace("."),
            "code" => code::run_code_all("."),
            "checklist" => engine::run_all(engine::HookScope::PrePush),
            other => {
                eprintln!("unknown pre-push topic: {}", other);
                vec![]
            }
        };
        findings.extend(topic_findings);
    }

    // dispatch.yaml `severity_overrides:` first, then the global
    // severity_overrides.yaml as the last word (parity with pre-commit).
    crate::shared::apply_severity_overrides(&mut findings, cfg.as_ref());
    apply_global_overrides(&mut findings);
    print_findings(&findings);
    exit_code(&findings)
}
