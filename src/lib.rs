//! canon — spec-driven quality gate + MCP spec server，一个二进制。
//!
//! 两个模块：
//!
//! - [`gate`]：全部实现，占 `src/gate/`。`engine` 是唯一执行面（零检测逻辑，
//!   每条规则都是 `checklist_*.yaml` + 外部 harness 命令），`catalog` 收 spec
//!   树上的规则清单，`shared` 放 Finding 契约与 gh api client，`rules` /
//!   `tools` 是 gh-workflow 政策层（issue/PR/review 合规、merge 编排、gh 拦截）。
//! - [`mcp`]：JSON-RPC 2.0 over stdio 单文件，把 `spec_catalog` /
//!   `spec_explain` / `preflight` 三个工具暴露给 agent。
//! - [`cli`]：clap 命令行定义，bin 只做路由。

pub mod cli;
pub mod gate;
pub mod mcp;
