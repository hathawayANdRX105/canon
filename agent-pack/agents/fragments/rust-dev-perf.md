# canon 体系：Rust 开发性能（sccache + 限流已接入）

本仓 `.cargo/config.toml` 已配 `jobs = 4`（多会话并发上限）与
`rustc-wrapper = sccache`（跨 worktree 编译缓存），`Cargo.toml` 已关增量、
降 debuginfo。配置随 cargo 向上搜索对 `.wt/*` worktree 自动生效。

- 跑测试用 `just test-fast`：testless 函数级影响分析，只跑本次改动可能破坏的
  测试；testless 异常/零命中自动降级全量，绝不静默跳过。全量务必
  `cargo test --workspace`（根包 workspace 下裸 `cargo test` 只跑根包）。
- 不要在会话里自行 `export RUSTC_WRAPPER` 或改 jobs——统一走仓配置；
  重命令照旧套 cgroup CPU 配额（`systemd-run --user --scope -p CPUQuota=70% --`）。
- 增量编译已关（缓存优先）：同树连续小改动按 crate 级重编是预期行为，不是
  回归；若本仓热重载明显变慢，提 issue 议局部放开。
- 新建 `.wt` worktree 直接用；旧布局 worktree 若报 workspace 收编错误，
  根因与修法见 canon 仓 `Cargo.toml` 的 `exclude` 注释。
- 配置细节、坑清单与实测基线：skill `rust-dev-perf`。
