# 提交与 PR

## 分支

- 默认分支是 `main`（本仓若不同以本仓为准），功能从默认分支拉。
- 一个任务一个分支，分支名带类型前缀（`feat/` / `fix/` / `refactor/` / `chore/`）。
- 合并后清理已合并分支与 worktree，不留 stale 分支。

## Commit

- 标题走 conventional commit（`feat:` / `fix:` / `refactor:` / `docs:` / `chore:` /
  `test:` / `ci:` / `build:` / `perf:` / `style:` / `revert:`）。
- 标题**用英文**，正文可用中文。
- 一个 commit 一件事。不把无关改动、格式化噪声、生成物混进逻辑改动。
- 提交前跑对应检查（`canon pre-commit` / `canon pre-push`），不靠推送失败才发现。

## Issue

Issue 是**追踪单元**，不是 PR 的前置条件——默认开发流是纯 PR 开发，不要求先建 issue。
只有这些情况才建 issue：记录遗留/暂缓事项、登记需要后续开发的工作、留下需要检索的决策记录。

- 标题中文；正文 heading 英文、内容中文。
- sub-issue 必须自包含：正文不写 `Parent:` / `Related:` / PR 占位符，直接写清它要什么。
- 关闭前 `Done when` 的 checkbox 全勾。

## PR

- 标题纯英文（conventional commit 风格）；正文小节标题英文、内容中文。
- 正文按仓库模板（`.github/PULL_REQUEST_TEMPLATE.md`）写：背景 / 改了什么 / 为什么 /
  实现步骤 / 交付记录 / 怎么验证 / 检查清单。
- 不强制关联 issue：确实在关闭某个 issue 时才写 `Fixes #<n>`（一个 PR 只关一个）；
  纯 PR 开发什么都不用写。审查发现的问题在同一 PR 上继续提交修复，不另开 issue/PR。
- 开启或更新 PR 后看 CI 结果到底（`gh pr checks`），红了就修，不等用户来问。
- 被 canon 拦下就修代码，**不改规则**。规则确有缺陷 → 开 issue 交维护者裁决。

## 收尾

- 收尾时清掉：已合并分支、临时 worktree、临时进程、跑完的 dev server。
- 资源及时释放；只保留维护者需要的进程（如用户要看的 web 前端）。
