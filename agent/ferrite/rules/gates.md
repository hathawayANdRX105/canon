# 闸门（gate）与 GitHub 操作

**什么时候读这份文档**：提交或推送被拦下、创建 PR/issue 被拒绝、或者想知道有哪些检查规则的时候。

**这份文档解决什么**：本仓有一道自动闸门，会在你提交、推送、创建 PR 时拦截不规范的内容。
被拦下时它输出的每一条都必须处理完，不能绕过。

---

## 一、遇到什么问题（含曾经踩过的坑）

| 你看到的现象 | 意思 | 怎么办 |
|---|---|---|
| 提交时输出一堆 `checklist.xxx` 或 `WS-xxx` 行 | 闸门检查项。`FAIL` 是硬拦截，`WARN` 只是提醒 | FAIL 必须全部修掉；WARN 可以说明理由后放过 |
| 创建 PR 时报 `闸门: 校验 FAIL，拒绝创建` | PR 的标题或正文不符合模板 | 修到 FAIL 清零再重新创建；改已有 PR 用 `gh pr edit` |
| 报错说 `headings contain CJK` | PR 正文里的标题用了中文 | 标题必须用英文（`## What`、`## Why` 等），正文内容用中文 |
| 报错说缺 type label | PR 没有类型标签 | 加 `bug` / `feature` / `chore` / `refactor` / `tests` / `documentation` / `epic` 中的至少一个 |
| 报错说某段没有中文 | 模板要求某些段落必须用中文写 | 按模板补中文说明 |
| 用 `\| head -5` 看输出，以为改完了，其实后面还有 FAIL | 过滤截断了输出 | **完整读**闸门的输出，不要用 `head` / `tail` / `grep -v` 过滤 |

---

## 二、维护者希望做什么事

- **拦截信息逐条读完再修根因**。禁止用 `--no-verify` 跳过，禁止截断输出后假装没看见。
- **FAIL 必须清零。** WARN 可以在说明理由后放过。
- **GitHub 操作在创建时就会校验**（不是等推送才发现）。所以创建 PR / issue 之前先跑预检，不要等撞墙。
- 加新规则只改 `.githooks/spec/checklist_*.yaml`，不要改闸门程序本身。

---

## 三、可能的情况

### 3.1 三种钩子

闸门挂在三个位置（都在 `.githooks/hooks/`）：

- **pre-commit**：提交前
- **pre-push**：推送前
- **merge**：合并前

出问题最常跑的两条命令：

```bash
gate pre-commit    # 提交前的完整检查
gate pre-push      # 推送前的完整检查
```

合并前还有一道预检：`hooks/merge --dry-run`，不能绕过。

### 3.2 GitHub 侧校验（创建 PR / issue 时）

创建 PR 或 issue 时如果被闸门拦截，它会输出 `FAIL` 和 `WARN` 两类行：

- **FAIL**：直接拒绝创建。必须逐条修掉再重试。
- **WARN**：不拒绝，但每条都说明缺什么（比如"缺 type label"、"建议也挂某个标签"、"缺 `Fixes #` 关联"）。
  能补就补（用 `gh pr edit --label` 或 `gh issue edit --label`），补不了的要在 PR 正文里写明理由——
  理由写在 PR 正文对应的段落里（如 `## Issue` 段末尾加一句"未关联 issue 的原因：…"）。

创建前可以先跑预检：

```bash
gate check          # 列出当前检查项
gate issue          # issue 预检
gate pr             # PR 预检
```

**注意**：`head`、`tail`、`grep -v` 这些过滤会吞掉部分提示行，看闸门输出时必须完整读。

### 3.3 PR 正文模板要求

创建 PR 时正文需要包含这些段落（标题用英文，内容用中文）：

```text
## Issue          关联的 issue（没有就说明原因）
## What           改了什么
## Why            为什么改
## Construction plan   实现步骤（checklist）
## Delivery record      交付记录（改动文件、验证方式）
## How to test    怎么验证
## Checklist      检查清单
```

另外：**PR 标题必须是纯英文**（如 `fix(metering): ...`）；正文里的小节标题（`## What` 等）也必须是英文，
但正文内容用中文写。「标题不得包含中文」指的是 PR 标题那行，不是正文内容。

### 3.4 规则文档在哪

- `.githooks/GATE_HANDBOOK.md`：完整手册。三层检查的意思：
  **l1 结构层**（文件放哪、有没有写文档这类格式检查）、
  **l2 语义层**（依赖方向、命名这类代码语义检查）、
  **l3 LLM 层**（用大模型判断的深层问题，只在合并时跑）。
  共 16 条规则，每条标了触发时机和严重程度。
- `.githooks/spec/SPEC_OVERVIEW.md`：规则对照清单（新增或修改规则后必须同步更新这个文件）。
- `.githooks/spec/github_pr_gates.yaml`、`.githooks/spec/checklist_pr_*.yaml`：GitHub 相关的具体规则。
- 手动跑某个检查：先看 `SPEC_OVERVIEW.md` 找规则名（如 `rust_todo_needs_issue`），
  再跑 `gate check <规则名> --sla l1`。
- `gate` 命令装在 `~/.local/bin/gate`（已在 PATH 里）；钩子脚本在 `.githooks/hooks/`。

### 3.5 代码里的占位符要求

未实现的函数或 trait 必须用 Rust 原生宏，并且带上 issue 号：

```rust
todo!("TODO(#123): 说明这里要做什么")
unimplemented!("...")
```

TODO / FIXME 注释也必须带 issue 号，写成 `// TODO(#123): ...`。
这是闸门的 `rust_todo_needs_issue` 检查项，不带号会被拦。

---

## 四、约束事项（简略）

- 禁止 `--no-verify` 跳过钩子。
- 禁止截断闸门输出后忽略（`head` / `tail` / `grep -v` 会吞提示行）。
- FAIL 必须清零才能继续；WARN 每条都要处理或说明理由。
- 不绕过 `.githooks/` 的拦截，不绕过 `hooks/merge --dry-run` 预检。
- 占位符用 `todo!()` / `unimplemented!()` 并带 issue 号；TODO 注释同样要带。
