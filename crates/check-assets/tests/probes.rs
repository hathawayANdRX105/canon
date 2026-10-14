//! D7 probe：css-var-mirror 的 intact / broken 双例。

use check_core::Report;
use check_core::config::ChecksConfig;
use check_core::context::Ctx;
use check_core::registry::{Registry, RunOptions};
use std::path::{Path, PathBuf};

fn write(root: &Path, rel: &str, body: &str) -> PathBuf {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, body).unwrap();
    PathBuf::from(rel)
}

fn registry() -> Registry {
    let config: ChecksConfig = toml::from_str(
        r#"
[kinds.css-var-mirror.theme-dark]
paths_include = ["**/theme.css"]
message = "theme.css 的 .dark 段必须是语义 :root 块的同 token 镜像"
"#,
    )
    .expect("valid toml");
    let (registry, errors) = Registry::build(Vec::new(), &check_assets::factories(), &config);
    assert!(errors.is_empty(), "{errors:?}");
    registry
}

fn run(files: &[PathBuf], root: &Path) -> Report {
    let ctx = Ctx::new(root, files.to_vec(), None);
    registry().run(
        &ctx,
        &RunOptions {
            classes: None,
            semantic_endpoint: None,
            overrides: None,
        },
    )
}

const MIRRORED: &str = r#":root {
  --background: oklch(1 0 0);
  --foreground: oklch(0.1 0 0);
}

.dark {
  --background: oklch(0.1 0 0);
  --foreground: oklch(1 0 0);
}
"#;

#[test]
fn mirror_intact_zero() {
    let dir = tempfile::tempdir().unwrap();
    let theme = write(dir.path(), "assets/theme.css", MIRRORED);
    let report = run(&[theme], dir.path());
    assert!(report.findings.is_empty(), "{report:?}");
}

#[test]
fn mirror_missing_token_reports() {
    let dir = tempfile::tempdir().unwrap();
    let broken = write(
        dir.path(),
        "assets/theme.css",
        r#":root {
  --background: oklch(1 0 0);
  --foreground: oklch(0.1 0 0);
}

.dark {
  --background: oklch(0.1 0 0);
}
"#,
    );
    let report = run(&[broken], dir.path());
    assert_eq!(report.findings.len(), 1, "{report:?}");
    assert_eq!(report.findings[0].id, "CSS-VAR-MIRROR.theme-dark");
    assert_eq!(report.findings[0].severity, check_core::Severity::Fail);
    assert!(
        report.findings[0].message.contains("缺 1 个"),
        "{}",
        report.findings[0].message
    );
}

#[test]
fn no_dark_block_is_silent() {
    // 无 .dark 块的文件自动安静（可移植：不是所有仓都做暗色）
    let dir = tempfile::tempdir().unwrap();
    let plain = write(
        dir.path(),
        "assets/theme.css",
        ":root {\n  --background: red;\n}\n",
    );
    let report = run(&[plain], dir.path());
    assert!(report.findings.is_empty(), "{report:?}");
}

#[test]
fn non_included_css_is_ignored() {
    // paths_include 之外的 CSS 不参与
    let dir = tempfile::tempdir().unwrap();
    let other = write(
        dir.path(),
        "assets/other.css",
        ":root {\n  --background: red;\n}\n.dark {\n}\n",
    );
    let report = run(&[other], dir.path());
    assert!(report.findings.is_empty(), "{report:?}");
}
