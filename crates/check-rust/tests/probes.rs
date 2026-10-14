//! D7 probe：三条 Rust 结构规则各配 intact / broken 双例。

use check_core::context::Ctx;
use check_core::registry::{Registry, RunOptions};
use check_rust::rules;

fn registry() -> Registry {
    let (registry, errors) = Registry::build(rules(), &[], &Default::default());
    assert!(errors.is_empty(), "{errors:?}");
    registry
}

fn write_fixture(root: &std::path::Path, rel: &str, body: &str) -> std::path::PathBuf {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, body).unwrap();
    std::path::PathBuf::from(rel)
}

fn run(
    registry: &Registry,
    root: &std::path::Path,
    files: &[std::path::PathBuf],
) -> check_core::Report {
    let ctx = Ctx::new(root, files.to_vec(), None);
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
fn test_file_in_src_intact_zero_broken_one() {
    let dir = tempfile::tempdir().unwrap();
    let registry = registry();
    let intact = vec![
        write_fixture(dir.path(), "src/lib.rs", "pub fn f() {}\n"),
        write_fixture(dir.path(), "tests/api.rs", "use my_crate::f;\n"),
        write_fixture(
            dir.path(),
            "tests/foo_test.rs",
            "// tests/ 下允许 *_test.rs\n",
        ),
        write_fixture(dir.path(), "src/test_helper.rs", "pub fn helper() {}\n"),
    ];
    assert!(run(&registry, dir.path(), &intact).findings.is_empty());

    let mut broken = intact.clone();
    broken.push(write_fixture(
        dir.path(),
        "src/foo_test.rs",
        "fn test_foo() {}\n",
    ));
    let report = run(&registry, dir.path(), &broken);
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "RUST.test-file-in-src");
    assert_eq!(report.findings[0].path, "src/foo_test.rs");
}

#[test]
fn cfg_test_in_tests_intact_zero_broken_one_src_stays_legal() {
    let dir = tempfile::tempdir().unwrap();
    let registry = registry();
    let intact = vec![
        write_fixture(
            dir.path(),
            "tests/api.rs",
            "use my_crate::f;\n#[test]\nfn a() {}\n",
        ),
        // src/ 里的 #[cfg(test)] 是正当单测形态，规则不得触碰
        write_fixture(dir.path(), "src/lib.rs", "#[cfg(test)]\nmod tests {\n}\n"),
    ];
    assert!(run(&registry, dir.path(), &intact).findings.is_empty());

    let mut broken = intact.clone();
    broken.push(write_fixture(
        dir.path(),
        "tests/wired.rs",
        "#[cfg(test)]\nmod inner {\n}\n",
    ));
    let report = run(&registry, dir.path(), &broken);
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "RUST.cfg-test-in-tests");
    assert_eq!(report.findings[0].line, 1);
}

#[test]
fn dead_code_allow_same_line_reason_passes_bare_fails() {
    let dir = tempfile::tempdir().unwrap();
    let registry = registry();
    let intact = vec![
        // 同行 // 理由放行
        write_fixture(
            dir.path(),
            "src/legacy.rs",
            "#[allow(dead_code)] // legacy wire format, removal tracked (#123)\nfn old() {}\n",
        ),
        write_fixture(dir.path(), "src/used.rs", "pub fn used() {}\n"),
    ];
    assert!(run(&registry, dir.path(), &intact).findings.is_empty());

    let mut broken = intact.clone();
    // 无理由 + 多字段变体都触发
    broken.push(write_fixture(
        dir.path(),
        "src/never_used.rs",
        "#[allow(dead_code)]\nfn unused() {}\n",
    ));
    broken.push(write_fixture(
        dir.path(),
        "src/half_used.rs",
        "#[allow(dead_code, unused_variables)]\nfn gaps() {}\n",
    ));
    let report = run(&registry, dir.path(), &broken);
    assert_eq!(report.findings.len(), 2, "{report:?}");
    assert!(
        report
            .findings
            .iter()
            .all(|f| f.id == "RUST.dead-code-allow")
    );
}
