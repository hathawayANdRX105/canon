# 豁免、严重度裁决与拦截点全清单

`canon-check` 的附属参考，按需查，不必通读。**这些是低频查询**：被拦了先回 SKILL.md
第五节的自查循环走完，确实需要调开关或搞清拦截点时才读这里。

改约束（加规则、动 yaml）走 `gate-spec` skill；本文件只回答「现有哪些开关能调」。

---

## 一、严重度最终值由谁决定（降级前必读）

每条 checklist finding 的最终严重度 = `min(yaml fail_severity, harness 报告的 severity)`
（序：`FAIL < WARN < INFO`，取更严的一档；`engine.rs:485` `merge_severity` 是
`std::cmp::min(base_severity, reported)`）。

**含义：只把 yaml 的 `fail_severity` 从 FAIL 改成 WARN 是无效降级**——只要 harness 的
输出 JSON 里还写着 `severity: "FAIL"`，finding 依然硬拦。降级必须同时改两处：

1. 该规则的 `fail_severity:`
2. harness 命令输出 JSON 里的 `severity` 字段（`checklist_*.yaml` 的 jq 片段里写死）

**降级不是消费侧的动作。** 走 issue 交维护者裁决，不要在提交前就地改。

已降级记录：`rust_tests_in_tests_dir` FAIL→WARN（架构偏好规则；`bin/*/src/*.rs` 的
内嵌单测在存量大仓会全仓命中，agent 只会学会忽略；finding 仍打印，仍需逐条处置）。

---

## 二、怎么豁免

**原则：要不要拦截全部在 spec yaml 里配，不改代码。**

### checklist 系

改该 `checklist_*.yaml` 文件的 `fail_severity`（如把 `slop_comment` 从 WARN 降 INFO）。
注意第一节的 `min` 规则——见上。

### 检查能力选择（`checks:` 白名单）

各 family yaml（`github_*.yaml` / `cleanup_*.yaml` / `workspace_*.yaml` / `code_*.yaml`）
顶部可加 `checks: [ID或前缀]`——只启用列出的检查项；缺省 = 全部启用。

### 家族严重度（`fail_severity`）

CL/WS 系 family yaml 的 `fail_severity: WARN|FAIL|INFO` 统一改本家族检查项严重度
（INFO 不可被提升）。

### github 系

改各文件的 `severity_overrides:` 段，按 `规则ID` 覆盖严重度，如 `IS-16: "WARN"`。
检查开关也在这一层：

- `github_issues.yaml`：`close_requires_comment`(GT-COMMENT) / `close_done_when_gate`(GT-04) /
  `done_when_judge.enabled`(DWJ 模型评审) / `epic_sub_issue_gate`(GT-06) /
  `merge_fixes_gate`(GT-05)——`false` = 整块跳过
- `github_pull_requests.yaml`：`merge_requires_body`(GT-BODY) /
  `merge_checkbox_gate`(GT-CHK) / `merge_title_gate`(CM-01/02 squash 标题)
- `github_reviews.yaml`：`merge_review.required: false` 关掉 RV-07 强制审查
  （`merge_review.timeout_secs` 调超时）

### 模型审查开关（review_chain）

配环境变量选档——`TYPESAFE_API_KEY`(+`TYPESAFE_API_BASE`/`JEV_MODEL`) 启用 jev，
`REVIEW_LLM_BASE_URL`/`REVIEW_LLM_API_KEY`/`REVIEW_LLM_MODEL` 启用小模型降级，
都不配则 INFO 跳过；问题集/阈值改 `.githooks/spec/harness/jev_questions_review.json`
（每问题 `fail`/`warn`）。

### gh 拦截闸门（GT-* / CM-* / RV-*）

- 严重度降级：`dispatch.yaml` 的 `severity_overrides:` 段，或全局
  `.githooks/spec/severity_overrides.yaml` 按 ID 覆盖（如 `GT-06: "WARN"`）
- commit 检查（CM-01/02/03）：`dispatch.yaml` 的 `severity_overrides:` 段
- 数据解析/子查询失败仍 fail-closed 硬拦（**安全属性，不可配**）

### 单条放行

`git commit --no-verify` —— **不推荐**，绕过全部钩子。属于违规，不是配置项。

---

## 三、拦截点全清单（26 处）

| 类别 | 位置 | 规则 | 可否配置 |
|---|---|---|---|
| 数据错误 | `engine.rs` run_all/run_named | `gate.setup`（无 `.githooks/`、无 `checklist_*.yaml`、yaml 损坏） | **否**，fail-closed 硬拦 |
| 数据错误 | `shared.rs` `load_spec_yaml` | 缺 `dispatch.yaml` / `github_*.yaml`（pre-commit、pre-push、merge） | **否**，fail-closed |
| git commit | `pre_commit.rs:113` / `:118` / `:141` | `CM-01` 非 conventional / `CM-02` 标题含 CJK / `CM-03` commit type 与 PR type 不一致 | 是（`dispatch.yaml` `severity_overrides`） |
| git push | `pre_commit.rs` 之后的 l1/l2 链 | 任一 checklist `FAIL`（含 `code_*` 六条工具链、`checklist_ccn`、`rust_no_process_cmd`） | 是（yaml + `severity_overrides.yaml`） |
| git merge | `merge.rs:140-170` | `RV-07` 审查强制（`merge_review.required`） | 是（`github_reviews.yaml` 开关 + override） |
| git merge | `cleanup.rs:47-90` | `CL-01` 分支已合并/孤儿/临时前缀需清理 | 是（`cleanup_branch_cleanup.yaml`） |
| gh issue create | `gh_wrap.rs:371-553` | `GT-01`/`GT-03` 标题/正文/label/父子关联（映射 `IS-*`） | 是（`github_issues.yaml` + override） |
| gh issue close | `gh_wrap.rs:555-731` | `GT-04` Done when 未勾 / `GT-05` 缺 Fixes / `GT-06` epic 子 issue / `GT-07` 关联 PR | 是（同上，含 `done_when_judge`） |
| gh pr create | `gh_wrap.rs:733-805` | `GT-02` PR 标题/正文/head/label（映射 `PR-*`） | 是（`github_pull_requests.yaml`） |
| gh pr merge | `gh_wrap.rs:807-950` | PR 内容复检 + 关联 issue 的 GT-04/05/06 | 部分（正文类可覆盖；数据解析失败仍 fail-closed） |
| 旁路审计 | `audit.rs:140-145` | 打印 `IS-*`/`PR-*` 的 FAIL finding（不改变退出码语义） | 是 |

**原则**：policy 类（可配）才是降级候选；**数据错误类（fail-closed）是安全属性，不动**。

---

## 四、fail-closed：规范缺失本身是错误状态

规则 yaml 缺失或写坏（键名拼错）时 gate 直接 FAIL 报错（`gate.setup`），不会静默放行。

**「没有规范 / 规范坏掉」不是通过，是故障。** 见到 `gate.setup` 先修配置，不要当成
「没有规则所以没事」。

---

## 五、相关文件

| 想改什么 | 去哪 |
|---|---|
| 加一条 checklist / 改 harness | `gate-spec` skill |
| 自定义 spec（jev_rule 四件套） | `gate-spec` skill |
| 只想知道有哪些豁免开关 | 本文件第二节 |
| 想知道某条规则怎么检测 | `spec_explain(rule_id)` |
| 想知道有哪些规则 | `spec_catalog` |
