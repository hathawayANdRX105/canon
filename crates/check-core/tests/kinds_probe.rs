//! D7 probe 约定（T0 框架验证）：同一实例 intact 必须 0 finding，broken 必须 ≥1。

use std::path::{Path, PathBuf};

use check_core::config::ChecksConfig;
use check_core::context::Ctx;
use check_core::kinds::builtin_factories;
use check_core::registry::{Registry, RunOptions};

fn build_rules(toml_text: &str) -> Registry {
    let config: ChecksConfig = toml::from_str(toml_text).expect("valid checks toml");
    let (registry, errors) = Registry::build(Vec::new(), &builtin_factories(), &config);
    assert!(errors.is_empty(), "unexpected build errors: {errors:?}");
    registry
}

fn write_fixture(root: &Path, rel: &str, body: &str) -> PathBuf {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, body).unwrap();
    PathBuf::from(rel)
}

fn no_overrides() -> RunOptions<'static> {
    RunOptions {
        classes: None,
        semantic_endpoint: None,
        overrides: None,
    }
}

#[test]
fn text_pattern_intact_zero_broken_one() {
    let dir = tempfile::tempdir().unwrap();
    let clean = write_fixture(
        dir.path(),
        "src/clean.rs",
        "fn main() {\n    println!(\"hi\");\n}\n",
    );
    let registry = build_rules(
        r#"
[kinds.text-pattern.no-dbg]
patterns_deny = ["\\bdbg!\\("]
message = "禁止 dbg! 残留"
"#,
    );
    let ctx = Ctx::new(dir.path(), vec![clean.clone()], None);
    assert!(registry.run(&ctx, &no_overrides()).findings.is_empty());

    let broken = write_fixture(dir.path(), "src/broken.rs", "fn f() {\n    dbg!(1);\n}\n");
    let ctx = Ctx::new(dir.path(), vec![clean, broken], None);
    let report = registry.run(&ctx, &no_overrides());
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "TEXT-PATTERN.no-dbg");
    assert_eq!(report.findings[0].path, "src/broken.rs");
    assert_eq!(report.findings[0].line, 2);
}

#[test]
fn text_pattern_include_globs_stay_shallow() {
    // literal_separator：`*` 不跨目录分隔符，深层同名模式不进扫描面
    let dir = tempfile::tempdir().unwrap();
    let top = write_fixture(dir.path(), "src/top.rs", "fn a() {\n    dbg!(2);\n}\n");
    let deep = write_fixture(
        dir.path(),
        "src/deep/nested.rs",
        "fn b() {\n    dbg!(3);\n}\n",
    );
    let registry = build_rules(
        r#"
[kinds.text-pattern.shallow-only]
paths_include = ["src/*.rs"]
patterns_deny = ["\\bdbg!\\("]
message = "禁止 dbg! 残留"
"#,
    );
    let ctx = Ctx::new(dir.path(), vec![top, deep], None);
    let report = registry.run(&ctx, &no_overrides());
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].path, "src/top.rs");
}

#[test]
fn path_forbidden_intact_zero_broken_one() {
    let dir = tempfile::tempdir().unwrap();
    let registry = build_rules(
        r#"
[kinds.path-forbidden.page-ui]
paths = ["src/pages/*/ui.rs"]
message = "page 不应自带 ui 子模块"
"#,
    );
    let intact = write_fixture(dir.path(), "src/pages/a/mod.rs", "// ok\n");
    let ctx = Ctx::new(dir.path(), vec![intact.clone()], None);
    assert!(registry.run(&ctx, &no_overrides()).findings.is_empty());

    let broken = write_fixture(dir.path(), "src/pages/a/ui.rs", "// bad\n");
    let ctx = Ctx::new(dir.path(), vec![intact, broken], None);
    let report = registry.run(&ctx, &no_overrides());
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "PATH-FORBIDDEN.page-ui");
}

#[test]
fn name_pattern_intact_zero_broken_one_with_exclude() {
    let dir = tempfile::tempdir().unwrap();
    let registry = build_rules(
        r#"
[kinds.name-pattern.snake]
paths_include = ["crates/**/*.rs", "crates/*.rs"]
paths_exclude = ["crates/**/generated/**"]
pattern = "^[a-z0-9_]+\\.rs$"
message = "Rust 源文件名必须 snake_case"
"#,
    );
    // intact：snake_case 名 + 排除目录里的违规名
    let ok = write_fixture(dir.path(), "crates/alpha/src/lib.rs", "// ok\n");
    let excluded = write_fixture(
        dir.path(),
        "crates/alpha/generated/BadName.rs",
        "// excluded\n",
    );
    let ctx = Ctx::new(dir.path(), vec![ok.clone(), excluded.clone()], None);
    assert!(registry.run(&ctx, &no_overrides()).findings.is_empty());

    // broken：crates 下的非 snake_case 文件名
    let broken = write_fixture(dir.path(), "crates/alpha/src/BadName.rs", "// bad\n");
    let ctx = Ctx::new(dir.path(), vec![ok, excluded, broken], None);
    let report = registry.run(&ctx, &no_overrides());
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "NAME-PATTERN.snake");
    assert_eq!(report.findings[0].path, "crates/alpha/src/BadName.rs");
}
