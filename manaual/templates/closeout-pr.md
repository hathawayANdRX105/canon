# 任务书：PR 收尾与合并（closeout-pr）

**什么时候用**：PR 已就绪（代码改动已全部提交、CI 在跑）之后的收尾全流程——
审查、smoke、修复、合并、清理。两种场景都适用：
(a) 你刚完成开发，进入收尾；(b) 接手一个别人已就绪的 PR，负责把它合掉。

完整开发流程（从零开始）见 `.agent/rules/pr-workflow.md`；闸门细则见 `.agent/rules/gates.md`。

---

## 规范（必须遵守，你是主控）

- 你是主控 agent。你审查 PR、派子代理修复问题、记录到 PR、合并、清理。**不要开发新功能**。
- **工作目录门禁**：所有子代理必须在 `.wt/<branch>` 工作；主控先确认 / 创建 worktree，
  子代理 prompt 必须写明 `cwd=.wt/<branch>`，禁止在仓库根目录写入。
- **任务量门禁**：单个子任务 ≤ 5 个文件、单一主题、单一修改范围；
  能按文件 / 范围 / 主题 / 调用链 / 测试拆就拆，不把半个模块丢给一个子代理。
- CPU-heavy 命令（build / test / install / bundle）必须 `cpulimit -l 65 -i --`；
  本地只跑 <2 min 快速针对性检查，其余测试一律推到 PR CI。
- 审查走 **CRG（结构层面）+ ocr（规范层面）双层**；ocr 必须按文件 / 模块分批调用，
  不许一次喂全 repo。
- 每轮「审查 + 修复」写 **一条** PR comment：标题 `Agent 🤖 - <topic>`，
  正文先列发现的问题、再写修复情况（附修复 commit SHA 与验证命令）；
  smoke 验证再单独 **一条** PR comment。两种留言可多次出现。
- `gh` 命令必须走 `~/.local/bin/gh` 拦截版；gate 打出的拦截 / FAIL 信息**不许忽略**——
  FAIL 即停手，按提示修正后重过 gate，通过才继续（细则见 `.agent/rules/gates.md`）。
- 不绕过 `.githooks/`，merge 前必须跑过 `hooks/merge --dry-run` 预检。
- **终止条件**：任何循环（fix→audit、CI、ocr、smoke、gate 重试）同一问题修 2 轮仍不过 →
  停下向用户报备已试过的方案，不无限循环。

## 前置条件

- PR 已存在，base 仓库正确。
- 所有代码改动已提交到 `<branch>`。
- 工作树在 `.wt/<branch>`（无 worktree 需求可跳过）。
- `base_sha` 已记录（用于 CRG diff，不要写死 `main`）。

## workflow（按阶段执行）

### 1. review（CRG + ocr）

- `code-review-graph detect-changes --brief --base <base_sha>` 确认改动范围与风险，逐条过。
- ocr 按 PR diff / 模块分批喂。
- 发现 bug / problem → 回 fix 修复 → 重新 review，直到干净。
- 每轮（review + fix）→ 1 条 PR comment（含发现、修复 commit、验证命令）。

### 2. smoke

- 真实用户路径跑一遍：CLI 命令 / 真实 URL / 真实进程；UI 截图或 OCR 对比。
- 发现问题 → 回 fix 做二次修复。
- 通过 → 在 PR 写一条「smoke 验证通过 / 用的方法 / 结果」comment。

### 3. fix → audit（审查 / smoke 发现的问题）

```text
loop1:
  fix   → 派子代理按问题范围做，最多并行 2 个互不冲突子任务；
          同文件 / 同模块写入必须串行
  audit → 主控（你）独立校验：
          - 跑子代理提供的验收命令（真跑，不只看输出）
          - diff 看改动是否只落在声明的文件
          - 检查 root cause、调用方、边界输入
失败 → 重拆或回 fix
```

### 4. test

- 全部问题修复后，本地只跑 <2 min 快速针对性检查；其余一律推 PR CI。
- **CI 未绿不得进入后续阶段**；CI 失败 → 当新问题回 loop1 修复，重新推送直到 CI 绿。

### 5. tidy

- **file/dir**：分支目录里没有跟本次无关的杂物（旧脚本、临时文件、废弃产物），
  要么 `gio trash` 移入回收站、要么加 `.gitignore`（**严禁 `rm` / `git clean`**）。
- **code**：formatter 跑一遍；无调试 log、commented-out code、调试 surrogate。
  formatter 如修改文件，必须重跑最小验收命令、review、smoke，并更新 PR comment。

### 6. merge

- 前置全满足：CI 全绿；审查 + smoke comment 齐全；
  `git diff --name-only <base_sha>..<branch>` 复核改动只落在声明文件；
  `hooks/merge --dry-run` 通过；gh gate 无拦截。
- 通过后把 draft 转 ready，`gh pr merge <N> --squash --delete-branch`（远端 + 本地分支一并清掉）。

### 7. cleanup + report

- merge 后清理：`--delete-branch` 已清本地 + 远端分支；
  确认 `.wt/<branch>` 工作树目录已删——残留用 `git worktree remove` 清（**严禁 rm**），只清本会话自己建的。
- report：PR 链接、改了哪些文件、跑了哪些测试、CRG / ocr / CI / smoke 结果、
  剩余风险（含未跑的测试与已知问题）。
