//! 左侧常驻 sidebar：一级 = 项目组，二级 = 该项目的 task。
//!
//! 数据走 `/api/board`（`GET`，含每 task 的 steps 计数），不额外请求；
//! 选中 task id 由上层持有（`sel_task`），点二级项只回传 id。

use dioxus::events::MouseEvent;
use dioxus::prelude::*;
use ui_kit::layout::{
    SidebarContent, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarHeader,
    SidebarMenu, SidebarMenuBadge, SidebarMenuButton, SidebarMenuItem,
};
use ui_kit::primitive::badge::BadgeVariant;
use ui_kit::primitive::Badge;

use crate::api;

/// 一级项目 + 二级任务列表。
#[component]
pub fn ProjectSidebar(sel_task: Signal<Option<i64>>, reload: Signal<u32>) -> Element {
    let board = use_signal(|| None::<api::BoardView>);

    use_effect(move || {
        let _dep = reload();
        let mut b = board;
        spawn(async move {
            if let Ok(view) = api::board().await {
                b.set(Some(view));
            }
        });
    });

    let view = board();
    rsx! {
        SidebarHeader {
            div { class: "flex flex-col gap-0.5",
                span { class: "role-title", "canon flow" }
                span { class: "ui-type-label", "看板 · 任务领取" }
            }
        }
        SidebarContent {
            if let Some(v) = view {
                if v.projects.is_empty() {
                    p { class: "p-3 text-xs text-muted-foreground",
                        "还没有 project：用 MCP 的 project_create 建一个。"
                    }
                } else {
                    for p in &v.projects {
                        SidebarGroup {
                            SidebarGroupLabel {
                                span { class: "truncate", "{p.name}" }
                                span { class: "ui-type-label ml-auto shrink-0",
                                    "{p.tasks.len()}"
                                }
                            }
                            SidebarGroupContent {
                                SidebarMenu {
                                    if p.tasks.is_empty() {
                                        SidebarMenuItem {
                                            div { class: "px-3 py-1.5 text-xs text-muted-foreground", "空项目" }
                                        }
                                    } else {
                                        for t in &p.tasks {
                                            TaskItem { panel: p.clone(), task: t.clone(), sel_task }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                p { class: "p-3 text-xs text-muted-foreground", "加载…" }
            }
        }
    }
}

/// 二级任务项：state badge + 标题 + step 进度；点 = 切主区。
#[component]
fn TaskItem(
    panel: api::ProjectPanel,
    task: api::TaskCard,
    mut sel_task: Signal<Option<i64>>,
) -> Element {
    let tid = task.id;
    let active = sel_task() == Some(tid);
    // 管道位置：决定标题左侧的 phase 徽标色（进度感）
    let pos = panel
        .states
        .iter()
        .position(|s| s == &task.state)
        .map(|i| (i + 1, panel.states.len()));

    rsx! {
        SidebarMenuItem {
            SidebarMenuButton {
                active,
                on_click: move |_: MouseEvent| sel_task.set(Some(tid)),
                div { class: "flex min-w-0 items-center gap-1.5",
                    span { class: "truncate", "{task.title}" }
                    if let Some((i, n)) = pos {
                        span { class: "ui-type-label shrink-0", "{i}/{n}" }
                    }
                }
            }
            // 右侧：未领取提示 or 进度 badge
            if task.claimant.is_none() {
                SidebarMenuBadge {
                    Badge { variant: BadgeVariant::Outline, "未领取" }
                }
            } else {
                SidebarMenuBadge {
                    span { class: "ui-type-label", "{task.steps_done}/{task.steps_total}" }
                }
            }
        }
    }
}
