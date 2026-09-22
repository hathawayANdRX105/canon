# 任务书：功能开发全流程（dev）

**什么时候用**：从零开发一个功能 / 修复的完整流程——建 worktree、draft PR、拆子任务、
派子代理、审查、测试、合并。PR 已就绪只需收尾的，用 `.agent/tasks/closeout-pr.md`。

---

## 规范（必须遵守，你是主控）

- 你是主控 agent。你编排任务、派子代理执行、审查子代理产出，**不要亲自把核心实现写完**。
- 开工先进 goal 模式（omp `goal` 工具：objective = 本次功能与验收，完成前不退出）；
  建 todo（每个子任务一条，完成即勾）；每个子代理 prompt 必须带一个明确 goal。
- **工作目录门禁**：所有子代理必须在 `.wt/<branch>` 工作；主控先确认 / 创建 worktree
  （与 draft PR 一起就绪后才派子代理），子代理 prompt 必须写明 `cwd=.wt/<branch>`，
  禁止在仓库根目录写入。
- **任务量门禁**：单个子任务 ≤ 5 个文件、单一主题、单一修改范围；
  能按文件 / 范围 / 主题 / 调用链 / 测试拆就拆，不把半个模块丢给一个子代理。
- **codegraph 门禁**：scope 阶段主控用 codegraph 找到改动范围（suspect area 写进子任务 prompt，
  不进验收项）；**子代理动手前必须先调用 codegraph skill（`cg`）了解清楚自己子任务的范围**——
  改动点、调用方 / 被调方、相邻边界——摸清再写代码；
  纯配置 / 文档类子任务（无符号图）豁免：退化为对 suspect area 做定向 grep / 读。
- **测试全部放 PR CI 跑**；本地只跑 <2 min 快速针对性检查；
  确需本地执行的重命令（build / install / bundle）必须 `cpulimit -l 65 -i --`。
- 审查走 **CRG（结构层面）+ ocr（规范层面）双层**；ocr 必须按文件 / 模块分批调用，不许一次喂全 repo。
- 每轮「审查 + 修复」写 **一条** PR comment（先列问题、再写修复，附修复 commit SHA 与验证命令）；
  smoke 验证再单独 **一条** comment。两种留言可多次出现。
- `gh` 命令必须走 `~/.local/bin/gh` 拦截版；gate 打出的拦截 / FAIL 信息**不许忽略**——
  FAIL 即停手，按提示修正后重过 gate，通过才继续；不绕过 `.githooks/`，
  merge 前必须跑过 `hooks/merge --dry-run` 预检。
- **终止条件**：任何循环（dev→audit、CI、ocr、smoke、gate 重试）同一问题修 2 轮仍不过 →
  停下向用户报备已试过的方案，不无限循环。

## workflow（按阶段执行）

### 0. setup

- 从目标 base 拉 `<branch>`，worktree 放 `.wt/<branch>`；不在仓库根目录改。
  创建规则与防嵌套自检见 `AGENTS.md`「开发方式」。
- worktree / 分支就位后立即建 draft PR（无 commit 先 `git commit --allow-empty` 占位，
  squash 后不留痕）；此后审查 / 修复 / smoke 的 comment 全记在这条 PR，merge 前转 ready。
  建不起来（worktree 冲突 / gate FAIL）→ 停下报备，不硬绕。
- 建 todo；与其他在跑会话的工作面重叠时停下问用户。
- 记录 `base_sha`，后续 CRG / diff review 用 `--base <base_sha>`，不要写死 `main`。

### 1. scope

- 跑一次 `codegraph update`；找到本次要动的模块、调用方、被调用方、相邻边界。
- 输出：suspect area、风险点、可能波及的文件清单（写进 PR body / 子任务 prompt）。

### 2. break down

- 先按文件拆；同文件内再按修改范围拆；仍然太大就按主题 / 调用链 / 测试拆。
- 每个子任务 prompt 必须写清：`cwd=.wt/<branch>`、开发什么功能（goal）、补什么测试
  （**逻辑层**：单测 / 断言，进 CI；形式跟仓库现有测试一致，没有测试框架就退化为可复验的验收命令）、
  验收条件（哪条命令跑通 = 完成）、smoke 要验证的功能路径（**功能层**）；
  外加允许修改 / 禁止触碰的文件清单（仓库相对路径），以及含风险点 + 可能波及文件清单的 suspect area。
- **子代理开工前必须先用 codegraph skill（`cg`）了解清楚任务范围，再动手**。
- 不相信子代理会自动完成：每个子任务都要有主控可复验的 diff 边界和验收证据。
- 派单前主控自查：goal 具体到可执行粒度（名称 / 定义 / 取值写全，不留「加 3 个 abbr」这种半句）；
  验收命令与允许 / 禁止清单自洽——验收要动的路径必须在允许清单内；
  没有测试框架的退化验收命令写进 PR body 验证方式。
- 跨模块协调、文件边界不清 → 继续拆；禁止一个子代理干完半个模块。

### 3. dev → audit

```text
loop1:
  dev   → 派子代理按划分任务做，最多并行 2 个子任务，且仅当文件不相交、无调用链耦合；
          同文件 / 同模块 / 紧耦合写入必须串行，拿不准就串行
  audit → 主控（你）独立校验：真跑子代理给的验收命令（不只看输出）、
          diff 只落在声明文件、查 root cause / 调用方 / 边界输入；
          改动跨模块 / 高风险时再派一个校验子代理交叉 confirm
失败 → 重拆或回 dev；子代理失联 / 产出废 → 重派一次，仍败主控接手该子任务或报备
```

### 4. test

- 全部子任务过 audit 后，本地只跑 <2 min 快速针对性检查；其余一律推 PR CI。
- **CI 未绿不得进入 review / merge**；CI 失败 → 当新子任务回 loop1，修到绿。
- test 只保证**代码逻辑**；**功能正确性由 smoke 兜底**，CI 绿 ≠ 功能正确。

### 5. review（CRG + ocr）

- CRG：`code-review-graph detect-changes --brief --base <base_sha>` 确认改动范围与风险，逐条过。
- ocr：按 PR diff / 模块分批喂。
- 发现 bug / problem → 回 loop1 修复 → 重跑审查，直到干净。
- 每轮（审查 + 修复）→ 1 条 PR comment（格式见规范）。

### 6. smoke

- smoke 是**功能层**的最终验证：真实用户路径跑一遍（CLI 命令 / 真实 URL / 真实进程；
  UI 截图或 OCR 对比），确认功能真的正确，不许拿 CI 绿替代；判据要可脚本化、主控可复跑。
- 发现问题 → 更新 todo → 回 loop1 二次修复。
- 通过 → PR 写一条 smoke comment（验证方法 + 结果）。

### 7. tidy

- **file/dir**：分支里没有跟本次无关的杂物（旧脚本、临时文件、废弃产物），
  `gio trash` 移入回收站或加 `.gitignore`（**严禁 `rm` / `git clean`**）。
- **code**：测试代码进 `tests/`；跑 formatter；清调试 log、commented-out code；
  formatter 如改了文件 → 重跑验收命令、review、smoke，更新 PR comment。
- **docs**：同步代码注释、`AGENTS.md` / `README.md` / `docs/` 过期段落，引用与新增一致。

### 8. merge

- PR：title 用 conventional commit，body 写改动清单、验证方式、suspect area。
- merge 前置全满足：CI 全绿；审查 + smoke comment 齐全；
  `git diff --name-only <base>..<branch>` 复核改动只落在声明文件；
  `hooks/merge --dry-run` 通过；gh gate 无拦截。
- 通过后把 draft 转 ready，`gh pr merge <N> --squash --delete-branch`（远端 + 本地分支一并清掉）。

### 9. cleanup + report

- merge 后清理：`--delete-branch` 已清本地 + 远端分支；
  确认 `.wt/<branch>` 工作树目录已删——残留用 `git worktree remove` 清（**严禁 rm**），
  只清本会话自己建的。
- report：PR 链接、改了哪些文件、跑了哪些测试、CRG / ocr / CI / smoke 结果、
  剩余风险（含未跑的测试与已知问题）。
