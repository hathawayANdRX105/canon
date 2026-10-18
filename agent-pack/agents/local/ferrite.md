# Ferrite 工作约定

## 开始工作前（按顺序读）

1. 读根 `README.md` 的 crate 清单，确定当前结构与依赖（模块职责的唯一真身）。
2. 按任务书 / issue 的文件清单工作，用其验收命令验证后提交 conventional commit。
3. 本文件只写**硬性约束**与**场景 → 文档导航**；操作细节都在 `.agent/rules/` 下，按导航去读，不要一开始全读。

---

## 闸门：动手前先自查，别等提交后被拦

维护者口径：**审查与查规范走 canon 命令**，不要靠撞闸门反推规矩。

| 命令 | 什么时候用 |
|---|---|
| `canon list` | 想知道有哪些规范：规则 ID / 查什么 / 严重度 / 拦在哪个阶段（`--json` 给 agent 读） |
| `canon check --sla l1` | 提交前本地预检（会拦 pre-commit / pre-push / merge 的那层） |
| `canon pr check --body-file <正文>` | PR 正文写好先本地验（heading 齐不齐、是否含中文 heading、checkbox 够不够），别等 `gh pr create` 被拒 |

被拦下时闸门会打全**所有** finding（不是遇到第一条就停）并给出下一步命令；**逐条处置**，禁
`--no-verify`、禁截断输出、禁改规则（规则有缺陷 → 上报维护者裁决）。

---

## 开发方式（worktree）

- 每个会话 `cd {{root}}` 再 `git worktree add .wt/<name> -b <branch>`（目录名 = 分支尾段）。
  **在 worktree 里执行会嵌套**，自检 `git worktree list` 出现第二个 `.wt/` 立即 `remove` 重来。
- 建完分支立刻 `git push -u origin <branch>`：带远端 start-point 的 `worktree add` 会把 upstream
  设成那个 start-point，一次 `git push` 就打到 **main 或别人的分支**。
- 派子代理写**全局绝对路径**（`{{root}}/.wt/<name>/`），禁止让它推导相对路径。
- `.wt/` 删除保护：只删**本会话创建**且 PR 已合并的 worktree（`git worktree remove`）；
  严禁 `rm -rf .wt/`、`git clean`——那是别人的开发会话，等同删库。

---

## 本机环境（只有红线，细节读导航表）

- 共享后端 `127.0.0.1:3211` + 库 `uf-local-postgres/ferrite_smoke`：**严禁停 3211、严禁对共享库 `db-reset`**；
  启停只走 `scripts/dev-backend.sh`。需要独立验证时用 `FERRITE_DEV_LISTEN=127.0.0.1:<端口>` + 独立库。
- 长跑服务用**会话持久后台任务**（禁 `nohup ... &`）；起后 `ss -ltn` 验证端口。
- 禁宽匹配 `pkill -f cargo|rustc`；清理前 `readlink /proc/<pid>/cwd` 认归属。
- CPU 密集命令（编译 / 测试 / 装包）一律套 `.githooks/heavy`。

---

## 测试与 CI（入口只有两个）

- 本地：`just test-fast`（testless 影响分析，只跑受波及用例；异常/零命中会报警退出，不静默跳过）。
  **中等及以上测试禁止本地跑**（整包 / 整个 test binary / e2e 一律交壳仓 CI）。
- 合并门：PR 的 `shell-ci` 状态绿了才能 merge；全量（main push 后自动跑，写成 `shell-ci-full`）
  异步跟进、不阻塞合并，但红了要开修复任务并汇报。

---

## 硬约束

- **只能通过 PR 干活**；禁止新建 issue、禁止改 epic 结构（维护者明确要求时先报备，批准才建）。
- 合并只走 squash：`gh pr merge <N> --squash --delete-branch --body "Agent 🤖 - Merge: <原因>"`。
- 提交身份是维护者本人（`hathawayANdRX105`），不得用 `-c user.name=...` 覆盖，不得署 `Co-authored-by:` 机器人。
- `crates/harness-agent-prompts` 与 `crates/ui/` 下的前端 crate（含 `coding` 投影层）必须支持
  `wasm32-unknown-unknown`；`crates/harness-tools` **不在此约束内**（是否 wasm 化属待裁决）。
- 测试放 crate 同级 `tests/`，不在 `src/` 写 `#[cfg(test)]`；新增/移动功能 crate 同步根 `Cargo.toml` 与 README。
- 凭据 / 真实 IP / 上游地址绝不入库、不进 PR 正文；示例用占位符（`<API_KEY>`、`127.0.0.1`）。

---

## 该读哪份文档

| 场景 | 读这里 |
|---|---|
| 开 PR 干活 / 派子代理 / worktree 全流程 | `.agent/rules/pr-workflow.md` |
| 写测试 / 跑测试 / 怀疑「CI 绿了但没验东西」 | `.agent/rules/testing-ci.md` |
| 提交被拦 / 建 PR 被拒 / 查规则 | `.agent/rules/gates.md`（规则总览 `gate-spec` skill） |
| 起后端 / 数据库 / 前端 / 进程卫生 | `.agent/rules/dev-env.md` |
| Dioxus UI / Rust 公共 API / 代码风格 / 调查审查 | `.agent/rules/conventions.md` |
| 任务编排与所有权矩阵 | `.agent/rules/workflow-state.md` |
| `.agent/` 目录组织 | `.agent/README.md` |
