//! web 数据层收口：REST（gloo-net fetch）+ WS 推送（web_sys WebSocket）。
//! UI 只碰 signals；API base 缺省 127.0.0.1:10081（`canon serve`）。

use gloo_net::http::Request;
use serde::Deserialize;

/// `canon serve` 默认端口（10081；web UI 在 10080）。
pub fn api_base() -> &'static str {
    "http://127.0.0.1:10081"
}

// ---------------------------------------------------------------------------
// wire 类型（镜像 canon API 的 JSON；flow 模型是 Serialize-only，web 侧独立解）
// ---------------------------------------------------------------------------

#[derive(Deserialize, Clone, PartialEq)]
pub struct BoardView {
    pub projects: Vec<ProjectPanel>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct ProjectPanel {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub states: Vec<String>,
    pub tasks: Vec<TaskCard>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct TaskCard {
    pub id: i64,
    pub title: String,
    pub state: String,
    pub claimant: Option<String>,
    pub priority: i64,
    pub steps_done: i64,
    pub steps_total: i64,
    pub updated_at: i64,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct TaskBundle {
    pub task: TaskFull,
    pub project: Project,
    pub steps: Vec<StepRow>,
    pub allowed_transitions: Vec<String>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct TaskFull {
    pub id: i64,
    pub title: String,
    pub state: String,
    pub claimant: Option<String>,
    pub priority: i64,
    pub updated_at: i64,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct StepRow {
    pub id: i64,
    pub seq: i64,
    pub title: String,
    pub kind: String,
    pub done: bool,
    pub state_hint: Option<String>,
    pub spec_ref: Option<String>,
    pub evidence: Option<String>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct JournalEvent {
    pub task_id: i64,
    pub ts: i64,
    pub state_at: String,
    pub kind: String,
    pub payload: serde_json::Value,
    pub actor: String,
}

/// GET /api/projects 出参（flow 的 Project 投影）。
#[derive(Deserialize, Clone, PartialEq)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub states: Vec<String>,
}

// ---------------------------------------------------------------------------
// REST
// ---------------------------------------------------------------------------

async fn get<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T, String> {
    let url = format!("{}{path}", api_base());
    Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<T>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn board() -> Result<BoardView, String> {
    get("/api/board").await
}

pub async fn task_bundle(task: &str) -> Result<TaskBundle, String> {
    get(&format!("/api/task/{task}")).await
}

pub async fn journal(task: &str, limit: usize) -> Result<Vec<JournalEvent>, String> {
    get(&format!("/api/journal/{task}?limit={limit}")).await
}

/// 任意写/规范工具（与 MCP 同一 16 工具面）；4xx 返回 Err(body)。
pub async fn tool(name: &str, args: &serde_json::Value) -> Result<String, String> {
    let url = format!("{}/api/tools/{name}", api_base());
    let resp = Request::post(&url)
        .json(args)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let text = resp.text().await.map_err(|e| e.to_string())?;
    if resp.status() >= 400 {
        return Err(text);
    }
    Ok(text)
}

// ---------------------------------------------------------------------------
