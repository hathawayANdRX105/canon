//! Probes for the crate placement catalog: file-to-package mapping, ordered
//! `[[files]]` matching, path allow/deny, spec validation, and the
//! `crate-placement` kind running through the canon registry.

use std::path::{Path, PathBuf};

use check_core::config::ChecksConfig;
use check_core::context::Ctx;
use check_core::registry::{Registry, RunOptions};

use check_crate::catalog::{Catalog, DiagnoseOptions, Diagnostic, LoadConfig};

fn write(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, body).unwrap();
}

fn workspace(root: &Path) {
    write(
        root,
        "Cargo.toml",
        "[workspace]\nresolver = \"2\"\nmembers = [\"crates/alpha\", \"crates/beta\"]\n",
    );
    for package in ["alpha", "beta"] {
        write(
            root,
            &format!("crates/{package}/Cargo.toml"),
            &format!("[package]\nname = \"{package}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"),
        );
        write(
            root,
            &format!("crates/{package}/src/lib.rs"),
            "pub fn run() {}\n",
        );
    }
}

fn spec(root: &Path, package: &str, body: &str) {
    write(root, &format!("crates/{package}/placement.toml"), body);
}

fn load(root: &Path) -> Catalog {
    Catalog::load(root, &LoadConfig::default()).expect("catalog")
}

fn diagnose(
    catalog: &Catalog,
    changed: &[&str],
    require_spec: bool,
    all: bool,
    allow_cjk: bool,
) -> Vec<Diagnostic> {
    let changed: Vec<PathBuf> = changed.iter().map(PathBuf::from).collect();
    catalog.diagnose(
        &changed,
        &DiagnoseOptions {
            require_spec,
            all,
            allow_cjk,
        },
    )
}

fn diagnostic_ids(diagnostics: Vec<Diagnostic>) -> Vec<String> {
    diagnostics
        .into_iter()
        .map(|diagnostic| diagnostic.id)
        .collect()
}

#[test]
fn file_rules_match_in_order() {
    let dir = tempfile::tempdir().unwrap();
    workspace(dir.path());
    spec(
        dir.path(),
        "alpha",
        r#"
schema = 1
crate = "alpha"
responsibility = "Alpha crate."
[[files]]
path = "src/lib.rs"
responsibility = "Public surface."
entry = true
[[files]]
path = "src/**"
responsibility = "Other source files."
[paths]
allow = ["src/**", "Cargo.toml", "placement.toml"]
"#,
    );

    let catalog = load(dir.path());
    assert!(diagnose(&catalog, &["crates/alpha/src/lib.rs"], false, false, false).is_empty());
    assert!(
        diagnose(
            &catalog,
            &["crates/alpha/src/other.rs"],
            false,
            false,
            false
        )
        .is_empty(),
        "src/** covers other source files"
    );

    let assignment = catalog
        .assignment(Path::new("crates/alpha/src/lib.rs"))
        .expect("assignment");
    assert_eq!(assignment.file_pattern.as_deref(), Some("src/lib.rs"));
    assert_eq!(
        assignment.responsibility.as_deref(),
        Some("Public surface.")
    );
    assert!(assignment.entry);
}

#[test]
fn unmapped_file_is_reported() {
    let dir = tempfile::tempdir().unwrap();
    workspace(dir.path());
    spec(
        dir.path(),
        "alpha",
        r#"
schema = 1
crate = "alpha"
responsibility = "Alpha crate."
[[files]]
path = "src/lib.rs"
responsibility = "Public surface."
entry = true
[paths]
allow = ["src/**", "Cargo.toml", "placement.toml"]
"#,
    );

    let catalog = load(dir.path());
    let ids = diagnostic_ids(diagnose(
        &catalog,
        &["crates/alpha/src/other.rs"],
        false,
        false,
        false,
    ));
    assert_eq!(ids, vec!["CRATE-FILE-UNMAPPED"]);
}

#[test]
fn deny_wins_over_allow() {
    let dir = tempfile::tempdir().unwrap();
    workspace(dir.path());
    spec(
        dir.path(),
        "alpha",
        r#"
schema = 1
crate = "alpha"
responsibility = "Alpha crate."
[[files]]
path = "src/**"
responsibility = "Source files."
[paths]
allow = ["src/**", "Cargo.toml", "placement.toml"]
deny = ["src/secret.rs"]
"#,
    );

    let catalog = load(dir.path());
    let ids = diagnostic_ids(diagnose(
        &catalog,
        &["crates/alpha/src/secret.rs"],
        false,
        false,
        false,
    ));
    assert_eq!(ids, vec!["CRATE-PATH-DENY"]);
}

#[test]
fn path_outside_allow_is_reported() {
    let dir = tempfile::tempdir().unwrap();
    workspace(dir.path());
    spec(
        dir.path(),
        "alpha",
        r#"
schema = 1
crate = "alpha"
responsibility = "Alpha crate."
[[files]]
path = "src/**"
responsibility = "Source files."
[paths]
allow = ["src/**", "Cargo.toml", "placement.toml"]
"#,
    );

    let catalog = load(dir.path());
    let ids = diagnostic_ids(diagnose(
        &catalog,
        &["crates/alpha/README.md"],
        false,
        false,
        false,
    ));
    assert_eq!(ids, vec!["CRATE-PATH-OUTSIDE"]);
}

#[test]
fn missing_spec_respects_require_spec() {
    let dir = tempfile::tempdir().unwrap();
    workspace(dir.path());
    spec(
        dir.path(),
        "alpha",
        r#"
schema = 1
crate = "alpha"
responsibility = "Alpha crate."
[[files]]
path = "src/**"
responsibility = "Source files."
"#,
    );

    let catalog = load(dir.path());
    let changed = ["crates/beta/src/lib.rs"];
    assert!(diagnose(&catalog, &changed, false, false, false).is_empty());
    let ids = diagnostic_ids(diagnose(&catalog, &changed, true, false, false));
    assert_eq!(ids, vec!["CRATE-SPEC-MISSING"]);
}

#[test]
fn cjk_spec_is_reported() {
    let dir = tempfile::tempdir().unwrap();
    workspace(dir.path());
    spec(
        dir.path(),
        "alpha",
        "schema = 1\ncrate = \"alpha\"\nresponsibility = \"Alpha 职责\"\n[[files]]\npath = \"src/**\"\nresponsibility = \"Source files.\"\n",
    );

    let catalog = load(dir.path());
    let changed = ["crates/alpha/src/lib.rs"];
    let ids = diagnostic_ids(diagnose(&catalog, &changed, false, false, false));
    assert_eq!(ids, vec!["CRATE-SPEC-CJK"]);
    assert!(diagnose(&catalog, &changed, false, false, true).is_empty());
}

#[test]
fn redirect_must_resolve() {
    let dir = tempfile::tempdir().unwrap();
    workspace(dir.path());
    spec(
        dir.path(),
        "alpha",
        r#"
schema = 1
crate = "alpha"
responsibility = "Alpha crate."
[[files]]
path = "src/**"
responsibility = "Source files."
[[does_not_belong]]
concern = "Beta work"
to = "ghost"
"#,
    );

    let catalog = load(dir.path());
    let ids = diagnostic_ids(diagnose(
        &catalog,
        &["crates/alpha/src/lib.rs"],
        false,
        false,
        false,
    ));
    assert_eq!(ids, vec!["CRATE-SPEC-REDIRECT"]);
}

#[test]
fn entry_must_exist() {
    let dir = tempfile::tempdir().unwrap();
    workspace(dir.path());
    spec(
        dir.path(),
        "alpha",
        r#"
schema = 1
crate = "alpha"
responsibility = "Alpha crate."
[[files]]
path = "src/nope.rs"
responsibility = "Missing entry."
entry = true
"#,
    );

    let catalog = load(dir.path());
    let ids = diagnostic_ids(diagnose(
        &catalog,
        &["crates/alpha/src/nope.rs"],
        false,
        false,
        false,
    ));
    assert_eq!(ids, vec!["CRATE-FILE-ENTRY-MISSING"]);
}

#[test]
fn kind_reports_through_registry() {
    let dir = tempfile::tempdir().unwrap();
    workspace(dir.path());
    spec(
        dir.path(),
        "alpha",
        r#"
schema = 1
crate = "alpha"
responsibility = "Alpha crate."
[[files]]
path = "src/lib.rs"
responsibility = "Public surface."
entry = true
[paths]
allow = ["src/**", "Cargo.toml", "placement.toml"]
"#,
    );

    let config: ChecksConfig = toml::from_str(
        r#"
[kinds.crate-placement.test]
spec_file = "placement.toml"
members_include = ["crates/**"]
"#,
    )
    .unwrap();
    let (registry, errors) = Registry::build(Vec::new(), &check_crate::factories(), &config);
    assert!(errors.is_empty(), "{errors:?}");

    let ctx = Ctx::new(
        dir.path(),
        Vec::new(),
        Some(vec![PathBuf::from("crates/alpha/src/other.rs")]),
    );
    let report = registry.run(
        &ctx,
        &RunOptions {
            classes: None,
            semantic_endpoint: None,
            overrides: None,
        },
    );
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "CRATE-PLACEMENT.test");
    assert!(
        report.findings[0].message.contains("CRATE-FILE-UNMAPPED"),
        "{report:?}"
    );
}
