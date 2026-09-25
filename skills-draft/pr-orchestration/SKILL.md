---
name: pr-orchestration
description: >
  PR 开发编排（主控 → 子代理）：铺地基（重构/移植时）→ 拆任务派子代理 → 逐任务验收
  → CI 驱动 → CRG/jev 工具审查 → smoke → 收尾报告 → 合并清理，九阶段含硬性门禁。
  凡是用户说「开 PR 干活」「拆任务」「派子代理」「主控子代理分工」「重构/移植/复刻某模块」
  「铺骨架再派子代理」「开 worktree」时都用本技能——即使用户只是在描述一次多模块改动
  的前后半段，也应先用本技能确认阶段位置。
allowed-tools: Bash, Read, Grep, Glob, Write
license: MIT
---
> **适用范围**：流程通用；worktree 路径约定（`.wt/<branch>/`）与 PR body 字段以目标仓
> `.github/PULL_REQUEST_TEMPLATE.md` 实际模板为准。仓名、crate 布局按实际仓库核对。
> 本技能只管 **PR 生命周期的主控流程**。scope/architect 阶段方法走 `wf-scope`/`wf-architect`；
> 任务书写作细则走 `task-brief`；收尾全流程（审查→修复→记录→清场）走 `closeout`。

## Workflow Overview

```
Stage 0  Scaffold (refactor/port only — skip for ordinary features)
  │   Main controller works in a .wt worktree: cargo check green + placeholder inventory + README
  ▼
Stage 1  Scope + split tasks + open draft PR
  │   Map the crates/files in scope; split sub-tasks (each <=5 files, one theme)
  │   Open draft PR; body records the task list and acceptance commands
  ▼
Stage 2  Dispatch sub-agents (dev)
  │   Max 2 in parallel; same-crate files must serialize
  │   Each prompt states: absolute path, allowed/forbidden files, goal, acceptance command
  ▼
Stage 3  Per-task audit
  │   Main controller verifies diff boundaries + runs the acceptance command
  ▼
Stage 4  CI-driven verification
  │   push PR -> CI selects packages -> all green before proceeding
  │   CI failure -> extract logs -> precise fix back in Stage 2
  ▼
Stage 5  Tool review (CRG + gate/jev)
  │   code-review-graph update -> detect-changes -> gate check (jev semantic layer)
  │   Bugs found -> fix in Stage 2 -> re-review until clean
  ▼
Stage 6  Smoke verification
  │   Walk a real user path once; screenshots where UI changed
  ▼
Stage 7  Tidy + report
  │   gate re-check + fmt + docs sync -> PR comment
  ▼
Stage 8  Merge + cleanup
      User confirms -> squash merge -> git worktree remove
```

## 附属参考（按需读，不必每次通读）
- `references/stage-details.md` — 各阶段展开：铺地基/拆任务/派代理/审查/CI/工具审查/收尾的命令与验收细节
- `references/pr-workflow.md` — 另一套阶段编号（准备/摸清范围/开发循环/冒烟/清理/汇报）+ 硬性门禁
