# AGENTS.md — algorchemy

算法模板库：实现参考 + 实测证据。

## 构建与验证（CI 驱动）

**所有测试、全量构建、bench、lint 放 PR 的 CI，本地不跑重型命令。**

本地只做轻量验证：
- `cargo fmt --check`（秒级）
- `cargo check`（类型检查）
- `grep` / `ls` / 文件读写等只读命令

**如果实在要本地跑重命令**（`cargo build`、`cargo test`、`cargo bench`、`cargo clippy` 等），**必须套 `.githooks/heavy`**（`heavy` 按 `nproc` 的一半现算 CPUQuota；注意该百分比是**单核**百分比，不是整机——写死 70% 在 8 核上只有 0.7 核，理由见 `.githooks/heavy` 头注）：

```bash
.githooks/heavy cargo test
.githooks/heavy cargo bench
```

`git`、`grep`、`ls` 等轻量命令不需要套。

## 项目结构

- `src/` — 算法实现（每个算法一个模块）
- `tests/` — 集成测试
- `benches/` — criterion 基准（union_find, text_buffers, eytzinger, line_buffer, ngram_index）
- `proptest-regressions/` — proptest 失败用例记录

## 提交规范

Conventional commits：`feat:` / `fix:` / `chore:` / `docs:` / `test:` / `bench:`。

## 删除规范

- 非 git 跟踪文件一律 `gio trash <path>`（可恢复）；禁止 `rm` / `rm -rf` / `git clean`。
- git 跟踪文件用 `git rm`。
