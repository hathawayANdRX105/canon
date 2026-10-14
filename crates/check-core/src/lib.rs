//! check-core：canon 规则库的地基。
//!
//! 职责边界（rules-crates-impl-plan §3）：本 crate 只提供词汇（Finding/Class）、
//! 规则 trait、注册表与运行器、参数化 rule kinds（TOML 多实例）和成员仓 bin 的
//! CLI 胶水；「要不要拦、何时跑、多严重」的策略权在 canon（dispatch /
//! severity_overrides）与成员仓 checks.toml，不在规则代码里。
//!
//! 报告语义（借 jev-code）：只有 findings / notChecked / errors，**没有 pass**；
//! 退出码 0=完整执行、10=报告不完整（有 notChecked）、2=环境或装配不可用。

pub mod cli;
pub mod config;
pub mod context;
pub mod finding;
pub mod kinds;
pub mod registry;
pub mod rule;

pub use finding::{Class, Finding, NotChecked, RuleOutcome, Severity};
pub use registry::{Registry, Report, RunOptions};
pub use rule::Rule;
