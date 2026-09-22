# 测试与 CI

**什么时候读这份文档**：要写测试、要跑测试、要看 CI 结果、或怀疑"CI 绿了但根本没验东西"的时候。

**这份文档解决什么**：本仓的测试有**三种「看起来通过、其实没验证」的情况**——测试写了、CI 也绿了，但实际上一个断言都没执行。
新写测试前必须知道这三件事，否则你以为的回归保护是空的。

---

## 一、遇到什么问题（含曾经踩过的坑）

| 你看到的现象 | 实际发生了什么 | 怎么确认 |
|---|---|---|
| CI 全绿，但某个测试的断言从没执行过 | 测试被 `cfg(feature = ...)` 门禁，而 CI 没有任何路径开启这个 feature | CI 日志里搜 `Running <你的测试文件>.rs`，紧跟的是 `running 0 tests`（而不是 `running N tests`） |
| e2e 测试显示 passed，但耗时正好是 30 秒 / 60 秒 / 90 秒 | CI 里没有 Postgres，测试连不上库，超时后 `return` 跳过——却被记为 passed | 耗时是 30 秒的整数倍；日志里搜不到 `skipping e2e`（这条是 stderr，CI 吞掉了） |
| 有 61 个测试从来没在 CI 里跑过 | 它们标了 `#[ignore]`，而 CI 从不带 `--ignored` 参数运行 | 全仓 `#[ignore]` 出现在 18 个文件里，CI 一律跳过 |
| 本地跑绿了，CI 却报 lint 错 | 本地 clippy 版本比 CI 旧 | CI 用 `dtolnay/rust-toolchain@stable`；先跑 `rustup update stable` |
| 本地跑测试把机器跑死 | 本机可用内存常年不到 2GB，多 crate 一起编译会耗尽内存 | 本地只做 `cargo check -p <crate>`，测试全部交给 CI |

---

## 二、维护者希望做什么事

- **所有测试在 CI 跑**，本地只验证"能不能编译"。
- **CI 全绿是必要条件**（CI 没跑完不许合并），但记住：**绿不等于所有断言都验过**——
  下面的三个失效模式都会造成假绿。所以绿之后，还要按本文 §3.3–3.5 的判据自查一遍。
- **测试必须真的在执行**：新增测试后要确认 CI 日志里能看到它在跑、能看到断言数量。做不到这一点，就等于没写测试。
- 因为 CI 没有数据库，凡是依赖数据库的测试：**文件头加注释「CI 不验证此断言」**，
  并且**在本地对着真库（`uf-local-postgres` 的 5433 端口，`ferrite_e2e` 库）手动验证过**。注释写法示例：

  ```rust
  // ⚠️ CI 不验证此断言（CI 无 postgres）。本地验证：
  //   FERRITE_E2E_DATABASE_URL=postgres://ferrite:ferrite@127.0.0.1:5433/ferrite_e2e \
  //   cargo test -p tests-e2e --test <文件名>
  ```

---

## 三、可能的情况

### 3.1 本地能跑什么、不能跑什么

| 场景 | 命令 | 说明 |
|---|---|---|
| 验证代码能编译 | `cargo check -p <crate>` | 本地**唯一**常规验证方式，必须套 `cpulimit -l 65 -i --` |
| 调试单个失败用例 | `cargo test -p <crate> -- <测试名>` | 仅用于调试，不能替代 CI 验收；测试要能在 3 秒内跑完 |
| 预览 CI 会跑哪些包 | `bash scripts/ci-affected.sh --base newxapi/main --dry-run` | 提 PR 前可以看，不影响任何东西。`newxapi/main` 是上游主分支的名字（本仓的 remote 叫 `newxapi`，主分支叫 `main`），直接照抄即可 |
| 跑全量测试 | **禁止** | `cargo test --all`、整个 workspace 编译会耗尽本机内存导致假死 |

### 3.2 CI 怎么决定跑哪些包

CI 脚本在 `scripts/ci-affected.sh`。规则：

| 触发 | 跑什么 |
|---|---|
| PR | 按改动的文件路径选出直接受影响的包，再沿依赖关系**反向**补齐所有依赖它们的包 |
| 推送到 main | 全量：`cargo build --all-targets` 加 `cargo test --all` |
| 两者都跑 | `cargo fmt --all --check`、`cargo clippy --all-targets -- -D warnings` |

反向补齐是必需的：比如改 `crates/api/tavern-storage`，如果只跑这个包，就会漏掉依赖它的 `api` 和 `tests-e2e`——编译能过，但测试断言可能已经被改坏了。

未影响的包不跑：前端 crate（`crates/web/*`、`apps/admin-web`、`apps/tavern-web`）做 wasm32 编译检查，其余做本机编译检查；有 `tests/` 目录的包额外跑 `cargo test -p`；纯文档改动直接跳过。

只有下面这些改动会升级成全量：`Cargo.toml`、`Cargo.lock`、`rust-toolchain.toml`、`.github/` 下的文件、`scripts/` 下的文件。

### 3.3 失效模式一：feature 门禁的测试，CI 不执行

**症状**：测试文件开头写了 `#![cfg(feature = "xxx")]`，CI 里这个文件被编译、被执行，但输出是 `running 0 tests`，整体仍然绿。

**原因**：CI 的命令是裸的 `cargo test -p <包名>`（见 `scripts/ci-affected.sh`），没有 `--features xxx`。`.github/workflows/ci.yml` 里的 `cargo test --all`、`cargo clippy --all-targets` 同样没有。

**实例**：`crates/gateway/metering/tests/ledger_concurrency.rs`（Shuttle 并发测试）。它需要 `cargo test -p metering --features shuttle` 才会真正跑；CI 日志里它的三个测试名一次都没出现。

**怎么办**：
1. 优先改成**不需要 feature 就能跑**（把依赖改成普通 dev-dependency）。
2. 如果必须靠 feature，就在 `scripts/ci-affected.sh` 的测试段里为该包补上 `--features`。
3. 无论如何，在测试文件头部写明它需要哪个 feature、CI 是否已经开启。

### 3.4 失效模式二：e2e 测试在 CI 里是假绿

**症状**：CI 显示 e2e 测试 passed，但耗时正好是 30 秒、60 秒、90 秒这样的整数倍。

**原因**：CI 的工作流里**没有 Postgres 服务**（`.github/workflows/` 下搜 `postgres` 是 0 次命中）。而 `tests/` 目录下的 e2e 测试默认连 `postgres://ferrite:ferrite@127.0.0.1:5433/ferrite_e2e`，连不上就打印 `skipping e2e: postgres unreachable` 然后 `return`——测试框架把它记为通过。每个测试的连接超时是 30 秒，所以 N 个测试就是 N×30 秒。

**受影响文件**（都在 `tests/` 目录下）：`billing_lifecycle.rs`、`web_wire_contract.rs`、`users_wire_contract.rs`、`manage_wire_contract.rs`、`overview_wire_contract.rs`、`network_write_path.rs`、`usage_log_type.rs`、`admin_gateway_flow.rs`。

例外：`tests/gateway_e2e.rs` 用 MockEgress 模拟上游，不连数据库，在 CI 里是真跑（耗时 0.02 秒）。

**本机是用真库跑的**：Docker 容器 `uf-local-postgres` 把 5433 端口映射出来，`ferrite_e2e` 库已经建好。
所以：**改 e2e 时，本地验证有效，CI 验证无效。**

**怎么办**：要么给 CI 加 Postgres 服务并设置环境变量 `FERRITE_E2E_DATABASE_URL`，要么明确接受"e2e 只本地跑"，在文件头写清楚。

### 3.5 失效模式三：`#[ignore]` 的测试永远不跑

**症状**：测试标了 `#[ignore]`，本地要手动加 `--ignored` 才跑，CI 里从不跑。

**原因**：CI 从没带过 `--ignored` 参数（`scripts/` 和 `.github/` 下搜不到）。

**规模**：全仓 **61 个** `#[ignore]`，分布在 18 个文件。主要集中在：
- `crates/api/admin-catalog/tests/models_channels.rs`（15 个）
- `crates/api/auth/tests/integration.rs`（11 个）
- `crates/api/admin-catalog/tests/channels_groups.rs`（6 个）
- `crates/api/admin-observe/tests/logs.rs`（5 个）

**仓库里已有两种相反的做法**：
- `crates/api/admin-billing/tests/` 下的文件（`invitees_list.rs`、`topup_affiliate.rs`、`topup_provider.rs`）注释里明确写了：
  用"连不上数据库就 return"的方式跳过，不用 `#[ignore]`——理由是 `#[ignore]` 在 CI 上等于没写。
- `crates/api/admin-catalog/`、`crates/api/auth/` 下的文件仍用 `#[ignore]`。

两种做法在 CI 里的效果**一样：都不验证断言**。区别只在本地：用 return 方式的，本地有数据库时会自动真跑；用 `#[ignore]` 的，本地也要手动加参数。

**怎么办**：新写依赖数据库的测试，用"连不上就 return"的方式，并在文件头注释写明"CI 无数据库，此测试不验证断言"。

---

## 四、约束事项（简略）

- 本地只跑两种命令：`cargo check -p <crate>` 和 3 秒内能跑完的单用例调试。都必须套 `cpulimit -l 65 -i --`。
- 禁止本地 `cargo test --all`、禁止整个 workspace 编译（内存不足会假死）。
- CI 未全绿不许合并；CI 失败要拉云端日志，当新任务修复。
- 本地 clippy 与 CI 同版本：改动前先 `rustup update stable`。
- 新增测试必须确认 CI 真的会执行它——避开上面三种情况。
