//! canon — spec-driven quality gate + MCP spec server + flow workflow
//! recorder，一个二进制。
//!
//! 三个顶层模块对应原来的三个 crate，依赖单向、无环：
//!
//! - [`gate`]：门禁核心。`engine` 是唯一执行面（零检测逻辑，每条规则都是
//!   `checklist_*.yaml` + 外部 harness 命令），`catalog` 收 spec 树上的规则
//!   清单，`shared` 放 Finding 契约与 gh api client，`rules` / `tools` 是
//!   gh-workflow 政策层（issue/PR/review 合规、merge 编排、gh 命令拦截）。
//! - [`flow`]：Project → Task(state) → Step + journal，含 SQLite store 与
//!   `canon serve` 的 HTTP/WebSocket 面板。
//! - [`mcp`]：JSON-RPC 2.0 over stdio，把 spec 工具族与 flow 工具族暴露给 agent。

pub mod flow;
pub mod gate;
pub mod mcp;
