<!-- managed by canon agents.yaml @ 2026-09-24 -->
## canon 约定

canon 是**规范与 agent 文档的正本仓**。别的项目想改规范、改 agent 约定，来这里改，再分发回去。

### 这个仓是什么

- 规范正本：`specs/rules/`、`specs/skills/`、`rules/`（gate spec）、`manual/`（人查手册）
- 项目任务书：`tasks/`（closeout / feature-dev-handbook）
- agent 文档正本：`specs/agents/`
- gate 源码：`bin/gate/`（Rust）；各项目 `.githooks/gate` 是构建产物
- 待部署的暂存改动：`specs/pending-*/`（**故意不部署**，见各目录 README）

### 项目 AGENTS.md 是生成物

各项目的 `AGENTS.md` **由 canon 组装生成**，不要在项目里直接改——会被覆盖。

```
specs/agents/fragments/    共享章节：跨项目复用，改一处所有引用项目受益
specs/agents/local/<项目>   项目独有内容：架构约定、目录规则、坑
agents.yaml                 每个项目包含哪些章节、什么顺序、什么参数
bin/agents                  组装脚本
```

常用命令：

```bash
bin/agents list             # 章节库 + 每章被哪些项目引用
bin/agents show <项目>      # 该项目的组合计划
bin/agents build <项目>     # 预览组装结果
bin/agents push <项目>      # 生成并写入项目 AGENTS.md
bin/agents status           # 全项目漂移检查
```

**改规范的正确路径**：

1. 通用规则 → 改 `specs/agents/fragments/<章节>.md`
2. 项目独有 → 改 `specs/agents/local/<项目>.md`
3. 组合调整（加章、换顺序） → 改 `agents.yaml`
4. `bin/agents push <项目>` 重新生成
5. 在项目仓提交生成物

`push` 检测到生成物被手工改过会拒绝覆盖（要 `--force`）。真要改内容，回 canon 改源。

### 其他文档的分发

`tasks/`、`specs/rules/`、`specs/skills/` 里的文档走 `agent-sync`：

```bash
bin/agent-sync status <项目>   # 看漂移
bin/agent-sync push <项目>     # canon → 项目
bin/agent-sync backport <项目> <src>   # 项目本地修正 → 回流 canon
```

`backport` 方向是**项目 → canon**（把项目里的改进收回来），不是下发。

### AGENTS.md 不走 agent-sync

`agent-sync.yaml` **不再**分发 `AGENTS.md`。AGENTS 由 `bin/agents` 单独管理：
两者职责不同 —— `agent-sync` 分发任务书与规则文档（整文件替换），`bin/agents`
组装 AGENTS（共享片段 + 项目独有两层结构）。混在一个工具里，这两层无处安放。

## 发现处置纪律

自动检查（gate 的 `FAIL`/`WARN`、`jev` L3 语义发现、CRG / `ocr review` 审查意见）
产出的是**发现**，不是判决。每条发现都必须被显式处置，不存在"绕过"这个选项。

### 先读规范，再改代码

1. 拿到 finding，先读规则原文，确认这条发现到底要求什么：
   - gate 规则总览：`.githooks/GATE_HANDBOOK.md`
   - 单条规则的参数（匹配范围 / 严重度 / harness）：`.githooks/spec/**/<rule>.yaml`
   - 项目适配说明（本仓为什么这么定）：`.agent/rules/gates.md`（无此文件则看 `canon/manual/gate.md`）
2. 不确定 finding 是否成立时，读完规则仍不能判定 → **记为待裁决**并在交付记录里写明，
   不要凭猜测改代码，也不要直接忽略。

### 按根因修，不按症状修

- finding 指向的**约束**是根因。修代码使约束成立，而不是让检查不再报。
- 修完自问：这条约束在本仓还成立吗？下次同类改动还会不会触发？

### 禁止糊弄式修复

以下动作一律视为违规（无论 gate 是否因此变绿）：

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

## 代码风格

- 函数名动宾结构、见名知目的（`parse_channel_config` 而不是 `do_config`）。
- 公共 API 写文档注释（用途、参数、错误、示例），模块头写 `//!`。
- 不留 AI 味注释（`// Step 1:` / `// This function` / `// 该函数…`）。注释只写"为什么"，不复述代码。
- 未实现的函数或 trait 用语言原生宏 + issue 号：`todo!("TODO(#123): …")` / `unimplemented!("…")`。
- TODO / FIXME 注释必须带 issue 号：`// TODO(#123): …`。
- 命名、缩进、格式化交给项目工具（`cargo fmt` / `gofmt` / `ruff format` / `prettier`），不手工对齐。
- 优先复用已有实现：先找同仓同类代码与已装依赖，再考虑新写。
- 删除优于新增：不留兼容垫片、旧别名、废弃分支。

## 构建与验证

- 改动前先确认基线是干净的；基线已经红就先说清，别把自己的问题和既有问题混在一起。
- 改完跑**真实命令**验证，不靠推断："跑一下" = 启动实际程序 / 调用实际接口，观察输出或状态。
- 永久性改动要留一个能抓住真实回归的检查；不写凑数测试（不测 plumbing、不测源码文本、不测永真断言）。
- 测试断言可观察行为与边界：转换、状态迁移、优先级、真实错误。
- 全量测试 / 构建 / lint 在收尾跑一次，不在改动过程中反复跑。
- 验证不了的部分（无运行环境、缺凭据）明确说"未验证 + 为什么"，不假装通过。

## 提交与 PR

- commit 标题走 conventional commit（`feat:` / `fix:` / `refactor:` / `docs:` / `chore:` / `test:` / `ci:` / `build:` / `perf:` / `style:` / `revert:`）。
- commit 标题**用英文**，正文可用中文。
- 提交 / 推送前先跑对应检查：`gate pre-commit` / `gate pre-push`，不靠推送失败才发现。
- 创建 PR / issue 前先读模板（`.github/PULL_REQUEST_TEMPLATE.md` / `.github/ISSUE_TEMPLATE/`），正文按模板写。
- PR 标题纯英文；正文小节标题英文、内容中文。
- 关联 issue 用 `Fixes #<n>` 收尾行。
- 被 gate 拦下就修代码，**不改规则**（`.githooks/` 属 gate 领地）。规则确有缺陷 → 开 issue。

## 工具与命令

- 先用仓库已有的构建/测试/检查入口（`Makefile`、`justfile`、`package.json` scripts、`.githooks/hooks/`），不自己拼裸命令。
- 装依赖、跑重命令（长构建、全量测试、打包）前确认不会抢占用户正在用的资源。
- 长驻进程（dev server、watcher、调试器）用后台管理，不用一次性命令挂着。
- 破坏性操作（删除目录、系统级配置、凭据）先停下来说明影响，确认后再做。
- 命令跑不通时读完整输出，不要只看第一行就下结论。
