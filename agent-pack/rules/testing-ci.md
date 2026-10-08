# 测试与 CI

**什么时候读这份文档**：要写测试、要跑测试、要看 CI 结果、或怀疑"CI 绿了但根本没验东西"的时候。

**这份文档解决什么**：本仓的测试有**三种「看起来通过、其实没验证」的情况**——测试写了、CI 也绿了，但实际上一个断言都没执行。
新写测试前必须知道这三件事，否则你以为的回归保护是空的。

---

## 一、遇到什么问题（含曾经踩过的坑）

| 你看到的现象 | 实际发生了什么 | 怎么确认 |
|---|---|---|
| CI 全绿，但某个测试的断言从没执行过 | 测试被 `cfg(feature = ...)` 门禁，而 CI 没有任何路径开启这个 feature | CI 日志里搜 `Running <你的测试文件>.rs`，紧跟的是 `running 0 tests`（而不是 `running N tests`） |
| 某个 e2e 测试的耗时正好是 30 秒 / 60 秒 / 90 秒 | DB 门禁测试连不上 Postgres（test 关卡服务容器缺失 / 故障）时超时后 `return` 跳过——却被记为 passed。CI 带 PG18 服务，出现这个信号 = **服务坏了，是 CI 事故** | 耗时是 30 秒的整数倍；日志里搜 `skipping e2e`（这条是 stderr，CI 吞掉了） |
| 有一批测试从来没在 CI 里跑过 | 它们标了 `#[ignore]`，而 CI 从不带 `--ignored` 参数运行 | 全仓 `#[ignore]` 出现在 13 个文件里共 56 处，CI 一律跳过 |
| 本地能跑绿，CI 却报 lint 错 | CI 与本地使用的 Rust 工具链版本不一致（新 clippy 可能新增 lint） | 版本由仓库 `rust-toolchain.toml` 固定（CI 检出源码后自动生效）；改 pin 前先查 `rustc --version`，禁止 `rustup update stable` 临时升级本地 |
| 本地跑测试把机器跑死 | 本机可用内存常年不到 2GB，多 crate 一起编译会耗尽内存 | 本地只做 `cargo check -p <crate>`，测试全部交给 CI |

---

## 二、维护者希望做什么事

- **所有测试在 CI 跑**（公开壳仓 `hathawayANdRX105/ferrite-ci`，见 AGENTS.md「CI 处理」），本地只验证"能不能编译"和 3 秒内的单用例调试。
- **CI 全绿是必要条件**（主仓 `shell-ci` 状态没绿不许合并），但记住：**绿不等于所有断言都验过**——
  下面的三个失效模式都会造成假绿。所以绿之后，还要按本文 §3.3–3.5 的判据自查一遍。
- **测试必须真的在执行**：新增测试后要确认 CI 日志里能看到它在跑、能看到断言数量。做不到这一点，就等于没写测试。
- CI 的 test 关卡挂了 Postgres 服务（`postgres:18-alpine`，`DATABASE_URL`/`FERRITE_E2E_DATABASE_URL` 均指向它，
  测试自带幂等迁移），**依赖真库的测试在 CI 上真跑**。剩余两种假绿风险：
  ① DB 门禁测试 catch-and-return——服务容器缺失 / 故障时打印提示后 `return`，仍被记为 passed；② `#[ignore]` 的测试从不执行。
  新写依赖数据库的测试：用"连不上就 return"的写法（不用 `#[ignore]`），**文件头加注释「此用例在 CI 上真跑；连不上库会 `return` 并被记为 passed」**。
  `FERRITE_E2E_DATABASE_URL` 是**本地替代**（`uf-local-postgres` 的 5433 端口，`ferrite_e2e` 库），不是 CI 限制——CI test 关卡已设置该变量。注释写法示例：

  ```rust
  // ⚠️ DB 门禁：此用例在 CI 上真跑（test 关卡带 Postgres 服务）；连不上库会 return 并被记为 passed（假绿）。
  //   本地替代：FERRITE_E2E_DATABASE_URL=postgres://ferrite:ferrite@127.0.0.1:5433/ferrite_e2e \
  //   cargo test -p tests-e2e --test <文件名>
  ```

---

## 三、可能的情况

### 3.1 本地能跑什么、不能跑什么

| 场景 | 命令 | 说明 |
|---|---|---|
| 验证代码能编译 | `cargo check -p <crate>` | 本地**唯一**常规验证方式，必须套 cgroup CPU 配额 |
| 调试单个失败用例 | `cargo test -p <crate> -- <测试名>` | 仅用于调试，不能替代 CI 验收；测试要能在 3 秒内跑完 |
| 预览 PR 的 test 关卡会选哪些测试 | `testless select --from origin/main` | 只读影响分析结果，`tests` 字段列出候选用例；不会在本地编译或运行。`scripts/ci_scope.py` 未接入当前 CI，不能用于预览 |
| 中等及以上测试 | **禁止** | 整包（无用例名过滤）、整个 test binary、多包 / `cargo test --all` / `--workspace` / `just test` / e2e——全部交给 PR 的 CI。本机内存不足会假死 |

### 3.2 CI 怎么跑测试（壳仓架构）

CI 全部在公开壳仓 `hathawayANdRX105/ferrite-ci` 跑（计费原因见 AGENTS.md「CI 处理」），
主仓 `.github/workflows/ci-dispatch.yml` 只做派发。壳仓两关卡**并行**：

| 关卡 | 跑什么 |
|---|---|
| `lint-check` | `cargo fmt --all --check` + `cargo clippy --all-targets -- -D warnings`（始终全量）+ ui-kit 契约测试（显式全跑） |
| `test` | PR：快关 `just test-fast origin/<base>`；main push / 手动 `full=true`：全量 `just test`（= `cargo test --all`）。挂 `postgres:18-alpine` 服务 |

| 怎么触发 | 命令 |
|---|---|
| 自动 | PR push / synchronize、main push——主仓 workflow 自动派发，无需操作 |
| 手动全量 | 主仓目录：`gh workflow run ci-dispatch.yml --ref <branch> -f full=true` |
| 看结果 | 主仓 PR 的 `shell-ci` 状态（点进去是壳仓 run）；`gh run list -R hathawayANdRX105/ferrite-ci -L 5` |

快关机制：`testless select --from <base>` 选出受影响的**测试名**过滤器 → `cargo test --workspace -- <过滤名>` 只跑这些测试。
`just test-fast` 的三种收尾（都绝不静默跳测试）：

- `testless` 非 0 退出（如只有 `Cargo.lock` 变动、无法解析）或输出不是合法 JSON → 直接 `cargo test --workspace` 全量；
- 选出了过滤名但全通过数为 0（零命中）→ 判定可疑，也降级全量；
- 选出的过滤名为空（本次 diff 真的不影响任何测试，典型是纯文档 / `.github/` / workflow 改动）→ 打印「本次改动不影响任何测试」并 `exit 0`。

**最后这个分支的坑**：纯文档、`.github/`、workflow 改动会走「零选中 → exit 0」，此时 test 关卡对代码运行 0 个测试——
**这种绿不代表 CI 运行链路本身被验证过**。改了 `ci.yml` / 选包逻辑后，务必手动触发全量验证。

`scripts/ci_scope.py` 仍在仓库里，但**已不再接入当前 CI**——选测试由 `justfile` 的 `test-fast`（`testless`）负责；勿把它当作 CI 选包预览工具。

缓存：只有写入方（main push / 手动 full）回填 sccache/cargo 缓存，PR 只读。改缓存键前先读壳仓 workflow 注释。

### 3.3 失效模式一：feature 门禁的测试，CI 不执行

**症状**：测试文件开头写了 `#![cfg(feature = "xxx")]`，CI 里这个文件被编译、被执行，但输出是 `running 0 tests`，整体仍然绿。

**原因**：CI 的 test 关卡跑 `cargo test --workspace -- <过滤>`（快关）或 `cargo test --all`（全量），lint 关卡跑 `cargo clippy --all-targets`——**这些命令都没带 `--features xxx`**，feature 门禁的测试依旧被编译但 `running 0 tests`。

**实例**：`crates/gateway/metering/tests/ledger_concurrency.rs`（Shuttle 并发测试）。它需要 `cargo test -p metering --features shuttle` 才会真正跑；CI 日志里它的三个测试名一次都没出现。

**怎么办**：
1. 优先改成**不需要 feature 就能跑**（把依赖改成普通 dev-dependency）。
2. 如果必须靠 feature，就在壳仓 workflow 的测试段里按包名为该包补上 `--features`。
3. 无论如何，在测试文件头部写明它需要哪个 feature、CI 是否已经开启。

**连带缺口**：快关（testless）不跑 doctest。doc 注释里的围栏失衡 / 示例编译错误只有全量关卡（main push / 手动 full）才拦——实例：#47 之前 `dispatch/src/retry.rs` 模块 doc 提前闭合把 main 打红，PR 快关全程绿。写文档注释时给 ``` 围栏配上语言标注。

### 3.4 失效模式二：e2e 测试在 CI 连不上 Postgres 时假绿

**现状（壳仓改造后）**：CI 的 test 关卡挂了 Postgres 服务容器（`postgres:18-alpine`，`ferrite`/`ferrite`，5432，`pg_isready` 探活，`DATABASE_URL` 与 `FERRITE_E2E_DATABASE_URL` 均指向它），测试自带幂等迁移，依赖真库的 e2e 测试**在 CI 上真跑**。旧规约「CI 无数据库，文件头写『CI 不验证此断言』+ 本地真库手动验收」已废止。

**残留的失效信号**：门禁写法是"连不上库就 return"——服务容器缺失 / 故障时，`tests/` 下的 e2e（连接 URL 取 `FERRITE_E2E_DATABASE_URL`，未设时本地默认 `postgres://ferrite:ferrite@127.0.0.1:5433/ferrite_e2e`）打印 `skipping e2e: postgres unreachable` 然后 `return`，被记为通过。每个测试连接超时 30 秒，N 个测试就是 N×30 秒。**现在出现 30 秒整数倍 = PG 服务或环境变量坏了，是 CI 事故**：拉壳仓 run 日志查 service 段，修好重跑，不当「正常跳过」放行。

例外：`tests/gateway_e2e.rs` 用 MockEgress 模拟上游，不连数据库，任何环境都真跑。

**本地替代**：`FERRITE_E2E_DATABASE_URL` 指向本地真库（Docker 容器 `uf-local-postgres` 的 5433 端口，`ferrite_e2e` 库）只用于开发中快速复现单用例，不承担验收职责。

### 3.5 失效模式三：`#[ignore]` 的测试永远不跑

**症状**：测试标了 `#[ignore]`，本地要手动加 `--ignored` 才跑，CI 里从不跑。

**原因**：CI 从没带过 `--ignored` 参数。

**规模**：全仓 **56 个** `#[ignore]`（13 个文件），主要集中在：
- `crates/api/control-plane/tests/models_channels.rs`（15 个）
- `crates/api/auth/tests/integration.rs`（11 个）
- `crates/api/control-plane/tests/channels_groups.rs`（8 个）
- `crates/api/observe/tests/logs.rs`（5 个）

**仓库里已有两种相反的做法**：
- `crates/api/billing/tests/` 下的文件（`invitees_list.rs`、`topup_affiliate.rs`、`topup_provider.rs`）用"连不上数据库就 return"的方式跳过，不用 `#[ignore]`。
- `crates/api/control-plane/`、`crates/api/auth/` 下的文件仍用 `#[ignore]`。

两种做法在 CI 里的效果**不同**：return 方式在 CI test 关卡真跑（带 Postgres 服务、自带迁移）；`#[ignore]` 方式在 CI 里从不执行，本地也要手动加参数。

**怎么办**：新写依赖数据库的测试，用"连不上就 return"的方式（不用 `#[ignore]`），并在文件头注释写明"此用例在 CI 上真跑（test 关卡带 Postgres 服务）；连不上库会 return 并被记为 passed（假绿）"。确实不能进 CI 的（慢、烧资源）在测试名或注释里写明原因。

---

## 四、约束事项（简略）

- 本地只跑两种命令：`cargo check -p <crate>` 和 3 秒内能跑完的单用例调试。都必须套 cgroup CPU 配额。
- **不准在本地运行中等及以上的测试**：整包（无用例名过滤）、整个 test binary、多包 / `cargo test --all` / `--workspace` / `just test` / e2e 一律交给 PR 的 CI（本机内存不足会假死）。
- CI 未全绿（主仓 `shell-ci` 状态）不许合并；CI 失败要拉壳仓 run 日志，当新任务修复。
- 工具链版本以仓库 `rust-toolchain.toml` 为准（壳仓检出源码后自动生效）；CI 报新 lint 时先核对该文件与本地 `rustc --version`，不要用 `rustup update stable` 临时升级本地。
- 新增测试必须确认 CI 真的会执行它——避开上面三种失效模式；并记住 PR 快关不含 doctest，文档示例只有全量关卡验收。
