//! Code lint dispatcher: CD-01..CD-06.
//!
//! Languages are discovered from `code_*.yaml` in `.githooks/spec/` — drop a
//! yaml, get a language, zero code. Each language only runs when a **changed**
//! file (staged ∪ committed-vs-HEAD) matches its `paths_include`; no changes
//! in that language ⇒ the linter never spawns.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_yaml::Value as YamlValue;

use crate::shared::{Finding, Severity, load_yaml, run_external};
use crate::tools::git;

fn repo_root() -> PathBuf {
    git::git_root().unwrap_or_else(|| {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .canonicalize()
            .unwrap_or_else(|_| PathBuf::from("."))
    })
}

/// Union of staged and committed-vs-HEAD changed files — covers both the
/// pre-commit and pre-push scopes in one list.
fn changed_files() -> Vec<String> {
    let mut out = Vec::new();
    for args in [
        ["diff", "--cached", "--name-only", "--no-color"],
        ["diff", "HEAD", "--name-only", "--no-color"],
    ] {
        if let Ok(outp) = Command::new("git").args(args).output()
            && outp.status.success()
        {
            out.extend(
                String::from_utf8_lossy(&outp.stdout)
                    .lines()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty()),
            );
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Languages available in this repo: file stem of every `code_*.yaml` under
/// `.githooks/spec/` (recursive). No yaml ⇒ no check (nothing is hardcoded).
fn available_langs(root: &Path) -> Vec<String> {
    let spec_dir = git::find_githooks_dir()
        .unwrap_or_else(|| root.join(".githooks"))
        .join("spec");
    let mut langs = collect_yaml_stems(&spec_dir, "code_");
    langs.sort();
    langs
}

/// Walk `spec_dir` recursively collecting `<prefix>*.yaml` file stems.
fn collect_yaml_stems(spec_dir: &Path, prefix: &str) -> Vec<String> {
    fn walk(dir: &Path, prefix: &str, out: &mut Vec<String>, depth: u8) {
        if depth > 3 {
            return;
        }
        for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, prefix, out, depth + 1);
                continue;
            }
            let name = path.file_name().map(|n| n.to_string_lossy().to_string());
            if let Some(n) = name
                && n.starts_with(prefix)
                && n.ends_with(".yaml")
            {
                out.push(
                    n.trim_start_matches(prefix)
                        .trim_end_matches(".yaml")
                        .to_string(),
                );
            }
        }
    }
    let mut out = Vec::new();
    walk(spec_dir, prefix, &mut out, 0);
    out
}

fn strings(cfg: &YamlValue, key: &str) -> Vec<String> {
    cfg.get(key)
        .and_then(|v| v.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn matches_include(rel: &str, include: &str) -> bool {
    let pat = include.strip_prefix("**/").unwrap_or(include);
    if let Some(prefix) = pat.strip_suffix("/*") {
        return rel
            .strip_prefix(&format!("{prefix}/"))
            .is_some_and(|rest| !rest.contains('/'))
            || rel.contains(&format!("/{prefix}/"));
    }
    if let Some(ext) = pat.strip_prefix("*.") {
        return rel.ends_with(&format!(".{ext}"));
    }
    if let Some(suffix) = pat.strip_prefix('*') {
        return rel.ends_with(suffix);
    }
    rel.ends_with(pat) || rel.contains(&format!("/{pat}"))
}

pub fn run_lang(lang: &str, target: &str, changed: &[String]) -> Vec<Finding> {
    let root = repo_root();
    let cfg_path = crate::shared::find_spec_file(
        &git::find_githooks_dir()
            .unwrap_or_else(|| root.join(".githooks"))
            .join("spec"),
        &format!("code_{lang}.yaml"),
    )
    .unwrap_or_else(|| {
        root.join(".githooks/spec")
            .join(format!("code_{lang}.yaml"))
    });
    if !cfg_path.exists() {
        return vec![Finding::new(
            &format!("code-{lang}"),
            Severity::Warn,
            &format!("config not found: code_{lang}.yaml"),
        )];
    }
    let cfg = load_yaml(cfg_path.to_str().unwrap_or("")).unwrap_or(YamlValue::Null);
    if !cfg.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true) {
        return vec![Finding::new(
            &format!("code-{lang}"),
            Severity::Info,
            &format!("{lang}: disabled in config"),
        )];
    }

    let command = cfg.get("command").and_then(|v| v.as_str()).unwrap_or("");
    if command.is_empty() {
        return vec![Finding::new(
            &format!("code-{lang}"),
            Severity::Warn,
            &format!("{lang}: no command configured"),
        )];
    }
    let args = strings(&cfg, "args");
    let includes = strings(&cfg, "paths_include");
    let excludes = strings(&cfg, "paths_exclude");
    let fail_severity = if cfg.get("fail_severity").and_then(|v| v.as_str()) == Some("FAIL") {
        Severity::Fail
    } else {
        Severity::Warn
    };

    // Dynamic on-demand gate: no changed file matches this language's
    // include patterns ⇒ the linter never spawns (silent skip, zero cost).
    if !includes.is_empty() {
        let relevant = changed.iter().any(|rel| {
            !excludes.iter().any(|x| rel.contains(x))
                && includes.iter().any(|pat| matches_include(rel, pat))
        });
        if !relevant {
            return vec![];
        }
    }

    let mut cmd: Vec<&str> = Vec::with_capacity(args.len() + 2);
    cmd.push(command);
    cmd.extend(args.iter().map(|s| s.as_str()));
    if command != "cargo" {
        cmd.push(target);
    }

    let (mut rc, mut output) = match run_external(&cmd, Some(root.to_string_lossy().as_ref())) {
        Ok(result) => result,
        Err(_) => {
            return vec![Finding::new(
                &format!("code-{lang}"),
                Severity::Warn,
                &format!("{lang}: {command} not installed, skipped"),
            )];
        }
    };

    if rc != 0 && !excludes.is_empty() && !output.is_empty() {
        let kept: Vec<&str> = output
            .lines()
            .filter(|line| !excludes.iter().any(|x| line.contains(x)))
            .collect();
        if kept.is_empty() {
            rc = 0;
            output.clear();
        } else {
            output = kept.join("\n");
        }
    }

    if rc == 0 {
        return vec![Finding::new(
            &format!("code-{lang}"),
            Severity::Info,
            &format!("{lang}: {command} passed"),
        )];
    }
    if rc == 127
        || output.to_lowercase().starts_with("command not found")
        || output.to_lowercase().contains("no such file or directory")
    {
        return vec![Finding::new(
            &format!("code-{lang}"),
            Severity::Warn,
            &format!("{lang}: {command} not installed, skipped"),
        )];
    }
    let msg = if output.is_empty() {
        format!("{command} exited {rc}")
    } else if output.len() > 500 {
        output.chars().take(500).collect()
    } else {
        output
    };
    vec![Finding::new(
        &format!("code-{lang}"),
        fail_severity,
        &format!("{lang}: {command} reported issues:\n{msg}"),
    )]
}

pub fn run_code(langs: Option<&[String]>, target: &str) -> Vec<Finding> {
    let root = repo_root();
    let changed = changed_files();
    let languages: Vec<String> = match langs {
        Some(langs) => langs.to_vec(),
        None => available_langs(&root),
    };
    let mut findings = Vec::new();
    for lang in languages {
        findings.extend(run_lang(&lang, target, &changed));
    }
    findings
}

pub fn run_code_all(target: &str) -> Vec<Finding> {
    run_code(None, target)
}
