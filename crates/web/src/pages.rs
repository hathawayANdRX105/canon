//! P1 全部任务页 + P2 项目页（E.2 口径：项目面板 + task 卡 /
//! state 纵栏 + 时间线 + 未完成 todo；业务视图 = 原生 div + ui-* 类）。
//!
//! dioxus 0.7 组件不支持生命周期 prop → 子组件一律 owned 数据（wire 皆 Clone）；
//! rsx 内避免 `match`（解析不稳），用 `if let` + `if/else`；
//! 信号取 owned 用 `.peek().clone()`。

use dioxus::events::MouseEvent;
use dioxus::prelude::*;
use ui_kit::primitive::badge::BadgeVariant;
use ui_kit::primitive::Badge;

use crate::api::{self, BoardView, P2Data, ProjectPanel, StepRow};

// ---------------------------------------------------------------------------
// P1 全部任务页
// ---------------------------------------------------------------------------

#[component]
pub fn AllTasks(reload: Signal<u32>) -> Element {
    let data = use_signal(|| None::<BoardView>);
    let err = use_signal(String::new);

    use_effect(move || {
        let _dep = reload(); // reload bump → 重取
        let (mut d, mut e) = (data, err);
        spawn(async move {
            match api::board().await {
                Ok(view) => {
                    d.set(Some(view));
                    e.set(String::new());
                }
                Err(msg) => e.set(msg),
            }
        });
    });

    let board = data(); // 响应式读（.set 触发重渲染）
    rsx! {
        div {
            if !err().is_empty() {
                p { class: "text-destructive text-sm", "{err()}" }
            }
            if let Some(view) = board {
                if view.projects.is_empty() {
                    div { class: "ui-state-panel max-w-md",
                        h2 { class: "role-title mb-2", "还没有 project" }
                        p { class: "text-muted-foreground text-sm mb-3",
                            "建一个 project 开始（state 管道按 kind 模板缺省，可在项目页自定义）。"
                        }
                        CreateProjectForm {}
                    }
                } else {
                    div { class: "flex flex-wrap gap-4",
                        for panel in &view.projects {
                            ProjectPanelCard { panel: panel.clone() }
                        }
                    }
                }
            } else {
                div { class: "text-muted-foreground",
                    "加载…（10081 未起则先跑 canon serve）"
                }
            }
        }
    }
}

/// 项目面板：头 = 项目名 + kind Badge + 任务计数；体 = task 卡列表。
#[component]
fn ProjectPanelCard(panel: ProjectPanel) -> Element {
    rsx! {
        div { class: "ui-state-panel w-80 shrink-0",
            div { class: "flex items-center gap-2 mb-2",
                h3 { class: "role-title", "{panel.name}" }
                Badge { variant: BadgeVariant::Secondary,
                    "{panel.kind}"
                }
                span { class: "ml-auto ui-type-label", "{panel.tasks.len()} task" }
            }
            div {
                if panel.tasks.is_empty() {
                    p { class: "text-muted-foreground text-sm", "空项目" }
                } else {
                    for t in &panel.tasks {
                        TaskCard {
                            panel: panel.clone(),
                            task: t.clone(),
                        }
                    }
                }
            }
        }
    }
}

/// task 卡：标题 + 自定义 state Badge + step 进度 + claimant；hover 快捷操作。
#[component]
fn TaskCard(panel: ProjectPanel, task: api::TaskCard) -> Element {
    let act = use_signal(String::new);
    let tid = task.id;
    // 管道位置 → 前进目标（states 有序；前进只许下一态）
    let next = panel
        .states
        .iter()
        .position(|s| s == &task.state)
        .and_then(|i| panel.states.get(i + 1))
        .cloned();
    // 打回 = 任意前序 state（取首态按钮，完整打回矩阵随后落地）
    let first = panel.states.first().cloned();
    let claimant = match &task.claimant {
        Some(c) => format!("@{c}"),
        None => String::from("未领取"),
    };

    rsx! {
        div { class: "ui-task-card group",
            div { class: "flex items-center gap-2",
                Badge { variant: BadgeVariant::Outline,
                    "{task.state}"
                }
                span { class: "text-sm font-medium flex-1", "{task.title}" }
                span { class: "ui-type-label",
                    "{task.steps_done}/{task.steps_total}"
                }
            }
            div { class: "mt-1 flex items-center gap-2",
                span { class: "ui-type-label", "{claimant}" }
                div { class: "hidden group-hover:flex items-center gap-1 ml-auto",
                    ActionButton {
                        label: String::from("领取"),
                        disabled: task.claimant.is_some(),
                        on_click: move |_| {
                            spawn_tool(
                                "task_claim",
                                &serde_json::json!({ "task": tid.to_string(), "claimant": "me" }),
                                act,
                            );
                        },
                    }
                    if let Some(to) = next {
                        ActionButton {
                            label: format!("→{to}"),
                            disabled: false,
                            on_click: move |_| {
                                spawn_tool(
                                    "task_transition",
                                    &serde_json::json!({ "task": tid.to_string(), "to": to.clone() }),
                                    act,
                                );
                            },
                        }
                    }
                    if let Some(fb) = first {
                        ActionButton {
                            label: String::from("↩首态"),
                            disabled: false,
                            on_click: move |_| {
                                spawn_tool(
                                    "task_transition",
                                    &serde_json::json!({ "task": tid.to_string(), "to": fb.clone() }),
                                    act,
                                );
                            },
                        }
                    }
                }
            }
            if !act().is_empty() {
                p { class: "text-xs text-destructive mt-1", "{act()}" }
            }
        }
    }
}

/// 建 project 表单（P1 空态；POST project_create，成功后 WS 推送自动刷新）。
#[component]
fn CreateProjectForm() -> Element {
    let mut name = use_signal(String::new);
    let mut kind = use_signal(|| String::from("general"));
    let msg = use_signal(String::new);
    rsx! {
        div { class: "space-y-2",
            input {
                class: "ui-input w-full",
                placeholder: "project 名（如 canon / ferrite）",
                value: "{name()}",
                oninput: move |e| name.set(e.value()),
            }
            div { class: "flex items-center gap-2",
                select {
                    class: "ui-input",
                    value: "{kind()}",
                    oninput: move |e| kind.set(e.value()),
                    option { value: "general", "general" }
                    option { value: "backend", "backend" }
                    option { value: "frontend", "frontend" }
                }
                button {
                    class: "ui-btn-primary",
                    onclick: move |_| {
                        spawn_tool(
                            "project_create",
                            &serde_json::json!({ "name": name(), "kind": kind() }),
                            msg,
                        );
                    },
                    "建 project"
                }
            }
            if !msg().is_empty() {
                p { class: "text-xs text-destructive", "{msg()}" }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// P2 项目页
// ---------------------------------------------------------------------------

#[component]
pub fn ProjectPage(reload: Signal<u32>) -> Element {
    let projects = use_signal(Vec::<api::Project>::new);
    let sel = use_signal(|| 0i64);
    let bundle = use_signal(|| None::<P2Data>);
    let err = use_signal(String::new);

    use_effect(move || {
        let _dep = reload();
        let (mut p, mut s, b, mut e) = (projects, sel, bundle, err);
        spawn(async move {
            match api::projects().await {
                Ok(list) => {
                    p.set(list.clone());
                    // 保持当前选中（id 仍在则保留，否则落第一个）
                    let cur = *s.peek();
                    if !list.iter().any(|x| x.id == cur) {
                        s.set(list.first().map(|x| x.id).unwrap_or(0));
                    }
                    load_p2(s, b, e).await;
                }
                Err(msg) => e.set(msg),
            }
        });
    });

    let list = projects();
    let data = bundle();
    rsx! {
        div {
            if !err().is_empty() {
                p { class: "text-destructive text-sm", "{err()}" }
            }
            if let Some(d) = data {
                if list.is_empty() {
                    div { class: "text-muted-foreground", "无 project（P1 建一个）" }
                } else {
                    ProjectPageBody {
                        list: list.clone(),
                        sel,
                        data: d.clone(),
                    }
                }
            } else {
                div { class: "text-muted-foreground", "…" }
            }
        }
    }
}

/// P2 当前活跃 task（state 非末态；无则取第一个）。
fn active_task(d: &P2Data) -> Option<&api::TaskCard> {
    if d.panel.tasks.is_empty() {
        return None;
    }
    let last_state = d.panel.states.last();
    d.panel
        .tasks
        .iter()
        .find(|t| Some(&t.state) != last_state)
        .or_else(|| d.panel.tasks.first())
}

fn current_task_id(d: &P2Data) -> Option<i64> {
    active_task(d).map(|t| t.id)
}

#[component]
fn ProjectPageBody(list: Vec<api::Project>, sel: Signal<i64>, data: P2Data) -> Element {
    let active = active_task(&data);
    let active_id = active.map(|t| t.id);

    rsx! {
        div {
            ProjectHead {
                list,
                sel,
                data: data.clone(),
            }
            // task 纵列（P2 主体之一）
            if data.panel.tasks.is_empty() {
                div { class: "ui-state-panel mb-4", "无 task（P1 建一个）" }
            } else {
                div { class: "grid grid-cols-2 gap-2 mb-4",
                    for t in data.panel.tasks.iter().rev() {
                        TaskRow {
                            panel: data.panel.clone(),
                            task: t.clone(),
                        }
                    }
                }
            }
            // 纵向 state 分栏：每 state 一块面板，栏内按时间线（event.state_at 归栏）
            div { class: "grid grid-cols-2 gap-3",
                for state in &data.panel.states {
                    StateColumn {
                        data: data.clone(),
                        state: state.clone(),
                        current: active.map(|t| t.state.clone()) == Some(state.clone()),
                        current_task: active_id,
                    }
                }
            }
        }
    }
}

/// P2 头部：项目切换 + state 管道（可加态；改名/调序随后落地）。
#[component]
fn ProjectHead(list: Vec<api::Project>, sel: Signal<i64>, data: P2Data) -> Element {
    let mut new_state = use_signal(String::new);
    let flash = use_signal(String::new);
    let active = active_task(&data);
    let states0 = data.panel.states.clone();
    let pname = data.panel.name.clone();

    rsx! {
        div { class: "flex items-center gap-2 mb-3 flex-wrap",
            select {
                class: "ui-input",
                value: "{sel()}",
                onchange: move |e| {
                    if let Ok(v) = e.value().parse::<i64>() {
                        sel.set(v);
                    }
                },
                for p in &list {
                    option { value: "{p.id}", "{p.name}" }
                }
            }
            div { class: "ui-pipeline",
                for s in &data.panel.states {
                    span {
                        class: "ui-pipeline-node",
                        class: if Some(s) == active.map(|t| &t.state) {
                            "ui-pipeline-node-current"
                        } else { "" },
                        "{s}"
                    }
                    if Some(s) != data.panel.states.last() {
                        span { class: "text-muted-foreground text-xs", "›" }
                    }
                }
            }
            div { class: "flex items-center gap-1",
                input {
                    class: "ui-input w-32",
                    placeholder: "加 state…",
                    value: new_state,
                    oninput: move |e| new_state.set(e.value()),
                }
                button {
                    class: "ui-btn-primary",
                    onclick: move |_| {
                        let mut next_states = states0.clone();
                        next_states.push(new_state().clone());
                        spawn_tool(
                            "project_update",
                            &serde_json::json!({
                                "project": pname,
                                "states": next_states,
                            }),
                            flash,
                        );
                        new_state.set(String::new());
                    },
                    "+"
                }
            }
            if !flash().is_empty() {
                span { class: "text-xs text-destructive", "{flash()}" }
            }
        }
    }
}

/// task 行（P2 纵列）：标题 | 管道位置高亮 | n/m | claimant | 前进/打回。
#[component]
fn TaskRow(panel: ProjectPanel, task: api::TaskCard) -> Element {
    let flash = use_signal(String::new);
    let tid = task.id;
    let first_state = panel.states.first().cloned();
    let pos = panel
        .states
        .iter()
        .position(|s| s == &task.state)
        .unwrap_or(0);
    let next = panel.states.get(pos + 1).cloned();
    let claimant = match &task.claimant {
        Some(c) => format!("@{c}"),
        None => String::from("未领取"),
    };

    rsx! {
        div { class: "ui-state-panel",
            div { class: "flex items-center gap-2",
                Badge { variant: BadgeVariant::Outline, "{task.state}" }
                span { class: "text-sm font-medium flex-1", "{task.title}" }
                span { class: "ui-type-label", "{task.steps_done}/{task.steps_total}" }
                span { class: "ui-type-label", "{claimant}" }
            }
            div { class: "ui-pipeline mt-2 mb-2",
                for (i, s) in panel.states.iter().enumerate() {
                    span {
                        class: "ui-pipeline-node",
                        class: if i == pos { "ui-pipeline-node-current" } else { "" },
                        "{s}"
                    }
                    if i + 1 < panel.states.len() {
                        span { class: "text-muted-foreground text-xs", "›" }
                    }
                }
            }
            div { class: "flex items-center gap-1",
                if let Some(to) = next {
                    button {
                        class: "ui-btn-primary text-xs",
                        onclick: move |_| {
                            spawn_tool(
                                "task_transition",
                                &serde_json::json!({ "task": tid.to_string(), "to": to.clone() }),
                                flash,
                            );
                        },
                        "→{to}"
                    }
                }
                if pos > 0 {
                    button {
                        class: "ui-btn-ghost text-xs",
                        title: "打回首态",
                        onclick: move |_| {
                            spawn_tool(
                                "task_transition",
                                &serde_json::json!({
                                    "task": tid.to_string(),
                                    "to": first_state.clone(),
                                }),
                                flash,
                            );
                        },
                        "↩"
                    }
                }
                if !flash().is_empty() {
                    span { class: "text-xs text-destructive", "{flash()}" }
                }
            }
        }
    }
}

/// 单 state 栏：动作时间线 + （当前 state 时）未完成 todo。
#[component]
fn StateColumn(data: P2Data, state: String, current: bool, current_task: Option<i64>) -> Element {
    let flash = use_signal(String::new);
    let events: Vec<&api::JournalEvent> =
        data.events.iter().filter(|e| e.state_at == state).collect();
    // 未完成 todo 挂当前 state 栏底（取活跃 task 的未 done step）
    let todos: Vec<StepRow> = current_task
        .and_then(|id| data.bundles.iter().find(|b| b.task.id == id))
        .map(|b| b.steps.iter().filter(|s| !s.done).cloned().collect())
        .unwrap_or_default();

    rsx! {
        div { class: "ui-state-panel",
            div { class: "flex items-center gap-2 mb-1",
                span { class: "role-title", "{state}" }
                if current {
                    Badge { variant: BadgeVariant::Primary, "当前" }
                }
                span { class: "ml-auto ui-type-label", "{events.len()} 动作" }
            }
            // 时间线（ts 升序）
            div {
                for e in &events {
                    div { class: "ui-todo-row",
                        span { class: "ui-type-label w-14 shrink-0",
                            "{ts_short(e.ts)}"
                        }
                        span { class: "text-xs flex-1", "{event_label(e)}" }
                        span { class: "ui-type-label", "{e.actor}" }
                    }
                }
                if events.is_empty() {
                    p { class: "text-muted-foreground text-xs", "（无动作记录）" }
                }
            }
            // 未完成 todo
            if current {
                div { class: "mt-2",
                    p { class: "ui-type-label mb-1", "未完成 todo" }
                    for step in &todos {
                        TodoRow {
                            data: data.clone(),
                            step: step.clone(),
                        }
                    }
                    if todos.is_empty() {
                        p { class: "text-muted-foreground text-xs", "全部完成" }
                    }
                }
            }
            if !flash().is_empty() {
                p { class: "text-xs text-destructive", "{flash()}" }
            }
        }
    }
}

/// todo 行：勾选（work 直接 step_mark；spec 需先跑 spec 出 evidence）+
/// kind 标记 + spec_ref chip（点「运行」= spec_run）。
#[component]
fn TodoRow(data: P2Data, step: StepRow) -> Element {
    let flash = use_signal(String::new);
    let step_id = step.id;
    let task_id = current_task_id(&data).unwrap_or(0);
    let project_name = data.panel.name.clone();
    let spec_ready = step.evidence.is_some();
    let is_spec = step.kind == "spec";

    rsx! {
        div { class: "ui-todo-row",
            button {
                class: "ui-check",
                class: if step.done { "opacity-30" } else { "" },
                disabled: step.done || (is_spec && !spec_ready),
                title: if is_spec && !spec_ready {
                    "先跑 spec（spec step 需 evidence）"
                } else { "" },
                onclick: move |_| {
                    spawn_tool(
                        "step_mark",
                        &serde_json::json!({
                            "task": task_id.to_string(),
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
                span {
                    class: "ui-type-label flex items-center gap-1",
                    "{r}"
                    if is_spec && !step.evidence.is_some() {
                        button {
                            class: "ui-btn-ghost text-xs",
                            title: "spec_run（结果入 journal，可作 evidence）",
                            onclick: move |_| {
                                spawn_tool(
                                    "spec_run",
                                    &serde_json::json!({
                                        "project": project_name.clone(),
                                        "task": task_id.to_string(),
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
            if !flash().is_empty() {
                p { class: "text-xs text-destructive", "{flash()}" }
            }
        }
    }
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
            match action {
                "done" => format!(
                    "✓ {}",
                    p.get("title").and_then(|v| v.as_str()).unwrap_or("?")
                ),
                "add" => format!(
                    "+ {}",
                    p.get("title").and_then(|v| v.as_str()).unwrap_or("?")
                ),
                "bind" => format!(
                    "绑 spec {}",
                    p.get("spec_ref").and_then(|v| v.as_str()).unwrap_or("?")
                ),
                _ => p.to_string(),
            }
        }
        "note" => {
            if let Some(text) = p.get("text").and_then(|v| v.as_str()) {
                text.to_string()
            } else {
                let action = p.get("action").and_then(|v| v.as_str()).unwrap_or("");
                match action {
                    "claim" => format!(
                        "领取 @{}",
                        p.get("claimant").and_then(|v| v.as_str()).unwrap_or("?")
                    ),
                    "create" => format!(
                        "建 task {}",
                        p.get("title").and_then(|v| v.as_str()).unwrap_or("?")
                    ),
                    "migrate_state" => format!(
                        "迁 state {} → {}",
                        p.get("from").and_then(|v| v.as_str()).unwrap_or("?"),
                        p.get("to").and_then(|v| v.as_str()).unwrap_or("?")
                    ),
                    _ => action.to_string(),
                }
            }
        }
        "spec" => {
            let name = p.get("checklist").and_then(|v| v.as_str());
            match name {
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

// ---------------------------------------------------------------------------
// 共享：后台跑工具 + 快速按钮
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

#[component]
fn ActionButton(label: String, disabled: bool, on_click: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "ui-btn-ghost text-xs",
            disabled,
            onclick: on_click,
            "{label}"
        }
    }
}

/// P2 数据装载：board（选中项目）+ 每 task journal + task bundle。
async fn load_p2(sel: Signal<i64>, mut bundle: Signal<Option<P2Data>>, mut err: Signal<String>) {
    let target = *sel.peek();
    let board = match api::board().await {
        Ok(b) => b,
        Err(e) => {
            err.set(e);
            return;
        }
    };
    let Some(panel) = board.projects.into_iter().find(|p| p.id == target) else {
        return;
    };
    let mut bundles = Vec::new();
    let mut events = Vec::new();
    for t in &panel.tasks {
        let id = t.id.to_string();
        if let Ok(b) = api::task_bundle(&id).await {
            bundles.push(b);
        }
        if let Ok(j) = api::journal(&id, 50).await {
            events.extend(j);
        }
    }
    events.sort_by_key(|e| e.ts);
    bundle.set(Some(P2Data {
        panel,
        events,
        bundles,
    }));
}
