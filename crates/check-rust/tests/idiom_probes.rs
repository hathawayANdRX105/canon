//! rust-idiom kind 的 D7 probe：每个检测器配 intact（0 finding）/ broken（≥1 finding）。
//!
//! 每个检测器用独立实例（`idioms = ["<det>"]`）隔离验证，避免跨检测器干扰。

use std::path::{Path, PathBuf};

use check_core::config::ChecksConfig;
use check_core::context::Ctx;
use check_core::finding::Finding;
use check_core::registry::{Registry, RunOptions};
use check_rust::factories;

fn build_rules(toml_text: &str) -> Registry {
    let config: ChecksConfig = toml::from_str(toml_text).expect("valid checks toml");
    let (registry, errors) = Registry::build(Vec::new(), &factories(), &config);
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

fn idiom_instance(detector: &str) -> String {
    format!(
        r#"
[kinds.rust-idiom.probe]
paths_include = ["**/*.rs"]
idioms = ["{detector}"]
"#
    )
}

fn run(registry: &Registry, root: &Path, files: &[PathBuf]) -> Vec<Finding> {
    let ctx = Ctx::new(root, files.to_vec(), None);
    registry.run(&ctx, &no_overrides()).findings
}

/// 单个检测器的 probe：intact 必须 0 finding，broken 必须 ≥1 且带 `[detector]`。
fn assert_probe(detector: &str, intact: &str, broken: &str) {
    let dir = tempfile::tempdir().unwrap();
    let registry = build_rules(&idiom_instance(detector));
    let intact_file = write_fixture(dir.path(), "src/intact.rs", intact);
    let broken_file = write_fixture(dir.path(), "src/broken.rs", broken);

    let intact_report = run(&registry, dir.path(), std::slice::from_ref(&intact_file));
    assert!(
        intact_report.is_empty(),
        "{detector} intact 不应有 finding: {intact_report:?}"
    );

    let report = run(&registry, dir.path(), &[intact_file, broken_file]);
    assert!(!report.is_empty(), "{detector} broken 必须 ≥1 finding");
    assert_eq!(report[0].id, "RUST-IDIOM.probe");
    assert!(
        report[0].message.contains(&format!("[{detector}]")),
        "{detector} message 应带检测器名: {:?}",
        report[0].message
    );
}

#[test]
fn if_else_chain_intact_short_chain_broken_long_chain() {
    assert_probe(
        "if-else-chain",
        "fn f(a: bool, b: bool) -> i32 {\n    if a { 1 } else if b { 2 } else { 3 }\n}\n",
        "fn f(a: bool, b: bool, c: bool, d: bool) -> i32 {\n    if a { 1 } else if b { 2 } else if c { 3 } else if d { 4 } else { 5 }\n}\n",
    );
}

#[test]
fn collapsible_if_intact_flat_broken_nested() {
    assert_probe(
        "collapsible-if",
        "fn f(a: bool, b: bool) {\n    if a && b { work(); }\n}\n",
        "fn f(a: bool, b: bool) {\n    if a {\n        if b { work(); }\n    }\n}\n",
    );
}

#[test]
fn nested_if_let_intact_single_broken_nested() {
    assert_probe(
        "nested-if-let",
        "fn f(x: Option<i32>, y: Option<i32>) {\n    if let Some(a) = x { use_a(a); }\n}\n",
        "fn f(x: Option<i32>, y: Option<i32>) {\n    if let Some(a) = x {\n        if let Some(b) = y { use_both(a, b); }\n    }\n}\n",
    );
}

#[test]
fn manual_let_else_intact_uses_let_else_broken_manual() {
    assert_probe(
        "manual-let-else",
        "fn f(w: Option<i32>) -> i32 {\n    let Some(v) = w else { return 0 };\n    v\n}\n",
        "fn f(w: Option<i32>) -> i32 {\n    let v = if let Some(v) = w { v } else { return 0 };\n    v\n}\n",
    );
}

#[test]
fn needless_return_intact_tail_expr_broken_return_stmt() {
    assert_probe(
        "needless-return",
        "fn f(x: i32) -> i32 {\n    let y = x + 1;\n    y\n}\n",
        "fn f(x: i32) -> i32 {\n    let y = x + 1;\n    return y;\n}\n",
    );
}

#[test]
fn manual_question_mark_intact_uses_qmark_broken_match() {
    assert_probe(
        "manual-question-mark",
        "fn f(r: Result<i32, String>) -> Result<i32, String> {\n    let v = r?;\n    Ok(v)\n}\n",
        "fn f(r: Result<i32, String>) -> Result<i32, String> {\n    let v = match r {\n        Ok(v) => v,\n        Err(e) => return Err(e.into()),\n    };\n    Ok(v)\n}\n",
    );
}

#[test]
fn manual_question_mark_intact_qmark_broken_is_err_return() {
    assert_probe(
        "manual-question-mark",
        "fn f(r: Result<i32, String>) -> Result<i32, String> {\n    let v = r?;\n    Ok(v)\n}\n",
        "fn f(r: Result<i32, String>) -> Result<i32, String> {\n    if r.is_err() {\n        return r;\n    }\n    let v = r.unwrap();\n    Ok(v)\n}\n",
    );
}

#[test]
fn unwrap_in_fallible_intact_qmark_and_cfg_test_broken_unwrap() {
    assert_probe(
        "unwrap-in-fallible",
        "fn f(r: Result<i32, String>) -> Result<i32, String> {\n    let v = r?;\n    Ok(v)\n}\n\n#[cfg(test)]\nmod tests {\n    fn t(r: Result<i32, String>) -> i32 {\n        r.unwrap()\n    }\n}\n\nfn plain(x: Option<i32>) -> i32 {\n    x.unwrap_or(0)\n}\n",
        "fn f(r: Result<i32, String>) -> Result<i32, String> {\n    let v = r.unwrap();\n    let w = r.expect(\"should be ok\");\n    Ok(v + w)\n}\n",
    );
}

#[test]
fn hashmap_entry_intact_uses_entry_broken_contains_key() {
    assert_probe(
        "hashmap-entry",
        "use std::collections::HashMap;\nfn f(map: &mut HashMap<String, i32>, k: String) {\n    match map.entry(k) {\n        std::collections::hash_map::Entry::Occupied(mut e) => { *e.get_mut() += 1; }\n        std::collections::hash_map::Entry::Vacant(e) => { e.insert(1); }\n    }\n}\n",
        "use std::collections::HashMap;\nfn f(map: &mut HashMap<String, i32>, k: String) {\n    if map.contains_key(&k) {\n        let v = map.get_mut(&k).unwrap();\n        *v += 1;\n    } else {\n        map.insert(k, 1);\n    }\n}\n",
    );
}

#[test]
fn while_let_on_iterator_intact_for_loop_broken_next() {
    assert_probe(
        "while-let-on-iterator",
        "fn f(iter: impl Iterator<Item = i32>) {\n    for x in iter { use_x(x); }\n}\n",
        "fn f(iter: impl Iterator<Item = i32>) {\n    let mut iter = iter;\n    while let Some(x) = iter.next() { use_x(x); }\n}\n",
    );
}

#[test]
fn vec_push_loop_intact_collect_broken_push() {
    assert_probe(
        "vec-push-loop",
        "fn f(items: &[i32]) -> Vec<i32> {\n    items.iter().map(|x| x * 2).collect()\n}\n",
        "fn f(items: &[i32]) -> Vec<i32> {\n    let mut v = Vec::new();\n    for x in items {\n        v.push(x * 2);\n    }\n    v\n}\n",
    );
}

#[test]
fn bool_comparison_intact_direct_broken_eq_true() {
    assert_probe(
        "bool-comparison",
        "fn f(x: bool) {\n    if x { work(); }\n    if !x { other(); }\n}\n",
        "fn f(x: bool) {\n    if x == true { work(); }\n    if y != false { other(); }\n}\n",
    );
}

#[test]
fn redundant_pattern_matching_intact_method_broken_match() {
    assert_probe(
        "redundant-pattern-matching",
        "fn f(r: Result<i32, String>) -> bool {\n    r.is_ok()\n}\n",
        "fn f(r: Result<i32, String>) -> bool {\n    match r {\n        Ok(_) => true,\n        Err(_) => false,\n    }\n}\n",
    );
}

// ---- batch 2：chain 式语法 → 标准库 API 提示 ----

#[test]
fn manual_map_intact_method_broken_match() {
    assert_probe(
        "manual-map",
        "fn f(opt: Option<i32>) -> Option<i32> {\n    opt.map(|v| v * 2)\n}\n",
        "fn f(opt: Option<i32>) -> Option<i32> {\n    match opt {\n        Some(v) => Some(v * 2),\n        None => None,\n    }\n}\n",
    );
}

#[test]
fn manual_and_then_intact_method_broken_match() {
    assert_probe(
        "manual-and-then",
        "fn f(opt: Option<i32>) -> Option<String> {\n    opt.and_then(|v| v.to_string().parse().ok())\n}\n",
        "fn f(opt: Option<i32>) -> Option<String> {\n    match opt {\n        Some(v) => v.to_string().parse().ok(),\n        None => None,\n    }\n}\n",
    );
}

#[test]
fn manual_unwrap_or_intact_method_broken_match() {
    assert_probe(
        "manual-unwrap-or",
        "fn f(opt: Option<i32>) -> i32 {\n    opt.unwrap_or(0)\n}\n",
        "fn f(opt: Option<i32>) -> i32 {\n    match opt {\n        Some(v) => v,\n        None => 0,\n    }\n}\n",
    );
}

#[test]
fn manual_map_or_intact_method_broken_match() {
    assert_probe(
        "manual-map-or",
        "fn f(opt: Option<String>) -> usize {\n    opt.map_or(0, |v| v.len())\n}\n",
        "fn f(opt: Option<String>) -> usize {\n    match opt {\n        Some(v) => v.len(),\n        None => 0,\n    }\n}\n",
    );
}

#[test]
fn manual_ok_or_intact_method_broken_match() {
    assert_probe(
        "manual-ok-or",
        "fn f(opt: Option<i32>) -> Result<i32, &str> {\n    opt.ok_or(\"missing\")\n}\n",
        "fn f(opt: Option<i32>) -> Result<i32, &str> {\n    match opt {\n        Some(v) => Ok(v),\n        None => Err(\"missing\"),\n    }\n}\n",
    );
}

#[test]
fn option_filter_intact_method_broken_if_let() {
    assert_probe(
        "option-filter",
        "fn f(opt: Option<i32>) -> Option<i32> {\n    opt.filter(|v| *v > 0)\n}\n",
        "fn f(opt: Option<i32>) -> Option<i32> {\n    if let Some(v) = opt {\n        if *v > 0 { Some(v) } else { None }\n    } else {\n        None\n    }\n}\n",
    );
}

#[test]
fn manual_flatten_intact_method_broken_match() {
    assert_probe(
        "manual-flatten",
        "fn f(opt: Option<Option<i32>>) -> Option<i32> {\n    opt.flatten()\n}\n",
        "fn f(opt: Option<Option<i32>>) -> Option<i32> {\n    match opt {\n        Some(Some(v)) => Some(v),\n        _ => None,\n    }\n}\n",
    );
}

#[test]
fn iter_index_loop_intact_iter_broken_range() {
    assert_probe(
        "iter-index-loop",
        "fn f(v: &[i32]) {\n    for (i, x) in v.iter().enumerate() { use_pair(i, x); }\n}\n",
        "fn f(v: &[i32]) {\n    for i in 0..v.len() { use_x(v[i]); }\n}\n",
    );
}

#[test]
fn derive_able_impl_intact_derive_broken_manual() {
    assert_probe(
        "derive-able-impl",
        "#[derive(Clone)]\nstruct Foo { a: i32 }\n",
        "struct Foo { a: i32 }\nimpl Clone for Foo {\n    fn clone(&self) -> Self {\n        Self { a: self.a.clone() }\n    }\n}\n",
    );
}
