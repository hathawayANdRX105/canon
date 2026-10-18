# 新仓接入 checks 规则库（SOP）

把 canon 的 Rust 规则库（`check-core` / `check-rust` / `check-dioxus` / `check-deps` /
`check-crate` / `check-assets`）接进一个成员仓。形态已由 mono 的 T3–T7 五轮接线验证；照抄即可。

> 前置：目标仓是 Rust 仓。canon 仓是**公开仓**（`github.com/hathawayANdRX105/canon`，
> 匿名可拉），CI 不需要任何凭据。

## 1. 建 xtask/ 工作区（刻意不进目标仓 workspace）

为什么独立：xtask 有自己的依赖节奏——canon 规则库按 tag 升级，独立 lock 让
目标仓根 `Cargo.lock` 不被门禁依赖牵动，`cargo build/test --workspace` 也不必
编译 check-* crates。规则只在开发机与 merge 钩子上跑。

```
xtask/
├── Cargo.toml      # [workspace] 清单：members = ["check"]（未来加别的 xtask 命令）
└── check/
    ├── Cargo.toml  # [package] xtask-check + canon git 依赖（钉 tag）
    └── src/main.rs # 组装规则集（4 行）
```

`xtask/Cargo.toml`：

```toml
[workspace]           # xtask 独立工作区：不进目标仓 workspace（canon 按 tag 升级
resolver = "2"        # 不牵动目标仓根 lock）
members = ["check"]
```

`xtask/check/Cargo.toml`：

[package]
name = "xtask-check"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
check-core = { git = "https://github.com/hathawayANdRX105/canon", tag = "check-v0.1.5" }
check-rust = { git = "https://github.com/hathawayANdRX105/canon", tag = "check-v0.1.5" }
check-dioxus = { git = "https://github.com/hathawayANdRX105/canon", tag = "check-v0.1.5" }
check-deps = { git = "https://github.com/hathawayANdRX105/canon", tag = "check-v0.1.5" }
check-crate = { git = "https://github.com/hathawayANdRX105/canon", tag = "check-v0.1.5" }
check-assets = { git = "https://github.com/hathawayANdRX105/canon", tag = "check-v0.1.5" }

[[bin]]
name = "check"
path = "src/main.rs"
```

按需裁剪依赖（只做 rust 规则就只留 `check-core` + `check-rust`）。
**tag 钉版**，不用分支——升级是显式动作（见第 5 步）。提交 `xtask/Cargo.lock`。

`xtask/check/src/main.rs`：

```rust
fn main() {
    let mut static_rules = check_rust::rules();
    static_rules.extend(check_dioxus::rules());
    static_rules.extend(check_deps::rules());
    static_rules.extend(check_assets::rules());
    let mut factories = check_core::kinds::builtin_factories();
    factories.extend(check_dioxus::factories());
    factories.extend(check_deps::factories());
    factories.extend(check_assets::factories());
    std::process::exit(check_core::cli::run(check_core::cli::Frontend { static_rules, factories }));
}
```

### 1.5 入口别名：`.cargo/config.toml` 加 `[alias] xtask`

```toml
[alias]
xtask = "run -q --manifest-path xtask/Cargo.toml -p xtask-check --"
```

xtask/ 是独立工作区（不进目标仓 workspace），所以要走 `--manifest-path` +
`-p xtask-check`。此后
成员仓检查统一入口：`cargo xtask check <list|check|explain>`（kime/mono 已落地）。
将来加别的 xtask 命令：加 `xtask/<命令>/` crate + 一条 `xtask-<命令>` alias。

## 2. 写策略：仓根 `checks.toml`

一页文件装三类东西；**规则管判定、路径管策略**——存量债/非纪律面用 `scope_*`
收口，不改规则实现（这是 T4 踩过的坑：RSX 规则全仓口径在 mono 出过 ~1200 条 WARN）。

```toml
[semantic]
endpoint = "http://127.0.0.1:8000"   # laya（本机）；未运行时 semantic 类 → CHECKS-NOTCHECKED INFO

# 参数化实例：同一 Rust 实现，多实例（键 = kind 名; 见 `checks list`）
[kinds.rsx-nesting.web]
max_depth = 3

[kinds.dep-edge.domain]
edges_deny = [
  { from = "crates/ui/**", to = "crates/api/**" },   # 目录 glob（`/**` 含子树）
]

# 策略层覆盖：severity / enabled / scope_include / scope_exclude（glob）
[rules."RSX.cjk-text"]
scope_include = ["crates/ui/**"]
scope_exclude = ["apps/kit-demo"]
```

**语义路径表 `[domains]`**：仓内布局的单一事实源。`[rules.<id>]` 的
scope_include/exclude 条目可写 `@<域>` 标记，load() 展开为对应 glob 列表——
仓库改名只改 `[domains]` 一处，未知标记配置期硬报（笔误不会静默变零命中）：

```toml
[domains]
frontend = ["crates/ui/**", "apps/web/**"]

[rules."RSX.cjk-text"]
scope_include = ["@frontend"]
```

实例参数键由各 kind 的 serde 类型裁决（`deny_unknown_fields`）：**写错键会硬报**，
不会静默忽略。规则目录与条文：`cargo xtask check list`（`--json` 机器可读）与
`cargo xtask check explain <id>`。

## 3. 接门禁：canon checklist（成员仓本地）

`.githooks/spec/quality/checklist_checks_deterministic.yaml`：

```yaml
# CHECKS-DETERMINISTIC: canon 规则库成员仓入口
hooks: [merge]
sla: l1
fail_severity: WARN              # 起步 WARN，观察一周期后按实据升 FAIL
match:
  paths_include: ["**/*.rs", "**/Cargo.toml"]
  paths_exclude: ["target/", ".wt/"]
mode: grep
harness:
  command: "sh"
  args:
    - "-c"
    - |
      ROOT=$(git rev-parse --show-toplevel)
      cd "$ROOT" || exit 2
      BASE="${GATE_BASE:-origin/main}"
      cargo run -q --manifest-path xtask/Cargo.toml -p xtask-check -- check --root "$ROOT" \
        --scope changed --base "$BASE" --class deterministic --canon --json
optional: true # 依赖/工具缺失 → WARN 跳过，不假绿
timeout: 600
```

- **`--canon` 是必须的**：引擎契约只认 rc 0（解析 findings）/ 2（WARN 跳过）；
  `--canon` 把 notChecked 收敛成 `CHECKS-NOTCHECKED` INFO finding、退出码收敛 0/2。
  不带它时 agent 口径的 rc=10 会被引擎当成异常并丢弃 stdout。
- `optional` / `timeout` 是**顶层**字段，不在 `harness:` 里。
- `--scope changed`（+ `GATE_BASE`）：只判本次改动，存量债不进热路径。
  全仓审计手动跑：把 `--scope changed --base …` 换成 `--scope all`。
- 不挂 pre-commit（首次冷编成本）；要挂 pre-push 需先有预构建产物兜底。

## 4. 验收（照跑四条）

```bash
cargo xtask check list                                         # 规则集（静态 + 实例）
cargo xtask check --root . --scope all --class deterministic --canon --report-json | jq '{counts: (.findings | group_by(.id) | map({id: .[0].id, n: length})), not_checked}'
cargo xtask check --root . --scope changed --base origin/main --canon --report-json   # 本次改动应为空
canon check checks_deterministic                     # 走引擎接线（merge 口径）
```

**迁移对照**（若目标仓已有同义的 shell/yaml 规则）：双跑期两侧各跑一遍，比
`(path,line)` 集合；canon 侧 `spec/` 旧规则保留至逐一比对完成后再退役。

## 5. 升级与维护

- canon 侧新增 kind/规则 → 在 canon 仓打新 tag（**必须打在 origin/main**：
  `git tag check-vX.Y.Z origin/main && git push origin check-vX.Y.Z`；本地 main 可能
  含他人未推送提交）；目标仓升 tag 后 `rm xtask/Cargo.lock` 重建。
- 阈值/严重度调整只改 `checks.toml`，不动规则代码；规则条文用 `explain <id>` 查。
- 存量债处理次序：先 `scope_exclude` 把非纪律面摘出去，再逐条整改；
  存量清零后把 `fail_severity` 升到 FAIL。

## 已知边界

- 块级/行级口径的解析器不追求通用（如 CSS 只认列首选择器开块）；换布局先说清。
- `semantic` 类依赖本机 laya 端点；端点不可用时按 `CHECKS-NOTCHECKED` 显式标注，
  绝不做 candidates-raw 假绿。
