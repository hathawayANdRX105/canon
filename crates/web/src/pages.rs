//! 任务页（phase → step）+ 项目概览页。
//!
//! 任务页结构：顶部 state 管道条（当前 phase 高亮 + 前进/打回），
//! 下面按 `Project.states` 渲染 N 个纵向 phase 面板；每个面板内含
//! ① 该 phase 归属的 step 列表（`state_hint == phase`）② 该 phase 的动作
//! 时间线（journal `state_at == phase`）。step 归属校验在 store 层兜底：
//! `state_hint` 不在管道内的 step 归当前 phase（见 flow store.rs:678）。
//!
//! 信号读法：视图里一律用 `signal()`（响应式读）——`peek()` 不注册依赖，
//! `.set()` 后不触发重渲染（P2 卡「…」的根因）。

use dioxus::events::MouseEvent;
use dioxus::prelude::*;
use ui_kit::primitive::badge::BadgeVariant;
use ui_kit::primitive::Badge;

use crate::api::{self, ProjectPanel, StepRow};

// ---------------------------------------------------------------------------
// 任务页
// ---------------------------------------------------------------------------

/// 单任务页：phase 纵列 + 每列内 step 列表 + 时间线。
#[component]
pub fn TaskPage(task: i64, reload: Signal<u32>) -> Element {
    let bundle = use_signal(|| None::<api::TaskBundle>);
    let events = use_signal(Vec::<api::JournalEvent>::new);
    let err = use_signal(String::new);

    use_effect(move || {
        let _dep = reload();
        let tid = task.to_string();
        let (mut b, mut ev, mut e) = (bundle, events, err);
        spawn(async move {
            match api::task_bundle(&tid).await {
                Ok(v) => b.set(Some(v)),
                Err(msg) => e.set(msg),
            }
            if let Ok(j) = api::journal(&tid, 50).await {
                ev.set(j);
            }
        });
    });

    let b = bundle();
    rsx! {
        div {
            if !err().is_empty() {
                p { class: "text-destructive text-sm", "{err()}" }
            }
            if let Some(b) = b {
                TaskHeader { bundle: b.clone() }
                PhaseColumns { bundle: b.clone(), events: events() }
            } else {
                div { class: "text-muted-foreground", "…" }
            }
        }
    }
}

/// 任务头部：标题 + 管道条（当前 phase 高亮）+ 前进/打回按钮。
#[component]
fn TaskHeader(bundle: api::TaskBundle) -> Element {
    let flash = use_signal(String::new);
    let tid = bundle.task.id;
    let states = bundle.project.states.clone();
    let pos = states
        .iter()
        .position(|s| s == &bundle.task.state)
        .unwrap_or(0);
    let next = states.get(pos + 1).cloned();
    let back: Vec<String> = states.iter().take(pos).cloned().collect();
    let done = bundle.steps.iter().filter(|s| s.done).count();
    let total = bundle.steps.len();
    let claimant = match &bundle.task.claimant {
        Some(c) => format!("@{c}"),
        None => String::from("未领取"),
    };

    rsx! {
        div { class: "mb-4",
            div { class: "flex items-center gap-2 mb-2 flex-wrap",
                h1 { class: "role-title", "{bundle.task.title}" }
                Badge { variant: BadgeVariant::Outline, "{bundle.task.state}" }
                span { class: "ui-type-label", "{done}/{total} step" }
                span { class: "ui-type-label", "{claimant}" }
                div { class: "ml-auto flex items-center gap-1",
                    if bundle.task.claimant.is_none() {
                        button {
                            class: "ui-btn-ghost text-xs",
                            onclick: move |_: MouseEvent| {
                                spawn_tool(
                                    "task_claim",
                                    &serde_json::json!({
                                        "task": tid.to_string(),
                                        "claimant": "me",
                                    }),
                                    flash,
                                );
                            },
                            "领取"
                        }
                    }
                    if let Some(to) = next {
                        button {
                            class: "ui-btn-primary text-xs",
                            onclick: move |_: MouseEvent| {
                                spawn_tool(
                                    "task_transition",
                                    &serde_json::json!({
                                        "task": tid.to_string(),
                                        "to": to,
                                    }),
                                    flash,
                                );
                            },
                            "→{to}"
                        }
                    }
                    for b in back {
                        button {
                            class: "ui-btn-ghost text-xs",
                            onclick: move |_: MouseEvent| {
                                spawn_tool(
                                    "task_transition",
                                    &serde_json::json!({
                                        "task": tid.to_string(),
                                        "to": b,
                                    }),
                                    flash,
                                );
                            },
                            "↩{b}"
                        }
                    }
                }
            }
            // 管道条
            div { class: "ui-pipeline",
                for (i, s) in states.iter().enumerate() {
                    span {
                        class: "ui-pipeline-node",
                        class: if i == pos { "ui-pipeline-node-current" } else { "" },
                        "{s}"
                    }
                    if i + 1 < states.len() {
                        span { class: "text-muted-foreground text-xs", "›" }
                    }
                }
            }
            if !flash().is_empty() {
                span { class: "text-xs text-destructive", "{flash()}" }
            }
        }
    }
}

/// phase 纵列：每列 = 该 phase 的 step 列表 + 该 phase 动作时间线。
#[component]
fn PhaseColumns(bundle: api::TaskBundle, events: Vec<api::JournalEvent>) -> Element {
    rsx! {
        // phase 纵向面板：一排纵向 kanban 列（超宽横滚）
        div { class: "flex gap-3 overflow-x-auto pb-2",
            for state in &bundle.project.states {
                PhaseColumn {
                    bundle: bundle.clone(),
                    state: state.clone(),
                    events: events.clone(),
                }
            }
        }
    }
}

/// 单 phase 面板：本 phase 的 step（`state_hint` 命中或无归属）＋ 动作时间线。
#[component]
fn PhaseColumn(bundle: api::TaskBundle, state: String, events: Vec<api::JournalEvent>) -> Element {
    let flash = use_signal(String::new);
    let current = bundle.task.state == state;
    // 本 phase 归属的 step：state_hint 命中本 phase 的；无 hint 的只归当前 phase
    // （对齐 store 兜底：hint 不在管道里 → 归当前栏）。
    let steps: Vec<StepRow> = bundle
        .steps
        .iter()
        .filter(|s| match &s.state_hint {
            Some(h) => h == &state,
            None => current,
        })
        .cloned()
        .collect();
    let done = steps.iter().filter(|s| s.done).count();
    let phase_events: Vec<&api::JournalEvent> =
        events.iter().filter(|e| e.state_at == state).collect();

    rsx! {
        div {
            class: "ui-state-panel w-72 shrink-0",
            class: if current { "border-primary" } else { "" },
            div { class: "flex items-center gap-2 mb-2",
                span { class: "role-title", "{state}" }
                if current {
                    Badge { variant: BadgeVariant::Primary, "当前" }
                }
                span { class: "ml-auto ui-type-label", "{done}/{steps.len()} step" }
            }
            // step 列表
            div {
                for s in &steps {
                    StepRowView {
                        bundle: bundle.clone(),
                        step: s.clone(),
                    }
                }
                if steps.is_empty() {
                    p { class: "text-muted-foreground text-xs", "（无 step）" }
                }
            }
            // 动作时间线
            if !phase_events.is_empty() {
                p { class: "ui-type-label mt-2 mb-1", "动作" }
                for e in &phase_events {
                    div { class: "ui-todo-row",
                        span { class: "ui-type-label w-14 shrink-0", "{ts_short(e.ts)}" }
                        span { class: "text-xs flex-1", "{event_label(e)}" }
                        span { class: "ui-type-label", "{e.actor}" }
                    }
                }
            }
            if !flash().is_empty() {
                p { class: "text-xs text-destructive", "{flash()}" }
            }
        }
    }
}

/// step 行：work 直接勾（step_mark）；spec 需先 spec_run 出 evidence 才可勾。
#[component]
fn StepRowView(bundle: api::TaskBundle, step: StepRow) -> Element {
    let flash = use_signal(String::new);
    let tid = bundle.task.id;
    let step_id = step.id;
    let project = bundle.project.name.clone();
    let is_spec = step.kind == "spec";
    let ready = !is_spec || step.evidence.is_some();

    rsx! {
        div { class: "ui-todo-row",
            button {
                class: "ui-check",
                class: if step.done { "opacity-30" } else { "" },
                disabled: step.done || !ready,
                title: if !ready { "先运行 spec（spec step 需 evidence）" } else { "" },
                onclick: move |_: MouseEvent| {
                    spawn_tool(
                        "step_mark",
                        &serde_json::json!({
                            "task": tid.to_string(),
                            "step": step_id,
                            "evidence": "ui-check",
                        }),
                        flash,
                    );
                },
                "✓"
            }
            span {
                class: "text-sm flex-1",
                class: if step.done { "line-through text-muted-foreground" } else { "" },
                "{step.title}"
            }
            if step.kind != "work" {
                Badge {
                    variant: if is_spec { BadgeVariant::Secondary } else { BadgeVariant::Outline },
                    "{step.kind}"
                }
            }
            if let Some(r) = &step.spec_ref {
                span { class: "ui-type-label", "{r}" }
                if is_spec && !ready {
                    button {
                        class: "ui-btn-ghost text-xs",
                        title: "spec_run（结果入 journal，可作 evidence）",
                        onclick: move |_: MouseEvent| {
                            spawn_tool(
                                "spec_run",
                                &serde_json::json!({
                                    "project": project.clone(),
                                    "task": tid.to_string(),
                                    "step": step_id,
                                }),
                                flash,
                            );
                        },
                        "运行"
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 项目概览（未选任务时的落地页）
// ---------------------------------------------------------------------------

/// 项目概览：项目面板 + task 卡列表，点卡回传 task id 给上层切任务页。
#[component]
pub fn ProjectPage(reload: Signal<u32>, on_open_task: EventHandler<i64>) -> Element {
    let nav = use_signal(|| None::<api::BoardView>);
    let err = use_signal(String::new);

    use_effect(move || {
        let _dep = reload();
        let (mut n, mut e) = (nav, err);
        spawn(async move {
            match api::board().await {
                Ok(v) => n.set(Some(v)),
                Err(msg) => e.set(msg),
            }
        });
    });

    let n = nav();
    rsx! {
        div {
            if !err().is_empty() {
                p { class: "text-destructive text-sm", "{err()}" }
            }
            if let Some(v) = n {
                if v.projects.is_empty() {
                    div { class: "ui-state-panel max-w-md",
                        h2 { class: "role-title mb-2", "还没有 project" }
                        p { class: "text-muted-foreground text-sm",
                            "用 MCP 工具 project_create 建一个，或在 sidebar 里选项目下的任务。"
                        }
                    }
                } else {
                    div { class: "space-y-4",
                        for p in &v.projects {
                            ProjectCard {
                                panel: p.clone(),
                                on_open: on_open_task,
                            }
                        }
                    }
                }
            } else {
                div { class: "text-muted-foreground", "加载…" }
            }
        }
    }
}

/// 项目卡：项目头 + task 列表（点 task → 切任务页）。
#[component]
fn ProjectCard(panel: ProjectPanel, on_open: EventHandler<i64>) -> Element {
    rsx! {
        div { class: "ui-state-panel",
            div { class: "flex items-center gap-2 mb-2",
                h2 { class: "role-title", "{panel.name}" }
                Badge { variant: BadgeVariant::Secondary, "{panel.kind}" }
                div { class: "ml-auto flex items-center gap-1",
                    for s in &panel.states {
                        span { class: "ui-pipeline-node", "{s}" }
                    }
                }
                span { class: "ui-type-label", "{panel.tasks.len()} task" }
            }
            div {
                if panel.tasks.is_empty() {
                    p { class: "text-muted-foreground text-sm", "空项目" }
                } else {
                    for t in panel.tasks.clone() {
                        div {
                            class: "ui-task-card cursor-pointer",
                            onclick: move |_: MouseEvent| on_open.call(t.id),
                            div { class: "flex items-center gap-2",
                                Badge { variant: BadgeVariant::Outline, "{t.state}" }
                                span { class: "text-sm font-medium flex-1", "{t.title}" }
                                span { class: "ui-type-label", "{t.steps_done}/{t.steps_total}" }
                                span { class: "ui-type-label",
                                    if t.claimant.is_some() { "@有人" } else { "未领取" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 共享
// ---------------------------------------------------------------------------

/// 后台跑工具；出错写进 `flash` 信号。
fn spawn_tool(name: &str, args: &serde_json::Value, flash: Signal<String>) {
    let name = name.to_string();
    let args = args.clone();
    let mut flash = flash;
    spawn(async move {
        if let Err(msg) = api::tool(&name, &args).await {
            flash.set(msg);
        }
    });
}

/// journal 事件 → 短标签（时间线展示用）。
fn event_label(e: &api::JournalEvent) -> String {
    let p = &e.payload;
    match e.kind.as_str() {
        "transition" => {
            let to = p.get("to").and_then(|v| v.as_str()).unwrap_or("?");
            let reason = p.get("reason").and_then(|v| v.as_str()).unwrap_or("");
            if reason.is_empty() {
                format!("→ {to}")
            } else {
                format!("→ {to}（{reason}）")
            }
        }
        "step" => {
            let action = p.get("action").and_then(|v| v.as_str()).unwrap_or("");
            let title = p.get("title").and_then(|v| v.as_str()).unwrap_or("?");
            match action {
                "done" => format!("✓ {title}"),
                "add" => format!("+ {title}"),
                "bind" => {
                    let r = p.get("spec_ref").and_then(|v| v.as_str()).unwrap_or("?");
                    format!("绑 spec {r}")
                }
                _ => action.to_string(),
            }
        }
        "note" => {
            if let Some(text) = p.get("text").and_then(|v| v.as_str()) {
                text.to_string()
            } else {
                let action = p.get("action").and_then(|v| v.as_str()).unwrap_or("");
                match action {
                    "claim" => {
                        let c = p.get("claimant").and_then(|v| v.as_str()).unwrap_or("?");
                        format!("领取 @{c}")
                    }
                    "create" => {
                        let t = p.get("title").and_then(|v| v.as_str()).unwrap_or("?");
                        format!("建 task {t}")
                    }
                    "migrate_state" => {
                        let f = p.get("from").and_then(|v| v.as_str()).unwrap_or("?");
                        let t = p.get("to").and_then(|v| v.as_str()).unwrap_or("?");
                        format!("迁 state {f} → {t}")
                    }
                    _ => action.to_string(),
                }
            }
        }
        "spec" => {
            let n = p.get("checklist").and_then(|v| v.as_str());
            match n {
                Some(n) => format!("spec {n}"),
                None => "spec".into(),
            }
        }
        _ => p.to_string(),
    }
}

/// ts（unix 秒）→ 短展示（HH:MM:SS）。
fn ts_short(ts: i64) -> String {
    let s = ts % 86400;
    format!("{:02}:{:02}:{:02}", s / 3600, (s % 3600) / 60, s % 60)
}
