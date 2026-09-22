# agent/ — 各项目 .agent 文档正本

> 2026-09-23 从 ferrite / omenic / silverq / kime 四仓 `.agent/` **原样移植**（未改内容）。
> canon 是唯一正本：项目侧副本由 `agent-sync.yaml` 分发，**在项目里改文档前先想清楚**——
> 正确流向是「项目本地修正 → `agent-sync backport` 回流 canon → `agent-sync push` 再下发」。
> 与 `tasks/` 的分工：`tasks/` 是四仓通用的任务书（closeout / version-stats）；
> 这里是**项目特有**的规矩（rules）、技能（skills）与任务书（tasks），按项目分目录。

## 目录

| 目录 | 内容 | 来源 |
|---|---|---|
| `agent/ferrite/` | 6 rules + 5 skills + 5 tasks + README | ferrite `.agent/` |
| `agent/omenic/` | task-templates-handbook + 3 skills | omenic `.agent/` |
| `agent/silverq/` | tasks/versioning.md（版本口径项目真相源） | silverq `.agent/` |
| `agent/kime/` | 无——kime 的 `.agent/` 只有 canon 分发的 closeout/version-stats，无独有内容 | — |

## 不在本目录的

- `tasks/closeout.md`、`tasks/version-stats.md`：四仓通用，正本在 `tasks/`，由 agent-sync 直接分发。
- `rules/**`（gate 规则包）：正本在 `rules/`，与本文档目录无关，别混。

## 已知分叉（待用户裁决，不擅自统一）

- `ferrite/skills/ui-validation/SKILL.md` vs `omenic/skills/ui-validation/SKILL.md：
  omenic 版更新（web Dioxus + TUI ratatui 双覆盖，ui-spec/tui-spec），ferrite 版仍是旧口径
  （纯 web + 显式声明「gate 不强制」）。
- `ferrite/skills/refactor-workflow/SKILL.md` vs `omenic` 版：仅第 1 步读的 README 位置不同
  （仓根 vs 所属域目录）——反映两仓 crate 布局差异。
- 两版都有道理时，以项目副本为准；要统一就改正本后 push，让分叉在明处。
