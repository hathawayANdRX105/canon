# specs/ — 默认 spec 与工具使用说明（不策展区）

> 这里是**项目侧默认 spec 与工具使用文档**的 canon 正本：代码约定、环境/测试规范、
> skill 用法、任务模板。**不逐份策展**——它们大多是默认规则与工具说明，按需查即可；
> 维护动作只有两种：项目本地修改 → `agent-sync backport` 回流；统一修改 → 改正本后 push。
> 需要**手动发给 agent 的任务书**不在这里，在 `../tasks/`（closeout / feature-dev-handbook /
> version-stats）。人查手册在 `../manual/`。

## 目录

| 目录 | 内容 | 装到 |
|---|---|---|
| `rules/` | conventions / dev-env / gates / pr-workflow / testing-ci / web-lanes + agent-readme | ferrite `.agent/rules/`、`.agent/README.md` |
| `skills/` | `<名>.md`（同源单版）或 `<名>.<项目>.md`（分叉双版） | `<项目>/.agent/skills/<名>/SKILL.md` |
| `templates/` | task-templates-handbook（omenic 编排模板）+ ferrite tasks 5 份（template / dev / dev-web-lane / webfix-lane / closeout-pr） | omenic `.agent/task-templates-handbook.md`；ferrite `.agent/tasks/` |

## 分叉待决（信息保留，不擅自统一）

- `skills/refactor-workflow.ferrite.md` vs `.omenic.md`：仅第 1 步读的 README 位置不同
  （仓根 vs 所属域目录），反映两仓 crate 布局差异。

统一某份 = 删分叉副本、改正本、push；在那之前按项目分发当前版本。
