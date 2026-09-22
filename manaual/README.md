# manaual/ — 共同主题文档索引

> canon 的**文档正本**都按「共同主题」归拢在本目录（目录名 manaual 为用户指定，勿「修正」拼写），由 `agent-sync.yaml` 决定哪个文件
> 装进哪个项目（安装路径沿用项目侧 `.agent/` 布局）。
> **改文档的正确流向**：项目本地修正 → `agent-sync backport` 回流 canon → 改 canon 正本 →
> `agent-sync push` 再下发。别只在项目里改——那会静默分叉。

## 主题目录

| 主题 | 文件 | 装到 |
|---|---|---|
| `gate/` | `GATE_HANDBOOK.md` — gate 总手册（三层 SLA / 规则清单 / 豁免 / DWJ） | 人查，不分发 |
| `github/` | `GITHUB_ISSUE_PR.md` — issue/PR 创建与关联操作 | 人查，不分发 |
| `workflow/` | `PR_DEV_WORKFLOW.md`（PR 开发流）、`WORKFLOW.md`（.wt 隔离规范） | 人查，不分发 |
| `closeout/` | `closeout.md` — 收尾全流程（CRG+gate 全栈审查/修复/PR 记录/清场/资源释放） | 四仓 `.agent/tasks/closeout.md` |
| `versioning/` | `version-stats.md`（版本三段通用骨架）、`silverq.md`（silverq 项目真相源） | 四仓 `.agent/tasks/version-stats.md`；`silverq.md` → silverq `.agent/tasks/versioning.md` |
| `conventions/` | `conventions` `dev-env` `gates` `pr-workflow` `testing-ci` `web-lanes` + `agent-readme`（ferrite rules 与 .agent 说明） | ferrite `.agent/rules/` + `.agent/README.md` |
| `skills/` | `<名>.md`（同源单版本）或 `<名>.<项目>.md`（分叉双版本） | `<项目>/.agent/skills/<名>/SKILL.md` |
| `templates/` | `task-templates-handbook.md`（omenic 编排模板）、ferrite tasks 5 份（`TEMPLATE` `dev` `dev-web-lane` `webfix-lane` `closeout-pr`） | omenic `.agent/task-templates-handbook.md`；ferrite `.agent/tasks/` |

## 分叉待决（信息保留，不擅自统一）

- `skills/ui-validation/`：`ui-validation.omenic.md` 更新（web Dioxus + TUI ratatui 双覆盖，
  ui-spec/tui-spec）；`ui-validation.ferrite.md` 旧口径（纯 web + 显式「gate 不强制」）。
- `skills/refactor-workflow/`：仅第 1 步读的 README 位置不同（仓根 vs 所属域目录）——
  反映两仓 crate 布局差异，可能各自合理。

统一某份 = 删掉分叉副本、改正本、push；在那之前按项目分发当前版本。

## 与 `rules/` 的分工

`rules/` 是 gate 规则包（yaml + harness + 协议文档，`gate init` 播种到各仓
`.githooks/spec/`）；本目录是**给 agent 看**的文档（规矩/技能/任务书），
由 agent-sync 分发到各仓 `.agent/`。两条分发线别混。
