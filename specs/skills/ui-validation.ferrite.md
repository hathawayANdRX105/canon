---
name: ui-validation
description: 'Dioxus web UI 验证约定：data-testid + ARIA + tab.ariaSnapshot() 结构化断言，替代截图肉眼判断。纯 agent 约定，gate 不强制。'
license: MIT
---

# UI 验证约定

## 目标

agent 验证 UI 时**不依赖截图**，用结构化断言（role + name + data-testid）。

> **范围**：这是 agent 在 PR smoke 阶段自觉遵守的约定。`.githooks/gate` **不强制**——
> `.githooks/spec/` 里没有任何 ui-spec 检查（规则清单见
> `.githooks/spec/SPEC_OVERVIEW.md`）。要让它变成真门槛，得另写一条
> `checklist_*.yaml` 并解决“契约数据如何入库”的问题。

## Web 流程（dioxus）

1. **交互元素加 `data-testid`**：所有 `button` / `input` / `tab` 加唯一 testid
2. **容器/标签加 ARIA**：`role="tab"` + `aria_selected="{i == active}"` + `aria-label`
3. **PR smoke 用 `tab.ariaSnapshot()`**：断言 role+name+testid，点击走
   `[data-testid="..."]` 选择器
4. **截图仅作辅助**：视觉风格/品牌相关才用

## 验证示例

```rust
let snap = tab.ariaSnapshot().await?;
assert!(snap.contains("role=\"tab\" name=\"用户管理\" data-testid=\"tab-users\""));
tab.run("document.querySelector('[data-testid=\"tab-users\"]').click()").await?;
let snap2 = tab.ariaSnapshot().await?;
assert!(snap2.contains("role=\"table\" name=\"用户列表\""));
```

## 禁做项

- ❌ 只用截图肉眼判断（agent 看不清/看不全）
- ❌ 用 CSS class 选择器（会因样式调整失效）

## 共享 skill

OMP `ui-validate` skill（managed-skills）封装上述流程，触发词：ariaSnapshot、
data-testid、validate ui、tab verification。

## 相关文件

- 已加 testid 的组件：`crates/web/ui-components/src/`
- 页面 testid 分布：各 `crates/web/admin-page-*/src/*.rs` 的 `"data-testid": "..."` 属性
