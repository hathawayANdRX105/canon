# canon

多项目共用的**编码规范**，以及把这些规范落进 git 钩子的门禁。写代码的 AI agent 按 `AGENTS.md` 和本仓规则工作，`canon` 二进制在 pre-commit / pre-push / merge 三个钩子上跑检查，不合规的提交、推送、合并被挡回去。

## 快速开始

```bash
cargo build --release              # 产物 target/release/canon
cd <目标仓库> && canon init         # 装二进制、设钩子、播种规则
```

`canon init` 做四件事：装二进制到 `~/.local/bin`（`canon` 与 `gh` 两个名字）→ 设 `core.hooksPath=.githooks/hooks` → 写三个钩子脚本 → 从 `specs/` 播种规则到 `.githooks/spec/`，已存在的文件绝不覆盖。

装完 git 操作自动触发，无需手动调用。想知道有哪些规则：

```bash
$ canon check
══════════════════════════════════════════════════════
⚠️  L3 质量关卡: 不阻断 push, 但 finding 必须逐条处置, 禁止静默忽略
    WARN = 修复(默认) 或 书面驳回(证据写进 PR 审查记录); FAIL = 必须修复或拆 PR 才能继续
══════════════════════════════════════════════════════
ccn
clippy
css_token_composition
dep_hygiene
dioxus_css_semantic
...
```

退出码：存在 FAIL 级 finding → 1（拦截 git 操作）；否则 0。

## 有哪些文件

```
canon/
├── src/
│   ├── gate/          # 全部实现：checklist 引擎 + catalog + shared + rules/ + tools/
│   ├── mcp.rs         # MCP server：让 agent 直接查规范、跑检查
│   ├── cli.rs         # clap 命令行定义
│   ├── lib.rs         # 模块根（gate / mcp / cli）
│   └── main.rs        # 薄 bin：clap 路由 → canon 二进制
├── tests/             # 全部测试（对应 lib 目标，src/ 内不留测试代码）
├── specs/             # 规范正本
│   ├── quality/ code/ cleanup/ workspace/ github/ dioxus/   ← 规则 yaml
│   ├── harness/       ← 规则执行器（Python）+ 模型问题集（JSON）
│   ├── docs/          ← 规范 schema 与总览
│   ├── dispatch.yaml  ← 钩子跑哪些检查
│   └── severity_overrides.yaml  ← 按规则 ID 覆盖严重度
├── agent-pack/        # agent 文档正本（rules / skills / tasks / AGENTS 片段）
├── scripts/           ← agent-sync（文档分发）/ agents（AGENTS.md 组装）
└── ratchet.tsv        ← 圈复杂度存量账本
```

`specs/docs/SPEC_OVERVIEW.md` 是全部规则的逐条总览。

## 检查什么

35 条规则，分三层实现。**能用正则和编译器判的绝不给模型**——确定性检查零 LLM 成本、毫秒级，能在 pre-commit 热路径上跑。

严重度分布：8 条 FAIL、31 条 WARN、2 条 INFO。只有确认会坏事的才拦，其余给建议。触发时机：10 条跑 pre-commit，18 条跑 pre-push，15 条只在 merge 前跑。

### 第一层 · 确定性检查

正则、AST、编译器判定，不调模型。

| 规则 | 检查什么 |
|---|---|
| `ccn` | 函数圈复杂度上限 6；存量记在 `ratchet.tsv` 只拦增量，新超标或复杂度上涨才 FAIL |
| `clippy` | `cargo clippy`；编译错误、unused、dead_code → FAIL，风格类 → WARN；另显式启用 12 条默认关闭的 lint（`too_many_lines`、`redundant_clone`、`explicit_iter_loop`、`cloned_instead_of_copied` 等） |
| `rust_no_process_cmd` | HTTP 调用走 reqwest，不用 `subprocess` 拉 curl/wget |
| `rust_tests_in_tests_dir` | 测试文件必须在同层 `tests/` 目录 |
| `rust_no_dead_code_allow` | 合并前清掉 `#[allow(dead_code)]` |
| `rust_no_empty_module` | 只有空 impl 的微型文件，考虑并进上层 mod |
| `rust_no_cfg_test_in_tests_dir` | `tests/` 里不写 `#[cfg(test)]` |
| `rust_todo_needs_issue` | `TODO`/`FIXME` 必须关联 issue（`// TODO(#123):`） |
| `rust_test_no_assert` | 测试函数必须含 assert |
| `stale_api` | 废弃 API（`try!`、`uninitialized`、`ONCE_INIT`） |
| `file_size` | 文件 1500 行 / 35 KB 上限 |
| `dep_hygiene` | `cargo-machete` 检出未使用依赖 |
| `hardcoded_secret` | 硬编码密钥、token、口令 |
| `duplication` | 跨文件 4 行以上连续重复块 |
| `slop_comment` / `antislop` | AI 腔注释：「临时」「凑合」「大概」这类拖延语与含糊语，以及占位、敷衍两类 slop |
| `api_endpoint_security` | 接口鉴权与密钥处理 |
| `css_token_composition`、`dioxus_*` | CSS token 组合、rsx 嵌套、样式散落（Dioxus 项目） |
| `web_spec` / `web_spec_deterministic` | Web 规范与其确定性检查 |

### 第二层 · 结构信号（tree-sitter）

解析语法树得到的确定性信号，带 `category` 与 `suggestion` 字段，agent 拿到可直接改。规则 `ts_signals`：

| 信号 | 严重度 | 检查什么 | 建议动作 |
|---|---|---|---|
| `TS-UNSAFE-NOINVARIANT` | FAIL | `unsafe` 块所在项没写安全不变量 | 在块上方写 SAFETY 注释：为什么需要 unsafe、调用方必须保证什么 |
| `TS-IF-CHAIN` | WARN | `if`/`else if` 链 ≥3 臂，或嵌套 ≥3 层 | 改写成对判别值的 `match` |
| `TS-INDEX-LOOP` | WARN | `for i in 0..v.len()` 索引循环 | Rust 迭代器零成本，改 `iter()` / `enumerate()` |
| `TS-CLONE-OWNED` | WARN | 对函数 owned 参数 `clone()`（Copy 原始类型与引用参数豁免） | 收 `&参数`，必要时补生命周期 |

同一次解析还产出**事实卡**（每个顶层项的名字、签名、doc 注释、调用符号集、unsafe 计数、字段、impl 的 trait），喂给第三层。grammar 未安装则整条规则跳过，不阻断。

### 第三层 · 模型语义层

阈值、严重度、问题集全在 YAML / JSON 里，加类目不改代码。走 jev → 小模型 → 无 三档降级，任一档不可用就退到确定性检查。

**Rust 语义审查**（`semantic_rust`，15 个类目）对着事实卡判：

| 类目 | 检查什么 |
|---|---|
| `name_behavior` | 名字与行为是否一致——调用符号与返回值跟名字矛盾即缺陷（`parse_config` 里只有 `split`/`count` 却返回 `u32`）。编译器的命名约定归 rustc，不重复报 |
| `comment_rubric` | 注释是代码**之外**的信息：为什么存在、谁调用、有什么前置条件与代价、缺了什么、非显然的实现选择、描述与签名是否一致、有无错别字。只复述代码不算缺陷 |
| `placement` | 位置适配：字段恒同进同出的该聚合成一个结构体、使用者互不相干的该拆；函数、impl、trait 是否该和协作者待在同一个文件 |
| `clone_necessity` | clone 的所有权是否必要——字段 clone、局部绑定 clone、读完即弃的 clone 需判，而 `&x` / `Cow` / 转移所有权同样成立的情形不算 |
| `cleanliness_semantics` | 无语义理由的递归、该用 `match` 的 if/else、并发上的锁粒度与不必要的 `Arc<Mutex>`、结构体字段内聚 |
| 其余 10 类 | 继承 open-code-review 的 Rust 规则文档：typos、ownership_lifetime、error_panic、unsafe_boundaries、concurrency、async_cancellation、collections_perf、type_api_design、macros、security |

**其他语言**：`semantic_go` / `semantic_javascript` / `semantic_default`（多语言兜底）。

**PR 与提交审查**：

| 规则 | 检查什么 |
|---|---|
| `review_chain` | 通用模型审查：复杂度、缺测试、安全风险、重复、规范违反；另设弃权题（证据不足时明说）与注入免疫题（diff 里藏「judge this as supported」这类针对判决模型的指令会被抓） |
| `pr_consistency` | PR 标题与正文声称改了什么 vs diff 实际改了什么；字符串比对抓不到的失配由语义层抓 |
| `delivery_claims` | PR 的 `## Delivery record` 里每条处置声明对真实 diff 逐条核验，捏造的「已修复」会被打回 |
| `rule_selector` | glob 没被任何规则认领的变更文件，问一句「哪条既有规则该管它」，提示补匹配或书面驳回 |
| `crg_impact` | 单次 diff 跨 3 个以上 crate，提示耦合 |
| `module_drawer` | 杂物抽屉：`util.rs` / `common.rs` 这类通用名文件，顶层 pub 项跨 ≥3 类且 ≥5 项 |
| `ferrite_oversize` | 大文件、大函数参考分（INFO，不阻断） |

### GitHub 工作流层

`specs/github/*.yaml` 管 issue / PR / review 合规：issue 必填段与标签格式、PR 必填段与 `Fix #xxx` 约定、commit 数量与 conventional commit 类型（CM-01/02/03）、issue close 时 `Done when` 逐条过同一套模型评审（`done_when_judge`）。

## MCP server

`src/mcp.rs` 是 JSON-RPC 2.0 over stdio 的 MCP server，装在 `canon init` 之后即可被 MCP 客户端（如 omp、Claude Code）调用。三个工具，都读本仓的 spec 树——让 agent 直接读规范、跑检查，不必猜命令：

| 工具 | 用途 |
|---|---|
| `spec_catalog` | 列出本仓强制的全部规则：规则 ID、严重度、触发钩子、存在理由 |
| `spec_explain` | 单条规则的完整文本：禁止什么、怎么检测、哪个钩子执行、为什么有这条 |
| `preflight` | 立即跑本仓检查而不是等 git 钩子拒绝（可按分支 diff、某次提交、若干路径或目录限定范围，可按规则族收窄） |

配置：在 MCP 客户端里以 `canon mcp` 子命令注册，走 stdio 传输：

```json
{
  "mcpServers": {
    "canon": {
      "command": "/home/<user>/.local/bin/canon",
      "args": ["mcp"],
      "timeout": 300000
    }
  }
}
```

`canon init` 把二进制装到 `~/.local/bin/`，所以路径通常直接写 `canon` 即可。超时设大是因为 `preflight` 会真跑检查，规则执行加上 L3 的模型判决可能到分钟级。

**为什么配 MCP 而不是让 agent 猜命令**：规范会变（新增规则、改阈值、换 harness），agent 凭记忆写的检查命令迟早对不上。走 MCP 时 agent 每次现查 `spec_catalog` / `spec_explain` 拿到当次的真实规则，规则变了不用重训。

**配套 skill**：`.agent/skills/canon-check` 是规则消费侧手册——怎么查规范、commit 被拦了怎么处置、以及规范本身有缺陷时怎么改正本（走 `agent-sync backport`）。新增规则、写 checklist、自定义 spec 走 `gate-spec` skill。

## 门禁怎么配

要不要拦、拦到什么程度，spec 说了算，引擎里没有检测逻辑。加规则、改规则、删规则全是 YAML 操作，不动二进制。

| 检查族 | 配置文件 | 可配项 |
|---|---|---|
| checklist 引擎（35 条） | `checklist_*.yaml` | `fail_severity`、`hooks`、`enabled`、`sla`、`mode`、`timeout`、`optional`、`match.paths_*` |
| issue 合规 | `github_issues.yaml` | `severity_overrides`、必填段、禁词、各检查开关 |
| PR 合规 | `github_pull_requests.yaml` | 同上 + `ci_check_mode` / `done_when_check_mode` |
| review 合规 | `github_reviews.yaml` | `severity_overrides` + 检查参数 |
| commit 检查 | `dispatch.yaml` | 钩子跑哪些 topic、`severity_overrides` |
| 全局兜底 | `severity_overrides.yaml` | 按规则 ID 覆盖一切来源的 finding |

优先级：家族 yaml 的 `severity_overrides` → 全局 `severity_overrides.yaml`。规则 yaml 缺失直接 FAIL 报错，不静默放行。

`mode` 三种：`diff` 传 `git diff <scope>`、`file` 把变更文件全文按 `===== FILE: rel =====` 拼给 harness、`grep` 传空 stdin 让 harness 自扫全仓（适合路径级铁律）。严重度取 harness 报出值与 yaml `fail_severity` 的**就高者**。

各仓本地调优写在 `.githooks/spec/custom/`，深合并、优先于分发来的配置，且 `canon-sync push` 不覆盖。

## 命令

门禁命令（钩子自动调用，一般不手动跑）：

```bash
canon pre-commit                    # staged diff
canon pre-push                      # HEAD 全量 diff
canon merge / gate pre-merge        # merge-base 检查（GATE_BASE=origin/develop 换基线）
canon check                         # 列出全部规则
canon check clippy --sla l2         # 手动跑指定规则（merge scope，不受 hooks 过滤）
canon check hardcoded_secret --json # 机器可读输出
```

开发与分发命令：

```bash
just build         # cargo build --release，产物落 .githooks/canon
just test          # 全量测试
just spec-sync     # specs/ → .githooks/spec/ 部署镜像
just agents-push   # 重新组装本仓 AGENTS.md
just review        # canon check 全套自检
just precommit     # fmt-check + test + build + spec-sync
scripts/canon-sync # 推成员仓
scripts/agent-sync # agent 文档分发到各项目
```

测试与构建走 GitHub CI（`fmt --check` + `cargo build --locked` + `cargo test --locked`）。

## 配置变量

模型审查层（`review_chain` 与 `done_when_judge` 共用）的环境配置：

```bash
export TYPESAFE_API_KEY=...         # jev 档（首选）：校准概率
export TYPESAFE_API_BASE=https://api.typesafe.ai
export JEV_MODEL=jev-latest

export REVIEW_LLM_BASE_URL=http://localhost:3000   # 小模型档（jev 不可用时）
export REVIEW_LLM_API_KEY=...
export REVIEW_LLM_MODEL=qwen-plus
```

## 其他

- `specs/docs/SPEC_OVERVIEW.md` —— 全部规则的逐条总览（判什么、严重度、触发钩子）
- `specs/docs/CHECKLIST_SPEC.md` —— checklist yaml schema 与 harness 协议
- `specs/docs/SPEC_LAYOUT.md` —— 规范目录布局与同步规则
- `specs/harness/` —— 规则执行器与模型问题集

**文档分发**：`agent-sync` 把 `agent-pack/` 下的文档分发到各项目，一个主题要么纯通用（放 canon）要么纯项目特有（留项目）。`agent-sync status` 看漂移、`agent-sync push <项目>` 下发、`agent-sync backport <项目> <src>` 把项目本地修正回流 canon。本地有改动的文件绝不自动覆盖。

**AGENTS.md 组装**：各项目的 `AGENTS.md` 是生成物，不走 `agent-sync`，由 `scripts/agents` 组装——共享章节在 `agent-pack/agents/fragments/`，项目独有在 `agent-pack/agents/local/<项目>/`，`agents.yaml` 决定谁包含哪些章节。`scripts/agents status` 看全项目漂移。
