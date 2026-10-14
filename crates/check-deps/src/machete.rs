//! 未使用依赖：cargo-machete 输出翻译成 finding（advisory 桶里的集成规则）。
//! 对应 canon dep_hygiene 的意图；machete 缺失时静默（canon 口径：工具缺失
//! 规则可选跳过），不产伪 finding。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

pub struct UnusedDeps;

impl Rule for UnusedDeps {
    fn id(&self) -> &str {
        "DEPS.unused-deps"
    }

    fn class(&self) -> Class {
        Class::Advisory
    }

    fn title(&self) -> &str {
        "未使用依赖（cargo-machete）"
    }

    fn doc(&self) -> &str {
        "运行 cargo machete 翻译其输出：未被引用的依赖拖慢编译、扩大供应链面。\
         需要本机安装 cargo-machete，缺失时本规则不产 finding。"
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let output = match std::process::Command::new("cargo")
            .current_dir(ctx.root())
            .args(["machete"])
            .output()
        {
            Ok(out) => out,
            Err(_) => return Vec::new(),
        };
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut pkg = String::new();
        let mut findings = Vec::new();
        for line in stdout.lines() {
            let trimmed = line.trim_start();
            if line == trimmed {
                // 顶格行是 "crate_name manifest_path:" 记录当前包
                pkg = line
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .trim_end_matches(':')
                    .to_string();
            } else if let Some(dep) = trimmed.strip_prefix("-- ") {
                findings.push(Finding::new(
                    self.id(),
                    Severity::Warn,
                    "Cargo.toml",
                    0,
                    format!("crate {pkg} 未使用依赖: {dep}（删除，或加 cargo-machete ignored）"),
                ));
            }
        }
        findings
    }
}
