# 暂存：wf-* 工件目录改 `.workflow/`（未部署）

**状态**：✅ 已部署（dotfiles `3b41d0b` / `b741236`）。补丁留档作变更证据，勿重复应用。
**原因**：改动会经 `~/.omp/agent/skills/wf-*` 的软链实时进入正在运行的 omp 会话；
会话中途换 skill 内容会让在途任务读到半新半旧的规则。等人手空时再部署。

## 改动内容

把 wf-* 九件套写进项目的工件从 `docs/` 挪到点目录 `.workflow/`，避免污染项目文档树
（也不会被 docusaurus / vitepress 之类的文档站构建收进去）：

| 原路径 | 新路径 | 所有者 |
|---|---|---|
| `docs/scope/` | `.workflow/scope/` | wf-scope |
| `docs/specs/` | `.workflow/specs/` | wf-architect |
| `docs/reviews/` | `.workflow/reviews/` | wf-check |
| `docs/audit/` | `.workflow/audit/` | wf-audit |
| `docs/releases/` | `.workflow/releases/` | wf-document |
| `docs/postmortems/` | `.workflow/postmortems/` | wf-document |

21 个文件、82 处路径替换，外加三处 `Artifact base` 规则重写（`wf-architect/SKILL.md`、
`wf-scope/SKILL.md`、`wf-check/modes/review.md`）：新规则明确「默认 `.workflow/`，绝不新建
顶层 `docs/`」。

**保留一条例外**：已经在用 `docs/scope/` 或 `docs/specs/` 的老仓继续用原基目录，不迁移在途
工件。当前命中 4 个仓：`claude-code`、`deskctl`、`herdr`、`oh-my-pi`。

未动 `verify.md` 与 `test-preferences.json`（仓根的开发者入口文件，不是 wf-* 过程工件）。

## 补丁文件

`0001-wf-workflow-dotdir.patch` — dotfiles 仓提交 `237cffb` 的完整补丁，21 files changed,
61 insertions(+), 61 deletions(-)。

## 部署步骤（有空时做）

1. 确认 omp 没有在跑的 wf-* 任务（`hub list` 无 wf agent，或直接新开会话）。
2. 在 `~/projects/dotfiles` 应用补丁：
   ```bash
   git am specs/pending-wf-workflow-dotdir/0001-wf-workflow-dotdir.patch
   ```
   若 `git am` 冲突（skill 期间被别处改过），改用 `git apply -3` 或按 diff 手工并。
3. 同步 canon 侧的工件矩阵（本次也已回滚，部署时一并应用）：
   - `specs/rules/workflow-state.md` 的「目录与所有权矩阵」表与判位节
   - `specs/skills/jev.md` 的 WARN 驳回落点
4. `./bin/agent-sync push <项目>` 分发 `workflow-state.md`。
5. 验证：新开一个 omp 会话，`/wf-scope` 跑一次，确认工件落在 `.workflow/scope/` 而非
   `docs/scope/`。

## 回滚记录

- dotfiles：`237cffb` 已由 `98129d6` revert（dev 分支）
- canon：`794df0b` 已由 `104727e` revert（main 分支）

两仓 revert 提交尚未推送；推送与否由你决定（本地已生效，omp 已恢复旧行为）。
