//! duplicate-file kind 探针：D7 约定（intact 0 finding / broken ≥1）+ 阈值、停用词、limit 边界。

use std::path::Path;

use check_core::config::ChecksConfig;
use check_core::context::Ctx;
use check_core::kinds::builtin_factories;
use check_core::registry::{Registry, RunOptions};

fn build_rules(toml_text: &str) -> (Registry, Vec<String>) {
    let config: ChecksConfig = toml::from_str(toml_text).expect("valid checks toml");
    Registry::build(Vec::new(), &builtin_factories(), &config)
}

fn write_fixture(root: &Path, rel: &str, body: &str) -> std::path::PathBuf {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, body).unwrap();
    std::path::PathBuf::from(rel)
}

fn no_overrides() -> RunOptions<'static> {
    RunOptions {
        classes: None,
        semantic_endpoint: None,
        overrides: None,
    }
}

/// 40 行唯一 token 的语料；`keep` 行与 A 相同，其余行换成 beta token。
fn body_with_shared_lines(keep: usize, suffix: &str) -> String {
    (0..40)
        .map(|i| {
            if i < keep {
                format!("let token_alpha_{i} = compute_alpha_{i}();\n")
            } else {
                format!("let token_beta_{suffix}_{i} = compute_beta_{suffix}_{i}();\n")
            }
        })
        .collect()
}

const DUP_TOML: &str = r#"
[kinds.duplicate-file.dup]
paths_include = ["**/*.rs"]
min_tokens = 20
min_similarity = 0.85
limit = 5
stopwords = []
message = "疑似复刻既有实现"
"#;

/// 与 [`DUP_TOML`] 同参数，仅 limit=1（边界：只报最相似的一条）。
const DUP_TOML_LIMIT1: &str = r#"
[kinds.duplicate-file.dup]
paths_include = ["**/*.rs"]
min_tokens = 20
min_similarity = 0.85
limit = 1
stopwords = []
message = "疑似复刻既有实现"
"#;

#[test]
fn near_copy_reports_once_with_source_path() {
    let dir = tempfile::tempdir().unwrap();
    let original = write_fixture(
        dir.path(),
        "src/original.rs",
        &body_with_shared_lines(40, "x"),
    );
    let copy = write_fixture(dir.path(), "src/copy.rs", &body_with_shared_lines(38, "y"));
    let (registry, errors) = build_rules(DUP_TOML);
    assert!(errors.is_empty(), "{errors:?}");

    // intact：只有 original（scope 内），全仓没有它的近亲。
    let ctx = Ctx::new(dir.path(), vec![original.clone()], None);
    assert!(registry.run(&ctx, &no_overrides()).findings.is_empty());

    // broken：copy 与 original 共享 76/84 token → 0.90 ≥ 0.85。
    // 语料要含 original 本身，scope 只圈 copy（hook 口径：全仓语料 × 变更文件）。
    let ctx = Ctx::new(
        dir.path(),
        vec![original.clone(), copy.clone()],
        Some(vec![copy.clone()]),
    );
    let report = registry.run(&ctx, &no_overrides());
    assert_eq!(report.findings.len(), 1, "{report:?}");
    let finding = &report.findings[0];
    assert_eq!(finding.id, "DUPLICATE-FILE.dup");
    assert_eq!(finding.path, "src/copy.rs");
    assert!(
        finding.message.contains("src/original.rs"),
        "findings 要带最相似文件路径：{}",
        finding.message
    );
}

#[test]
fn distinct_files_stay_below_threshold() {
    let dir = tempfile::tempdir().unwrap();
    let a = write_fixture(dir.path(), "src/a.rs", &body_with_shared_lines(40, "x"));
    write_fixture(dir.path(), "src/b.rs", &body_with_shared_lines(20, "y"));
    let (registry, errors) = build_rules(DUP_TOML);
    assert!(errors.is_empty(), "{errors:?}");

    // 共享 40/120 token → 0.33 < 0.85，无误报（语料含 b，scope 圈 a）。
    let ctx = Ctx::new(
        dir.path(),
        vec![a.clone(), std::path::PathBuf::from("src/b.rs")],
        Some(vec![a]),
    );
    assert!(registry.run(&ctx, &no_overrides()).findings.is_empty());
}

#[test]
fn tiny_files_are_skipped_and_limit_caps_hits() {
    let dir = tempfile::tempdir().unwrap();
    let big = write_fixture(dir.path(), "src/big.rs", &body_with_shared_lines(40, "x"));
    // 三个近亲（彼此也和 big 近亲），limit=1 只报最相似的一条。
    write_fixture(dir.path(), "src/kin1.rs", &body_with_shared_lines(38, "k1"));
    write_fixture(dir.path(), "src/kin2.rs", &body_with_shared_lines(39, "k2"));
    write_fixture(dir.path(), "src/kin3.rs", &body_with_shared_lines(38, "k3"));
    let (registry, errors) = build_rules(DUP_TOML_LIMIT1);
    assert!(errors.is_empty(), "{errors:?}");

    let tracked = vec![
        big.clone(),
        std::path::PathBuf::from("src/kin1.rs"),
        std::path::PathBuf::from("src/kin2.rs"),
        std::path::PathBuf::from("src/kin3.rs"),
    ];
    let ctx = Ctx::new(dir.path(), tracked, Some(vec![big.clone()]));
    let report = registry.run(&ctx, &no_overrides());
    assert_eq!(report.findings.len(), 1, "limit=1 只报最相似：{report:?}");

    // 小文件（< min_tokens）不参评：2 行完全一致的 rs 不报。
    let dir2 = tempfile::tempdir().unwrap();
    let small = write_fixture(dir2.path(), "src/small.rs", "let a = 1;\nlet b = 2;\n");
    write_fixture(dir2.path(), "src/small2.rs", "let a = 1;\nlet b = 2;\n");
    let ctx2 = Ctx::new(dir2.path(), vec![small], None);
    assert!(registry.run(&ctx2, &no_overrides()).findings.is_empty());
}

#[test]
fn stopwords_suppress_boilerplate_similarity() {
    let dir = tempfile::tempdir().unwrap();
    // 两个文件共享全部“样板”token，但业务 token 完全不同。
    let boiler = "pub fn handle_thing(mut ctx: Ctx) -> Result<(), Error> {\n    let value = ctx.read();\n    Ok(())\n}\n";
    let a = write_fixture(
        dir.path(),
        "src/handle_users.rs",
        &format!("{boiler}fn users_impl() {{ let users_payload = 1; }}\n"),
    );
    write_fixture(
        dir.path(),
        "src/handle_orders.rs",
        &format!("{boiler}fn orders_impl() {{ let orders_payload = 2; }}\n"),
    );

    // 不设停用词：样板 token 占绝大多数，相似度被顶高。
    let noisy = format!(
        "{DUP_TOML}\n[kinds.duplicate-file.noisy]\npaths_include = [\"**/*.rs\"]\nmin_tokens = 5\nmin_similarity = 0.6\nlimit = 5\nstopwords = []\nmessage = \"noisy\"\n"
    );
    let (registry, errors) = build_rules(&noisy);
    assert!(errors.is_empty(), "{errors:?}");
    let tracked = vec![a.clone(), std::path::PathBuf::from("src/handle_orders.rs")];
    let ctx = Ctx::new(dir.path(), tracked, Some(vec![a.clone()]));
    assert!(
        !registry.run(&ctx, &no_overrides()).findings.is_empty(),
        "无停用词表时样板应触发（对照基线）"
    );

    // 停用词覆盖业务 token 之外的一切 → 相似度归零。
    let clean = format!(
        "{DUP_TOML}\n[kinds.duplicate-file.clean]\npaths_include = [\"**/*.rs\"]\nmin_tokens = 5\nmin_similarity = 0.6\nlimit = 5\nstopwords = [\"pub\", \"ctx\", \"read\", \"error\", \"result\", \"handle\", \"thing\", \"mut\", \"ok\", \"let\", \"value\", \"fn\"]\nmessage = \"clean\"\n"
    );
    let (registry, errors) = build_rules(&clean);
    assert!(errors.is_empty(), "{errors:?}");
    assert!(
        registry.run(&ctx, &no_overrides()).findings.is_empty(),
        "停用词清掉样板后不应报"
    );
}

#[test]
fn bad_params_fail_at_build_time() {
    let bad = r#"
[kinds.duplicate-file.bad]
paths_include = ["**/*.rs"]
min_similarity = 1.5
message = "x"
"#;
    let (_, errors) = build_rules(bad);
    assert!(
        errors.iter().any(|e| e.contains("min_similarity")),
        "{errors:?}"
    );

    let unknown = r#"
[kinds.duplicate-file.unknown]
paths_include = ["**/*.rs"]
messages = "typo"
"#;
    let (_, errors) = build_rules(unknown);
    assert!(!errors.is_empty(), "未知参数键必须报错：{errors:?}");
}
