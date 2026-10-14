//! D7 probe：dep-edge 与 machete 各配 intact / broken 双例。

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
