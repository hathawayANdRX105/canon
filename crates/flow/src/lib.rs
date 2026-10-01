//! flow — 工作流程记录 + 任务领取：`Project → Task(state) → Step` + Journal。
//!
//! 状态层是**每项目自定义管道**（`states_text` 有序数组）；转移规则
//! （一般版）：前进 = 只许下一 state，后退 = 任意前序 state（打回），
//! 完成 = 到达末 state 且 step 全 done。step 二态（done/未 done），spec
//! step 标 done 必须挂 evidence。
//!
//! 纯模型 + 转移规则 + 模板在本 crate 根（零 DB 依赖，wasm/web 直接拉）；
//! SQLite 持久层在 `store` 模块（feature `store`，默认开），spec 执行壳在
//! `spec`（chdir 进目标仓 + 同进程调 `gate::engine::run_named_in`）。

#[cfg(feature = "serve")]
pub mod serve;
#[cfg(feature = "store")]
mod spec;
#[cfg(feature = "store")]
pub mod store;
pub mod templates;
#[cfg(feature = "store")]
pub mod tools;

#[cfg(feature = "store")]
use std::path::PathBuf;
#[cfg(feature = "store")]
use std::time::SystemTime;
#[cfg(feature = "store")]
use std::time::UNIX_EPOCH;
#[cfg(feature = "store")]
pub use store::Store;
#[cfg(feature = "store")]
pub(crate) use store::db_err;

use serde::Serialize;
use serde_json::Value;

/// 稳定错误码：MCP 层据此给 agent 结构化错误（`-32000` + `{code}: {msg}`）。
pub struct FlowError {
    pub code: &'static str,
    pub msg: String,
}

impl FlowError {
    pub fn new(code: &'static str, msg: impl Into<String>) -> Self {
        Self {
            code,
            msg: msg.into(),
        }
    }
}

impl std::fmt::Debug for FlowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlowError({}: {})", self.code, self.msg)
    }
}

// ---------------------------------------------------------------------------
// 模型
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub repo_path: String,
    pub kind: String,
    pub spec_dir: Option<String>,
    pub states: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
pub type FlowResult<T> = Result<T, FlowError>;
#[derive(Debug, Clone, Serialize)]
pub struct Task {
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub state: String,
    pub claimant: Option<String>,
    pub claim_ts: Option<i64>,
    pub priority: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Step {
    pub id: i64,
    pub task_id: i64,
    pub seq: i64,
    pub title: String,
    /// `work` / `spec` / `record`
    pub kind: String,
    /// 二态：未 done / done（无 doing/skipped）
    pub done: bool,
    pub completed_at: Option<i64>,
    /// 该 step 归属的 state 栏（项目管道里没有该 state 时 P2 归入当前栏）
    pub state_hint: Option<String>,
    /// spec step 绑定的 checklist 名
    pub spec_ref: Option<String>,
    /// spec 步骤的完成证据（spec_run 结果快照）
    pub evidence: Option<String>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct EventRow {
    pub id: i64,
    pub task_id: i64,
    pub ts: i64,
    /// 事件发生时 task 所在的 state（P2 归栏；transition 事件记**到达**的 state）
    pub state_at: String,
    /// `transition` / `step` / `spec` / `note`
    pub kind: String,
    pub payload: Value,
    pub actor: String,
}

/// task_get 出参：task + 所属 project + steps + 当前合法转移列表。
#[derive(Debug, Serialize)]
pub struct TaskBundle {
    pub task: Task,
    pub project: Project,
    pub steps: Vec<Step>,
    pub allowed_transitions: Vec<String>,
}

/// 合法转移：前进只许下一 state；后退任意前序 state（打回）。
pub fn allowed_transitions(states: &[String], cur: &str) -> Vec<String> {
    let Some(i) = states.iter().position(|s| s == cur) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    if i + 1 < states.len() {
        out.push(states[i + 1].clone());
    }
    out.extend(states.iter().take(i).cloned());
    out
}

// ---------------------------------------------------------------------------
// 小工具
// ---------------------------------------------------------------------------
#[cfg(feature = "store")]
fn known_kind(kind: &str) -> bool {
    matches!(kind, "backend" | "frontend" | "general")
}
/// 管道必须是去重非空：重复 state 会让「前进只许下一态」失去意义。
#[cfg(feature = "store")]
fn validate_states(states: &[String]) -> FlowResult<Vec<String>> {
    if states.is_empty() {
        return Err(FlowError::new("BAD_INPUT", "states must be non-empty"));
    }
    let mut seen = std::collections::HashSet::new();
    for s in states {
        if s.is_empty() || !seen.insert(s.as_str()) {
            return Err(FlowError::new(
                "BAD_INPUT",
                format!("states must be non-empty and unique (problem at {s})"),
            ));
        }
    }
    Ok(states.to_vec())
}

#[cfg(feature = "store")]
fn parse_states(text: &str) -> Vec<String> {
    serde_json::from_str(text).unwrap_or_default()
}

#[cfg(feature = "store")]
fn serde_json_states(states: &[String]) -> String {
    serde_json::to_string(states).unwrap_or_else(|_| "[]".into())
}

#[cfg(feature = "store")]
fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(feature = "store")]
fn home_dir() -> PathBuf {
    std::env::var("HOME").map(PathBuf::from).unwrap_or_default()
}
