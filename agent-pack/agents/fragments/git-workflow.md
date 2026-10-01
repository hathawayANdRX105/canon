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

## 提交身份

- commit 作者固定是维护者本人账号 `hathawayANdRX105`（大小写逐字一致）。
- **不得**用 `git -c user.name=... -c user.email=...` 覆盖身份提交。历史上
  `agent@local` / `ci@local` 这类签名就是这么来的：GitHub 账号对不上，
  贡献归属、追责、审计全丢。
- 提交前若 `git config user.name` / `user.email` 不是上面这个账号，先改成本仓配置
  （`git config user.name hathawayANdRX105`），别带着错的身份往下走。
- 邮箱两套都算合法：`2635254302@qq.com`（本地提交）与 GitHub 的
  `61958173+hathawayANdRX105@users.noreply.github.com`（服务端 squash 落库时写的）。
- 禁止 `Co-authored-by:`  trailer 署其他人或机器人账号。

## Issue

- 标题中文；正文 heading 英文、内容中文。
- sub-issue 必须自包含：正文不写 `Parent:` / `Related:` / PR 占位符，直接写清它要什么。
- 关闭前 `Done when` 的 checkbox 全勾。

## PR

- 标题纯英文（conventional commit 风格）；正文小节标题英文、内容中文。
- 正文按仓库模板（`.github/PULL_REQUEST_TEMPLATE.md`）写：背景 / 改了什么 / 为什么 /
  实现步骤 / 交付记录 / 怎么验证 / 检查清单。
- 关联 issue 用 `Fixes #<n>` 收尾行；draft 阶段用 `Related #<n>`，合并授权前改 `Fixes`。
- 开启或更新 PR 后看 CI 结果到底（`gh pr checks`），红了就修，不等用户来问。
- 被 canon 拦下就修代码，**不改规则**。规则确有缺陷 → 开 issue 交维护者裁决。

## 合并

- **只走 squash merge**：
  `gh pr merge <N> --squash --delete-branch --body "Agent 🤖 - Merge: <原因>"`。
- 禁用 `--merge` / `--rebase`（含 `-m` / `-r` 短形式）。merge commit 会让 PR
  记录的分支历史消失，同一分支再合要重新三方合并、当初的冲突裁决全部丢失；
  rebase-merge 还会逐个改写 commit 作者。两者都让 `main` 失去审计价值。
- 不带任何合并方式的 `gh pr merge` 会弹交互菜单 —— agent 不该触发交互，一律显式
  写 `--squash`。
- 禁止本地 `git merge <分支>` 直接合进 `main` 再推 remote。要合就走 PR。
- 各仓 GitHub 设置已关闭 merge commit 与 rebase merge，squash 是唯一可选项。

## 收尾

- 收尾时清掉：已合并分支、临时 worktree、临时进程、跑完的 dev server。
- 资源及时释放；只保留维护者需要的进程（如用户要看的 web 前端）。
