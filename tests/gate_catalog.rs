//! `gate::catalog` 单元测试：spec 头注释抽取与 checklist id 归一。

use std::path::Path;

use canon::gate::catalog::{checklist_id, why_from_head};

#[test]
fn why_strips_markers_and_keeps_prose() {
    let raw = "# 为什么: 防烂注释\n# 检测: 匹配 AI 味\n\n# 另一段\nenabled: true\n";
    assert_eq!(why_from_head(raw), "为什么: 防烂注释\n检测: 匹配 AI 味");
}
#[test]
fn why_empty_when_file_starts_with_key() {
    assert_eq!(why_from_head("enabled: true\n# 尾部注释\n"), "");
}
#[test]
fn checklist_id_strips_affixes() {
    assert_eq!(checklist_id(Path::new("checklist_ccn.yaml")), "ccn");
    assert_eq!(
        checklist_id(Path::new("rust_no_dead_code_allow.yaml")),
        "rust_no_dead_code_allow"
    );
}
