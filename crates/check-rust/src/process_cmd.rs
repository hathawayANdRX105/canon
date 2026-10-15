//! HTTP 调用走 reqwest，不要 subprocess 拉 curl/wget/httpie。
//! 对应 canon rust_no_process_cmd；文本口径（旧行为），跳过注释行（旧版不跳，改进）。
//! 测试 mock server 与工具层包装的例外走 checks.toml 覆盖链（scope_exclude）。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

const PATTERN: &str = "std::process::Command";
const HTTP_CLI: &[&str] = &["\"curl\"", "\"wget\"", "\"httpie\""];

pub struct ProcessCmd;

impl Rule for ProcessCmd {
    fn id(&self) -> &str {
        "RUST.process-cmd"
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "禁 subprocess 拉 HTTP CLI"
    }

    fn doc(&self) -> &str {
        "`std::process::Command` 与 \"curl\"/\"wget\"/\"httpie\" 同行出现即 FAIL——\
         HTTP 调用走 reqwest（超时/代理/TLS 策略统一，subprocess 绕过全部运行时保障）。\
         注释行不误伤；测试 mock server 与工具层包装的例外走 checks.toml 覆盖链。\
         文本口径（与旧 shell 规则一致）：Command 构造形态多样，AST 化收益低。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Fail
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let mut findings = Vec::new();
        for rel in ctx.scope_files() {
            if !crate::is_rust_file(rel) {
                continue;
            }
            let Some(text) = ctx.read(rel).ok() else {
                continue;
            };
            for (idx, line) in text.lines().enumerate() {
                let trimmed = line.trim_start();
                if trimmed.starts_with("//") {
                    continue;
                }
                if line.contains(PATTERN) && HTTP_CLI.iter().any(|cli| line.contains(cli)) {
                    findings.push(Finding::new(
                        self.id(),
                        Severity::Fail,
                        rel.display().to_string(),
                        u32::try_from(idx + 1).unwrap_or(u32::MAX),
                        format!("HTTP 调用走 reqwest，不要 subprocess：{}", line.trim()),
                    ));
                }
            }
        }
        findings
    }
}
