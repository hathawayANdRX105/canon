# Sentinel Agent Guide

## Scope

- 本仓库只有 Sentinel 审查工具，不含小说正文。
- 外部 novel 数据通过 CLI 路径传入；默认真实 smoke 路径见 [`GUIDE.md`](GUIDE.md)。
- 包在 `src/` 下；不要重新引入顶层 `sentinel/` 包。
- 跨 agent 使用说明：根目录 [`GUIDE.md`](GUIDE.md)（已合并原 `docs/agent-usage.md`）。

## Workflow

- 有 `.beads/` 时用 `bd` 管任务状态。
- 规则只改 `configs/rules/review.yaml`；不要在 Python 里新硬编码中文词库。
- 保守删除：仅在测试覆盖行为后再删重复 loader / 死包装。

## Verification

```bash
cd ~/projects/sentinel
export PYTHONPATH=src
python3 -m unittest tests.test_rules_config tests.test_outputs tests.test_real_draft_smoke -v
# 或
just test
```

CLI / smoke：

```bash
python3 -m audit.plan --input path/to/plan.md --output /tmp/plan.md
python3 -m audit.draft --input path/to/ch01.md --format markdown --output /tmp/draft.md
python3 -m stats.plan --input path/to/plans --output-root /tmp/plan-stats-out
python3 -m stats.draft --input path/to/story-dir --output-root /tmp/draft-stats-out
just smoke-real-stats
just smoke-real-audit
```

## 发现处置纪律（gate / jev / review）

自动检查的每条 finding（gate `FAIL`/`WARN`、`jev` L3 发现、CRG / `ocr review` 意见）必须逐条处置：

1. **先读规范再改代码**：先读本仓规范（本文各节与 `docs/` 下的约定）确认要求，再动代码。判定不了就记为待裁决写进交付记录，不猜、不忽略。
2. **修根因**：让规则约束成立，不是让检查不再报。
3. **禁止糊弄式修复**：改/删 `.githooks/spec` 规则降严重度、`--no-verify`、`head`/`tail`/`grep -v` 截断输出、`#[allow(...)]`/`# noqa` 压制、空文件/空目录占位、`assert!(true)` 填数、拆分改名只为躲匹配范围——一律违规。
4. **逐条留痕**：修复写 `规则 ID → 根因 → 改法(file:line)`；驳回写 `规则 ID + 理由 + 依据` 交维护者裁决。落点 = PR 正文 `## Delivery record` 或 issue 交付评论。沉默即违规。
5. **WARN ≠ 可忽略**：与 FAIL 同等处置。

完整版与判例：canon `specs/agents/_discipline.md`；本仓 `AGENTS.md` 由 canon 维护并 agent-sync 下发，勿单独改。
