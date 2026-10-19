//! CLI 集成测试：库 crate 不带 bin，CLI 行为（list / explain / check）由本测试驱动。
//!
//! 覆盖点：参数解析与子命令路由（list/explain/check 的标志组合）、未知规则 exit 2、
//! 以及 `--canon` 与 agent 口径的退出码差异（10 vs 0）。规则判定本身由各 kind/域
//! crate 的 probe 测试覆盖，这里只测 CLI 契约。

use std::path::Path;

use check_core::cli::{self, Frontend};
use check_core::context::Ctx;
use check_core::finding::{Class, Finding};
use check_core::kinds::builtin_factories;
use check_core::rule::Rule;

/// text-pattern 实例：**裸串** 写法避免多层转义踩坑（regex `dbg!\(` 在 TOML 里
/// 必须是 `"dbg!\\("`，用 r#"…"# 保持可读）。
const CHECKS_TOML: &str = r#"[kinds.text-pattern.no-dbg]
paths_include = ["**/*.rs"]
patterns_deny = ["dbg!\\("]
message = "禁止 dbg! 残留"
"#;

fn argv(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

fn write(root: &Path, rel: &str, body: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, body).unwrap();
}

/// check 走 `git ls-files` 取跟踪集，fixture 必须是真 git 仓（与实仓同路径）。
fn init_repo(root: &Path) {
    for args in [vec!["init", "-q"], vec!["add", "-A"]] {
        let out = std::process::Command::new("git")
            .current_dir(root)
            .args(&args)
            .output();
        assert!(out.is_ok_and(|o| o.status.success()), "git {args:?} failed");
    }
}

/// 最小仓：checks.toml（text-pattern 实例）+ 一个命中文件；真 git 仓。
fn fixture_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    write(dir.path(), "checks.toml", CHECKS_TOML);
    write(dir.path(), "bad.rs", "fn f() {\n    dbg!(1);\n}\n");
    init_repo(dir.path());
    dir
}

fn cfg(dir: &Path) -> String {
    dir.join("checks.toml").display().to_string()
}

fn base_argv(dir: &Path) -> Vec<String> {
    argv(&[
        "checks",
        "--config",
        &cfg(dir),
        "--root",
        &dir.display().to_string(),
    ])
}

/// 假 semantic 规则：无端点时进 notChecked（用于验 CLI 的退出码口径）。
struct FakeSemantic;

impl Rule for FakeSemantic {
    fn id(&self) -> &str {
        "FAKE.semantic"
    }

    fn class(&self) -> Class {
        Class::Semantic
    }

    fn title(&self) -> &str {
        "fake semantic"
    }

    fn doc(&self) -> &str {
        "cli probe"
    }

    fn run(&self, _ctx: &Ctx) -> Vec<Finding> {
        Vec::new()
    }
}

fn frontend_with(static_rules: Vec<Box<dyn Rule>>) -> Frontend {
    Frontend {
        static_rules,
        factories: builtin_factories(),
    }
}

#[test]
fn list_accepts_flags_and_exits_zero() {
    let dir = fixture_repo();
    let mut args = base_argv(dir.path());
    args.push("list".into());
    args.push("--json".into());
    assert_eq!(cli::run_with(frontend_with(vec![]), args), 0);
}

#[test]
fn explain_known_rule_ok_unknown_exits_two() {
    let dir = fixture_repo();
    let mut known = base_argv(dir.path());
    known.extend(["explain".into(), "TEXT-PATTERN.no-dbg".into()]);
    assert_eq!(cli::run_with(frontend_with(vec![]), known), 0);

    let mut unknown = base_argv(dir.path());
    unknown.extend(["explain".into(), "NO-SUCH.rule".into()]);
    assert_eq!(cli::run_with(frontend_with(vec![]), unknown), 2);
}

#[test]
fn check_flags_route_and_agent_mode_exit_codes() {
    let dir = fixture_repo();
    // fixture 有命中（dbg!，WARN 级）→ 不改变退出码；deterministic-only 无
    // notChecked → 0。canon 口径同样 0（rc 收敛 0/2）。
    for canon in [false, true] {
        let mut args = base_argv(dir.path());
        args.extend([
            "check".into(),
            "--scope".into(),
            "all".into(),
            "--class".into(),
            "deterministic".into(),
            "--json".into(),
        ]);
        if canon {
            args.push("--canon".into());
        }
        assert_eq!(
            cli::run_with(frontend_with(vec![]), args),
            0,
            "canon={canon}"
        );
    }
}

#[test]
fn check_path_missing_exits_two_existing_path_exits_zero() {
    // --path 契约：不存在的路径必须 exit 2（不能静默空跑）；
    // 存在的目录/文件正常执行（findings 不改变退出码）。
    let dir = fixture_repo();
    write(dir.path(), "src/deep.rs", "fn h() {\n    dbg!(2);\n}\n");
    init_repo(dir.path());

    let run = |extra: &[&str]| {
        let mut args = base_argv(dir.path());
        args.extend(["check".into(), "--scope".into(), "all".into()]);
        args.extend(extra.iter().map(|s| s.to_string()));
        cli::run_with(frontend_with(vec![]), args)
    };

    assert_eq!(run(&["--path", "no-such.rs", "--json"]), 2);
    assert_eq!(run(&["--path", "src", "--json"]), 0);
    assert_eq!(run(&["--path", "bad.rs", "--json"]), 0);
}

#[test]
fn canon_flag_collapses_not_checked_exit_code() {
    // semantic 规则 + 无端点：agent 口径 rc=10（报告不完整）；--canon 收敛 0
    let dir = tempfile::tempdir().unwrap();
    write(dir.path(), "checks.toml", "");
    init_repo(dir.path());
    let run = |canon: bool| {
        let mut args = base_argv(dir.path());
        args.extend([
            "check".into(),
            "--scope".into(),
            "all".into(),
            "--json".into(),
        ]);
        if canon {
            args.push("--canon".into());
        }
        cli::run_with(frontend_with(vec![Box::new(FakeSemantic)]), args)
    };
    assert_eq!(run(false), 10, "agent 口径：notChecked → 10");
    assert_eq!(run(true), 0, "canon 口径：notChecked 转 INFO finding → 0");
}
