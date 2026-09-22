# 任务书：web 审查修复车道（webfix 角色）

**什么时候用**：审查 `web-dev` 上自上次发布以来新落的 commit，修复问题，补测试/e2e，
并为 `web-dev → main` 发布 PR 产出 CRG 审查结论（pr_crg_review 门禁要的那条评论）。
流程总规则读 `.agent/rules/web-lanes.md`。

**角色定位**：只审、只修、不开发新功能。发现「该加新功能」→ 记进汇报交回 dev 车道，
不自行扩范围。

---

## 0. 收集上下文（防信息漏洞，第一步且必须做全）

你没有 dev 角色的对话历史，以下四样是全部事实来源，缺一项就可能审漏：

```bash
export CARGO_TARGET_DIR=/home/hathaway/projects/ferrite/target-web
cd /home/hathaway/projects/ferrite/.wt/web-fix
git fetch origin
git log --oneline <base_sha>..origin/web-dev      # 本轮要审的 commit 清单
git log -p <base_sha>..origin/web-dev             # 逐 commit 看改了什么；message 里的「为什么」是意图来源
git diff --stat <base_sha>..origin/web-dev        # 波及文件面
git checkout -b webfix/xxx origin/web-dev         # base_sha = 上次发布记录的 web-dev tip（PR 正文/上轮汇报里）
```

- `base_sha` 找不到 → 停下向维护者要，**不许拿 `main` 顶替**（会把别人域的提交卷进审查）。
- `.wt/web-fix` 是常驻 detached HEAD 的 worktree（git 不允许同一分支进两个 worktree），
  每轮循环从 `origin/web-dev` 切新 `webfix/xxx`，不在旧 webfix 分支上续。
- **用户原话/视觉意见**：读 ainotation 反馈（MCP `ainotation_get_feedback` /
  `ainotation_get_image`）——那是 dev 为什么改的直接依据。
- **功能真貌**：自己把环境跑起来看实际页面（启动命令同 dev 任务书 §1），
  审查结论必须包含「跑过的路径 + 看到的行为」，不许只读 diff 下结论。

## 1. 审查协议

- **结构层**：`code-review-graph detect-changes --brief --base <base_sha>`，逐条过影响面；
  diff 跨 3+ crate 时重点看耦合（发布期 crg_impact 是 WARN，但真耦合要指出）。
- **规范层**：按文件/模块分批人工读（禁一次喂全仓）：交互元素 `data-testid`、
  容器 `role`+`aria-label`、公共 API `///` 文档、模块头 `//!`、TODO 必须挂 issue 号。
- **交互改动必跑浏览器**：点一遍改动路径；涉及请求回路的（轮询、effect、分页）必须看
  真实后端回路——本仓库的忙轮询白屏、effect 自循环都只在真实回路复现。
- **测试补齐规则**：逻辑层改动补 Rust 测试进 `tests/`（本地只跑 <2min 针对性单测，
  全量交给 CI）；**交互层**分支若已有 `e2e/`（Playwright，见 #241），补/改 spec：
  ```bash
  just dev-web 8091 debug                          # 审查车道自己的 debug 预览（免登录）
  cd e2e && bun install                            # 仅首次
  E2E_BASE_URL=http://127.0.0.1:8091 bunx playwright test
  ```
  分支没有 `e2e/` → 浏览器自测 + 汇报里注明「本分支无 e2e 套件」，不新建套件（维护者决策）。

## 2. 修复协议

- 修复只落在审查发现的问题上；一个 webfix 分支一个主题，超 5 个文件就拆分支或停下报备。
- 修完重跑被该修复影响的验证（单测 / e2e / 浏览器路径），不带着未验证的修复合回。
- commit 用 conventional message，写清「修的是哪条审查发现」。

## 3. 合回 web-dev

```bash
gate check                        # FAIL 清零（WARN 说明理由放行）
git checkout web-dev && git pull --ff-only
git merge --no-ff webfix/xxx      # 保留审查轨迹，不 squash
git push origin web-dev && git branch -d webfix/xxx
```

## 4. 发布预备（给 web-dev → main 的 PR 备料）

- 本地跑一次 `cargo clippy -p <改动的 web crate> --all-targets`（clippy 是 merge 期 FAIL，
  别等 CI 才发现；同版本工具链，改动前 `rustup update stable`）。
- 产出 **CRG 审查结论评论稿**：审查范围（base_sha..web-dev）、发现的问题、修复与验证记录
  （Fix/采纳/驳回 + commit 或验证结论）——发布 PR 必须挂这条评论（pr_crg_review FAIL）。
- 给出发布 PR 需要的 type label 建议（feature/bug/chore/refactor/tests/documentation）。

## 5. 汇报格式

- 审查范围（base_sha → web-dev tip，commit 数）
- 发现问题清单 + 各自处置（修复 commit SHA / 驳回理由）
- 跑了哪些验证（单测命令、e2e 命令、浏览器路径及结果）
- CRG 结论评论稿（可直接贴 PR）
- 交回 dev 车道的新功能建议（如有）
- 冲突或未完成事项

## 禁止

- 禁止自行开发新功能、禁止顺手重构审查范围外的代码
- 禁止 force push / rebase 共享分支、禁止把其他域的提交卷进 webfix 分支
- 禁止对共享库 `db-reset`、禁止停共享 3211、禁止 `pkill -f cargo` / `rm` / `git clean`
