# canon — agent 共享文档正本

多项目共用的规范 / 任务书 / skill 的**单一正本**。各项目拿到的是**真文件**（非软链、非 submodule），文件头盖版本戳，`agent-sync` 按 commit SHA 比对分发。

## 为什么这样

- 软链 / submodule 在别人 clone、CI、跨机器时都会断，不能用。
- 纯手工复制 = 漂移（`versioning.md` 曾在 kime/silverq/ferrite 三份各不相同）。
- 本仓管**共享内容**；机器配置归 dotfiles；同步工具在 `bin/`（也算工具，随仓走）。

## 目录

```
canon/
├── bin/agent-sync      # 同步工具（status / push / backport / pull）
├── tasks/              # 共享任务书（通用骨架）
├── rules/              # 共享规范
├── skills/             # 共享 skill
└── agent-sync.yaml       # 分发单一配置（项目 = 路径 + 安装路径清单）
```

## 内容拆分原则（重要）

一个主题要么**纯通用**（放 canon，可同步），要么**纯项目特有**（留项目，不同步）。混合内容按自然接缝拆成两份：

- 通用骨架 → `canon/tasks/version-stats.md`（三门口径、判定规则、执行流程）
- 项目特有（路径 / 清单 / tag 格式 / 产物）→ 各项目的 `.agent/tasks/versioning.md`

两份冲突时**以项目文件为准**——它绑死了真实路径。

## 分发配置格式（agent-sync.yaml，单一文件）

一个项目 = 一条 `root`（安装根路径）+ 一份 `files` 安装清单（源路径 → 安装路径）：

```yaml
projects:
  kime:
    root: /abs/path/to/kime        # 项目本地绝对路径（安装根）
    files:
      - src: tasks/version-stats.md  # canon 源路径
        to: .agent/tasks/version-stats.md  # 安装路径（相对 root）
        # pin: abc1234               # 可选：固定同步自某次 canon 提交
```

**文件名映射约定**：默认 `to = <安装前缀>/<src 的 basename>`（保留源文件名，按前缀归位）；
需要改名就显式写 `to`。这份清单是「哪份文档装到哪里」的唯一留存记录，
`agent-sync status/push/backport` 全部以它为准，新增分发文档 = 加一行。

## 用法

```bash
agent-sync status              # 所有项目的漂移报告（只读）
agent-sync status kime         # 单个项目
agent-sync push kime [--commit]  # 同步 canon -> kime（只推 behind/missing/unstamped）
agent-sync backport kime tasks/version-stats.md   # kime 的本地修正回流到 canon
agent-sync pull                # canon 仓自更新
```

## 分类（status 的判定）

| 状态 | 含义 | push 行为 |
|---|---|---|
| `in-sync` | 本地 == 正本，戳 == HEAD | 无操作 |
| `behind` | 本地没动，canon 更新了 | **更新**（快进，安全） |
| `unstamped` | 有文件无戳，但内容 == HEAD | 只补戳 |
| `local-mod` | 本地改了，canon 没动 | 拒绝，建议 `backport` |
| `diverged` | 两边都改了 | **拒绝**，人工三方合并 |
| `unknown` | 有文件无戳且内容 != HEAD | 拒绝，先 backport 或 `--force` |

**安全底线**：本地有改动的文件绝不自动覆盖——那是丢数据。

## 版本戳

每个同步文件首行盖：

```
<!-- canon: hathawayANdRX105/canon @ abc1234 (synced 2026-09-19) -->
```

戳 = 「这份副本来自哪」。agent 在项目里看到它，就知道改文档去 canon，**别改本地副本**。

## gate — 规范执行引擎（bin/gate）

canon 管规范的**存储、分发与执行**，是 gate 的**唯一源码正本**（omenic 的 `bin/gate` 与 `spec` 的 gate 部分已删除，omenic 只留 `spec::template` 模板库供其 CLI 使用）。

### 拦截配置化（原则：要不要拦，spec 说了算）

| 检查族 | 配置文件 | 可配项 |
|---|---|---|
| checklist 引擎（20 条） | `checklist_*.yaml` | `fail_severity`（拦不拦）、`hooks`（何时跑）、`enabled`、`timeout`、`optional` |
| issue 合规 | `github_issues.yaml` | `severity_overrides:` 按规则 ID 定严重度；`garbled_content_check` / `labels_section_forbidden` / `title_must_be_chinese` 等开关直接启停检查；`required_headings` / `forbidden_keywords` / `keyword_label_suggestions` 等参数 |
| PR 合规 | `github_pull_requests.yaml` | 同上 + `ci_check_mode` / `done_when_check_mode`（FAIL / WARN 切换拦截级别） |
| review 合规 | `github_reviews.yaml` | `severity_overrides:` + 检查参数 |
| commit 检查（CM-01/02/03） | `dispatch.yaml` | `severity_overrides:` 段 |
| 全局兜底 | `severity_overrides.yaml` | 按 `规则ID` 覆盖一切来源的 finding（最后发言权） |

优先级：**家族 yaml `severity_overrides` → 全局 `severity_overrides.yaml`**；规则 yaml 缺失 → `gate.setup` FAIL 报错，绝不静默放行。

**分层**：
- `bin/gate/src/engine.rs` — checklist 引擎，**零检测逻辑**：按 scope 收集 payload（staged diff / 全量 diff / 变更文件）→ 喂给 yaml 声明的外部 harness 命令 → 解析 finding JSON 聚合放行或拦截。加规则/改规则/删规则全部是 yaml 操作，不动二进制
- `bin/gate/src/rules/` + `tools/` — gh 工作流策略层（issue/PR/review 合规、merge 编排、gh 命令拦截），检测逻辑由 `github_*.yaml` 驱动
- `rules/` — 默认规则包正本（43 份：quality checklist ×17 + code/cleanup/workspace/github 系 + harness 4 件套 + docs 4 份 + dispatch/severity_overrides）

与 omenic 内嵌版的差异（去硬编码）：

| omenic 内嵌版 | canon gate |
|---|---|
| 规则缺失静默跳过（换仓库静默扫 0 文件假绿） | checklist 缺失直接 FAIL 报错 |
| merge base 写死 `origin/main...HEAD` | `GATE_BASE` 环境变量可覆盖 |
| 引擎与策略、模板混在同一 crate | engine（零检测）/ rules+tools（策略）/ template（留 omenic）三者分离 |

**残留硬编码**（已收敛到最小）：pre_commit/merge 的 topic 路由 `match`（topic 名 → 内建 runner 的映射；topic 列表本身已由 dispatch.yaml 外部化）；`github_pull_requests.yaml` 的 `fixes_epic_severity` 为声明未接线键（对应 API 层检查尚不存在）。检查规范（开关/参数/严重度）已全部 yaml 化，缺失即 `gate.setup` FAIL。

### 构建与验证（CI 驱动）

**测试与构建一律走 GitHub CI**（`.github/workflows/ci.yml`：fmt --check + `cargo build --locked --all-targets` + `cargo test --locked --all-targets`，绿才算验证过）。本地只允许 `cargo fmt --check` 和 `cargo check --offline` 这类秒级轻量检查。唯一例外：需要把新二进制装进 `~/.local/bin` 时本地 `cargo build --release`（CI 产物进不了本机钩子路径）。

### 构建与安装

```bash
cd bin/gate && cargo build --release    # 产物 target/release/gate
cd <目标仓库> && ~/projects/canon/bin/gate/target/release/gate init
```

`gate init` 做四件事：装二进制到 `~/.local/bin/`（**gate + gh 两个名字**，gh 用于拦截 issue/PR 命令）→ 设 `core.hooksPath=.githooks/hooks` → 写三个 hook 脚本（pre-commit / pre-push / merge，带 PATH→仓内二进制的兜底查找）→ 从 `rules/`（自动探测 canon 仓，或 `--rules-dir` 指定）播种规则到 `.githooks/spec/`，**已存在的文件绝不覆盖**。

### 用法

```bash
gate pre-commit          # staged diff 检查（钩子自动调）
gate pre-push            # HEAD 全量 diff 检查（钩子自动调）
gate pre-merge           # merge-base 检查（merge 工具调；GATE_BASE=origin/develop gate pre-merge 换基线）
gate check               # 列出全部规则
gate check clippy --sla l2   # 手动跑指定规则（merge scope，忽略 hooks 过滤）
gate check hardcoded_secret --json   # 机器可读输出（含 score/confidence 等 extra）
```

退出码：存在 FAIL 级 finding → 1（拦截 git 操作）；否则 0。严重度可用 `.githooks/spec/severity_overrides.yaml` 按 rule_id 覆盖。

### 模型审查层（review_chain / DWJ）的环境配置

`review_chain`（pre-push/merge 的语义审查）与 `done_when_judge`（issue close 的 Done-when 逐条评审）共用三档降级：**jev → 小模型 → 无**，两者互斥只跑一个；都不配时模型层出 INFO，确定性工具检查（ccn/duplication/antislop/slop_comment）永远照跑。

```bash
# jev 档（TypeSafe System One，校准概率，首选）
export TYPESAFE_API_KEY=...            # 官方 api.typesafe.ai 用 Bearer；自建网关可能只认 x-api-key，harness 双发
export TYPESAFE_API_BASE=https://api.typesafe.ai
export JEV_MODEL=jev-latest

# 小模型档（jev 不可用时降级，OpenAI 兼容 /v1/chat/completions）
export REVIEW_LLM_BASE_URL=http://localhost:3000
export REVIEW_LLM_API_KEY=...
export REVIEW_LLM_MODEL=qwen-plus
```

per-question 阈值、问题集分别在 `rules/harness/jev_questions_review.json` / `jev_questions_done_when.json`；close 路径总开关在 `github_issues.yaml` 的 `done_when_judge.enabled`。

### 新规则包怎么进 canon

1. 规则 yaml 放 `canon/rules/quality/checklist_<名字>.yaml`（schema 见 `rules/docs/CHECKLIST_SPEC.md`）
2. 各项目 manifest（`agent-sync.yaml` / gate 规则包）加一行把 `rules/quality/checklist_<名字>.yaml` 分发到该仓 `.githooks/spec/`
3. `agent-sync push <项目>` 下发
