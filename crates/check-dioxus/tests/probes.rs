//! D7 probe：四条 dioxus 规则各配 intact / broken 双例。

use check_core::Report;
use check_core::config::ChecksConfig;
use check_core::registry::{Registry, RunOptions};
use check_dioxus::{factories, rules};
use std::path::PathBuf;

fn plain_registry() -> Registry {
    let (registry, errors) = Registry::build(rules(), &factories(), &Default::default());
    assert!(errors.is_empty(), "{errors:?}");
    registry
}

fn registry_with_kinds(toml_text: &str) -> Registry {
    let config: ChecksConfig = toml::from_str(toml_text).expect("valid checks toml");
    let (registry, errors) = Registry::build(rules(), &factories(), &config);
    assert!(errors.is_empty(), "{errors:?}");
    registry
}

fn write_fixture(root: &std::path::Path, rel: &str, body: &str) -> PathBuf {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, body).unwrap();
    PathBuf::from(rel)
}

fn run(registry: &Registry, root: &std::path::Path, files: &[PathBuf]) -> Report {
    let ctx = check_core::context::Ctx::new(root, files.to_vec(), None);
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
fn one_rsx_intact_zero_broken_one() {
    let dir = tempfile::tempdir().unwrap();
    let intact = write_fixture(
        dir.path(),
        "src/app.rs",
        "fn app() {\n    rsx! { div { \"hi\" } }\n}\nfn free() {\n    println!(\"no rsx here\");\n}\n",
    );
    let registry = plain_registry();
    assert!(run(&registry, dir.path(), &[intact]).findings.is_empty());

    let broken = write_fixture(
        dir.path(),
        "src/two.rs",
        "fn two() {\n    rsx! { div { \"a\" } }\n    let _ = rsx! { div { \"b\" } };\n}\n",
    );
    let report = run(&registry, dir.path(), &[broken]);
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "RSX.one-rsx-per-component");
}

#[test]
fn nesting_kind_reports_deepest_element_line() {
    let dir = tempfile::tempdir().unwrap();
    let registry = registry_with_kinds("[kinds.rsx-nesting.web]\nmax_depth = 3\n");
    let three = write_fixture(
        dir.path(),
        "src/three.rs",
        "fn t() {\n    rsx! {\n        div {\n            div {\n                div { \"ok\" }\n            }\n        }\n    }\n}\n",
    );
    // 三层在上限内
    assert!(run(&registry, dir.path(), &[three]).findings.is_empty());

    let four = write_fixture(
        dir.path(),
        "src/four.rs",
        "fn app() {\n    rsx! {\n        div {\n            div {\n                div {\n                    div { \"deep\" }\n                }\n            }\n        }\n    }\n}\n",
    );
    let report = run(&registry, dir.path(), &[four]);
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "RSX-NESTING.web");
    // 第 4 个 div（深度 4）所在行
    assert_eq!(report.findings[0].line, 6);
}

#[test]
fn cjk_text_catches_rsx_text_only() {
    let dir = tempfile::tempdir().unwrap();
    let registry = plain_registry();
    // 注释与普通字符串字面量不在 rsx 元素体内，不受限
    let intact = write_fixture(
        dir.path(),
        "src/ok.rs",
        "// 注释里中文不受限\nfn ok() {\n    let label = \"普通字符串字面量\";\n    rsx! { div { \"{label}\" } }\n}\n",
    );
    assert!(run(&registry, dir.path(), &[intact]).findings.is_empty());

    let broken = write_fixture(
        dir.path(),
        "src/bad.rs",
        "fn bad() {\n    rsx! { div { \"你好\" } }\n}\n",
    );
    let report = run(&registry, dir.path(), &[broken]);
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "RSX.cjk-text");
}

#[test]
fn list_key_requires_keyed_row_node() {
    let dir = tempfile::tempdir().unwrap();
    let registry = plain_registry();
    let intact = write_fixture(
        dir.path(),
        "src/ok.rs",
        "#[component]\nfn list(items: Vec<String>) {\n    rsx! {\n        for item in items {\n            div { key: \"{item}\", \"{item}\" }\n        }\n    }\n}\n",
    );
    assert!(run(&registry, dir.path(), &[intact]).findings.is_empty());

    let broken = write_fixture(
        dir.path(),
        "src/bad.rs",
        "fn list(items: Vec<String>) {\n    rsx! {\n        for item in items {\n            div { \"{item}\" }\n        }\n    }\n}\n",
    );
    let report = run(&registry, dir.path(), &[broken]);
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "RSX.list-key");
}
