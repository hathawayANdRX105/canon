# AGENTS.md — ui-kit

ferrite 家族共享的 Dioxus 组件库：lib 名 `ui_kit`，crate `ui-kit`（dioxus 0.7.10、
edition 2024），消费方是 ferrite（经 `ui-components` 再导出）与 kymido。
分类判据、样式分层设计取舍、版本口径正本写在 `README.md`，本篇只写「改本仓时容易踩的」。

## 收录门槛（硬约束）

- **零业务、零 DTO**：依赖产品内部 crate（如 ferrite 的 `contract`）的组件不进本仓，
  留在产品侧。
- 收录门槛 = 无业务语义，或被 ≥2 个产品用到。产品差异化走「组合 + props + 主题变量」，
  不在 kit 里复制一份样式。
- 新组件先定落点：前四组（`container` / `primitive` / `widget` / `pending`）是**角色**分组，
  判据由 `jev_classify` 跑分类 + 读码定夺，落在对应 `mod.rs` 头注释里；
  `form` / `layout` / `overlay` / `button` / `icons` 是**主题**分组，不按角色拆。
  **没有 `display/` 伞目录**（已解散，理由见 README）——别把它重建回来。

## 样式层（改 CSS 前必读）

- 三层单向流动：`theme.css`（`:root` 变量 + `@utility role-*` 原子语义）→
  `components.css`（`ui-*` 组件类）。组件层 `@apply role-*`，**import 顺序不能反**。
- 刻度变量统一 `--ui-` 前缀（`--ui-space-*` / `--ui-radius-*` / `--ui-shadow-*` /
  `--ui-weight-*` / `--ui-leading-*` / `--ui-tracking-*` / `--ui-font-*` /
  `--ui-duration-*`，字号 `--ui-type-*-size`），取值与 Tailwind 内置刻度逐值对齐。
- 原子类必须写成 `@utility`：Tailwind v4 的 `@apply` 不认 `@layer components` 里的类，
  会直接报 `Cannot apply unknown utility class`。
- **没被 `@source` 扫到的类会被摇掉**：新原子类暂时没有调用点时用 `@source inline("…")` 桥接，
  否则产物里根本没有这个类（页面看起来「改了没反应」）。
- `assets/dx-components-theme.css` 是 dioxus-components 的厂商主题（`html[data-theme]`
  技巧），只服务 `dxc-system`，与本仓 `theme.css` 零变量共享，别混进同一套刻度讨论。
- `demo/assets/tailwind.out.css` 是生成物（gitignore），改样式后由 `bun run css` 再生。

## 构建与验证

- CI（`.github/workflows/ci.yml`）跑 `cargo fmt --check` + `cargo clippy --all-targets`
  + `cargo test` + tag 校验。全量跑放 CI；本地只跑轻量的 `cargo fmt --check` /
  `cargo check`，重命令套 `systemd-run --user --scope -p CPUQuota=70% --`（`justfile` 里的 demo 已套）。
- clippy **刻意不加 `-D warnings`**：CI 注释记着两条既有告警（`src/form/mod.rs` 歧义
  glob 再导出、`src/layout/avatar_menu.rs` 未使用变量），是主动留的，别顺手改成红的；
  真要清干净就把那两条一起清完再收紧。
- **视觉改动必须过 demo 页面**，不能只读 diff 交付：仓库根 `just demo`——端口**不再写死**，
  从 **10000-16000** 抽一个空闲端口并打印 URL，实际端口落盘 `demo/.demo-port`；
  `just demo-stop` 按它停本 worktree 的实例。同一 worktree 重启会复用上次端口，
  要钉死用 `UI_KIT_DEMO_PORT=<n> just demo`。改了 `assets/*.css` 先重跑 `bun run css`，
  否则页面加载的还是旧产物。
- `tests/` 与被测模块同名对应（表契约 / 视觉契约 / props / 行为）。**纯函数契约优先**，
  组件渲染断言交给消费方的 e2e——本仓断言渲染等于把消费方的选择面提前钉死。
- **多会话不抢端口**：`10000-16000` 是所有 worktree 共享区间，`just demo` 启动时探测空闲位，
  谁先起谁先得，**不要手工去占某个固定端口**。`demo` 与 `demo-stop` 都用
  `readlink /proc/<pid>/cwd` 校验归属，**只停本 worktree 自己的 dx**；跨 worktree 拒停并告警。
  禁止无条件 `kill` 占用端口的进程、禁止 `pkill -f dx`——那是别的会话正在用的页面。
  查当前端口看启动时打印的 URL，或 `cat demo/.demo-port`。

## 发布：钉 tag，不发注册表

- 仓私有，**不发 crates.io / npm**（发布到注册表即公开）。Rust 侧走 git dep。
- 消费方必须钉**不可变 `tag`**，不许跟 `main`：不钉的后果是本仓每次 push 都改变消费方
  构建结果，消费方没动、CI 却红。本地迭代用 `path = "../ui-kit"` 覆盖，**勿提交**。
- **`tag == Cargo.toml version`**，由 CI 校验；不一致的 tag 比没 tag 更糟。
- 版本三段口径正本在 README「功能域清单」：major 由用户拍板；minor = 功能域数（当前 8）；
  patch = `main` 上 `fix` 开头提交数。改那张表必须同时改 `Cargo.toml` 的 minor。
- 删组件 = 删代码 + 删 `src/*/mod.rs` 再导出 + 改 `tests/` + 改 README 功能域表 +
  改 `Cargo.toml` minor。半删会让消费方编译断。

## 被消费时：先读消费方的 AGENTS，再动手

本仓是**被依赖的库**，不是应用。你改这里的每个字节都会流到消费方的构建、页面和测试里，
所以**改本仓 ≠ 改消费方**——两边的约定可能冲突，冲突时以**消费方为准**。

- **动笔前先读消费方的 `AGENTS.md`**（ferrite：`/home/hathaway/projects/ferrite/AGENTS.md`）。
  那里才有它的工作方式与测试纪律，本篇不重复、也不覆盖它。本仓的规则和消费方不一致时，
  **按消费方的来**，并在交付记录里写明这处冲突。
- **消费方的硬约束优先于本仓的偏好**，尤其这三条最常打架：
  - **测试归属**：本篇说「组件渲染断言交给消费方 e2e」，所以**别在消费方仓里替它加 UI 快照**；
    反过来消费方要你补 props/行为契约测试时，照做——那是消费方的目录规矩，不是本仓的。
  - **版本与 tag**：本仓改 `Cargo.toml` 的 version / 打 tag 会立刻改变消费方的构建结果。
    消费方要求钉 tag（不许跟 main），所以**本仓没准备好 tag 就别推 main**——否则消费方 CI 会在
    它没动过的情况下变红。改版本号前先确认消费方那边能跟上。
  - **构建与验证**：本仓说「视觉改动过 demo 页面」，消费方可能要求浏览器冒烟 + `data-testid`
    / `ariaSnapshot` 断言（ferrite 就是这套）。**两边的验证都要做**，本仓 demo 过 ≠ 消费方验收过。
- **路径依赖不是发布手段**：`path = "../ui-kit"` 只用于本地迭代，**勿提交**；消费方必须钉 tag
  （见「发布」节）。在消费方仓里看到 `path` 覆盖，视为待修的临时状态。
- **别越界改消费方**：修 ui-kit 的问题优先在本仓解决；确实要动消费方时，先确认那不是
  消费方自己的域目录（各项目的域目录独占规则由它们自己定）。

## 门禁与部署

- `.githooks/spec/` 是 canon 的**部署镜像**，`.githooks/canon` 是 canon 仓构建的二进制
  （gitignore，不入库）。两者都不在本仓手改——规则缺陷回 canon 正本提 PR。
- 本仓 `AGENTS.md` 由 canon `agents.yaml` 组装生成（文件头有 managed 戳）：
  改内容改 `canon/agent-pack/agents/local/ui-kit.md` 与 `canon/agents.yaml`，再
  `python3 scripts/agents push ui-kit`。**别直接编辑本文件**，下次 push 会被覆盖。
- gate 规则总览看 `.githooks/spec/docs/SPEC_OVERVIEW.md`；本仓 web 判据看
  `.githooks/spec/docs/WEB_SPEC.md`。
- `justfile` 目前只有 demo 两个 recipe；跑命令优先用它，别裸 `dx serve`。

## 删除纪律

- 非 git 跟踪文件一律 `gio trash <path>`（可恢复），禁 `rm` / `rm -rf` / `git clean`；
  git 跟踪文件用 `git rm`。
- 删的是别人的产物、看不懂用途的文件、或 gitignore 里的东西 → 先问，别顺手清。