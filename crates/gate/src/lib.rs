//! gate — canon 的「审查/门禁」核心：checklist 引擎 + 规则 catalog +
//! gh-workflow 政策层。
//!
//! `engine`：checklist runner——唯一执行面，零检测逻辑（每条规则都是
//! `checklist_*.yaml` + 外部 harness 命令）。
//! `catalog`：spec 树上的规则清单——id、severity、why。
//! `shared`：Finding 契约、severity overrides、gh api client。
//! `rules` / `tools`：gh-workflow 政策检查（issue/PR/review 合规、merge
//! 编排、gh 命令拦截）。

pub mod catalog;
pub mod engine;
pub mod rules;
pub mod shared;
pub mod tools;
