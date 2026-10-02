# canon 约定

canon 是**规范与 agent 文档的正本仓**。别的项目想改规范、改 agent 约定，来这里改，再分发回去。

## 这个仓是什么

- 规范正本：`agent-pack/rules/`、`agent-pack/skills/`、`specs/`（spec 正本，`canon init` 的 seed 源）
- 人查手册与任务书：已全部转成 dotfiles skill（gate-spec / pr-orchestration / task-brief / closeout / worktree-isolation）
- agent 文档正本：`agent-pack/agents/`
- canon 源码：**canon 仓根本身就是 Rust crate**（仓根 `src/` + `tests/` + `Cargo.toml`）；
  `cargo build --release` 产物 `target/release/canon` 复制到 `.githooks/canon`，再由 canon-sync 分发到各项目
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

## 分发到成员仓：`.wt` 工作树 + squash merge

canon 的产物（AGENTS 生成物、`.githooks/`、任务书与规则文档）推进成员仓时：

- **不在目标仓主工作树里直接改**。主树常挂着他人的在制改动，直接写会把它卷进这次提交；
- **不留散乱提交**。落地一律 squash merge，项目历史只多一条干净提交。

标准动线（以目标仓 `<项目>` 为例）：

```bash
cd ~/projects/<项目>
git worktree add .wt/<编号>-<描述> -b <type>/<描述>-<编号> origin/main

# 产物落进 worktree（AGENTS 生成物示例；戳行与 `agents push` 写出的格式一致）
{ printf '<!-- managed by canon agents.yaml @ %s -->\n' "$(date +%F)"
  python3 ~/projects/canon/scripts/agents build <项目>; } > .wt/<编号>-<描述>/AGENTS.md
# agent-sync / canon-sync 的产物同理：先落进该 worktree，别碰主树

git -C .wt/<编号>-<描述> add -A && git -C .wt/<编号>-<描述> commit -m "<type>(agents): …"

# 落地：回主树 squash merge 到默认分支，然后清理
git merge --squash <type>/<描述>-<编号> && git commit
git worktree remove .wt/<编号>-<描述> && git branch -d <type>/<描述>-<编号>
```

目标仓自己的合并约定优先：若该仓明确禁止 squash（如 ferrite `web-dev → main` 走 merge commit），
按目标仓规则走，不套本条。目标仓若没忽略 `.wt/`，先在它的 `.gitignore` 补一行 `.wt/`，
否则 worktree 会以未跟踪目录形式出现在项目状态里。worktree 的建立/清理细节见
dotfiles `worktree-isolation` skill。

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
just build      # cargo build --release，产物落 .githooks/canon
just test       # canon crate 全量测试
just push       # canon-sync 推 8 个成员仓（custom/ 受保护）
just agents-push     # agents push canon（重新组装本仓 AGENTS.md）
just spec-sync       # specs/ → .githooks/spec/ 正本同步部署镜像
just review          # canon check 全套自检
```

改规范的标准动线：改 `specs/` 或 `agent-pack/` → `just spec-sync`（如动 spec）
→ `just build && just push`（如动二进制）→ `just agents-push`（如动 agent 文档）。
