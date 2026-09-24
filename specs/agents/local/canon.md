# canon 约定

canon 是**规范与 agent 文档的正本仓**。别的项目想改规范、改 agent 约定，来这里改，再分发回去。

## 这个仓是什么

- 规范正本：`specs/rules/`、`specs/skills/`、`rules/`（gate spec）、`manual/`（人查手册）
- 项目任务书：`tasks/`（closeout / feature-dev-handbook）
- agent 文档正本：`specs/agents/`
- gate 源码：`bin/gate/`（Rust）；各项目 `.githooks/gate` 是构建产物
- 待部署的暂存改动：`specs/pending-*/`（**故意不部署**，见各目录 README）

## 项目 AGENTS.md 是生成物

各项目的 `AGENTS.md` **由 canon 组装生成**，不要在项目里直接改——会被覆盖。

```
specs/agents/fragments/    共享章节：跨项目复用，改一处所有引用项目受益
specs/agents/local/<项目>   项目独有内容：架构约定、目录规则、坑
agents.yaml                 每个项目包含哪些章节、什么顺序、什么参数
bin/agents                  组装脚本
```

常用命令：

```bash
bin/agents list             # 章节库 + 每章被哪些项目引用
bin/agents show <项目>      # 该项目的组合计划
bin/agents build <项目>     # 预览组装结果
bin/agents push <项目>      # 生成并写入项目 AGENTS.md
bin/agents status           # 全项目漂移检查
```

**改规范的正确路径**：

1. 通用规则 → 改 `specs/agents/fragments/<章节>.md`
2. 项目独有 → 改 `specs/agents/local/<项目>.md`
3. 组合调整（加章、换顺序） → 改 `agents.yaml`
4. `bin/agents push <项目>` 重新生成
5. 在项目仓提交生成物

`push` 检测到生成物被手工改过会拒绝覆盖（要 `--force`）。真要改内容，回 canon 改源。

## 其他文档的分发

`tasks/`、`specs/rules/`、`specs/skills/` 里的文档走 `agent-sync`：

```bash
bin/agent-sync status <项目>   # 看漂移
bin/agent-sync push <项目>     # canon → 项目
bin/agent-sync backport <项目> <src>   # 项目本地修正 → 回流 canon
```

`backport` 方向是**项目 → canon**（把项目里的改进收回来），不是下发。

## AGENTS.md 不走 agent-sync

`agent-sync.yaml` **不再**分发 `AGENTS.md`。AGENTS 由 `bin/agents` 单独管理：
两者职责不同 —— `agent-sync` 分发任务书与规则文档（整文件替换），`bin/agents`
组装 AGENTS（共享片段 + 项目独有两层结构）。混在一个工具里，这两层无处安放。
