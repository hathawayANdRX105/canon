//! gate — spec-driven quality gate.
//!
//! `engine`: checklist runner — the only execution surface, zero detection
//! logic (every rule is a `checklist_*.yaml` + external harness).
//! `rules` / `tools`: gh-workflow policy checks (issue/PR/review compliance,
//! merge orchestration, gh command interception).
//! `shared`: Finding contract, severity overrides, gh api client.

pub mod engine;
pub mod rules;
pub mod shared;
pub mod tools;
