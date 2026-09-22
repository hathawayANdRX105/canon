# Web 双车道流程（web-dev / webfix 角色）

**什么时候用**：改动只落在 web 域（`crates/web/*`、`apps/admin-web`、`apps/tavern-web`）的
快速开发。其他域（gateway / harness / api / contract）不受影响，PR 照旧直提 `main`。

本文件是 `todo/web-dev-and-web-fix.md`（维护者原始策略）的修订版，补了四个洞：
main→web-dev 同步干线、main 接受的域限定、web-fix 是角色不是分支、squash 机制说明。
**两份冲突时以本文件为准。**

任务书：开发用 `.agent/tasks/dev-web-lane.md`；审查修复用 `.agent/tasks/webfix-lane.md`。

---

## 分支模型

```text
main ──merge（单向同步）──> web-dev ──PR merge-commit（单向发布）──> main
                              │
                              ├── feat/xxx ──squash 合回──┐
                              └── webfix/xxx ──no-ff 合回─┘
```

- **web-dev**：唯一长期分支，web 域的唯一集干/发布源。
- **webfix 不是分支，是角色**：一个会话 + 一个 worktree（`.wt/web-fix`），
  从 web-dev 切短分支 `webfix/xxx`，修完合回 web-dev 即删。不建长期 web-fix 分支。
- 全流程只有两条长距离单向边：`main → web-dev`（同步）、`web-dev → main`（发布）。
  任何方向的第二遍都是混乱源，禁止。

## 一次性初始化（主检出执行）

```bash
cd /home/hathaway/projects/ferrite          # 仓库根，防 .wt 嵌套事故
git checkout main && git pull --ff-only
git checkout -b web-dev && git push -u origin web-dev
git worktree add .wt/web-dev web-dev        # 开发角色
git worktree add --detach .wt/web-fix       # 审查角色：常驻 detached（同一分支不能同时 checkout 两个 worktree），每轮循环切 webfix/xxx
git worktree list                            # 自检：新条目路径必须是 .wt/ 下且不出现第二个 .wt/
```

两个 web worktree **共享一个编译目录**（避免每开一个 worktree 付一次冷 wasm target）：

```bash
export CARGO_TARGET_DIR=/home/hathaway/projects/ferrite/target-web   # 两个车道的会话各自 export
```

cargo 用文件锁串行化并发构建；registry 依赖（dioxus 等）只编一次。

## 循环 A：开发（角色 = dev agent）

```bash
cd /home/hathaway/projects/ferrite/.wt/web-dev
git checkout web-dev && git merge main          # 同步干线：每轮开发开始前必做
git checkout -b feat/xxx                        # 从 web-dev 切（内容 ≈ 刚同步过的 main）
# 改 → just dev-web-rebuild <port> debug → 浏览器强刷验证
git commit ...                                  # conventional commit，message 写清「为什么」
git checkout web-dev && git merge --squash feat/xxx
git commit -m "feat(web): ..."                  # 合回即一条干净 commit
git push origin web-dev && git branch -d feat/xxx
```

## 循环 B：审查修复（角色 = review agent）

```bash
cd /home/hathaway/projects/ferrite/.wt/web-fix
git fetch origin && git checkout -b webfix/xxx origin/web-dev   # 从最新 web-dev 切
# 收集上下文：git log/diff <base_sha>..origin/web-dev、ainotation 反馈、跑起来看
# 审查（CRG + 人工）→ 修复 → 测试/e2e → 合回：
git checkout web-dev && git pull --ff-only
git merge --no-ff webfix/xxx                    # 保留审查轨迹，不 squash
git push origin web-dev && git branch -d webfix/xxx
```

`base_sha` = 上一次发布（web-dev→main）时记下的 web-dev tip。每次发布必须更新并写进
PR 正文和汇报；审查循环靠它界定「这一轮要审什么」。

## 循环 C：发布（web-dev → main）

```bash
cd /home/hathaway/projects/ferrite/.wt/web-dev
git checkout web-dev && git pull --ff-only
git merge main                                   # 先同步 main（其他域的提交），冲突双保留
git push origin web-dev
# 建 PR：base = main，head = web-dev，merge commit（不 squash），挂 type label
# PR 正文：改动清单 + base_sha + 审查结论（CRG comment，由 review 角色产出）
# CI 全绿 + gate 无 FAIL → merge；merge 后记录新 base_sha（新 web-dev tip）
```

**squash 禁令**：web-dev → main **禁止 squash**。squash 不记录祖先关系：下一次再合
同一个 web-dev 时 git 仍以当初分叉点做三方合并，上次 squash 的冲突裁决全部丢失、
重新冲突，且 main 上看不到 web-dev 真实历史。想要干净 main 只能从 web-dev 切一次性
`release/xxx` 再 squash（用完即删）——快速车道不加这层仪式，默认 merge commit。
`feat/xxx → web-dev` 可以 squash（trunk 干净，意图留在 commit message 和 PR 记录）；
`webfix/xxx → web-dev` 用 `--no-ff`（审查轨迹是记录本体）。

## 合并纪律

- 操作前 `git status` 干净；同步用 `git pull --ff-only`。
- **禁止**对 web-dev / main rebase 后 force push；禁止 web-dev 与其他分支双向反复 merge。
- 冲突解决保留双方有效修改，不丢代码；拿不准停下报备。
- 每轮循环结束按任务书的汇报格式报告（分支/提交/合并/push/冲突）。

## Gate 门禁交互（已核对，不挡快速开发）

本地 commit/push 跑 `pre-commit`/`pre-push`（workspace + code + checklist + doc_sync + code_doc），
merge 期跑 `github/pr_gates` + clippy 等。结论：

| 时机 | 门禁 | 对本流程的影响 |
|---|---|---|
| 每次本地 commit/push | doc_sync（新 crate 必须配 README）FAIL | 新 web crate 记得同时建 README |
| 每次本地 commit/push | shared_components_check（≥2 个 page 共用的组件必须进共享 crate）FAIL | 组件被第二个 page 用时主动搬 `ui-components` |
| 每次本地 commit/push | structure_check（面板禁直用 `mock::`）FAIL / copy_constants WARN | 老规矩，见 `.agent/rules/gates.md` |
| 仅 web-dev→main 的 PR | pr_labels FAIL（type label）、pr_crg_review FAIL（PR 需 CRG 结论评论） | **review 角色的产出就是这条评论**，闭环 |
| 仅 merge 期 | clippy FAIL、crg_impact WARN（diff 跨 3+ crate） | webfix 角色发布前本地跑一次 clippy |

**pr 级门禁只在发布 PR 触发，不碰开发/修复循环的本地提交**——快速开发不被 gate 拖慢，
唯一gate化的动作就是发布，而那正是审查角色的交付物。动手前仍须读 `.agent/rules/gates.md`。

## 已知坑

- `.wt/web-fix` 常驻 detached HEAD（git 不允许同一分支 checkout 进两个 worktree），
  每轮循环从 `origin/web-dev` 切新 `webfix/xxx`，不在旧 webfix 分支上续。
- 共享 target 时两个车道同时构建会互相等锁（cargo 串行），属预期，不要 kill 对方的 cargo。
- dx / ainotation / 后端的具体启动命令在任务书里，不在本文件重复。
