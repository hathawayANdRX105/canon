# canon 约定

canon 是**规范与 agent 文档的正本仓**。别的项目想改规范、改 agent 约定，来这里改，再分发回去。

## 这个仓是什么

- 规范正本：`agent-pack/rules/`、`agent-pack/skills/`、`specs/`（gate spec，`gate init` 的 seed 源）、`manual/`（人查手册）
- 项目任务书：`agent-pack/tasks/`（closeout / feature-dev-handbook）
- agent 文档正本：`agent-pack/agents/`
- gate 源码：**canon 仓根本身就是 Rust crate**（仓根 `src/` + `tests/` + `Cargo.toml`）；
  `cargo build --release` 产物 `target/release/gate` 复制到 `.githooks/gate`，再由 gate-sync 分发到各项目
- 待部署的暂存改动：`agent-pack/deployed/pending-*/`（**故意不部署**，见各目录 README）

## 项目 AGENTS.md 是生成物

各项目的 `AGENTS.md` **由 canon 组装生成**，不要在项目里直接改——会被覆盖。

```
agent-pack/agents/fragments/    共享章节：跨项目复用，改一处所有引用项目受益
agent-pack/agents/local/<项目>   项目独有内容：架构约定、目录规则、坑
agents.yaml                 每个项目包含哪些章节、什么顺序、什么参数
scripts/agents             组装脚本
```

常用命令：

```bash
scripts/agents list        # 章节库 + 每章被哪些项目引用
scripts/agents show <项目> # 该项目的组合计划
scripts/agents build <项目> # 预览组装结果
scripts/agents push <项目> # 生成并写入项目 AGENTS.md
scripts/agents status      # 全项目漂移检查
```

**改规范的正确路径**：

1. 通用规则 → 改 `agent-pack/agents/fragments/<章节>.md`
2. 项目独有 → 改 `agent-pack/agents/local/<项目>.md`
3. 组合调整（加章、换顺序） → 改 `agents.yaml`
4. `scripts/agents push <项目>` 重新生成
5. 在项目仓提交生成物

`push` 检测到生成物被手工改过会拒绝覆盖（要 `--force`）。真要改内容，回 canon 改源。

## 其他文档的分发

`agent-pack/tasks/`、`agent-pack/rules/`、`agent-pack/skills/` 里的文档走 `agent-sync`：

```bash
scripts/agent-sync status <项目>    # 看漂移
scripts/agent-sync push <项目>      # canon → 项目
scripts/agent-sync backport <项目> <src>  # 项目本地修正 → 回流 canon
```

`backport` 方向是**项目 → canon**（把项目里的改进收回来），不是下发。

## AGENTS.md 不走 agent-sync

`agent-sync.yaml` **不再**分发 `AGENTS.md`。AGENTS 由 `scripts/agents` 单独管理：
两者职责不同 —— `agent-sync` 分发任务书与规则文档（整文件替换），`scripts/agents`
组装 AGENTS（共享片段 + 项目独有两层结构）。混在一个工具里，这两层无处安放。

## ratchet.tsv — 圈复杂度棘轮账本

仓根 `ratchet.tsv` 是 `ccn` 门禁（`specs/quality/checklist_ccn.yaml`）的**棘轮存量账本**，
由 `ccn_gate.py seed` 全仓扫描生成，记录每个函数的当前圈复杂度。

门禁据此**只拦增量**：

- 函数 ccn 超天花板 且**不在**账本 → FAIL（新增违规）
- 在账本 但复杂度**上升** → FAIL（存量恶化）
- 在账本 且持平 → 既有债， tolerated（只许降不许升）

**不要删它**：删掉会让所有既有高复杂度函数变成"新违规"而全部 FAIL。重构降复杂度后跑
`ccn_gate.py seed` 重新记账，账本只许往下走。

## 常用 CLI（Justfile）

`just` 封装了高频命令，见仓根 `Justfile`：

```bash
just gate-build      # cargo build --release，产物落 .githooks/gate
just gate-test       # gate crate 全量测试
just gate-push       # gate-sync 推 8 个成员仓（custom/ 受保护）
just agents-push     # agents push canon（重新组装本仓 AGENTS.md）
just spec-sync       # specs/ → .githooks/spec/ 正本同步部署镜像
just review          # gate check 全套自检
```

改规范的标准动线：改 `specs/` 或 `agent-pack/` → `just spec-sync`（如动 gate spec）
→ `just gate-build && just gate-push`（如动二进制）→ `just agents-push`（如动 agent 文档）。
