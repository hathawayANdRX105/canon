# AGENTS.md — deskctl

接手开发的 agent 必须先读这些约定。基础信息见 `README.md`。

## 分支与 worktree（最高优先）

- **默认分支是 `main`**，不是 dev。功能从 `main` 拉分支，PR 合回 `main`。
- **所有开发在 `.wt/<分支名>/` 里做**，不要在仓库根目录直接改。`.wt/` 已 gitignore。
  ```bash
  git worktree add -b <branch> .wt/<branch> main   # 开分支
  git worktree remove .wt/<branch>                  # 收尾
  git branch -D <branch>
  ```
- 提交前清理：已合并的 feature/epic 分支和 worktree 要删掉，不要留 stale 分支。

## Workflow 门禁

- PR 必须过 `python .githooks/github/pull_requests.py <owner/repo> <pr>`（`ALL PASS`）。
- issue 必须过 `validate_issues.sh <owner/repo> <parent> <sub...>`（`ALL PASS`）。
- issue 标题中文、正文 heading 英文、正文中文；sub-issue 自包含（无 Parent/Related/PR 占位）。
- PR 标题 conventional commit；draft 用 `Related #N`，合并授权前改 `Fixes #N`。
- 合并前跑 CRG：`code-review-graph update` + `detect-changes --base main`。

## 构建与验证（CI 驱动）

**所有测试、全量构建、lint 全部放 PR 的 CI（`.github/workflows/ci.yml`：fmt + clippy(-D) + test），本地不跑重型命令。**

本地只做轻量验证：
- `cargo fmt --check`（秒级）
- `cargo check -p deskctl`（类型检查，单 crate）
- `grep` / `ls` / 文件读写等只读命令

**如果实在要本地跑重命令**（`cargo test --workspace`、`cargo clippy`、`cargo build --release`、`install` 等），**必须套 `cpulimit -l 65 -i --` 限制 CPU 到 65%**：

```bash
cpulimit -l 65 -i -- cargo test --workspace --all-targets
cpulimit -l 65 -i -- cargo clippy --workspace --all-targets --all-features -- -D warnings
cpulimit -l 65 -i -- cargo build --release -p deskctl
cpulimit -l 65 -i -- install -Dm755 target/release/deskctl ~/.local/bin/deskctl
```

`git`、`grep`、`ls` 等轻量命令不需要套。

## 关键约定 / 坑

- snippets 模板目录 `~/.config/deskctl/snippets/<topic>/<template>`，正文逐字节原样，别加/丢换行。
- 复制走 `wl-copy`（`DESKCTL_WL_COPY` 可注入 fake），自动粘贴走 `wtype`（缺失则仅复制，不报错）。
- PTY 里 smoke 时设 `DESKCTL_SYS_DRY=1`，否则 Dock 会因 Mango 焦点丢失 900ms 后自动退出。
- 单一面板 `SnippetsPanel` 自带 topic tabs + 模板列表 + 预览；`l`/`h` 切 tab、`j`/`k` 选模板、`Enter` 复制、`Esc`/`q` 退出。
- 改 `panel-kit` 时别动主 Dock 其它面板的默认行为；snippets 用 `hide_tabs()/hide_status()` 只影响自己。
- 计划中的管理 CLI（未实现）：`deskctl snippets topic|template|link|unlink`（见 epic #56）。
