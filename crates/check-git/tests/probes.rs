//! D7 probe：git 历史信号三 kind（hotspot / coupling / defects）。
//! fixture 用真实 git 仓库（`git init` + 定向提交），git 不可用的环境显式跳过。

use std::path::{Path, PathBuf};
use std::process::Command;

use check_core::Report;
use check_core::config::ChecksConfig;
use check_core::context::Ctx;
use check_core::registry::{Registry, RunOptions};
use check_git::factories;

fn write(root: &Path, rel: &str, body: &str) -> PathBuf {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, body).unwrap();
    PathBuf::from(rel)
}

fn git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(root)
        .env("GIT_AUTHOR_NAME", "t")
        .env("GIT_AUTHOR_EMAIL", "t@t")
        .env("GIT_COMMITTER_NAME", "t")
        .env("GIT_COMMITTER_EMAIL", "t@t")
        .output()
        .expect("git");
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn commit_files(root: &Path, subject: &str, files: &[(&str, &str)]) {
    for (rel, body) in files {
        write(root, rel, body);
    }
    git(root, &["add", "-A"]);
    git(root, &["commit", "-qm", subject]);
}

fn run(root: &Path, toml_text: &str) -> Report {
    let config: ChecksConfig = toml::from_str(toml_text).expect("valid checks toml");
    let (registry, errors) = Registry::build(Vec::new(), &factories(), &config);
    assert!(errors.is_empty(), "{errors:?}");
    // 历史口径规则的 scope 过滤基于 tracked 集：把 fixture 文件当作跟踪文件传入
    let tracked: Vec<PathBuf> = ["Cargo.toml", "src/hot.rs", "src/cold.rs", "src/partner.rs"]
        .iter()
        .map(PathBuf::from)
        .collect();
    let ctx = Ctx::new(root, tracked, None);
    registry.run(
        &ctx,
        &RunOptions {
            classes: None,
            semantic_endpoint: None,
            overrides: None,
        },
    )
}

/// fixture：hot.rs 被改 6 次（含 3 次 fix），cold.rs 改 1 次；
/// hot.rs 与 partner.rs 有 4 次同提交共现。
fn seed_history(root: &Path) {
    write(
        root,
        "Cargo.toml",
        "[workspace]\nresolver = \"2\"\nmembers = []\n",
    );
    git(root, &["init", "-q", "-b", "main"]);
    commit_files(
        root,
        "init",
        &[
            ("src/hot.rs", "fn a() {}\n"),
            ("src/cold.rs", "fn c() {}\n"),
            ("src/partner.rs", "fn p() {}\n"),
        ],
    );
    for i in 0..3 {
        commit_files(
            root,
            &format!("fix: bug {i} in hot with partner"),
            &[
                ("src/hot.rs", &format!("fn a() {{ // v{i}\n}}\n")),
                ("src/partner.rs", &format!("fn p() {{ // v{i}\n}}\n")),
            ],
        );
    }
    for i in 0..3 {
        commit_files(
            root,
            &format!("refactor: hot tweak {i}"),
            &[("src/hot.rs", &format!("fn a() {{ // t{i}\n}}\n"))],
        );
    }
    commit_files(
        root,
        "chore: cold once",
        &[("src/cold.rs", "fn c() { /* once */ }\n")],
    );
}

#[test]
fn hotspot_coupling_defects_fire_on_seeded_history() {
    if !git_available() {
        eprintln!("skip: git unavailable");
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    seed_history(dir.path());

    // hotspot：hot.rs churn=6 ≥ min 3 → 报；cold.rs churn=1 < 3 → 不报
    let report = run(dir.path(), "[kinds.git-hotspot.main]\nmin_churn = 3\n");
    let paths: Vec<_> = report.findings.iter().map(|f| f.path.as_str()).collect();
    assert!(paths.contains(&"src/hot.rs"), "{report:?}");
    assert!(!paths.contains(&"src/cold.rs"), "{report:?}");

    // coupling：hot+partner 同提交 4 次（init 未计——init 只提交无 partner？有：3 fix）→ 共现 3 次 ≥ 3
    let report = run(dir.path(), "[kinds.git-coupling.main]\nmin_cochanges = 3\n");
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "GIT.coupling.main");
    assert!(
        report.findings[0].message.contains("partner"),
        "{:?}",
        report.findings[0].message
    );

    // defects：hot.rs 3 次 fix 提交 → ≥ min 3 报；cold.rs 0 次 fix → 不报
    let report = run(dir.path(), "[kinds.git-defects.main]\nmin_fixes = 3\n");
    assert!(
        report.findings.iter().any(|f| f.path == "src/hot.rs"),
        "{report:?}"
    );
    assert!(
        !report.findings.iter().any(|f| f.path == "src/cold.rs"),
        "{report:?}"
    );
}

#[test]
fn non_git_dir_is_silently_skipped() {
    // 非 git 仓 → 情报缺失静默（advisory 不产伪 finding），而非报错
    let dir = tempfile::tempdir().unwrap();
    write(dir.path(), "src/hot.rs", "fn a() {}\n");
    let report = run(dir.path(), "[kinds.git-hotspot.main]\n");
    assert!(report.findings.is_empty(), "{report:?}");
    assert!(report.errors.is_empty(), "{report:?}");
}

#[test]
fn scope_exclude_filters_hotspot_and_coupling_respects_scope_side() {
    if !git_available() {
        eprintln!("skip: git unavailable");
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    seed_history(dir.path());

    // scope_exclude 收口生成物/manifest 类噪声：hot.rs 被排除后热点不再报
    let toml = "[kinds.git-hotspot.main]\nmin_churn = 3\n";
    let config: ChecksConfig = toml::from_str(toml).unwrap();
    let (registry, errors) = Registry::build(Vec::new(), &factories(), &config);
    assert!(errors.is_empty(), "{errors:?}");
    let mut overrides = std::collections::BTreeMap::new();
    overrides.insert(
        "GIT.hotspot.main".to_string(),
        check_core::config::RuleOverride {
            severity: None,
            enabled: true,
            scope_include: Vec::new(),
            scope_exclude: vec!["src/hot.rs".to_string()],
        },
    );
    let tracked: Vec<PathBuf> = ["src/hot.rs", "src/cold.rs"]
        .iter()
        .map(PathBuf::from)
        .collect();
    let ctx = Ctx::new(dir.path(), tracked, None);
    let report = registry.run(
        &ctx,
        &RunOptions {
            classes: None,
            semantic_endpoint: None,
            overrides: Some(&overrides),
        },
    );
    assert!(
        report.findings.is_empty(),
        "scope_exclude 后不得再报：{report:?}"
    );
}
