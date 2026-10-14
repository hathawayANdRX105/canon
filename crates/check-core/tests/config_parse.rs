//! checks.toml 的严格性：未知键拒收、缺省启用（禁静默开关）、semantic 缺省值。

use check_core::config::{self, ChecksConfig};
use check_core::finding::Severity;

#[test]
fn unknown_top_level_key_is_rejected() {
    let err = toml::from_str::<ChecksConfig>("[typo]\nfoo = 1\n").unwrap_err();
    assert!(err.to_string().contains("unknown field"), "{err}");
}

#[test]
fn rules_default_to_enabled_without_silent_switches() {
    let cfg: ChecksConfig = toml::from_str("[rules.my-rule]\nseverity = \"fail\"\n").unwrap();
    assert_eq!(cfg.rules["my-rule"].severity, Some(Severity::Fail));
    assert!(cfg.rules["my-rule"].enabled);
}

#[test]
fn semantic_defaults_follow_laya_wiring() {
    let cfg: ChecksConfig =
        toml::from_str("[semantic]\nendpoint = \"http://127.0.0.1:8000\"\n").unwrap();
    let semantic = cfg.semantic.expect("endpoint parsed");
    assert_eq!(semantic.model, "jev-latest");
    assert_eq!(semantic.max_len, 8192);
}

#[test]
fn missing_config_file_is_empty_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = config::load(&dir.path().join("checks.toml")).unwrap();
    assert!(cfg.semantic.is_none());
    assert!(cfg.kinds.is_empty());
    assert!(cfg.rules.is_empty());
}
