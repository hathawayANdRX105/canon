# sightline 约定

## 项目性质

自研项目：**单二进制 MCP sidecar**，让任何 AI agent 通过 MCP 读到开发者对运行中
网页的视觉标注反馈。框架无关、零运行时依赖、不绑定 dev server。
总体设计见 `docs/architecture.md`（改动前先读）。

## 结构

单 crate（`src/`）+ `build.rs`；配套目录：`sdk/`（对外接口）、`demo/`（演示页）、
`ref/`（参考实现）、`docs/`（设计文档）。无 gate（仓内无 `.githooks/`）。

## 构建与验证

- 本地只跑针对性命令：`cargo check` / `cargo test -p sightline -- <测试名>`。
  重命令套 cgroup CPU 配额（见 `rust-dev-perf` 章节）。
- 内环测试选择：`just test-fast`（testless 影响分析）；全量：`just test`。
- 提交前 `cargo fmt --check` + `cargo clippy`；MCP 协议行为改动必须带协议层测试
  （`tests/` 放集成测试，禁在 `src/` 里堆 `#[cfg(test)]`）。
