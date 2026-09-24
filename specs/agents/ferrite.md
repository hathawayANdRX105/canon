# Ferrite 工作约定

## 开始工作前（按顺序读）

1. 读根 `AGENTS.md`（本文件）。
2. 读根 `README.md` 的 crate 清单，确定当前结构与依赖。
3. 读任务书 / issue 里的文件实现列表工作，按其验收命令验证后提交 conventional commit。

本文件写**每个会话都必须遵守的硬性约束**，和**遇到什么情况该去读哪份文档**。
详细的操作说明放在 `.agent/rules/` 下，需要时按本文指引去读，不要一开始全读。

---

## 开发方式（worktree）

- `.wt/<name>/` 是开发工作目录：每个开发会话用 `git worktree add .wt/<name> -b <branch>` 挂独立分支，
  worktree 目录名与分支名尾段一致（`.wt/admin-api` ↔ `feat/admin-api`）。
  仓库根目录只读（除了根 `Cargo.toml` 的 workspace member 变更）。
- **创建 worktree 的硬规则（防嵌套事故）**：
  1. **必须先 `cd /home/hathaway/projects/ferrite`（仓库根）再执行** `git worktree add .wt/<name> -b <branch>`。
     `git worktree add` 的相对路径是相对**当前所在目录**解析的——若 cwd 在某个 worktree 内部，
     `.wt/<name>` 会落进那个 worktree 里形成嵌套（历史事故：13 层嵌套 + 321G 重复编译产物）。
  2. 执行后**必须自检**：`git worktree list` 中新条目的路径必须是
     `/home/hathaway/projects/ferrite/.wt/<name>`。路径里出现第二个 `.wt/` 就是嵌套，
     立即 `git worktree remove` 撤销重来。
  3. 闸门有 `checklist_no_nested_worktree` 检查项（提交 / 推送 / 合并时扫描），命中即 FAIL。
  4. 派子代理时，prompt 里必须写**全局绝对路径**（如 `/home/hathaway/projects/ferrite/.wt/<name>/`），
     禁止让子代理自己推导相对路径。

---

## `.wt/` 工作目录保护（硬约束）

`.wt/<name>/` 是开发工作目录，也是其他会话的代码容器。
**任何会话严禁在未经确认的情况下删除整个 `.wt/` 目录或他人 worktree 分支目录。**
违规删除 = 丢失他人整个开发会话，等同于删库。

- 只能删除**自己负责的 PR 对应的 worktree 目录**，且必须同时满足：
  1. PR 已 squash merge 到 upstream main；
  2. 维护者明确确认可以清理；
  3. 删除前 `git worktree list` 确认目标目录对应当前会话分支，不影响其他 worktree。
- 合并流程结束时：通过 `git worktree remove <自己目录>` + `gh pr merge --delete-branch` 正常释放。
  **严禁使用 `rm -rf .wt/`、`rm -rf .wt/*` 或 `git clean` 进行任何批量 / 暴力删除。**
- 发现 `.wt/` 目录意外丢失时，立即告知维护者，并尝试用
  `git worktree prune` + `git checkout -b <branch> <merge-commit>` 恢复。

---

## 域目录独占与越界规则

- `crates/<domain>/` 是高内聚的开发单元：一个会话接手某域目录即**独占**它——
  其他会话不会来干扰，它也**不准越界**改动其他域目录下的任何 crate。
- 唯一例外是重构开发需要跨域时：开工前在 PR 报备涉及的域目录清单，确认无在跑会话冲突再动。
- 跨域共享只有 `crates/contract`（共享 API 契约）：需要新 DTO 先声明变更，由一个会话统一修改。
- 粒度分层：域目录 = 大功能；域内 crate = 大功能开发单元；每个文件 = 小功能开发。
  `lib.rs` 尽量只放共用结构体和 trait，实现在各文件里。

---

## 目录与术语

```text
crates/api/<prefix-feature>/
crates/web/<prefix-feature>/
```

| 术语 | 位置 | 含义 |
|---|---|---|
| **后端域** | `crates/api/` | 全部后端服务平铺大容器，包含 `auth`（通用账号中心）、`admin-*`（管理服务）、`tavern-*`（酒馆服务）。 |
| **前端域** | `crates/web/` | 全部前端组件与界面平铺大容器，包含 `ui-components`（跨端通用组件）、`admin-page-*`、`tavern-page-*`。 |
| **共享契约** | `crates/contract/` | 跨端共享的独立数据传输对象 (DTO) 与纯协议错误定义。 |
| **网关与执行** | `crates/gateway/`、`crates/harness/` | 渠道调度转发引擎与 Agent 运行时。 |
| **功能 crate** | `crates/<domain>/<name>/` | 独立 Cargo Library Crate，各自拥有独立的 `Cargo.toml`、`src/lib.rs` 与 `tests/`。 |
| **应用** | `apps/<name>/` | 有 `main.rs` 的可执行单体程序，负责配置、状态和路由组装。例：`apps/api`、`apps/admin-web`、`apps/tavern-web`。 |
| **集成测试** | `tests/` | 项目顶层跨 Crate 端到端集成测试套件。 |

---

## 依赖与组装

- 功能 crate 只提供 library API；不定义进程入口。
- 域间禁止直接私有依赖：跨端数据交互必须基于 `crates/contract` DTO。
- `apps/api` 统一组装 `crates/api/*`、`crates/gateway/*` 和 `crates/harness/runtime`。
- `apps/admin-web` 组装 `crates/web/admin-page-*` 与 `crates/web/ui-components`。
- `apps/tavern-web` 组装 `crates/web/tavern-page-*` 与 `crates/web/ui-components`。
- 每个功能 crate 都有独立的 `Cargo.toml`、`src/lib.rs` 与 workspace member。

---

## 多会话文件所有权

- 会话所有权以域目录为边界（见上「域目录独占与越界规则」）；`crates/contract/` 是唯一跨域共享点。
- 根 `Cargo.toml` 只有新增或移动功能 crate 的会话修改；改完说明新增的 workspace member。
- `crates/contract/` 是共享 API 契约；需要新 DTO 时先声明变更，再由一个会话统一修改。
- `apps/api/src/` 只由 API 组装会话修改。
- `apps/admin-web/` 和 `apps/tavern-web/` 只由各自应用组装会话修改。

---

## 文件删除与清理（硬约束）

**核心原则：代码与工作区任何非版本控制的清理，都必须保留后悔药（可恢复），绝对禁止不可逆抹除。**

1. **严禁使用 `git clean`**：任何会话严禁在未经维护者明确同意的情况下运行任何形式的 `git clean`
   （包括 `git clean -f`、`-fd`、`-fx`、`-fX` 等）。它会直接物理擦除未跟踪的改动、临时脚本、
   本地配置（如 `.env*`、`config/config.toml`）、跨会话挂载点与实验分支，绕过回收站且不可撤销。
2. **严禁使用 Shell 原生永久删除命令**：
   - Bash / Linux / Unix：严禁 `rm`、`rm -rf`、`rmdir`、`unlink`。
   - PowerShell：严禁 `Remove-Item` 及其别名 `rm`、`rmdir`、`del`、`erase`、`ri`。
   - 严禁 `gio remove` / `gio rm`（同样是物理删除，不进回收站）。
3. **唯一合规的删除方式**：
   - 本地非 Git 跟踪的文件 / 目录：一律 `gio trash <path>`，移入回收站（`~/.local/share/Trash/`）。
     找回方法：读 `~/.local/share/Trash/info/<filename>.trashinfo` 拿原路径，再从
     `~/.local/share/Trash/files/<filename>` 移回。
     清空回收站（`gio trash --empty`）属于破坏性操作，必须获得维护者明确指令才能执行。
   - Git 跟踪的文件：用常规版本控制命令 `git rm <path>`。

---

## 密钥与敏感信息

- 禁止提交：真实 IP 地址、上游 / 内网地址、API key、token、密钥、密码。提交前扫一眼 diff。
- 本地配置放 `config/config.toml`（已 gitignore，模板见 `config/config.toml.example`）
  或 `.env*`（已 gitignore）；文档与示例用占位符（`<API_KEY>`、`127.0.0.1`）。

---

## CLI COMMAND 跟 CPU 节流（cpulimit）

- CPU 密集型命令必须套 `cpulimit -l 65 -i --`：编译、测试、装包类
  （`cargo build` / `cargo test` / `cargo clippy`、`npm` / `bun` 等），
  以及子代理产出的编译 / 测试 / 运行验证，一律不许裸跑。
  `git`、`grep`、文件读写等轻量命令不需要。

---

## 本机 dev 服务与进程卫生（硬约束）

> 要启动后端 / 数据库 / 前端，或遇到 500「Connection refused」、卡片 404 但 curl 200、
> 构建卡死、dx 不重建、`Failed to find binary package` 时 → 读
> **`.agent/rules/dev-env.md`**（含实测过的命令、症状对照表、修复方法）。

- **多会话的后端 / 数据库分两种情况**（完整说明见 `.agent/rules/dev-env.md`）：
  - **共享（默认）**：所有 `.wt/` 会话的前端代理都指向 `127.0.0.1:3211`，共用 dev 数据库
    `uf-local-postgres/ferrite_smoke`，生命周期只走 `scripts/dev-backend.sh`。
  - **隔离（需要独立验证时）**：用 `FERRITE_DEV_LISTEN=127.0.0.1:<端口>` 起独立后端，
    本 worktree 的 `config/config.toml` 指向另一个库。
  - **两条红线：严禁停共享的 3211 后端；严禁对共享库跑 `db-reset`（会清掉所有会话的数据，跑前必须报备）。**
- **长跑服务禁止用 `nohup ... &` 启动**：工具调用结束会回收整个进程组，服务静默死亡
  （症状：页面 500 "Connection refused"、dx 日志消失）。要用会话的持久后台任务机制启动，
  启动后用 `ss -ltn` 验证端口在监听再往下做。
- **禁止宽匹配 `pkill -f cargo` / `pkill -f rustc` 清进程**：多会话并行时那是别人正在跑的构建
  （cpulimit 节流下进程任意瞬间都处于 T 状态，**T 态 ≠ 死进程**）。
  清理前必须 `readlink /proc/<pid>/cwd` 确认归属，只处理无主残留。
- **dev 的启停 / 种子 / 体检一律走 `justfile` 配方**：
  `just dev-check`（环境体检）、`just dev-backend start|update|stop|status`（共享后端）、
  `just db-seed` / `just db-reset`（测试数据）、`just verify`（fmt + clippy + check 全套）。
  开工前先跑 `just dev-check` 自检环境，不要手工拼这些命令。
- **免登录调试前端**（`debug-auto-login`，默认关）：`just dev-web <端口> debug` 会自动登录
  dev 种子账号 `admin_dev`；想手动调登录页就打开 `#login`（auth hash 不触发自动登录）。
  主动「退出登录」不会被自动重登顶掉。

---

## 开发中遇到问题，该读哪份文档

**下面这份表是最重要的导航**。遇到对应的场景，去读对应的文档，不要在本文里找细节。

| 你要做的事 / 遇到的问题 | 读这份文档 |
|---|---|
| 要开 PR 干活，或要派子代理分担任务 | `.agent/rules/pr-workflow.md` |
| 要写测试、跑测试、看 CI 结果，或怀疑「CI 绿了但没验东西」 | `.agent/rules/testing-ci.md` |
| 提交 / 推送被拦、创建 PR 被拒、要查检查规则 | `.agent/rules/gates.md` |
| 启动后端 / 数据库 / 前端，或前端报错、构建卡住 | `.agent/rules/dev-env.md` |
| 写前端界面、写 Rust 公共接口、调查或审查代码 | `.agent/rules/conventions.md` |
| 要跑 web 双车道（快速开发 / 审查修复 / 发布） | `.agent/rules/web-lanes.md` + 任务书 `.agent/tasks/dev-web-lane.md` / `.agent/tasks/webfix-lane.md` |
| 要派一件具体的事给某个 agent（写任务书） | 开发任务用 `.agent/tasks/dev.md`，收尾合并用 `.agent/tasks/closeout-pr.md`；空白模板 `.agent/tasks/TEMPLATE.md` |
| 想了解 `.agent/` 目录本身怎么组织 | `.agent/README.md` |

---

## 测试与 CI（摘要）

> 要写测试、跑测试、看 CI 结果，或怀疑「CI 绿了但没验东西」时 → 读
> **`.agent/rules/testing-ci.md`**（含三种「看起来通过、其实没验证」的情况的判据和处理方法）。

- 本地只做 `cargo check -p <crate>`（编译验证）和极小的单用例调试（3 秒内跑完的（注意：除非需要快速测试，不然不准在本地跑测试，把测试放到PR的 CI 上进行）
  `cargo test -p <crate> -- <测试名>`）；所有 `cargo test` 交给 PR 的 CI 按 `git diff` 动态选包。
  本机可用内存不多，**严禁**本地跑 `cargo test --all` 或整个 workspace 编译（会假死）。
- 「通过」= CI 全绿；CI 未全绿不得 closeout / merge。
  本地 clippy 必须与 CI 同版本（改动前先 `rustup update stable`）。
- **注意三种「看起来通过、其实没验证」的情况**（新增测试前必须读 `.agent/rules/testing-ci.md`）：
  加 feature 门禁的测试 CI 不会执行（显示 `running 0 tests` 但整体绿）；
  e2e 测试在 CI 里因为没有数据库而超时跳过，却记为通过；
  标了 `#[ignore]` 的测试在 CI 里永远不跑（仓库现有 61 个）。

---

## 闸门（gate）与 GitHub（摘要）

> 提交 / 推送被拦、创建 PR 被拒、要查规则时 → 读 **`.agent/rules/gates.md`**。
> 规则总览在 `.githooks/GATE_HANDBOOK.md`（三层检查 + 16 条规则表）；
> 规则对照清单在 `.githooks/spec/SPEC_OVERVIEW.md`。

- 拦截信息**逐条读完再修根因**：禁止 `--no-verify`、禁止用 `| head -5` 之类截断后忽略。
  **FAIL 必须清零**；WARN 说明理由后可放行。
- `gh` 操作（建 issue / PR / 关 issue）在**创建时**就走校验：FAIL 会直接拦截该操作，
  必须逐条修到清零再重试；WARN 每条都要处理（能补就补，补不了的在 PR 正文写明理由）。
  操作前先跑 `gate check` 对应清单或 `gate issue` / `gate pr` 预检，不要等推送才撞墙。
- 占位符用 Rust 原生宏：`todo!("TODO(#<issue>): 说明")` 或 `unimplemented!(...)`；
  TODO / FIXME 注释必须带 issue 号（`TODO(#123): ...`）——这是 `rust_todo_needs_issue` 检查项。

## 发现处置纪律（gate / jev / review）

自动检查的每条 finding（gate `FAIL`/`WARN`、`jev` L3 发现、CRG / `ocr review` 意见）必须被显式处置：

1. **先读规范再改代码**：先读 `.githooks/GATE_HANDBOOK.md` 总览、`.githooks/spec/**/<规则>.yaml` 单条规则、`.agent/rules/gates.md` 项目适配（本仓没有该文件时省略最后一项）。
2. **修根因**：让规则约束成立，不是让检查不再报。
3. **禁止糊弄式修复**：改/删 `.githooks/spec` 规则或降 `fail_severity`；`--no-verify`；`head`/`tail`/`grep -v` 截断输出后当没看见；`#[allow(...)]`/`# noqa` 压制；建空文件/空目录占位；给无断言测试塞 `assert!(true)`；拆分/改名只为躲匹配范围。
4. **逐条留痕**：修复写 `规则 ID → 根因 → 改法(file:line)`；驳回写 `规则 ID + 理由 + 依据` 交维护者裁决。落点 = PR 正文 `## Delivery record` 或 issue 交付评论。沉默即违规。
5. **WARN ≠ 可忽略**：与 `FAIL` 同等处置。

完整版与判例见 canon `specs/agents/_discipline.md`。
---

## UI / Rust / 调查审查（摘要）

> 写前端界面（Dioxus）、写 Rust 公共 API、调查或审查代码时 → 读
> **`.agent/rules/conventions.md`**。

- **UI 验证**：交互元素加 `data-testid`（值取 `name` 属性），容器加 `role` + `aria-label`；
  PR 冒烟验证用 `tab.ariaSnapshot()` 做 role / name / data-testid 结构化断言
  （纯 agent 约定，gate 不强制——`specs/ui/` 契约文件从未被 gate 引用，已删除）。
  禁区：只靠截图肉眼判断、用 class 选择器、绕过 ariaSnapshot 直接提 PR。
  详细操作在 `.agent/skills/ui-validation/SKILL.md`。
- **Rust**：函数名用动宾结构、见名知目的（`parse_channel_config` 而不是 `do_config`）；
  公共 API 必须写 `///` rust doc（用途、参数、错误、示例），模块头写 `//!`。
  没有文档注释视为不完整交付。
- **调查与审查**：调查先用 `code-review-graph update` 建图谱再查调用关系，不要逐文件翻；
  审查分两层——先 CRG 看结构（`code-review-graph detect-changes`），
  再用 `ocr review` 看规范（按模块分批，禁止一次性喂整个仓库）。
  **注意：`OCR` 是截图识别工具，`ocr` 命令是代码审查工具**，别混淆。

---

## PR 开发流程（硬性门禁）

> 要开 PR 干活，或要派子代理分担任务时 → 读 **`.agent/rules/pr-workflow.md`**
> （完整的九个阶段：准备 → 摸清范围 → 拆任务 → 开发审查循环 → 测试 → 工具审查 →
> 冒烟验证 → 清理 → 汇报；以及下面这些门禁的完整说明）。

### UI 快车道（web 双车道）

改动**只落在** `crates/web/*` 与 `apps/admin-web` / `apps/tavern-web`，不碰 `crates/contract` /
`crates/api` / `crates/gateway` / `crates/harness` 时，走 web 双车道
（**权威规则：`.agent/rules/web-lanes.md`**），跳过本节重型流程：

- 开发角色（任务书 `dev-web-lane.md`）：不派子代理、不做 CRG、不建 PR；
  短分支 `feat/xxx` 从 `web-dev` 切、squash 合回即走人。
- 审查修复角色（任务书 `webfix-lane.md`）：短分支 `webfix/xxx` 审 `base_sha..web-dev`，
  修 + 测试/e2e，`--no-ff` 合回，并为发布 PR 产出 CRG 结论评论。
- 唯一 PR = `web-dev → main`（merge commit，禁 squash），发布前先 `merge main` 同步；
  推 web-dev / 建 PR 时 `.githooks` 闸门照跑（FAIL 清零，闸门不绕）。
- 验证 = dev 浏览器自测（`just dev-web-rebuild <port> [debug]` → 强刷）；
  车道内不单写 PR 评论。
- 车道用固定 worktree `.wt/web-dev` / `.wt/web-fix`，共享 `CARGO_TARGET_DIR`；
  不为小改动新开一次性 worktree。

你是**主控** agent：编排任务、派子代理执行、审查子代理产出，**不要亲自把核心实现写完**。

- **只能通过 PR 干活**：一切工作面以 PR 登记。禁止新建 GitHub issue、禁止改动 epic 结构
  （挂载 / 摘除子 issue）。只有维护者在对话中明确要求建 issue 时，先报备标题和完成标准，批准后才建。
- **工作目录门禁**：所有子代理必须在 `.wt/<分支名>/` 目录里工作；子代理 prompt 必须写明
  **全局绝对路径**（如 `/home/hathaway/projects/ferrite/.wt/<name>/`）和所属分支，
  限定它只能在该目录里读写、编译、提交；禁止在仓库根目录或其他 worktree 落文件。
- **任务量门禁**：单个子任务不超过 5 个文件、单一主题、单一修改范围。
- **登记要求**：难以确定的地方和风险点写进 PR 正文对应字段（不写进完成标准）；
  子任务以 checkbox 形式登记到 PR 正文的任务清单，完成后勾回。
- 每轮「审查 + 修复」在 PR 下写**一条**评论（包含修复的 commit 号）；
  冒烟验证单独再写**一条**评论，说明验证手段和结果。
- 不绕过 `.githooks/` 的拦截门，不绕过 `hooks/merge --dry-run` 预检。
- 开发目标的登记（`todo` / goal）与 PR 正文的任务清单全程保持同步；
  记录 `base_sha`，CRG 和 review 用 `--base <base_sha>`，不要写死 `main`。

---

## 目标约束

- `crates/harness/core`、`crates/harness/prompt`、`crates/harness/tools` 必须支持 `wasm32-unknown-unknown`。
- `crates/web/tavern-*` 和 `crates/web/admin-*` 必须支持 `wasm32-unknown-unknown`。
- 测试放同层 `tests/` 目录，不在 `src/` 里使用 `#[cfg(test)]`。
- 新增或移动功能 crate 时，更新根 `Cargo.toml` 的 `workspace.members` 与根 `README.md` 的 crate 清单。
