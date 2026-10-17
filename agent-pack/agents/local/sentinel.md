# Sentinel Agent Guide

## Scope

- 本仓库**只有** Sentinel 审查工具（纯 Rust，crate 在仓库根：`Cargo.toml` + `src/`，单二进制 `sentinel`），不含小说正文；小说目录一律按 CLI 输入路径传入。
- 不再使用 `review` 命名，避免与 novel 项目内部的 `review/` 包冲突。
- 规则只改 `configs/rules/review.yaml`；不要在 Rust 里新硬编码中文词库。
- `tests/fixtures/expected/` 的 golden 基线是**逐字节契约**：行为有意变更时重跑对应子命令再生基线并随改动入库；口径与归一化规则见 `README.md` 的「测试与 golden 基线」。

## Workflow

- 有 `.beads/` 时用 `bd` 管任务状态（`just bd`）。
- 保守删除：仅在测试覆盖行为后再删重复 loader / 死包装。
- 使用说明与子命令全表在 `README.md`；遗留事项与参考材料在 `todo/`。

## Verification

```bash
just build   # cargo build --all-targets --all-features
just test    # cargo test --all-targets --all-features（含 golden 基线逐字节对照）
just clippy  # cargo clippy --all-targets --all-features，零警告
just fmt     # cargo fmt --all -- --check
```

CLI smoke（`just smoke <args>` = `cargo run --bin sentinel -- <args>`）：

```bash
just smoke rules
just smoke audit-draft tests/fixtures/draft/standalone.md --format text
just smoke audit-plan tests/fixtures/plan/arc-plan.md --output /tmp/plan.md
just smoke stats-draft tests/fixtures/draft --output-root /tmp/draft-stats-out
```
