<!-- managed by canon agents.yaml @ 2026-10-01 -->
## canon 约定

canon 是**规范与 agent 文档的正本仓**。别的项目想改规范、改 agent 约定，来这里改，再分发回去。

### 这个仓是什么

- 规范正本：`agent-pack/rules/`、`agent-pack/skills/`、`specs/`（spec 正本，`canon init` 的 seed 源）
- 人查手册与任务书：已全部转成 dotfiles skill（gate-spec / pr-orchestration / task-brief / closeout / worktree-isolation）
- agent 文档正本：`agent-pack/agents/`
- canon 源码：**canon 仓根本身就是 Rust crate**（仓根 `src/` + `tests/` + `Cargo.toml`）；
  `cargo build --release` 产物 `target/release/canon` 复制到 `.githooks/canon`，再由 canon-sync 分发到各项目
- 待部署的暂存改动：`agent-pack/deployed/pending-*/`（**故意不部署**，见各目录 README）

### 项目 AGENTS.md 是生成物

各项目的 `AGENTS.md` **由 canon 组装生成**，不要在项目里直接改——会被覆盖。

```
agent-pack/agents/fragments/    共享章节：跨项目复用，改一处所有引用项目受益
agent-pack/agents/local/<项目>   项目独有内容：架构约定、目录规则、坑
agents.yaml                 每个项目包含哪些章节、什么顺序、什么参数
scripts/agents             组装脚本
```

常用命令：

```bash
scripts/agents list        # 章节库 + 每章被哪些项目引用
scripts/agents show <项目> # 该项目的组合计划
scripts/agents build <项目> # 预览组装结果
scripts/agents push <项目> # 生成并写入项目 AGENTS.md
scripts/agents status      # 全项目漂移检查
```

**改规范的正确路径**：

1. 通用规则 → 改 `agent-pack/agents/fragments/<章节>.md`
2. 项目独有 → 改 `agent-pack/agents/local/<项目>.md`
3. 组合调整（加章、换顺序） → 改 `agents.yaml`
4. `scripts/agents push <项目>` 重新生成
5. 在项目仓提交生成物

`push` 检测到生成物被手工改过会拒绝覆盖（要 `--force`）。真要改内容，回 canon 改源。

### 其他文档的分发

`agent-pack/tasks/`、`agent-pack/rules/`、`agent-pack/skills/` 里的文档走 `agent-sync`：

```bash
scripts/agent-sync status <项目>    # 看漂移
scripts/agent-sync push <项目>      # canon → 项目
scripts/agent-sync backport <项目> <src>  # 项目本地修正 → 回流 canon
```

`backport` 方向是**项目 → canon**（把项目里的改进收回来），不是下发。

### AGENTS.md 不走 agent-sync

`agent-sync.yaml` **不再**分发 `AGENTS.md`。AGENTS 由 `scripts/agents` 单独管理：
两者职责不同 —— `agent-sync` 分发任务书与规则文档（整文件替换），`scripts/agents`
组装 AGENTS（共享片段 + 项目独有两层结构）。混在一个工具里，这两层无处安放。

### ratchet.tsv — 圈复杂度棘轮账本

仓根 `ratchet.tsv` 是 `ccn` 门禁（`specs/quality/checklist_ccn.yaml`）的**棘轮存量账本**，
由 `ccn_gate.py seed` 全仓扫描生成，记录每个函数的当前圈复杂度。

门禁据此**只拦增量**：

- 函数 ccn 超天花板 且**不在**账本 → FAIL（新增违规）
- 在账本 但复杂度**上升** → FAIL（存量恶化）
- 在账本 且持平 → 既有债， tolerated（只许降不许升）

**不要删它**：删掉会让所有既有高复杂度函数变成"新违规"而全部 FAIL。重构降复杂度后跑
`ccn_gate.py seed` 重新记账，账本只许往下走。

### 常用 CLI（Justfile）

`just` 封装了高频命令，见仓根 `Justfile`：

```bash
just build      # cargo build --release，产物落 .githooks/canon
just test       # canon crate 全量测试
just push       # canon-sync 推 8 个成员仓（custom/ 受保护）
just agents-push     # agents push canon（重新组装本仓 AGENTS.md）
just spec-sync       # specs/ → .githooks/spec/ 正本同步部署镜像
just review          # canon check 全套自检
```

改规范的标准动线：改 `specs/` 或 `agent-pack/` → `just spec-sync`（如动 spec）
→ `just build && just push`（如动二进制）→ `just agents-push`（如动 agent 文档）。

## 发现处置纪律

自动检查（canon 的 `FAIL`/`WARN`、`jev` L3 语义发现、CRG / `ocr review` 审查意见）
产出的是**发现**，不是判决。每条发现都必须被显式处置，不存在"绕过"这个选项。

### 先读规范，再改代码

1. 拿到 finding，先读规则原文，确认这条发现到底要求什么：
   - canon 规则总览：`gate-spec` skill（正本）；各仓 `.githooks/spec/docs/SPEC_OVERVIEW.md` 为播种副本
   - 单条规则的参数（匹配范围 / 严重度 / harness）：`.githooks/spec/**/<rule>.yaml`
   - 项目适配说明（本仓为什么这么定）：`.agent/rules/gates.md`
2. 不确定 finding 是否成立时，读完规则仍不能判定 → **记为待裁决**并在交付记录里写明，
   不要凭猜测改代码，也不要直接忽略。

### 按根因修，不按症状修

- finding 指向的**约束**是根因。修代码使约束成立，而不是让检查不再报。
- 修完自问：这条约束在本仓还成立吗？下次同类改动还会不会触发？

### 完整读输出，不截断

- 拦截信息**逐条读完**再动手。`| head -5`、`| tail`、`grep -v` 会吞掉后面的 finding，
  让人误以为已经修完。
- 报告里出现「N checks passed」时，确认 N 覆盖了你改动的部分。

### 禁止糊弄式修复

以下动作一律视为违规（无论 canon 是否因此变绿）：

| 禁止 | 为什么 | 正确做法 |
|---|---|---|
| 改 `.githooks/spec/` 规则、降低 `fail_severity`、删 spec 文件 | 把约束改没，不是修问题 | 开 issue 说明规则缺陷，交维护者决定 |
| `--no-verify`、跳过钩子、直接推 | 绕过的是整个门禁体系 | 修到清零；规则有误走 issue |
| `head` / `tail` / `grep -v` 截断输出后当没看见 | 后面的 finding 被吞 | 完整读输出 |
| 加 `#[allow(dead_code)]` / `# noqa` 消告警 | 压制信号而非解决 | 删无用代码，或写清保留理由 |
| 建空文件 / 空目录 / 占位文件骗过目录类规则 | 结构噪音 | 真按规则合并或删除 |
| 给无断言测试塞 `assert!(true)` | 测试变成永真装饰 | 断言真实行为；无行为可测就删测试 |
| 拆分 / 改名 / 移动只为躲过匹配范围 | 破坏结构换绿灯 | 按规则设计的结构改 |

### 逐条处置并留下书面说明

- **每条 finding 一个处置**：修复（默认）或**书面驳回**。
- 修复 → 在交付记录里写：`规则 ID → 根因 → 改法（file:line）`。
- 驳回 → 必须写 `规则 ID + 不修理由 + 依据`，由维护者裁决。沉默即违规。
- 交付记录落点：PR 正文 `## Delivery record` 段，或 issue 的交付评论。
- WARN 与 FAIL 同等对待。WARN 只是不拦，不是可忽略。

### 规范层级

- `.githooks/` 是 canon 领地：agent 不改规则。
- `.agent/rules/`、`specs/rules/` 是规范正本：发现规则与现实冲突 → 提 issue，不自行改写。
- 本纪律与各仓既有条款冲突时，以本纪律为准（它更严格）。

## 代码风格

### 命名与结构

- 函数名动宾结构、见名知目的（`parse_channel_config` 而不是 `do_config`）。
- 公共 API 写文档注释（用途、参数、错误、示例），模块头写 `//!`。
- 变量与类型不缩写到看不出含义；短名只留给公认短物（`id`、`ctx`、`err`）。

### 注释

- 注释写**为什么**，不复述代码在做什么。
- 不留 AI 味注释（`// Step 1:` / `// This function` / `// 该函数…` / `// 首先…然后…`）。
- 需要解释的复杂逻辑，宁可提取成命名清晰的函数，也不要靠注释块描述流程。
- 注释掉的代码直接删；git 记得它。

### 占位符与未完成

- 未实现的函数或 trait 用语言原生宏，并带 issue 号：
  - Rust：`todo!("TODO(#123): 说明这里要做什么")` / `unimplemented!("…")`
- TODO / FIXME 注释必须带 issue 号：`// TODO(#123): …`。
- 不留空的 `todo!()` / `pass` / `NotImplemented` 桩而无说明。

### 复用与删除

- 动手前先找同仓同类实现与已装依赖。已有工具能解决就不新写。
- 新增依赖前确认：标准库能做完？已装依赖能做？确实都需要才加。
- **删除优于新增**：不留兼容垫片、旧别名、废弃分支、注释掉的旧实现。
- 改了接口就同步迁移所有调用方，不留双路径兼容。

### 工具

- 命名、缩进、格式化交给项目工具（`cargo fmt` / `gofmt` / `ruff format` / `prettier` / `biome`），
  不手工对齐，不在格式化工具之外争论风格。
- lint 报错逐条判断：真问题就修；误报就在规则允许的方式下局部豁免并写明理由，
  不整文件关掉。

## 构建与验证

### 基线

- 改动前先确认基线状态。基线已经红就先说清，别把自己的问题和既有问题混在一起报。

### 验证行为，不是验证代码存在

- 改完跑**真实命令**验证："跑一下" = 启动实际程序、调用实际接口、发真实请求、观察输出或状态。
- bug 修复先复现再修，修完确认复现路径不再触发。
- 永久性改动要留一个能抓住真实回归的检查。
- 测可观察行为与边界：状态迁移、转换、优先级、真实错误、边界值。
  不测 plumbing、不断言源码文本、不写永真断言、不测 mock 的回声。
- 测试与被测文件就近放 `tests/`（同名对应），保持全量套件可通过。

### 重命令放对位置

- 全量测试、全量构建、全量 lint 放 CI 或收尾阶段，不在改动过程中反复跑。
- 本地只跑轻量、快的针对性检查（单 crate `cargo check`、单包测试、`fmt --check`、
  类型检查）。
- 需要本地跑重命令时，套资源限制（`cpulimit -l 65 -i --` 或本仓等价手段），
  不抢占用户正在用的 CPU。
- 装依赖、打包等命令同样受限。

### 收尾

- 一次跑完该跑的检查（测试 + lint + 类型），不在半成品状态下宣称通过。
- 验证不了的部分（缺运行环境、缺凭据、缺硬件）明确说"未验证 + 为什么"，
  不把"没跑"说成"通过"。
- 不因为失败就改测试迎合实现。测试红了先判断是实现错还是测试错。

## 提交与 PR

### 分支

- 默认分支是 `main`（本仓若不同以本仓为准），功能从默认分支拉。
- 一个任务一个分支，分支名带类型前缀（`feat/` / `fix/` / `refactor/` / `chore/`）。
- 合并后清理已合并分支与 worktree，不留 stale 分支。

### Commit

- 标题走 conventional commit（`feat:` / `fix:` / `refactor:` / `docs:` / `chore:` /
  `test:` / `ci:` / `build:` / `perf:` / `style:` / `revert:`）。
- 标题**用英文**，正文可用中文。
- 一个 commit 一件事。不把无关改动、格式化噪声、生成物混进逻辑改动。
- 提交前跑对应检查（`canon pre-commit` / `canon pre-push`），不靠推送失败才发现。

### 提交身份

- commit 作者固定是维护者本人账号 `hathawayANdRX105`（大小写逐字一致）。
- **不得**用 `git -c user.name=... -c user.email=...` 覆盖身份提交。历史上
  `agent@local` / `ci@local` 这类签名就是这么来的：GitHub 账号对不上，
  贡献归属、追责、审计全丢。
- 提交前若 `git config user.name` / `user.email` 不是上面这个账号，先改成本仓配置
  （`git config user.name hathawayANdRX105`），别带着错的身份往下走。
- 邮箱两套都算合法：`2635254302@qq.com`（本地提交）与 GitHub 的
  `61958173+hathawayANdRX105@users.noreply.github.com`（服务端 squash 落库时写的）。
- 禁止 `Co-authored-by:`  trailer 署其他人或机器人账号。

### Issue

- 标题中文；正文 heading 英文、内容中文。
- sub-issue 必须自包含：正文不写 `Parent:` / `Related:` / PR 占位符，直接写清它要什么。
- 关闭前 `Done when` 的 checkbox 全勾。

### PR

- 标题纯英文（conventional commit 风格）；正文小节标题英文、内容中文。
- 正文按仓库模板（`.github/PULL_REQUEST_TEMPLATE.md`）写：背景 / 改了什么 / 为什么 /
  实现步骤 / 交付记录 / 怎么验证 / 检查清单。
- 关联 issue 用 `Fixes #<n>` 收尾行；draft 阶段用 `Related #<n>`，合并授权前改 `Fixes`。
- 开启或更新 PR 后看 CI 结果到底（`gh pr checks`），红了就修，不等用户来问。
- 被 canon 拦下就修代码，**不改规则**。规则确有缺陷 → 开 issue 交维护者裁决。

### 合并

- **只走 squash merge**：
  `gh pr merge <N> --squash --delete-branch --body "Agent 🤖 - Merge: <原因>"`。
- 禁用 `--merge` / `--rebase`（含 `-m` / `-r` 短形式）。merge commit 会让 PR
  记录的分支历史消失，同一分支再合要重新三方合并、当初的冲突裁决全部丢失；
  rebase-merge 还会逐个改写 commit 作者。两者都让 `main` 失去审计价值。
- 不带任何合并方式的 `gh pr merge` 会弹交互菜单 —— agent 不该触发交互，一律显式
  写 `--squash`。
- 禁止本地 `git merge <分支>` 直接合进 `main` 再推 remote。要合就走 PR。
- 各仓 GitHub 设置已关闭 merge commit 与 rebase merge，squash 是唯一可选项。

### 收尾

- 收尾时清掉：已合并分支、临时 worktree、临时进程、跑完的 dev server。
- 资源及时释放；只保留维护者需要的进程（如用户要看的 web 前端）。

## 工具与命令

- 先用仓库已有的构建/测试/检查入口（`Makefile`、`justfile`、`package.json` scripts、`.githooks/hooks/`），不自己拼裸命令。
- 装依赖、跑重命令（长构建、全量测试、打包）前确认不会抢占用户正在用的资源。
- 长驻进程（dev server、watcher、调试器）用后台管理，不用一次性命令挂着。
- 破坏性操作（删除目录、系统级配置、凭据）先停下来说明影响，确认后再做。
- 命令跑不通时读完整输出，不要只看第一行就下结论。
