//! D7 probe：dep-edge 与 machete 各配 intact / broken 双例；
//! dep-tag（git 依赖 branch 钉版 → WARN）用真实 git 仓库做依赖源。

use check_core::Report;
use check_core::config::ChecksConfig;
use check_core::context::Ctx;
use check_core::registry::{Registry, RunOptions};
use std::path::Path;

fn write(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, body).unwrap();
}

/// 临时 cargo workspace：crates/baker（用 crates/oven 的 path 依赖）。
fn workspace_with_baker_depends_on_oven(root: &Path) {
    write(
        root,
        "Cargo.toml",
        "[workspace]\nresolver = \"2\"\nmembers = [\"crates/baker\", \"crates/oven\"]\n",
    );
    write(
        root,
        "crates/oven/Cargo.toml",
        "[package]\nname = \"oven\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    write(root, "crates/oven/src/lib.rs", "pub fn bake() {}\n");
    write(
        root,
        "crates/baker/Cargo.toml",
        "[package]\nname = \"baker\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n\
         [dependencies]\noven = { path = \"../oven\" }\n",
    );
    write(root, "crates/baker/src/lib.rs", "pub use oven::bake;\n");
}

fn run(toml_text: &str, root: &Path) -> Report {
    let config: ChecksConfig = toml::from_str(toml_text).expect("valid checks toml");
    let (registry, errors) =
        Registry::build(Vec::new(), &[Box::new(check_deps::DepEdgeFactory)], &config);
    assert!(errors.is_empty(), "{errors:?}");
    let ctx = Ctx::new(root, Vec::new(), None);
    registry.run(
        &ctx,
        &RunOptions {
            classes: None,
            semantic_endpoint: None,
            overrides: None,
        },
    )
}

#[test]
fn dep_edge_deny_fires_only_on_matching_direction() {
    let dir = tempfile::tempdir().unwrap();
    workspace_with_baker_depends_on_oven(dir.path());

    // deny 反方向（oven -> baker）：今天不存在该依赖边 → intact
    let report = run(
        r#"
[kinds.dep-edge.reverse]
edges_deny = [{ from = "crates/oven/**", to = "crates/baker/**" }]
"#,
        dir.path(),
    );
    assert!(report.findings.is_empty(), "{report:?}");

    // deny 恰好方向：baker -> oven → broken
    let report = run(
        r#"
[kinds.dep-edge.forward]
edges_deny = [{ from = "crates/baker/**", to = "crates/oven/**" }]
"#,
        dir.path(),
    );
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "DEP-EDGE.forward");
    assert_eq!(report.findings[0].path, "crates/baker/Cargo.toml");
    assert_eq!(report.findings[0].severity, check_core::Severity::Fail);
}

#[test]
fn dep_edge_unknown_kind_config_is_setup_error() {
    let dir = tempfile::tempdir().unwrap();
    // 未知 kind 的容忍在 check-core（Registry::build 返回 errors）——这里验证语义
    let config: ChecksConfig = toml::from_str("[kinds.no-such-kind.x]\nfoo = 1\n").unwrap();
    let (_, errors) = Registry::build(Vec::new(), &[Box::new(check_deps::DepEdgeFactory)], &config);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("no-such-kind"), "{errors:?}");
    assert!(dir.path().exists());
}

#[test]
fn machete_translates_unused_dependency() {
    // 依赖跳过：cargo-machete 未装的机器上此 probe 无从下手
    if std::process::Command::new("cargo")
        .args(["machete", "--version"])
        .output()
        .map(|o| !o.status.success())
        .unwrap_or(true)
    {
        eprintln!("skip: cargo-machete unavailable");
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    // serde 声明但源码零使用 → machete 检出；used crate 干净
    write(
        dir.path(),
        "Cargo.toml",
        "[workspace]\nresolver = \"2\"\nmembers = [\"crates/with-unused\", \"crates/used\"]\n\n\
         [workspace.dependencies]\nserde = \"1\"\n",
    );
    write(
        dir.path(),
        "crates/with-unused/Cargo.toml",
        "[package]\nname = \"with-unused\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n\
         [dependencies]\nserde = { workspace = true }\n",
    );
    write(
        dir.path(),
        "crates/with-unused/src/lib.rs",
        "// 完全不用 serde\n",
    );
    write(
        dir.path(),
        "crates/used/Cargo.toml",
        "[package]\nname = \"used\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    write(dir.path(), "crates/used/src/lib.rs", "pub fn u() {}\n");

    let config: ChecksConfig = toml::from_str("").unwrap();
    let (registry, errors) = Registry::build(
        vec![Box::new(check_deps::machete::UnusedDeps)],
        &[],
        &config,
    );
    assert!(errors.is_empty(), "{errors:?}");
    let ctx = Ctx::new(dir.path(), Vec::new(), None);
    let report = registry.run(
        &ctx,
        &RunOptions {
            classes: None,
            semantic_endpoint: None,
            overrides: None,
        },
    );

    let hits: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.message.contains("with-unused"))
        .collect();
    assert_eq!(hits.len(), 1, "{report:?}");
    assert_eq!(hits[0].id, "DEPS.unused-deps");
    assert_eq!(hits[0].path, "Cargo.toml");
    // used crate 不误报
    assert!(
        !report
            .findings
            .iter()
            .any(|f| f.message.contains("crates/used") || f.message.contains("crate used"))
    );
}

// ---------------------------------------------------------------------------
// dep-tag：git 依赖 branch 钉版 → 恰好一条 WARN；tag / rev 与 registry 依赖 → 沉默。
// 用真实 git 仓库当依赖源：只有真实 `cargo metadata` 的 source 字段才带
// tag/branch 信息（伪造不出来）。git 不可用的环境跳过（环境缺失，非测试失败）。

fn git_available() -> bool {
    std::process::Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// 建一个本地 git 仓库（upstream）并提交；返回其路径。
fn git_upstream(root: &Path, name: &str) -> std::path::PathBuf {
    let dir = root.join(name);
    write(
        &dir,
        "Cargo.toml",
        "[package]\nname = \"upstream\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    write(&dir, "src/lib.rs", "pub fn u() {}\n");
    let run = |args: &[&str]| {
        let out = std::process::Command::new("git")
            .args(args)
            .current_dir(&dir)
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@t")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@t")
            .output()
            .expect("git");
        assert!(out.status.success(), "git {args:?} failed: {out:?}");
    };
    run(&["init", "-q", "-b", "main"]);
    run(&["add", "-A"]);
    run(&["commit", "-qm", "init"]);
    dir
}

fn run_dep_tag(root: &Path) -> Report {
    let (registry, errors) = Registry::build(
        vec![Box::new(check_deps::dep_tag::GitDepPin)],
        &[],
        &check_core::config::ChecksConfig::default(),
    );
    assert!(errors.is_empty(), "{errors:?}");
    let ctx = Ctx::new(root, Vec::new(), None);
    registry.run(
        &ctx,
        &RunOptions {
            classes: None,
            semantic_endpoint: None,
            overrides: None,
        },
    )
}

#[test]
fn branch_pinned_git_dep_is_flagged_tag_is_not() {
    if !git_available() {
        eprintln!("skip: git unavailable");
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let upstream = git_upstream(dir.path(), "upstream");
    let upstream = upstream.to_str().unwrap();

    write(
        dir.path(),
        "Cargo.toml",
        "[workspace]\nresolver = \"2\"\nmembers = [\"crates/consumer\"]\n",
    );
    write(
        dir.path(),
        "crates/consumer/Cargo.toml",
        &format!(
            "[package]\nname = \"consumer\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n\
             [dependencies]\nupstream = {{ git = \"file://{upstream}\", branch = \"main\" }}\n"
        ),
    );
    write(dir.path(), "crates/consumer/src/lib.rs", "pub fn c() {}\n");

    // broken：branch 钉版 → 恰好一条 finding，指回消费方 Cargo.toml
    let report = run_dep_tag(dir.path());
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "DEPS.git-dep-branch");
    assert_eq!(report.findings[0].path, "crates/consumer/Cargo.toml");
    assert_eq!(report.findings[0].severity, check_core::Severity::Warn);
    assert!(
        report.findings[0].message.contains("branch=main"),
        "{:?}",
        report.findings[0].message
    );

    // intact：同一依赖改 tag 钉版 → 0 finding
    write(
        dir.path(),
        "crates/consumer/Cargo.toml",
        &format!(
            "[package]\nname = \"consumer\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n\
             [dependencies]\nupstream = {{ git = \"file://{upstream}\", tag = \"v1\" }}\n"
        ),
    );
    let upstream_dir = std::path::PathBuf::from(upstream);
    let git = |args: &[&str]| {
        let out = std::process::Command::new("git")
            .args(args)
            .current_dir(&upstream_dir)
            .output()
            .unwrap();
        assert!(out.status.success(), "git {args:?}: {out:?}");
    };
    git(&["tag", "v1"]);
    let report = run_dep_tag(dir.path());
    assert!(
        report
            .findings
            .iter()
            .all(|f| f.id != "DEPS.git-dep-branch"),
        "{report:?}"
    );
}

#[test]
fn path_and_registry_deps_are_out_of_scope() {
    if !git_available() {
        eprintln!("skip: git unavailable");
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let upstream = git_upstream(dir.path(), "upstream");
    let upstream = upstream.to_str().unwrap();
    write(
        dir.path(),
        "Cargo.toml",
        "[workspace]\nresolver = \"2\"\nmembers = [\"crates/consumer\"]\n",
    );
    write(
        dir.path(),
        "crates/consumer/Cargo.toml",
        &format!(
            "[package]\nname = \"consumer\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n\
             [dependencies]\nupstream = {{ git = \"file://{upstream}\", rev = \"abcd\" }}\n\
             serde = \"1\"\n"
        ),
    );
    write(dir.path(), "crates/consumer/src/lib.rs", "pub fn c() {}\n");

    // rev 钉版 + registry 依赖都不报
    let report = run_dep_tag(dir.path());
    assert!(
        report.findings.is_empty(),
        "rev 钉版与 registry 依赖都应沉默：{report:?}"
    );
}

// ---------------------------------------------------------------------------
// dep-cycles：workspace path 依赖环 → FAIL（确定性、零误报）。

fn run_cycles(root: &std::path::Path, toml_text: &str) -> check_core::Report {
    let config: check_core::config::ChecksConfig = toml::from_str(toml_text).unwrap();
    let (registry, errors) =
        Registry::build(Vec::new(), &[Box::new(check_deps::CyclesFactory)], &config);
    assert!(errors.is_empty(), "{errors:?}");
    let ctx = Ctx::new(root, Vec::new(), None);
    registry.run(
        &ctx,
        &RunOptions {
            classes: None,
            semantic_endpoint: None,
            overrides: None,
        },
    )
}

#[test]
fn dep_cycles_fires_on_two_package_cycle_silent_on_dag() {
    let dir = tempfile::tempdir().unwrap();
    // 环：a ↔ b（path 互相依赖）
    write(
        dir.path(),
        "Cargo.toml",
        "[workspace]\nresolver = \"2\"\nmembers = [\"crates/a\", \"crates/b\"]\n",
    );
    write(
        dir.path(),
        "crates/a/Cargo.toml",
        "[package]\nname = \"a\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nb = { path = \"../b\" }\n",
    );
    write(dir.path(), "crates/a/src/lib.rs", "pub use b::f;\n");
    write(
        dir.path(),
        "crates/b/Cargo.toml",
        "[package]\nname = \"b\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\na = { path = \"../a\" }\n",
    );
    write(dir.path(), "crates/b/src/lib.rs", "pub use a::g;\n");

    let report = run_cycles(dir.path(), "[kinds.dep-cycles.main]\n");
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "DEP-CYCLES.main");
    assert_eq!(report.findings[0].severity, check_core::Severity::Fail);
    assert!(
        report.findings[0].message.contains("2 个包"),
        "{:?}",
        report.findings[0].message
    );

    // intact：解开环（b 不再依赖 a）→ 静默
    write(
        dir.path(),
        "crates/b/Cargo.toml",
        "[package]\nname = \"b\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    let report = run_cycles(dir.path(), "[kinds.dep-cycles.main]\n");
    assert!(report.findings.is_empty(), "{report:?}");
}
