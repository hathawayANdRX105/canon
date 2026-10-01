# 小说创作 Agent 入口图谱

本文件是项目文档的导航图，不是完整手册。进入任务时先定位节点，再按需读取相邻文档，避免一次性加载全部上下文。

## 0. 读取策略

1. 先读本文件，确认当前任务落在哪个节点。
2. 只打开该节点列出的 1-3 个必要文档。
3. 如果发现任务跨节点，再沿“相邻节点”继续读取。
4. 不确定任务类型时，先读 `docs/README.md` 或 `docs/project/map.md`，不要全仓库扫描。
5. 修改产物前，必须确认目标小说、目标阶段、权威文件和禁止触碰路径。

## 1. 项目图谱

```text
AGENTS.md
  ├─ Docs Index
  │    └─ docs/README.md
  ├─ Project Map
  │    ├─ README.md
  │    └─ novel1/README.md
  ├─ Writing Pipeline
  │    ├─ docs/writing/artifact-routing.md
  │    ├─ docs/writing/rules-audit.md
  │    ├─ skills/*.md
  │    └─ novel1/rules/*.md
  ├─ Review Loop
  │    ├─ docs/review/learning-loop.md
  │    ├─ skills/review.md
- `stats/draft/`、CHANGELOG / rules
  ├─ Tooling
  │    ├─ docs/tooling/review-tools.md
  │    ├─ review/docs/checks.md
  │    ├─ review/docs/generated-outputs.md
  │    ├─ review/docs/consistency-index.md
  │    ├─ review/docs/history/review-system-status.md
  │    ├─ review/docs/history/review-tool-optimization.md
  │    └─ review/ / tests/
  └─ Collaboration
       ├─ docs/collaboration/worktree-handoff.md
       └─ skills/review-handoff.md
```

## 2. 节点索引

| 节点 | 适用会话 | 起始文档 | 相邻节点 |
|------|----------|----------|----------|
| Docs Index | 不确定该读什么、需要模块列表 | `docs/README.md` | Project Map、Writing Pipeline、Tooling |
| Project Map | 了解项目、找目录、确认当前状态 | `docs/project/map.md` | Writing Pipeline、Tooling |
| Writing Pipeline | 写设定、大纲、章节规划、草稿、输出稿 | `docs/writing/artifact-routing.md` | Review Loop、Project Map |
| Writing Rules Audit | 检查 guide/rules 是否过长、重复或过时 | `docs/writing/rules-audit.md` | Writing Pipeline、Review Loop |
| Review Loop | 评审、复盘、沉淀、生成修改建议 | `docs/review/learning-loop.md` | Writing Pipeline、Tooling |
| Tooling | 改审查脚本、统计脚本、测试、误报规则 | `docs/tooling/review-tools.md` | Review Loop、Project Map |
| Tooling Checks | 运行审查脚本、理解统计生成物 | `review/docs/checks.md` | Tooling、Review Loop |
| Tooling Outputs | 解释统计报告、审稿资产、template backlog | `review/docs/generated-outputs.md` | Tooling Checks、Review Loop |
| Tooling Consistency | 处理称谓、地点、关系、物件漂移 | `review/docs/consistency-index.md` | Tooling Outputs、Review Loop |
| Collaboration | 多会话协作、worktree、交接、禁改范围 | `docs/collaboration/worktree-handoff.md` | Project Map、Review Loop |

## 3. 任务到文档的最短路径

| 用户意图 | 最小读取路径 |
|----------|--------------|
| “了解项目 / 当前结构” | `docs/project/map.md` -> `README.md` -> 目标 `novelN/README.md` |
| “写/改 concept” | `docs/writing/artifact-routing.md` -> `skills/concept.md` -> `novelN/rules/concept.md` |
| “写/改 arc-plan” | `docs/writing/artifact-routing.md` -> `skills/arc.md` -> `novelN/rules/arc-plan.md` |
| “写/改 story-plan 或 interlude” | `docs/writing/artifact-routing.md` -> `skills/story.md` -> `novelN/rules/story-plan.md` |
| “写/改 chapter-plan” | `docs/writing/artifact-routing.md` -> `skills/chapter.md` -> `novelN/rules/chapter-plan.md` |
| “写/修草稿” | `docs/writing/artifact-routing.md` -> `skills/draft.md` -> `novelN/rules/draft.md` |
| “输出最终稿” | `docs/writing/artifact-routing.md` -> `skills/output.md` -> `novelN/rules/output.md` |
| “评审一个产物” | `docs/review/learning-loop.md` -> `skills/review.md` -> 对应阶段 `rules` |
| “优化审查工具” | `docs/tooling/review-tools.md` -> `review/docs/history/review-system-status.md` -> `review/docs/history/review-tool-optimization.md` |
| “交接给另一个会话” | `docs/collaboration/worktree-handoff.md` -> `skills/review-handoff.md` |

## 4. 核心链路

写作产物链：

`concept -> arc-plan -> story-plan -> chapter-plan -> draft -> revise -> output`

优化闭环：

`写作 -> 评审 -> 复盘 -> 沉淀 -> 下一轮`

知识沉淀位置：

- 通用方法：`skills/`
- 单本小说经验：`novelN/rules/`
- 设定变更：`novelN/concept/CHANGELOG.md`
- 仓库结构 / 流程 / 工具变化：`CHANGELOG.md`

## 5. 入口级硬规则

1. 不要一次性读取全部文档；按节点和相邻节点渐进加载。
2. 写作产物必须先读对应 `skills/*.md`，再读对应 `novelN/rules/*.md`。
3. `skills/` 只放通用方法，`novelN/rules/` 只放当前小说专属规律。
4. 只沉淀会复发的模式，不沉淀一次性情绪、流水账或局部问题。
5. 评审必须指出 1-3 处亮点，不能只列问题。
6. 伏笔编号统一使用 `F-01` 到 `F-99`。
7. 审查工具开发默认不修改 `novel1/drafts/**` 正文。
8. 不再使用 `beads` 工作流。

## 6. 当前默认上下文

- 当前主要推进小说：`novel1`
- 当前小说简介与状态：`novel1/README.md`
- 当前设定主入口：`novel1/concept/cards/`
- 当前 Arc1 权威入口：`novel1/arcs/arc-1.md`
- 旧文件可取材，但不能默认可信。

## 5. 入口级硬规则（续）

9. **动笔前必读对应 rules 并跑 pre-gate**：`python3 -m review.cli gate pre <phase> [target]`
10. **交付前必跑 post-gate 判定门槛**：`python3 -m review.cli gate post <phase> [target]`，FAIL 不得进入下一阶段
11. **每轮返工后必须同步更新对应 rules**，把新发现的复发模式写进去
