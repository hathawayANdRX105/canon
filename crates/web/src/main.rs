//! canon-flow-web — 本地看板（Dioxus wasm + ui-kit + canon API）。
//!
//! 两个主页面（canon/todo/flowboard-impl-plan.md E.2）：
//! - P1 全部任务页：按项目划分面板 + task 卡 + hover 快捷操作
//! - P2 项目页：state 管道 + task 纵列 + 纵向 state 分栏时间线 + 未完成 todo
//!
//! 数据：`canon serve`（10081）REST；5s 轮询 bump `reload` 驱动各页 refetch
//! （v1；SSE/WS 推送随后接）。构建链：ui-kit path 依赖 + flow 纯模型
//! （wasm 侧无 store）+ gloo-net fetch。

mod api;
mod pages;

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use ui_kit::layout::TopNavBar;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.out.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut active = use_signal(|| 0usize);
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
        div { class: "flex min-h-svh flex-col",
            div { class: "flex items-center gap-2 px-3",
                TopNavBar {
                    tabs: vec![
                        "全部任务".to_string(),
                        "项目".to_string(),
                    ],
                    active: active(),
                    on_select: move |i| active.set(i),
                }
                span { class: "ml-auto ui-type-label", "5s 轮询" }
            }
            div { class: "flex-1 overflow-auto px-4 py-3",
                if active() == 0 {
                    pages::AllTasks { reload }
                } else {
                    pages::ProjectPage { reload }
                }
            }
        }
    }
}
