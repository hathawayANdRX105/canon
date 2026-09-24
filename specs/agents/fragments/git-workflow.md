# 提交与 PR

- commit 标题走 conventional commit（`feat:` / `fix:` / `refactor:` / `docs:` / `chore:` / `test:` / `ci:` / `build:` / `perf:` / `style:` / `revert:`）。
- commit 标题**用英文**，正文可用中文。
- 提交 / 推送前先跑对应检查：`gate pre-commit` / `gate pre-push`，不靠推送失败才发现。
- 创建 PR / issue 前先读模板（`.github/PULL_REQUEST_TEMPLATE.md` / `.github/ISSUE_TEMPLATE/`），正文按模板写。
- PR 标题纯英文；正文小节标题英文、内容中文。
- 关联 issue 用 `Fixes #<n>` 收尾行。
- 被 gate 拦下就修代码，**不改规则**（`.githooks/` 属 gate 领地）。规则确有缺陷 → 开 issue。
