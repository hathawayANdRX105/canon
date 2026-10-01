//! canon flow web — 本地看板（Dioxus wasm + ui-kit）。
//!
//! 两个主页面（见 canon/todo/flowboard-impl-plan.md E 节）：
//! - P1 全部任务页：按项目划分面板，task 卡 + 推进菜单
//! - P2 项目页：state 纵栏 + 时间线排序 todo + journal
//!
//! 数据来源：`canon serve` 的 REST + SSE（后续里程碑）；当前骨架验证
//! 构建链（ui-kit 组件 + flow 纯模型 + wasm 目标）。

use dioxus::prelude::*;
use ui_kit::layout::TopNavBar;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let pages = vec!["全部任务".to_string(), "项目".to_string()];
    // 骨架阶段：tab 固定第 0 项，交互（信号 + SSE）随 API 接线里程碑落地
    rsx! {
        TopNavBar {
            tabs: pages,
            active: 0,
            on_select: move |_| {},
        }
        div {
            class: "p-4 text-muted-foreground text-sm",
            "P1/P2 骨架：ui-kit TopNavBar + flow 纯模型构建链已验证；页面与 API 接线随后落地"
        }
    }
}
