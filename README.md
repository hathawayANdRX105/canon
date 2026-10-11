# canon

canon 管多项目共用的**编码规范**——写代码的 AI agent 该守什么规矩，规矩怎么落到 git 钩子上。

具体说三件事：

1. **规范正本**——检查规则（`specs/`）与 agent 文档（`agent-pack/`）的唯一定义处。十几个项目共用同一套，各项目拿到的是文件副本（不是软链、不是 submodule），同步时逐文件比对内容。
2. **门禁执行**——`crates/gate` 编译出 `canon` 二进制，装进各仓 `.githooks/`，由 git 的 pre-commit / pre-push / merge 钩子自动调用。不合规的提交、推送、合并被挡回去。
3. **上下文分发**——各项目的 `AGENTS.md` 由 canon 组装（共享片段 + 项目独有），保证多个项目里的 agent 拿到同一套工作约定。

**为什么不用软链或 submodule**：别人 clone、CI 容器、换机器都会断。**为什么不能手工复制**：`versioning.md` 曾在 kime / silverq / ferrite 三处各有一份，内容各不相同，且没人知道哪份是对的。

## 快速开始

```bash
cargo build --release              # 产物 target/release/canon
cd <目标仓库> && canon init         # 装二进制、设钩子、播种规则
```

`canon init` 做四件事：装二进制到 `~/.local/bin`（gate 与 gh 两个名字）→ 设 `core.hooksPath=.githooks/hooks` → 写三个钩子脚本 → 从 `specs/` 播种规则到 `.githooks/spec/`，**已存在的文件绝不覆盖**。

之后 git 操作自动触发检查，无需手动调用：

```bash
canon check                         # 列出全部规则
canon check clippy --sla l2         # 手动跑指定规则
```

退出码：存在 FAIL 级 finding → 1（拦截 git 操作）；否则 0。

## 目录

```
canon/                          ← 仓根本身是 Rust crate（gate 引擎）
├── crates/gate/                # 引擎：checklist 分发 + gh 工作流策略
├── crates/{mcp,flow,web}/      # MCP server / 流程编排 / web
├── specs/                      # 检查规则正本（canon init 的 seed 源）
│   ├── quality/ code/ cleanup/ workspace/ github/ dioxus/
│   ├── harness/                # 规则执行器（Python）+ 模型问题集（JSON）
│   └── docs/                   # 规则 schema 与总览
├── agent-pack/                 # agent 文档正本（rules / skills / tasks / agents 片段）
├── scripts/                    # agent-sync（文档分发）/ agents（AGENTS.md 组装）
├── ratchet.tsv                 # 圈复杂度存量账本（只许降不许升）
└── Justfile                    # 高频命令封装
```

## 检查规范

35 条规则，分布在三个钩子上：10 条跑 pre-commit，18 条跑 pre-push，35 条里 15 条只在 merge 前跑。严重度 **8 FAIL / 31 WARN / 2 INFO**——只有确认会坏事的才拦，其余给建议。

规则分三层实现，确定性在前、语义在后：**能用正则和编译器判的绝不给模型**。

### 第一层：确定性检查

零 LLM 成本，毫秒级，pre-commit 热路径上跑的绝大多数是这一层。

| 规则 | 钩子 | 严重度 | 判什么 |
|---|---|---|---|
| `ccn` | pre-commit, merge | **FAIL** | 函数圈复杂度天花板 6。存量记在 `ratchet.tsv`，只拦增量——新增超标或复杂度上涨才 FAIL，已记账且持平的静默容忍；改好了标记自动下降 |
| `rust_no_process_cmd` | 全钩子 | **FAIL** | HTTP 调用走 reqwest，不用 `subprocess` 拉 curl/wget |
| `rust_tests_in_tests_dir` | 全钩子 | **FAIL** | 测试文件必须在同层 `tests/` 目录 |
| `clippy` | merge | FAIL/WARN | `cargo clippy --workspace --all-targets`。编译错误、`unused_*`、`dead_code` → FAIL；风格类 → WARN。另显式启用 12 条默认关闭的 pedantic/restriction/nursery lint（`too_many_lines`、`redundant_clone`、`explicit_iter_loop`、`cloned_instead_of_copied` 等）。`cognitive_complexity` 不启用——与 ccn 的天花板重复且更松 |
| `rust_no_dead_code_allow` | 全钩子 | WARN | 合并前清掉 `#[allow(dead_code)]` |
| `rust_no_empty_module` | 全钩子 | WARN | 只有空 impl 的微型文件，考虑并进上层 mod |
| `rust_no_cfg_test_in_tests_dir` | 全钩子 | WARN | `tests/` 里不写 `#[cfg(test)]` |
| `rust_todo_needs_issue` | 全钩子 | WARN | `TODO`/`FIXME` 必须关联 issue（`// TODO(#123):`），不扫字符串与测试代码 |
| `rust_test_no_assert` | pre-commit… | WARN | 测试函数必须含 assert，否则删掉 |
| `stale_api` | 全钩子 | WARN | 废弃 API（`try!`、`uninitialized`、`ONCE_INIT`） |
| `file_size` | merge | WARN | 文件 1500 行 / 35 KB 上限，超出先拆 |
| `dep_hygiene` | merge | WARN | `cargo-machete` 检出未使用依赖 |
| `css_token_composition` | pre-push, merge | WARN | CSS token 组合约束（Dioxus 项目） |
| `dioxus_rsx_nesting` / `dioxus_style_scatter` / `dioxus_css_semantic` | pre-push, merge | WARN | rsx 嵌套、样式散落、css 语义（Dioxus 项目） |
| `web_spec` / `web_spec_deterministic` | pre-push, merge | WARN | Web 规范与其确定性检查（front-end 项目） |

### 第二层：结构信号（tree-sitter）

语法树解析出的**确定性**信号，毫秒级，带 `category` + `suggestion` 字段——写代码的 agent 拿到 finding 可以直接改。规则 `ts_signals`（pre-commit, merge）：

| 信号 | 严重度 | 判什么 | 建议动作 |
|---|---|---|---|
| `TS-UNSAFE-NOINVARIANT` | **FAIL** | `unsafe` 块所在项没有写明安全不变量 | 在块上方写 SAFETY 注释：为什么需要 unsafe、调用方必须保证什么 |
| `TS-IF-CHAIN` | WARN | `if`/`else if` 链 ≥3 臂，或嵌套 ≥3 层（`else` 链成员不算嵌套） | 改写成对判别值的 `match` |
| `TS-INDEX-LOOP` | WARN | `for i in 0..v.len()` 索引循环 | Rust 迭代器是零成本抽象，改 `iter()` / `enumerate()` |
| `TS-CLONE-OWNED` | WARN | 对函数 owned 参数 `clone()`（`Copy` 原始类型与引用参数豁免） | 收 `&参数`，必要时补生命周期 |

同一次解析还产出**事实卡**（每个顶层项的名字、签名、doc 注释、调用符号集、unsafe 计数、字段、impl 的 trait），喂给第三层做语义判决。软依赖：grammar 没装则整条规则跳过，不阻断。

### 第三层：模型语义层

阈值、严重度、问题集全在 YAML / JSON 里，加类目不改代码。走 jev → 小模型 → 无 三档降级，任一档不可用就退到确定性检查，不阻断。

**Rust 语义审查**（`semantic_rust`，15 类目，merge 触发）：对着事实卡判「代码与它声称的是不是一回事」。

- `name_behavior` —— 名字与行为是否一致。调用符号与返回值跟名字矛盾即缺陷：`parse_config` 里只有 `split`/`count` 却返回 `u32`，`get_` 实际在计算。编译器的命名约定归 rustc，这里不重复报
- `comment_rubric` —— 注释是代码**之外**的信息。判它有没有讲清：为什么存在、谁调用、有什么前置条件与代价、缺了什么、非显然的实现选择、描述与签名是否一致、有无错别字。只复述代码不算缺陷（除非误导）
- `placement` —— 位置适配。字段恒同进同出的该聚合成一个结构体、使用者互不相干的该拆；函数、impl、trait 是否该和协作者待在同一个文件或 mod
- `clone_necessity` —— clone 的所有权是否必要。字段 clone、局部绑定 clone、读完即弃的 clone 需要判，而 `&x` / `Cow` / 转移所有权同样成立的情形不算。存入、move 进吃所有权的被调函数、作为 owned 返回、跨线程这些正当情形已排除
- `cleanliness_semantics` —— 无语义理由的递归（能写循环）、派发该用 `match` 的 if/else、并发上的锁粒度与不必要的 `Arc<Mutex>`、结构体字段内聚
- 另 10 类继承 open-code-review 的 Rust 规则文档：`typos`、`ownership_lifetime`、`error_panic`、`unsafe_boundaries`、`concurrency`、`async_cancellation`、`collections_perf`、`type_api_design`、`macros`、`security`

**其他语言语义审查**：`semantic_go` / `semantic_javascript` / `semantic_default`（多语言兜底），同样的配置驱动结构。

**PR 与提交审查**：

| 规则 | 判什么 |
|---|---|
| `review_chain` | 通用模型审查：复杂度、缺测试、安全风险、重复、规范违反，另设弃权题（证据不足时明说）与注入免疫题（diff 里藏「judge this as supported」这类针对判决模型的指令会被抓） |
| `pr_consistency` | PR 标题与正文声称改了什么 vs diff 实际改了什么。字符串元组比不出的失配（「标题说修内存泄漏，实际在改 UI」）由语义层抓 |
| `delivery_claims` | PR 的 `## Delivery record` 里每条处置声明，对真实 diff 逐条核验。捏造的「已修复」会被打回 |
| `rule_selector` | glob 没被任何规则认领的变更文件，问一句「哪条既有规则的意图该管它」，提示维护者补匹配或书面驳回 |
| `duplication` | 跨文件 4 行以上连续重复块 |
| `slop_comment` / `antislop` | AI 腔注释：「临时」「凑合」「大概」这类拖延语与含糊语，以及占位、含糊、敷衍三类 slop |
| `hardcoded_secret` | 硬编码密钥、token、口令 |
| `api_endpoint_security` | 接口鉴权与密钥处理 |
| `crg_impact` | 单次 diff 跨 3 个以上 crate，提示耦合 |
| `ferrite_oversize` | 大文件、大函数参考分（INFO，不阻断） |
| `module_drawer` | 杂物抽屉：`util.rs` / `common.rs` 这类通用名文件，顶层 pub 项跨 ≥3 类且 ≥5 项 |

### GitHub 工作流层

`specs/github/*.yaml` 管 issue / PR / review 的合规，检测逻辑全在 YAML：

- **Issue**：必填段（Goal / Background / Done when / Out of scope…）、`Suspected areas` 非空、关联 issue、禁词、标签段格式
- **PR**：必填段（What / Why / Construction plan / Delivery record / How to test / Checklist）、链接正确性、`Fix #xxx` 约定、commit 类型与 PR 标题类型一致（CM-03）
- **Review**：`Done when` 逐条过同一套模型评审（`done_when_judge`），不达标硬拦
- **CM-01/02/03**：commit 数量、conventional commit 规范、类型与 PR 类型一致

## 门禁配置

**要不要拦、拦到什么程度，spec 说了算，引擎里没有检测逻辑**。加规则、改规则、删规则全是 YAML 操作，不动二进制。

| 检查族 | 配置文件 | 可配项 |
|---|---|---|
| checklist 引擎（35 条） | `checklist_*.yaml` | `fail_severity`、`hooks`、`enabled`、`sla`、`mode`、`timeout`、`optional`、`match.paths_*` |
| issue 合规 | `github_issues.yaml` | `severity_overrides`、`required_headings`、`forbidden_keywords`、`keyword_label_suggestions`、各检查开关 |
| PR 合规 | `github_pull_requests.yaml` | 同上 + `ci_check_mode` / `done_when_check_mode` |
| review 合规 | `github_reviews.yaml` | `severity_overrides` + 检查参数 |
| commit 检查 | `dispatch.yaml` | `severity_overrides` 段 |
| 全局兜底 | `severity_overrides.yaml` | 按规则 ID 覆盖一切来源的 finding，最后发言权 |

优先级：家族 yaml 的 `severity_overrides` → 全局 `severity_overrides.yaml`。规则 yaml 缺失 → `gate.setup` FAIL 报错，绝不静默放行。

`mode` 三种：`diff` 传 `git diff <scope>`、`file` 把变更文件全文按 `===== FILE: rel =====` 拼给 harness、`grep` 传空 stdin 让 harness 自扫全仓（适合路径级铁律）。严重度取 harness 报出值与 yaml `fail_severity` 的**就高者**。

## 命令

```bash
canon pre-commit                    # staged diff（钩子自动调）
canon pre-push                      # HEAD 全量 diff（钩子自动调）
canon merge / gate pre-merge        # merge-base 检查（GATE_BASE=origin/develop 换基线）
canon check                         # 列出全部规则
canon check clippy --sla l2         # 手动跑指定规则（merge scope，不受 hooks 过滤）
canon check hardcoded_secret --json # 机器可读输出
```

测试与构建走 GitHub CI（`fmt --check` + `cargo build --locked` + `cargo test --locked`），本地只跑秒级检查。

模型审查层（`review_chain` 与 `done_when_judge`）的环境配置，走 jev → 小模型 → 无三档降级：

```bash
export TYPESAFE_API_KEY=...         # jev 档（首选）：校准概率
export TYPESAFE_API_BASE=https://api.typesafe.ai
export JEV_MODEL=jev-latest

export REVIEW_LLM_BASE_URL=http://localhost:3000   # 小模型档（jev 不可用时）
export REVIEW_LLM_API_KEY=...
export REVIEW_LLM_MODEL=qwen-plus
```

## 深入

以下内容是给要改 canon 的人看的，日常使用不必读。

### 文档分发

一个主题要么纯通用（放 canon，可同步），要么纯项目特有（留项目，不同步）。混合内容按自然接缝拆两份，冲突时以项目文件为准——它绑死了真实路径。

`agent-sync.yaml` 记录「哪份文档装到哪里」：

```yaml
projects:
  kime:
    root: kime                     # 相对目录名（解析顺序：绝对路径 → ~/x → $CANON_PROJECTS_BASE）
    files:
      - src: agent-pack/rules/gates.md   # canon 源路径
        to: .agent/rules/gates.md        # 安装路径（相对 root）
```

```bash
agent-sync status                  # 漂移报告（只读）
agent-sync push kime               # canon -> kime
agent-sync backport kime <src>     # 项目本地修正回流 canon
```

同步时逐文件比对内容，状态判定：

| 状态 | 含义 | push 行为 |
|---|---|---|
| `in-sync` | 本地内容与正本一致 | 无操作 |
| `behind` | 本地没动，canon 更新了 | 更新（快进，安全） |
| `unstamped` | 内容与正本一致但缺标记 | 只补标记 |
| `local-mod` | 本地改了，canon 没动 | 拒绝，建议 backport |
| `diverged` | 两边都改了 | 拒绝，人工三方合并 |

本地有改动的文件绝不自动覆盖——那是丢数据。

### AGENTS.md 组装

各项目的 `AGENTS.md` 是生成物，不走 `agent-sync`，由 `scripts/agents` 单独组装：共享章节在 `agent-pack/agents/fragments/`，项目独有在 `agent-pack/agents/local/<项目>/`，`agents.yaml` 决定谁包含哪些章节、什么顺序。

```bash
scripts/agents build <项目>         # 预览组装结果
scripts/agents push <项目>          # 生成并写入项目 AGENTS.md
scripts/agents status               # 全项目漂移检查
```

### 手工命令

```bash
just build        # cargo build --release，产物落 .githooks/canon
just test         # 全量测试
just spec-sync    # specs/ → .githooks/spec/ 正本同步部署镜像
just agents-push  # 重新组装本仓 AGENTS.md
just push         # canon-sync 推成员仓（custom/ 受保护）
```
