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
