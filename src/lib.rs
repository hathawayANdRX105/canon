//! canon — spec-driven quality gate + MCP spec server.
//!
//! `engine`: checklist runner — the only execution surface, zero detection
//! logic (every rule is a `checklist_*.yaml` + external harness).
//! `rules` / `tools`: gh-workflow policy checks (issue/PR/review compliance,
//! `shared`: Finding contract, severity overrides, gh api client.
//! `catalog`: rule inventory over the spec tree — id, severity, why.
//! `mcp`: JSON-RPC 2.0 stdio surface exposing the catalog and a preflight run.

pub mod catalog;
pub mod engine;
pub mod flow;
pub mod mcp;
pub mod rules;
pub mod shared;
pub mod tools;
