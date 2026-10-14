//! 退出码契约：0=完整执行；10=有 notChecked（semantic 端点缺失）；2=环境或装配错误。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::registry::{Registry, Report, RunOptions};
use check_core::rule::Rule;

struct FakeSemantic;

impl Rule for FakeSemantic {
    fn id(&self) -> &str {
        "FAKE.semantic"
    }

    fn class(&self) -> Class {
        Class::Semantic
    }

    fn title(&self) -> &str {
        "fake semantic rule"
    }

    fn doc(&self) -> &str {
        "exit-code probe"
    }

    fn run(&self, _ctx: &Ctx) -> Vec<Finding> {
        vec![Finding::new(
            "FAKE.semantic",
            Severity::Warn,
            "dummy.rs",
            1,
            "should be suppressed",
        )]
    }
}

struct FakeDeterministic;

impl Rule for FakeDeterministic {
    fn id(&self) -> &str {
        "FAKE.deterministic"
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "fake deterministic rule"
    }

    fn doc(&self) -> &str {
        "exit-code probe"
    }

    fn run(&self, _ctx: &Ctx) -> Vec<Finding> {
        Vec::new()
    }
}

fn no_overrides() -> RunOptions<'static> {
    RunOptions {
        classes: None,
        semantic_endpoint: None,
        overrides: None,
    }
}

fn registry_with(rules: Vec<Box<dyn Rule>>) -> Registry {
    let (registry, errors) = Registry::build(rules, &[], &Default::default());
    assert!(errors.is_empty(), "{errors:?}");
    registry
}

#[test]
fn semantic_without_endpoint_is_not_checked_and_exit_10() {
    let registry = registry_with(vec![Box::new(FakeSemantic)]);
    let ctx = Ctx::new(".", Vec::new(), None);
    let report = registry.run(&ctx, &no_overrides());
    assert!(report.findings.is_empty(), "semantic run 不该伪装 finding");
    assert_eq!(report.not_checked.len(), 1);
    assert_eq!(report.exit_code(), 10);
}

#[test]
fn deterministic_clean_run_exits_zero() {
    let registry = registry_with(vec![Box::new(FakeDeterministic)]);
    let ctx = Ctx::new(".", Vec::new(), None);
    let report = registry.run(&ctx, &no_overrides());
    assert_eq!(report.checked, vec!["FAKE.deterministic".to_string()]);
    assert_eq!(report.exit_code(), 0);
}

#[test]
fn setup_errors_exit_two() {
    let report = Report {
        errors: vec!["unknown kind: typo".to_string()],
        ..Default::default()
    };
    assert_eq!(report.exit_code(), 2);
}
