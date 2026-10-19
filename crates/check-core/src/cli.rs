//! 成员仓 bin 的胶水：参数解析、注册表组装、输出与退出码。
//!
//! 退出码：0=完整执行；10=报告不完整（notChecked 非空）；2=环境或装配不可用。
//! 本 CLI 没有 "pass" 概念：只声明查了什么、没查什么、发现了什么。

use std::path::{Path, PathBuf};
use std::process::Command;

use clap::{Parser, Subcommand};

use crate::config::{self, ChecksConfig};
use crate::context::Ctx;
use crate::finding::Class;
use crate::kinds::KindFactory;
use crate::registry::{Registry, RunOptions};
use crate::rule::Rule;

/// 成员仓 bin 交给 CLI 的装配物：静态规则 + kind 工厂。
pub struct Frontend {
    /// 域 crate 组装的静态规则（T1–T8 逐步填充）。
    pub static_rules: Vec<Box<dyn Rule>>,
    /// 参数化 kind 工厂；至少含 [`crate::kinds::builtin_factories`]。
    pub factories: Vec<Box<dyn KindFactory>>,
}

#[derive(Parser)]
#[command(
    name = "checks",
    about = "仓库检查：canon 规则库的成员仓入口（无 pass 语义）"
)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
    /// checks.toml 路径；文件缺失视为空配置
    #[arg(long, global = true, default_value = "checks.toml")]
    config: PathBuf,
    /// 仓库根；缺省取 git toplevel
    #[arg(long, global = true)]
    root: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Cmd {
    /// 列出全部规则（静态 + checks.toml 实例）
    List {
        #[arg(long)]
        json: bool,
    },
    /// 打印规则条文与缺省策略
    Explain { id: String },
    /// 执行检查
    Check {
        /// all = 全部跟踪文件；changed = 相对 --base 的变更文件
        #[arg(long, default_value = "all")]
        scope: String,
        /// changed 口径的基线 rev
        #[arg(long, default_value = "HEAD")]
        base: String,
        /// 只审查该路径（文件或目录，相对仓库根）：跳过 git scope 逻辑，
        /// 文件未跟踪也纳入（审查未提交的新文件）；目录 = 其下全部 .rs 文件
        #[arg(long)]
        path: Option<PathBuf>,
        /// 只跑这些 class（可多次）
        #[arg(long = "class")]
        classes: Vec<String>,
        /// stdout 输出 canon findings JSON 数组（harness 契约）
        #[arg(long)]
        json: bool,
        /// stdout 输出完整报告（findings/notChecked/checked/errors）
        #[arg(long = "report-json")]
        report_json: bool,
        /// canon harness 口径：notChecked 转 INFO finding，退出码收敛 0/2
        #[arg(long)]
        canon: bool,
    },
}

/// 成员仓 main 的唯一入口：`std::process::exit(check_core::cli::run(frontend))`。
pub fn run(frontend: Frontend) -> i32 {
    run_with(frontend, std::env::args())
}

/// 与 [`run`] 同逻辑，但 argv 由调用方给出——集成测试用它驱动 CLI
/// （库 crate 不带 bin；CLI 行为由测试覆盖，见 tests/cli.rs）。
pub fn run_with(frontend: Frontend, args: impl IntoIterator<Item = String>) -> i32 {
    let args = Args::parse_from(args);
    let cfg = match config::load(&args.config) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("checks: config error: {e}");
            return 2;
        }
    };
    let (registry, build_errors) =
        Registry::build(frontend.static_rules, &frontend.factories, &cfg);
    // 装配错误三个子命令都要可见：list 静默吞掉的话，实例化失败的规则
    // 会「悄悄不在 list 里」（2026-10-16 实测：max_len 参数错，semantic 实例
    // 半天没注册进去，list/check 的可见性还不一致）。
    if !build_errors.is_empty() && !matches!(args.cmd, Cmd::Check { .. }) {
        for e in &build_errors {
            eprintln!("checks: {e}");
        }
    }
    match args.cmd {
        Cmd::List { json } => list(&registry, json),
        Cmd::Explain { id } => explain(&registry, &id),
        Cmd::Check {
            scope,
            base,
            path,
            classes,
            json,
            report_json,
            canon,
        } => check(CheckInvocation {
            registry: &registry,
            build_errors: &build_errors,
            cfg: &cfg,
            root: args.root.as_ref(),
            scope: &scope,
            base: &base,
            path: &path,
            classes: &classes,
            json,
            report_json,
            canon,
        }),
    }
}

fn list(registry: &Registry, json: bool) -> i32 {
    if json {
        let rows: Vec<serde_json::Value> = registry
            .rules()
            .iter()
            .map(|r| {
                serde_json::json!({
                    "id": r.id(),
                    "class": r.class(),
                    "title": r.title(),
                    "default_severity": r.default_severity(),
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&rows).unwrap_or_else(|_| "[]".to_string())
        );
        return 0;
    }
    for r in registry.rules() {
        println!(
            "{:<32} {:<14} {}",
            r.id(),
            format!("{:?}", r.class()).to_lowercase(),
            r.title()
        );
    }
    0
}

fn explain(registry: &Registry, id: &str) -> i32 {
    let Some(r) = registry.rule(id) else {
        eprintln!("checks: unknown rule: {id}");
        return 2;
    };
    println!("id:       {}", r.id());
    println!("class:    {:?}", r.class());
    println!("title:    {}", r.title());
    println!("severity: {:?}", r.default_severity());
    println!();
    println!("{}", r.doc());
    0
}

struct CheckInvocation<'a> {
    registry: &'a Registry,
    build_errors: &'a [String],
    cfg: &'a ChecksConfig,
    root: Option<&'a PathBuf>,
    scope: &'a str,
    base: &'a str,
    path: &'a Option<PathBuf>,
    classes: &'a [String],
    json: bool,
    report_json: bool,
    canon: bool,
}

fn check(p: CheckInvocation<'_>) -> i32 {
    if !p.build_errors.is_empty() {
        return fail_setup(p.build_errors);
    }
    let (root, tracked, changed) = match resolve_inputs(p.root, p.scope, p.base, p.path.as_ref()) {
        Ok(parts) => parts,
        Err(msg) => {
            eprintln!("checks: {msg}");
            return 2;
        }
    };
    let classes = match parse_classes(p.classes) {
        Ok(parsed) => parsed,
        Err(msg) => {
            eprintln!("checks: {msg}");
            return 2;
        }
    };
    let ctx = Ctx::new(root, tracked, changed);
    let opts = RunOptions {
        classes: if classes.is_empty() {
            None
        } else {
            Some(classes)
        },
        semantic_endpoint: p.cfg.semantic.as_ref(),
        overrides: Some(&p.cfg.rules),
    };
    emit(&p.registry.run(&ctx, &opts), p.json, p.report_json, p.canon)
}

fn fail_setup(errors: &[String]) -> i32 {
    for e in errors {
        eprintln!("checks: {e}");
    }
    2
}

/// (仓库根, 跟踪文件, 变更集)
type FileInputs = (PathBuf, Vec<PathBuf>, Option<Vec<PathBuf>>);

fn resolve_inputs(
    root: Option<&PathBuf>,
    scope: &str,
    base: &str,
    path: Option<&PathBuf>,
) -> Result<FileInputs, String> {
    let root = match root {
        Some(r) => r.clone(),
        None => resolve_git_root()?,
    };
    let tracked = git_out(Some(&root), &["ls-files", "-z"])
        .map(|out| split_paths(&out))
        .ok_or_else(|| "git ls-files failed".to_string())?;
    // --path 优先：文件 → 单文件（未跟踪也纳入）；目录 → 目录下全部 .rs。
    if let Some(p) = path {
        let selected = resolve_path_target(&root, p, &tracked)?;
        return Ok((root, tracked, Some(selected)));
    }
    let changed = match scope {
        "changed" => Some(
            git_out(Some(&root), &["diff", "--name-only", "-z", base])
                .map(|out| split_paths(&out))
                .ok_or_else(|| format!("git diff failed for base {base}"))?,
        ),
        "all" => None,
        other => return Err(format!("unknown scope {other:?} (all|changed)")),
    };
    Ok((root, tracked, changed))
}

/// `--path` 目标文件集：文件 → 该文件本身；目录 → 该目录下全部 .rs
/// （跟踪文件 + 未跟踪新文件），供按 crate/目录审查。
fn resolve_path_target(
    root: &Path,
    path: &Path,
    tracked: &[PathBuf],
) -> Result<Vec<PathBuf>, String> {
    let rel = normalize_rel(root, path)?;
    let abs = root.join(&rel);
    if abs.is_file() {
        return Ok(vec![rel]);
    }
    if abs.is_dir() {
        let mut selected: Vec<PathBuf> = tracked
            .iter()
            .filter(|t| t.starts_with(&rel))
            .cloned()
            .collect();
        collect_untracked_rs(&abs, &rel, &mut selected)?;
        selected.sort();
        selected.dedup();
        return Ok(selected);
    }
    Err(format!("--path 不存在：{}", rel.display()))
}

/// 把 `--path` 归一化为相对 root 的路径（绝对路径剥掉 root 前缀，去掉 `./`）。
fn normalize_rel(root: &Path, path: &Path) -> Result<PathBuf, String> {
    let abs = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    let rel = abs
        .strip_prefix(root)
        .map_err(|_| format!("--path 必须在仓库根内：{}", path.display()))?;
    let mut clean = PathBuf::new();
    for comp in rel.components() {
        if !matches!(comp, std::path::Component::CurDir) {
            clean.push(comp.as_os_str());
        }
    }
    Ok(clean)
}

/// 收集目录下未跟踪的 .rs 文件（新文件也能被 `--path <crate>` 审查到）。
fn collect_untracked_rs(dir: &Path, rel_dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in std::fs::read_dir(dir).map_err(|e| format!("read_dir {}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| format!("read_dir {}: {e}", dir.display()))?;
        let path = entry.path();
        let rel = rel_dir.join(entry.file_name());
        if path.is_dir() {
            collect_untracked_rs(&path, &rel, out)?;
        } else if path.extension().is_some_and(|e| e == "rs") && !out.contains(&rel) {
            out.push(rel);
        }
    }
    Ok(())
}

fn resolve_git_root() -> Result<PathBuf, String> {
    git_out(None, &["rev-parse", "--show-toplevel"])
        .map(|out| PathBuf::from(out.trim()))
        .ok_or_else(|| "not inside a git repository (pass --root)".to_string())
}

fn parse_classes(raw: &[String]) -> Result<Vec<Class>, String> {
    let mut classes = Vec::new();
    for source in raw {
        let parsed = source.parse::<Class>()?;
        classes.push(parsed);
    }
    Ok(classes)
}

fn emit(report: &crate::registry::Report, json: bool, report_json: bool, canon: bool) -> i32 {
    if report_json {
        println!(
            "{}",
            serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".to_string())
        );
    } else if json {
        // canon 口径：notChecked 收敛成 INFO finding，findings 一份不丢
        let findings = if canon {
            report.canon_findings()
        } else {
            report.findings.clone()
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&findings).unwrap_or_else(|_| "[]".to_string())
        );
    } else {
        print_human(report);
    }
    if canon {
        report.canon_exit_code()
    } else {
        report.exit_code()
    }
}

fn print_human(report: &crate::registry::Report) {
    for f in &report.findings {
        println!(
            "{:<32} {:<5} {}:{}  {}",
            f.id,
            format!("{:?}", f.severity).to_lowercase(),
            f.path,
            f.line,
            f.message
        );
    }
    for n in &report.not_checked {
        println!("NOT-CHECKED {:<25} {}", n.id, n.reason);
    }
    for e in &report.errors {
        eprintln!("checks: {e}");
    }
    println!(
        "checked={} notChecked={} findings={} errors={}",
        report.checked.len(),
        report.not_checked.len(),
        report.findings.len(),
        report.errors.len()
    );
}

fn git_out(dir: Option<&Path>, args: &[&str]) -> Option<String> {
    let mut cmd = Command::new("git");
    if let Some(d) = dir {
        cmd.arg("-C").arg(d);
    }
    cmd.args(args);
    let out = cmd.output().ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

fn split_paths(null_separated: &str) -> Vec<PathBuf> {
    null_separated
        .split('\0')
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(root: &Path, rel: &str, body: &str) {
        let path = root.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, body).unwrap();
    }

    #[test]
    fn path_file_selects_itself_even_when_untracked() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "src/lib.rs", "pub fn f() {}\n");
        let tracked = vec![PathBuf::from("src/lib.rs")];

        // 跟踪文件
        let selected = resolve_path_target(dir.path(), Path::new("src/lib.rs"), &tracked).unwrap();
        assert_eq!(selected, vec![PathBuf::from("src/lib.rs")]);

        // 未跟踪新文件也纳入（审查未提交文件）
        write(dir.path(), "new.rs", "fn n() {}\n");
        let selected = resolve_path_target(dir.path(), Path::new("new.rs"), &tracked).unwrap();
        assert_eq!(selected, vec![PathBuf::from("new.rs")]);
    }

    #[test]
    fn path_directory_collects_tracked_and_untracked_rs() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "crates/foo/src/lib.rs", "pub fn f() {}\n");
        write(dir.path(), "crates/foo/src/bar.rs", "pub fn g() {}\n");
        write(dir.path(), "crates/foo/README.md", "# not rust\n");
        write(dir.path(), "crates/other/src/lib.rs", "pub fn h() {}\n");
        let tracked = vec![
            PathBuf::from("crates/foo/src/lib.rs"),
            PathBuf::from("crates/foo/src/bar.rs"),
            PathBuf::from("crates/other/src/lib.rs"),
        ];

        // 未跟踪的新 .rs 也要进审查面
        write(dir.path(), "crates/foo/src/brand_new.rs", "pub fn n() {}\n");

        let selected = resolve_path_target(dir.path(), Path::new("crates/foo"), &tracked).unwrap();
        let expected = vec![
            PathBuf::from("crates/foo/src/bar.rs"),
            PathBuf::from("crates/foo/src/brand_new.rs"),
            PathBuf::from("crates/foo/src/lib.rs"),
        ];
        assert_eq!(selected, expected, "crates/other 与 README 不得混入");
    }

    #[test]
    fn path_missing_or_outside_root_errors() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "src/lib.rs", "pub fn f() {}\n");
        let tracked = vec![PathBuf::from("src/lib.rs")];

        assert!(resolve_path_target(dir.path(), Path::new("nope.rs"), &tracked).is_err());
        assert!(resolve_path_target(dir.path(), Path::new("/etc"), &tracked).is_err());
    }

    #[test]
    fn normalize_relative_path_strips_dot_and_absolute_prefix() {
        let root = Path::new("/repo");
        assert_eq!(
            normalize_rel(root, Path::new("./crates/foo/src/lib.rs")).unwrap(),
            PathBuf::from("crates/foo/src/lib.rs")
        );
        assert_eq!(
            normalize_rel(root, Path::new("/repo/crates/foo/src/lib.rs")).unwrap(),
            PathBuf::from("crates/foo/src/lib.rs")
        );
    }
}
