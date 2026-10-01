# ricq 约定（第三方 fork）

## 项目性质

`ricq`（QQ Android 协议的 Rust 实现，移植自 OICQ）的**本地 fork**，本仓不是上游。
本地改动集中在「能在 stable + edition 2024 上构建」这一条线（去掉 nightly-only 特性）。
同步上游时只 rebase 协议层，**不要把构建修复冲掉**。

## 结构

workspace 三 crate：`ricq`（协议主体）、`ricq-core`（核心抽象）、`ricq-guild`（包体）。
`rust-toolchain.toml` 钉工具链——**本地/CI 版本必须一致**，禁止 `rustup update stable`。
无 gate（仓内无 `.githooks/`），提交前自跑 fmt。

## 构建与验证

- 本地只跑针对性命令：`cargo check -p <crate>` / `cargo test -p <crate> -- <测试名>`。
  重命令套 cgroup CPU 配额（见 `rust-dev-perf` 章节），禁止裸跑。
- 内环测试选择：`just test-fast`（testless 影响分析）；全量：`just test`。
- 全量测试与 clippy 交 CI；本地复现失败前先看 CI 日志。
