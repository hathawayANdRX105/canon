//! 注册表与运行器：组装静态规则与 checks.toml 实例，产出 Report。

use std::collections::BTreeMap;

use serde::Serialize;

use crate::config::{ChecksConfig, RuleOverride};
use crate::context::Ctx;
use crate::finding::{Class, Finding, Severity};
use crate::kinds::KindFactory;
use crate::rule::Rule;

pub struct Registry {
    rules: Vec<Box<dyn Rule>>,
}

impl Registry {
    /// 组装注册表：静态规则 + kind 实例。装配错误（未知 kind / 参数不合法）
    /// 返回给调用方，check 阶段 exit 2。
    pub fn build(
        static_rules: Vec<Box<dyn Rule>>,
        factories: &[Box<dyn KindFactory>],
        config: &ChecksConfig,
    ) -> (Registry, Vec<String>) {
        let mut rules = static_rules;
        let mut errors = Vec::new();
        for (kind, instances) in &config.kinds {
            let Some(factory) = factories.iter().find(|f| f.kind() == kind.as_str()) else {
                errors.push(format!("unknown kind: {kind}"));
                continue;
            };
            for (name, params) in instances {
                match factory.instantiate(name, params) {
                    Ok(rule) => rules.push(rule),
                    Err(e) => errors.push(format!("kind {kind} instance {name}: {e}")),
                }
            }
        }
        (Registry { rules }, errors)
    }

    pub fn rules(&self) -> &[Box<dyn Rule>] {
        &self.rules
    }

    pub fn rule(&self, id: &str) -> Option<&dyn Rule> {
        self.rules.iter().find(|r| r.id() == id).map(|r| r.as_ref())
    }

    pub fn run(&self, ctx: &Ctx, opts: &RunOptions<'_>) -> Report {
        let mut report = Report::default();
        for rule in &self.rules {
            if let Some(classes) = &opts.classes
                && !classes.contains(&rule.class())
            {
                continue;
            }
            let override_cfg = opts.overrides.and_then(|m| m.get(rule.id()));
            if rule.class() == Class::Semantic && opts.semantic_endpoint.is_none() {
                report.not_checked.push(NotChecked {
                    id: rule.id().to_string(),
                    reason: "semantic endpoint not configured".to_string(),
                });
                continue;
            }
            let effective = override_cfg
                .and_then(|o| o.severity)
                .unwrap_or_else(|| rule.default_severity());
            // 策略层 scope 覆盖（checks.toml [rules.<id>] scope_include/exclude）
            let scoped_ctx = match override_cfg {
                Some(o) if !o.scope_include.is_empty() || !o.scope_exclude.is_empty() => {
                    ctx.scoped(&o.scope_include, &o.scope_exclude)
                }
                _ => ctx.clone(),
            };
            report.checked.push(rule.id().to_string());
            report
                .findings
                .extend(rule.run(&scoped_ctx).into_iter().map(|mut f| {
                    f.severity = effective;
                    f
                }));
        }
        report
    }
}

pub struct RunOptions<'a> {
    /// None = 全部 class。
    pub classes: Option<Vec<Class>>,
    /// semantic 类的决策端点；None = semantic 类 notChecked。
    pub semantic_endpoint: Option<&'a str>,
    /// checks.toml 的 [rules.<id>] 覆盖。
    pub overrides: Option<&'a BTreeMap<String, RuleOverride>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotChecked {
    pub id: String,
    pub reason: String,
}

/// 检查报告。findings 遵循 canon 契约；checked / not_checked / errors 支撑
/// 「无 pass 语义」——只声明查了什么、没查什么、发现了什么。
#[derive(Debug, Default, Serialize)]
pub struct Report {
    pub findings: Vec<Finding>,
    pub checked: Vec<String>,
    pub not_checked: Vec<NotChecked>,
    pub errors: Vec<String>,
}

impl Report {
    /// 0=完整执行；10=报告不完整（有 notChecked）；2=环境或装配不可用。
    pub fn exit_code(&self) -> i32 {
        if !self.errors.is_empty() {
            2
        } else if !self.not_checked.is_empty() {
            10
        } else {
            0
        }
    }

    /// canon harness 兼容口径：引擎只认 rc 0（解析 findings）/2（WARN 跳过），
    /// 因此 notChecked 以 INFO finding 表达、退出码收敛为 0/2。
    pub fn canon_exit_code(&self) -> i32 {
        if self.errors.is_empty() { 0 } else { 2 }
    }

    /// canon 口径的 findings：真实发现 + notChecked 的 INFO 占位（不丢信息）。
    pub fn canon_findings(&self) -> Vec<Finding> {
        let mut findings = self.findings.clone();
        if !self.not_checked.is_empty() {
            let detail = self
                .not_checked
                .iter()
                .map(|n| format!("{}: {}", n.id, n.reason))
                .collect::<Vec<_>>()
                .join("; ");
            findings.push(Finding::new(
                "CHECKS-NOTCHECKED",
                Severity::Info,
                ".",
                0,
                format!("本次未执行的检查（端点/工具缺失，非通过）：{detail}"),
            ));
        }
        findings
    }
}
