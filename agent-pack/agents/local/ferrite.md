# Ferrite 工作约定

## 开始工作前（按顺序读）

1. 读根 `AGENTS.md`（本文件）。
2. 读根 `README.md` 的 crate 清单，确定当前结构与依赖。
3. 按任务书 / issue 的文件清单工作，用其验收命令验证后提交 conventional commit。

本文件只写**每个会话必须遵守的硬性约束**与**场景 → 文档导航**。
操作细节都在 `.agent/rules/` 下，按导航去读，不要一开始全读。

---

## 开发方式（worktree）

- 每个开发会话 `git worktree add .wt/<name> -b <branch>` 挂独立分支；目录名与分支尾段一致
  （`.wt/admin-api` ↔ `feat/admin-api`）。仓库根只读（除根 `Cargo.toml` 的 member 变更）。
- **防嵌套硬规则**（历史事故：13 层嵌套 + 321G 重复编译产物）：
  1. 必须先 `cd {{root}}`（仓库根）再 `git worktree add .wt/<name> -b <branch>`——相对路径按 cwd 解析，
     在 worktree 内执行会嵌套。
  2. 执行后自检 `git worktree list`：新条目路径出现第二个 `.wt/` 即嵌套，立即 `git worktree remove` 重来。
     闸门 `checklist_no_nested_worktree` 命中即 FAIL。
  3. 派子代理 prompt 必须写**全局绝对路径**（如 `{{root}}/.wt/<name>/`），禁止让其推导相对路径。
- **`.wt/` 删除保护**：只能删**本会话自己创建**且 PR 已合并的 worktree，且需维护者确认 +
  对照 `git worktree list` 确认归属；用 `git worktree remove`。
  严禁对 `.wt/` 做任何批量/暴力删除（`rm -rf`、`git clean`）——那是别人的开发会话，等同删库。
  意外丢失：立即报告维护者，用 `git worktree prune` + `git checkout -b <branch> <merge-commit>` 恢复。

---

## 模块结构与所有权

| 术语 | 位置 | 含义 |
|---|---|---|
| 后端域 | `crates/api/` | 8 个 crate：`auth`、`billing`、`control-plane`（管理配置面）、`db`、`observe`、`router`、`api-mcp`、`tavern`。职责详单见 README |
| 前端域 | `crates/web/` | `ui-components`（跨端组件）、`admin-page-*`、`tavern-page-*` |
| 共享契约 | `crates/contract/` | 跨端 DTO 与协议错误；**唯一的跨域共享点** |
| 网关与执行 | `crates/gateway/`、`crates/harness/` | 调度转发引擎与 Agent 运行时 |
| 应用 | `apps/<name>/` | 唯一有 `main.rs` 的组装层：`api`、`admin-web`、`tavern-web` |

- 功能 crate 只出 library API（独立 `Cargo.toml` + workspace member），不定义进程入口；
  组装只在 `apps/`：`api` 装后端与 gateway/harness，两个 web app 装前端与 `ui-components`。
- 域间禁止私有依赖，跨端数据只走 `crates/contract` DTO；新 DTO 先声明、由一个会话统一改。
- **域目录独占**：会话接手 `crates/<domain>/` 即独占，不准越界改其他域；跨域重构先在 PR 报备清单确认无冲突。
  新增/移动 crate 才动根 `Cargo.toml`，改完在 PR 说明新增 member。

---

## CI 与测试（壳仓执行）

计费原因（私有仓 2000 分钟/月）→ CI 全部在公开壳仓 `hathawayANdRX105/ferrite-ci` 跑：
主仓 `ci-dispatch.yml` 秒级派发 → 壳仓 `lint-check` ∥ `test`（PG18 服务）→ 回写主仓 commit status。
主仓 PR 保护只认 context **`shell-ci`**；`ci` 这个名字**不得占用**（Actions app 绑定同名 check，PAT 回写无法满足）。

**两关口语义（重要）**：

| 关卡 | 性质 | 要求 |
|---|---|---|
| **快关**（PR push，`just test-fast` testless 影响分析） | **阻塞** | 合并门：`shell-ci` 绿了才能 merge；红了 `gh pr checks <N>` 拉日志修复重推，CI 没跑完/没跑完不许合并 |
| **全量**（main push / 手动 `full=true`，`just test`） | **异步、离线跟进** | 不阻塞合并，但**必须回查并汇报**：触发（含 main push 自动）后用 bash 长间隔轮询（如 `gh run watch <id> -R hathawayANdRX105/ferrite-ci --interval 120`），无论成败都要向维护者汇报 run id + 结论；红了立即开修复任务，不静默收尾 |

- 手动全量（主仓目录）：`gh workflow run ci-dispatch.yml --ref <branch> -f full=true`；
  应急绕开私仓直接打壳仓：`gh workflow run ci.yml -R hathawayANdRX105/ferrite-ci -f sha=<sha> -f pr=<N> -f base=main`。
- 看结果：PR 的 `shell-ci` 状态 → 壳仓 run；或 `gh run list -R hathawayANdRX105/ferrite-ci -L 5`。
- 发布同走壳仓：推 `ferrite-v*` tag → 主仓 `ferrite-release.yml` 秒级派发 → 壳仓 `release.yml`
  构建并把 Release（asset+notes）用 PAT 发回**私有主仓**；壳仓不落 artifact（私有代码产物
  不进公开仓）。主仓手动触发只 dry-run；**合并新派发器之前不得推 tag**（旧全量版会烧私有分钟）。
- **不准在本地运行中等及以上的测试**（只许 `cargo check -p` 与 3 秒内单用例调试）——
  失效模式判据（feature 门禁假绿、`#[ignore]`、e2e 30 秒整数倍=PG 事故）见 **`.agent/rules/testing-ci.md`**，新增测试前必读。
- 缓存不变量：只有写入方（main push / 手动 full）回填 sccache（日键）/ cargo（lock 键），PR 只读；
  不做定期全量清（7 天 LRU 自动淘汰），手动兜底跑壳仓 `cache-admin`。壳仓 `on:` 只有 `workflow_dispatch`（防 fork 借 PAT 投毒）。
- 坑：装 `just` 走 GitHub releases（just.systems CDN 403）；缓存中毒则 bump 键前缀 `v0.10.0` 并跑 cache-admin。
  workflow 逐行说明：`hathawayANdRX105/ferrite-ci/.github/workflows/ci.yml`。

---

## 本机环境与进程卫生

> 起后端 / 数据库 / 前端，或遇 500、卡片 404 但 curl 200、dx 不重建、`Failed to find binary package`
> → 读 **`.agent/rules/dev-env.md`**（实测命令 + 症状对照表）。

- 共享后端（默认）：前端代理统一指 `127.0.0.1:3211`，库 `uf-local-postgres/ferrite_smoke`，
  生命周期只走 `scripts/dev-backend.sh`；**红线：严禁停 3211、严禁对共享库跑 `db-reset`**（要独立验证用
  `FERRITE_DEV_LISTEN=127.0.0.1:<端口>` + 独立库）。
- 启停 / 种子 / 体检走 justfile：`just dev-check`（开工先跑）、`just dev-backend start|stop|status`、
  `just db-seed`、`just verify`。免登录调试：`just dev-web debug`（端口 auto 10000-16000；`#login` 不触发自动登录）。
- 长跑服务禁用 `nohup ... &`（进程组被回收，静默死亡）——用会话持久后台任务，起后 `ss -ltn` 验证端口。
- 禁宽匹配 `pkill -f cargo|rustc`（别人在跑的构建，T 态 ≠ 死）；清理前 `readlink /proc/<pid>/cwd` 认归属。
- CPU 密集命令（编译 / 测试 / 装包，含子代理的验证）一律套
  `systemd-run --user --scope -p CPUQuota=70% --`；`git`/`grep`/文件读写例外。

---

## 密钥与配置

- 凭据 / 真实 IP / 上游地址绝不入库、不进 PR 正文；文档示例用占位符（`<API_KEY>`、`127.0.0.1`）。
- relay 引导参数放 `config/config.toml`（gitignore；可选——缺文件走 env `DATABASE_URL`/`LISTEN`，
  见 `apps/api/src/config.rs`；模板 `deploy/config/config.toml`，运行时配置全在 PG）。

---

## 该读哪份文档

| 场景 | 读这里 |
|---|---|
| 开 PR 干活 / 派子代理 | `.agent/rules/pr-workflow.md` |
| 写测试 / 跑测试 / 怀疑「CI 绿了但没验东西」 | `.agent/rules/testing-ci.md` |
| 提交被拦 / 建 PR 被拒 / 查规则 | `.agent/rules/gates.md`（规则总览 `gate-spec` skill） |
| 起后端 / 数据库 / 前端 | `.agent/rules/dev-env.md` |
| Dioxus UI / Rust 公共 API / 调查审查 | `.agent/rules/conventions.md` |
| web 双车道 | `.agent/rules/web-lanes.md`（任务书用 `task-brief` / `pr-orchestration` skill 生成） |
| `.agent/` 目录组织 | `.agent/README.md` |

---

## PR 流程硬门禁（主控 agent）

你是**主控**：编排、派子代理、审产出，不亲自写核心实现。

- **只能通过 PR 干活**；禁止新建 issue、改 epic 结构（维护者对话明确要求建 issue 时，先报备标题 + 完成标准，批准才建）。
- 门禁细节走闸门前置：操作前先跑 `canon check` 对应清单或 `canon issue` / `canon pr` 预检；
  拦截信息**逐条读完**修根因（FAIL 清零，WARN 逐条处理或书面说明），禁 `--no-verify`、禁截断。
- 子代理：prompt 写全（目标文件 / 要做什么 / 验收标准 / 不做什么 + 全局绝对路径 + 所属分支），
  只能在 `.wt/<分支名>/` 内读写提交；单任务 ≤5 文件、单主题。
- 每轮「审查 + 修复」在 PR 写一条评论（含修复 commit）；冒烟验证单独一条（手段 + 结果）。
  「通过」= `shell-ci` 全绿；快关绿 ≠ 全绿——全量按上表离线回查后汇报。
- 记录 `base_sha`，CRG / review 用 `--base <base_sha>`，不写死 main。

### web 双车道快车道

改动**只落** `crates/web/*` + 两个 web app（不碰 contract/api/gateway/harness）时走双车道
（权威：`.agent/rules/web-lanes.md`），跳过重型流程：短分支从 `web-dev` 切、dev 浏览器自测
（`just dev-web-rebuild [port] [debug]`）、squash 合回 `web-dev` 即走人；发布 PR = `web-dev → main`
（merge commit），闸门照跑不绕。固定 worktree `.wt/web-dev` / `.wt/web-fix`，不为小改动新开。

---

## 目标约束

- `crates/harness/{core,prompt,tools}` 与 `crates/web/tavern-*`、`admin-*` 必须支持 `wasm32-unknown-unknown`。
- 测试放 crate 同级 `tests/`，不在 `src/` 用 `#[cfg(test)]`。
- 新增/移动功能 crate 同步更新根 `Cargo.toml` members 与 README crate 清单。
