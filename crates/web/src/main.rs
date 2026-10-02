//! canon-flow-web — 本地看板（Dioxus wasm + ui-kit + canon API）。
//!
//! 布局：左侧常驻 sidebar（一级 = 项目，二级 = 任务）+ 右侧主区（任务页：
//! phase 纵列，每列内嵌该 phase 的 step 列表 + 动作时间线）。
//!
//! 数据：`canon serve`（10081）REST；5s 轮询 bump `reload` 驱动 refetch
//! （v1；SSE/WS 推送随后接）。构建链：ui-kit path 依赖 + flow 纯模型
//! （wasm 侧无 store）+ gloo-net fetch。

mod api;
mod pages;
mod sidebar;

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use ui_kit::layout::{Sidebar, SidebarCollapsible, SidebarInset, SidebarProvider, SidebarTrigger};

use crate::sidebar::ProjectSidebar;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.out.css");

// Ainotation 标注工具 bundle，由 `bun run aino` 生成（源：ainotation-entry.ts）。
#[cfg(debug_assertions)]
const AINOTATION_JS: Asset = asset!("/assets/ainotation/ainotation.iife.js");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    // Ainotation 标注工具 bundle，由 `bun run aino` 生成（源：ainotation-entry.ts）。
    // 仅开发环境加载；release 构建自动排除。用法见 ferrite apps/admin-web/AINOTATION.md。
    #[cfg(debug_assertions)]
    let ainotation_js: Option<Asset> = Some(AINOTATION_JS);
    #[cfg(not(debug_assertions))]
    let ainotation_js: Option<Asset> = None;

    // 选中态：None = 未选任务（显示项目概览）；Some(task_id) = 该任务页
    let mut sel_task = use_signal(|| None::<i64>);
    let reload = use_signal(|| 0u32);

    // 轮询刷新：每 5s bump reload（v1；替代 WS 推送，demo 同款 Timer 先例）
    use_effect(move || {
        let mut bump = reload;
        spawn(async move {
            loop {
                TimeoutFuture::new(5000).await;
                bump.set(bump() + 1);
            }
        });
    });

    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        {ainotation_js.map(|src| rsx! { document::Script { src } })}
        SidebarProvider {
            default_open: true,
            collapsible: SidebarCollapsible::Icon,
            Sidebar {
                ProjectSidebar { sel_task, reload }
            }
            SidebarInset {
                header { class: "flex items-center gap-2 px-3 py-2 border-b border-border",
                    SidebarTrigger {}
                    span { class: "ui-type-label ml-auto", "5s 轮询" }
                }
                main { class: "flex-1 overflow-auto p-4",
                    if let Some(tid) = sel_task() {
                        pages::TaskPage { task: tid, reload }
                    } else {
                        pages::ProjectPage { reload, on_open_task: move |id| sel_task.set(Some(id)) }
                    }
                }
            }
        }
    }
}
