# gate 手册

> 本文件是 canon `docs/gate/` 主题手册正本（2026-09-23 从 `.githooks/` 移出归并——`.githooks/` 只留运行时：hooks + spec + 兜底二进制）。
> 规则协议文档在 `rules/docs/`（播种到各仓 `.githooks/spec/docs/`）；issue/PR 操作见 `../github/GITHUB_ISSUE_PR.md`；开发流见 `../workflow/`；收尾流程见 `../closeout/closeout.md`。

gate 是仓库自带的质量门禁：读 `.githooks/spec/*.yaml` 规则 → 调外部命令/LLM → 收 finding → 按严重度放行或拦截。
**加规则只改 yaml，不改二进制。** 本文件是人能查的一手总览；每条规则的参数以对应 `.githooks/spec/quality/checklist_*.yaml` 为准。

## 三层 SLA

| 层 | 性质 | 成本 | 是否阻断 |
|---|---|---|---|
| **l1 结构层** | 确定性（grep / clippy / machete / wc） | 毫秒～分钟，零 token | FAIL 硬拦 |
| **l2 语义层** | 轻量语义（重复块 / 跨 crate 影响面） | 秒级 | FAIL 硬拦 |
| **l3 LLM 层** | 按需 LLM，输出 `score`/`confidence` 参考分 | 分钟级 | **不阻断**，开发 agent 自行判阈值 |

`gate check` 默认只跑 l1；`--sla l2` / `--sla l3` 解锁更高层。重规则设 `hooks: [merge]` 不拖日常提交。

## 规则清单（20 条）

严重度列：`FAIL`=硬拦截，`WARN`=提示不拦，`INFO`=仅参考。

| 规则 | SLA | 自动触发 | 严重度 | 查什么 |
|---|---|---|---|---|
| `hardcoded_secret` | l1 | pre-commit/push/merge | WARN | 硬编码密钥/密码/Token（PCRE，5 语言） |
| `stale_api` | l1 | pre-commit/push/merge | WARN | 废弃 Rust API（`uninitialized`/`try!`/`ONCE_INIT`） |
| `slop_comment` | l1 | pre-commit/push/merge | WARN | AI 风格注释（步骤/叙述/拖延语/含糊语，如 `Step 1:`/`该函数`/`for now`/`临时`/`hopefully`/`估计`） |
| `ccn` | l1 | pre-commit/merge | **FAIL** | 函数 ccn 超天花板(默认 6)：新违规/恶化硬拦；存量记账 `ratchet.tsv` 容忍且只许降（`seed` 一次后记账进仓）；lizard 缺失静默跳过 |
| `antislop` | l1 | pre-commit/push/merge | WARN（harness 映射 HIGH→FAIL） | AI slop 五类（Placeholder/Deferral/Hedging/Stub/命名，`antislop` 二进制；缺失静默跳过） |
| `rust_no_process_cmd` | l1 | pre-commit/push/merge | **FAIL** | HTTP 走 reqwest，禁 subprocess 拉 curl/wget |
| `rust_tests_in_tests_dir` | l1 | pre-commit/push/merge | **FAIL** | 测试放同层 `tests/`，禁在 `src/` 留 `#[test]` |
| `rust_no_dead_code_allow` | l1 | pre-commit/push/merge | WARN | 合并前清理 `#[allow(dead_code)]`（同行带 `//` 理由放行） |
| `rust_no_empty_module` | l1 | pre-commit/push/merge | WARN | 微型空文件（≤2 行且无实现） |
| `rust_no_cfg_test_in_tests_dir` | l1 | pre-commit/push/merge | WARN | `tests/` 里不需要 `#[cfg(test)]` |
| `rust_test_no_assert` | l1 | pre-commit/push/merge | WARN | 测试函数必须含断言 |
| `rust_todo_needs_issue` | l1 | pre-commit/push/merge | WARN | TODO/FIXME 必须挂 issue 号（`// TODO(#N)` 或 `todo!("TODO(#N)")`） |
| `dep_hygiene` | l1 | merge | WARN | `cargo-machete` 未使用依赖（工具缺失则 WARN 跳过） |
| `clippy` | l1 | merge | **FAIL/WARN** | rustc 编译错误 + `unused_*`/`dead_code`→FAIL；`collapsible_if` 等风格→WARN |
| `file_size` | l1 | merge | WARN | 单 `.rs` >1500 行 或 >35KB → 提示按职责拆分（存量宽，清账后可升 FAIL） |
| `duplication` | l2 | merge | WARN | 跨文件 4+ 连续行重复块 |
| `crg_impact` | l2 | merge | WARN | diff 跨 3+ crate 改动，提示耦合 |
| `ferrite_oversize` | l3 | merge | INFO | 大文件/大函数参考分（wildtoken `fast-l`，带 `score`/`confidence`，不阻断） |
| `review_chain` | l3 | pre-push/merge | INFO（harness 透传 FAIL/WARN/INFO） | 模型审查层三档降级：jev（`TYPESAFE_API_KEY`）→ 小模型（`REVIEW_LLM_*`）→ 无（INFO）；每个问题带 per-question `fail`/`warn` 阈值，p≥fail FAIL 硬拦；finding 带 `tier`/`confidence` extra |

## 怎么跑

**自动**（已挂在钩子上，本机 `core.hooksPath=.githooks/hooks`）：`git commit` → pre-commit；`git push` → pre-push；`gate merge <repo> <pr>` → merge（含 checklist 全量）。
`RESULT: FAIL` 且存在 FAIL 级 finding → 退出码 1 → 对应 git 操作被拦截。

**手动**（调试 / CI / 按需）：
```text
gate check                       # 列出当前 l1 层全部规则（列表，不执行）
gate check clippy file_size     # 只跑指定规则
gate check --sla l3             # 解锁到 l3（含 LLM 参考层）
gate check --sla l3 --json      # 机器可读，带 score/confidence extra，给开发 agent 消费
```

## 怎么加一条规则

拷一份模板到 `.githooks/spec/quality/checklist_<名字>.yaml`，填参数：

```yaml
enabled: true
hooks: [pre-commit, pre-push, merge]   # 重活写 [merge]
sla: l1                                  # l1 确定性 | l2 语义 | l3 LLM(带分参考)
fail_severity: WARN                      # 兜底严重度；FAIL 才阻断
mode: grep                               # diff | file | grep(静态,自己扫)
match:
  paths_include: ["**/*.rs"]
  paths_exclude: ["target/", ".wt/"]
harness:
  command: "sh"
  args: ["-c", "<扫仓库根 + 输出 finding JSON 数组>"]
optional: true                           # 工具缺失时 WARN 跳过
timeout: 30
```

stdout 必须是 finding JSON 数组：`{"id","severity","path","line","message"}`（L3 可多带 `score`/`confidence`）。

**可移植标准（强制遵守，见 `spec/docs/CHECKLIST_SPEC.md`「mode: grep 规则编写标准」）：**
1. 扫仓库根 `"$ROOT"`，**禁止**写死 `crates/*/src` 布局（换仓库会静默扫 0 文件、假绿）。
2. grep 用 `--exclude-dir=target --exclude-dir=.wt --exclude-dir=.git`（按目录名，worktree 安全）。
3. find 用 `\( -name target -o -name .git -o -name .wt \) -prune -o ...`，**禁止** `-not -path "*/.wt/*"`（全路径 glob 在 `.wt/` worktree 下会把自己全排除）。
4. 跨语言测试文件命名一并 `--exclude`（`*_test.go`/`*.spec.ts` 等，`--exclude-dir=tests` 挡不住同目录测试）。

## 怎么豁免

**原则：要不要拦截全部在 spec yaml 里配，不改代码。**

- checklist 系（checklist_*.yaml）：改该文件的 `fail_severity`（如把 `slop_comment` 从 WARN 降 INFO）。
- 模型审查开关（review_chain）：配环境变量选档——`TYPESAFE_API_KEY`(+`TYPESAFE_API_BASE`/`JEV_MODEL`)启用 jev，`REVIEW_LLM_BASE_URL`/`REVIEW_LLM_API_KEY`/`REVIEW_LLM_MODEL` 启用小模型降级，都不配则 INFO 跳过；问题集/阈值改 `.githooks/spec/harness/jev_questions_review.json`（每问题 `fail`/`warn`）。
- 检查能力选择（`checks:` 白名单）：各 family yaml（github_*.yaml / cleanup_*.yaml / workspace_*.yaml / code_*.yaml）顶部可加 `checks: [ID或前缀]`——只启用列出的检查项；缺省 = 全部启用。
- 家族严重度（`fail_severity`）：CL/WS 系 family yaml 的 `fail_severity: WARN|FAIL|INFO` 统一改本家族检查项严重度（INFO 不可被提升）。
- github 系（github_issues.yaml / github_pull_requests.yaml / github_reviews.yaml）：改各文件的 `severity_overrides:` 段，按 `规则ID` 覆盖严重度，如 `IS-16: "WARN"`。检查开关也在这：`garbled_content_check: false` 直接关掉 IS-16，`ci_check_mode` / `done_when_check_mode` 控制 PR 检查是 FAIL 还是 WARN。
- gh 拦截闸门（GT-* 现在产出 Finding，可覆盖/可关）：
  - `github_issues.yaml` 开关：`close_requires_comment`（GT-COMMENT）/ `close_done_when_gate`（GT-04）/ `done_when_judge.enabled`（DWJ 模型评审，见下）/ `epic_sub_issue_gate`（GT-06）/ `merge_fixes_gate`（GT-05）——false = 整块跳过
  - `github_pull_requests.yaml` 开关：`merge_requires_body`（GT-BODY）/ `merge_checkbox_gate`（GT-CHK）/ `merge_title_gate`（CM-01/02 squash 标题）
  - `github_reviews.yaml`：`merge_review.required: false` 关掉 RV-07 的 CRG+ocr 强制；`merge_review.ocr_timeout_secs` 调 ocr 超时
  - 严重度降级：GT-*/CM-*/RV-07 在 `dispatch.yaml` 的 `severity_overrides:` 段或全局 `severity_overrides.yaml` 按 ID 覆盖（如 `GT-06: "WARN"`）
  - 数据解析/子查询失败仍 fail-closed 硬拦（安全属性，不可配）
- commit 检查（CM-01/02/03）：`dispatch.yaml` 的 `severity_overrides:` 段。
- 全仓统一兜底：`.githooks/spec/severity_overrides.yaml`（全局最后发言权，按 `规则ID` 覆盖一切来源的 finding）。
- 单条放行：`git commit --no-verify`（不推荐，绕过全部钩子）。

规则 yaml 缺失或写坏（键名拼错）时 gate 直接 FAIL 报错（`gate.setup`），不会静默放行——「没有规范/规范坏掉」本身是错误状态。

## DWJ：Done-when 模型评审（issue close 时）

`gh issue close` 在 GT-04（checkbox 全勾的机械门）过后，把 **Done when 每一条** 拿给模型评审是否真被证据满足——和 `review_chain` 同一套三档降级（jev → 小模型 → 跳过），证据 = 关联 PR diff，缺失时退化为 `--comment` 文本。

- 配置：`github_issues.yaml` → `done_when_judge:`（`enabled` / `command` / `args` / `timeout_secs`）；问题集 = `harness/jev_questions_done_when.json`（`default_fail: 0.85`）
- 语义：某条 p(未达标) ≥ 0.85 → **FAIL 硬拦**；否则 WARN/INFO 带 `tier`/`confidence` extra
- 降级：harness 缺失/超时/输出不可解析/两个模型档都不可用 → `DWJ-SKIPPED` INFO，**永不因基础设施阻断**——GT-04 机械门 + 工具检查仍是兜底
- 三档互斥与 review_chain 相同：jev 在就只跑 jev，小模型只兜底

## 路线图（已知短板，未启用）

| 项 | 工具 / 做法 | 状态 |
|---|---|---|
| 注释存在性门禁 | `RUSTFLAGS="-W missing_docs"`（public 59 处存量）；`clippy::missing_docs_in_private_items`（更严） | 存量清账前按 crate 灰度启用 |
| 测试强度 | `cargo-mutants` nightly（验证 agent 测试是否真在检验，抓自证测试）；轻量方案已落地：`done_when_judge`（close 时 jev 逐条判 p(未达标)，≥0.85 硬拦） | 轻量方案已上线；mutants 待接入 |
| 质量曲线 | `gate check --json` 每次 commit 落 jsonl（clippy 数/LOC/CRG risk/findings 分布） | 待接入 |
| 函数复杂度 | 已上线 `ccn` checklist（ccn 天花板 6 + ratchet 记账：`ccn_gate.py` 进 `rules/harness/`，`ratchet.tsv` 进仓）；余 lizard 进 CI 镜像 | 已接入 |
| AI slop 二进制 | `cargo install antislop` 进 CI 镜像（未装时 `antislop` 规则静默跳过） | 待接入 |
| 模块循环依赖 | `cargo-modules dependencies --lib --acyclic`（工具未装） | 待装 |
