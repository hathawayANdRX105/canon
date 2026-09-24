# AGENTS.md

## 项目概述

本仓库管理 `dotfiles`：CachyOS/Arch 环境的可复用配置与脚本。

项目总览见 `README.md`。本文件是 **agent 操作约定**：改哪里、怎么映射、别碰什么。

## 目录结构

```
(dotfiles)
├── home/                  # → ~ 的点文件（逐项 link，映射表见 install.conf.yaml）
├── config/                # → ~/.config/**（按子目录 glob）
├── bin/                   # → ~/.local/bin/**（脚本与 poll）
├── fonts/                 # → ~/.local/share/fonts/**
├── rime/                  # → ~/.local/share/fcitx5/rime/**
├── os/                    # 系统级配置（不走 dotbot，手动 rsync）
│   ├── systemd/           # systemd unit / drop-in
│   └── pacman.d/          # pacman 钩子
├── packages/              # 自维护 PKGBUILD（herdr / code-review-graph / oh-my-pi）
├── agent/                 # oh-my-pi：omp 配置 + skills + plugins
│   ├── omp/               # → ~/.omp/agent/（config.yml、models.yml 等）
│   ├── skills/            # → ~/.omp/agent/skills/
│   └── plugins/           # 本地插件 fork（不走 dotbot，由 package.json 引用）
├── README.md
└── install.conf.yaml      # dotbot 唯一映射源
```


## Dotbot：仓库 ↔ 用户目录

部署（改完配置后一般要跑一次）：

```bash
dotbot -d ~/projects/dotfiles -c ~/projects/dotfiles/install.conf.yaml
```

映射规则（每条以「仓库路径 → 用户目录」格式列出）：

- `bin/**` → `~/.local/bin/`（glob）
- `config/**` → `~/.config/`（glob；排除 `config/deskctl/snippets/**`）
- `fonts/**` → `~/.local/share/fonts/`（glob）
- `rime/**` → `~/.local/share/fcitx5/rime/`（glob）
- `home/<name>` → `~/<name>`（逐项 link；清单以 install.conf.yaml 为准）
- `agent/omp/**` → `~/.omp/agent/`（glob）
- `agent/skills/**` → `~/.omp/agent/skills/`（glob）

日常规则：

- `config/` 新增子目录（如 `config/foo/`）自动进 `~/.config/foo`，**不必**改 yaml。
- `home/` 新点文件（如 `home/.foo`）**必须**在 `install.conf.yaml` 加一行 `~/.foo: home/.foo`。
- `bin/` 新脚本 glob 后自动出现在 `~/.local/bin/`，但要保留可执行位。
- `os/` 不参与 dotbot。系统单元用 root rsync，例如 `sudo rsync -av --info=progress2 ~/projects/dotfiles/os/systemd/ /etc/systemd/system/`。
- `clean:` 会清掉目标里由 dotbot 管理、但源已消失的死链；别把非本仓库文件塞进被 clean 的目录当"唯一真相"。

### 分支验证与部署

- 每个改动在独立 worktree/分支完成并先做静态或隔离验证。
- 未合并的分支**不得**对家目录运行 Dotbot，也不得运行会安装软件的 `dotpkg`。
- `dev` 是唯一部署分支。合并后只从 `~/projects/dotfiles` 执行 dotbot 和 dotpkg；包重建和运行时 smoke 也只从 `dev` 进行。
- Herdr OMP 修复持久化在 `packages/herdr/omp-detection.patch`。
- `~/.omp/agent/extensions/herdr-omp-agent-state.ts` 是 `herdr integration install omp` 的安装产物，绝不放进 `agent/omp/**` 让 Dotbot 管理。

## 目录职责

### `config/`（最常改）

镜像 `~/.config`。当前所有子目录（按字母序）：

- `api-hub/` 本地 LLM 上游路由；`config.toml.example`（最小模板）+ `config.snapshot.toml`（当前生产结构脱敏快照）入库，真实 `config.toml` gitignore
- `deskctl/` 桌面/工作区控制；`snippets/` 不入 dotbot（见敏感清单）
- `fcitx5/` fcitx5 输入法配置（配合 `rime/`）
- `ferrite-gateway/` 本地 LLM gateway（`127.0.0.1:17893`，OpenAI 兼容）；同 api-hub 套路：`config.toml.example` + `config.snapshot.toml` 入库，真实 `config.toml` gitignore。snapshot 由 api-hub config 翻译生成
- `fish/` 主 shell 配置
- `fontconfig/` 字体全局观感 → **默认别乱动**（见敏感清单）
- `foot/` 终端
- `fuzzel/` 启动器
- `helix/` 编辑器
- `herdr/` OMP 监视器（见敏感清单）
- `mako/` 通知守护进程
- `mango/` Wayland 合成器；含 `exec-once` 启动项（见敏感清单）
- `matugen/` 主题色生成源；改错 = 全局配色漂移
- `opencode/` AI 编程代理
- `swappy/` 截图标注
- `systemd/` 用户级 systemd unit
- `television/` TUI 选择器
- `yazi/` 终端文件管理器
- `zen-browser/` Zen Browser profile；配 `bin/zen-deploy-userjs` 部署 user.js
- `mimeapps.list` 默认应用
- `qq-electron-flags.conf` QQ 启动参数

日常规则：

- 改这里 → `dotbot` → 用户目录生效。
- API key / 隐私：单独本地文件，**不入库**（如 `api-hub/config.toml`、`fish/conf.d/api_key.fish`）。

### `home/`

映射到 `~` 的点文件、`.desktop` 文件由 `install.conf.yaml` 逐项 link 管理，清单以该文件为准。

主 shell 是 **fish**（`config/fish/**`）；bash 环境变量尽量与 fish 对齐（`home/.bashrc`）。

大目录不进 git：`~/.cache` `~/.local/share` `~/.rustup` `~/.bun` `~/.cargo` `~/.npm` 等。

### `bin/`（脚本）

映射到 `~/.local/bin`。**dotbot glob 全量链接**；下面是脚本中的**核心**几个，**不是全集**：

- `bin/dotpkg` 构建/安装 `packages/<名>`
- `bin/poll-jobs` **间隔调度器**（唯一常驻 loop）
- `bin/conf/poll-jobs.conf` poll 任务表：`name interval_secs command`
- `bin/conf/app-logs.logrotate` 应用日志轮转+压缩配置（poll-jobs `log-rotate` 每日驱动）
- `bin/poll/*` **one-shot** 任务（被 poll-jobs 或手动调用）
- `bin/conf/*` 脚本配置（如 `ip_config.toml`）
- `bin/zen-deploy-userjs` 部署 user.js 到所有 Zen Browser profile
- `bin/herdr-nav` herdr agent 导航（panel 上下切换）
- `bin/restore-omp-plugins` 还原 omp plugins（`agent/plugins` 修复）
- `bin/tm-install` / `bin/tm-headless-install.py` tmux 安装 / 无头安装
- `bin/bt-repair` 蓝牙修复
- `bin/mem-guard-daemon` 内存守护（oom 防护）
- `bin/mango-toggle-zen` / `bin/mango-toggle-scratch` mango 模式切换
- `bin/mango-screenshot` / `bin/mango-record` mango 截图/录屏
- `bin/qq-original` / `bin/linuxqq` QQ 启动器
- `bin/qq-toggle-scratch` QQ scratchpad 切换
- `bin/wechat` / `bin/wechat-original` / `bin/wechat-sni-bridge` / `bin/wechat-notify` / `bin/wechat-toggle-scratch` WeChat 启动/通知/scratch
- `bin/with-rime-ascii` / `bin/rime-context-ascii` rime ascii 模式切换
- `bin/__pycache__/` **构建产物**（可再生，不入 git）

**完整列表以 `bin/` 实际目录为准**。新加脚本只需放进 `bin/`，**不必**改 `install.conf.yaml`。

**poll 机制（重要）：**

- 业务脚本在 `bin/poll/` 里写成 **跑完就退** 的 one-shot。
- `poll-jobs` 读 `bin/conf/poll-jobs.conf`，按间隔调用；状态/log 在 `~/.local/state/poll-jobs/`。
- 加定时任务 = 写 one-shot + 在 conf 加一行，**不要**再写第二个常驻 daemon。
- 调试：`poll-jobs --once`。
- 当前 conf 任务以 `bin/conf/poll-jobs.conf` 为准（wallpaper / hosts / pomodoro / wildtoken / qq-clipboard / proxy-chain / bai-proxy / api-hub / silverq / log-rotate / freenode-pool / agentrouter-direct / obscura / workbuddy2api）。主循环 `sleep 30`，间隔 <30s 无效；`~/.local/state/poll-jobs/qq-clipboard-sync.last` 是脚本自存的剪贴板状态，不是调度状态。
- **日志压缩（log-rotate，每日）**：`logrotate -s ~/.local/state/logrotate.status ~/.local/bin/conf/app-logs.logrotate` 统一轮转+压缩应用日志（silverq `serve.log`、`poll-jobs/logs/*.log`、kime `/tmp/kime-ime.log`；daily + copytruncate + compress + dateext，保留 7 天）。新增日志 = 在 app-logs.logrotate 加一个 block。系统 `logrotate.timer` 在本机 disabled，用户日志不走 `/etc/logrotate.d/`。

### `packages/`（自维护包）

不走 paru 默认 AUR 配方时用。**当前实际维护 3 个包**：

- `code-review-graph` omp 扩展：代码评审图
- `herdr` OMP 监视器（pkgname `herdr-custom`）；`omp-detection.patch` 持久化 OMP 修复
- `oh-my-pi` omp 主程序源码包（18.x）；补丁放同目录 `*.patch`（如 `gemini-xhigh-thinking.patch` 等）

布局：

```
packages/<name>/
├── PKGBUILD
├── *.patch          # 与配方同目录；dotpkg 拷进构建缓存
└── files/           # 可选：service、wrapper、.env.example …
```

```bash
dotpkg <包名> [版本]
dotpkg -f <包名>    # 强制重建
```

日常规则：

- **构建/安装/打包只走 `dotpkg`**（`makepkg` / `paru -U` 都不是入口）。
- 改上游行为 = 改 `packages/<名>/*.patch` 再 `dotpkg -f <名>`，**绝不直接改安装产物**。
- **构建缓存**：`~/.cache/pkg-build/<包名>`（可再生）。清理 = 删对应缓存目录。
- `packages/<名>/src/`、`pkg/`、`bin/__pycache__/` 同理是可再生产物，不进 git。
- 细节 skill：`agent/skills/repo-dotfiles-pkgbuild/`。

### `os/`（系统配置）

系统级文件，**不走 dotbot**，需 root 手动 rsync 到 `/etc/`。

- `os/systemd/` systemd unit / drop-in → `rsync` 到 `/etc/systemd/system/`
- `os/pacman.d/hooks/**` pacman 钩子 → 改坏可能让包管理失败（见敏感清单）

### silverq（全局代理，2026-09 起接管 sing-box）

silverq 是当前的全局代理，自包含测速 / 节点切换 / 国内回环分流，**不需要像 sing-box 那样外挂 rule-set 或 proxy-chain 白名单**。

- **二进制**：`~/.local/bin/silverq`（外部装，不属 `packages/`）；`silverq status` / `silverq reload` / `silverq serve <nodes.yaml>` 是常用子命令。
- **配置**：完全在 `~/.config/silverq/`（`silverq.toml` / `nodes.yaml` / `rules/`），**不进本仓库**（含节点凭据）；dotbot 不映射。
- **端口**：mixed `127.0.0.1:17321`（SOCKS5 + HTTP CONNECT 同口）。Zen browser、`bai-proxy`、`api-hub proxy=true`、`fish proxy_on` 等都指这里。
- **启动**：mango `exec-once` 拉一次 + `bin/poll/silverq-check` 每 120s 兜底进程存活（poll-jobs 表 `silverq     120 ...`）。silverq 自身管测速/切换，poll 只管进程。
- **改配置**：编辑 `~/.config/silverq/{silverq.toml,nodes.yaml}` 后 `silverq reload`；不必动本仓库。
- **退出 / 重启**：`pkill -f "silverq serve"` 后等 poll 拉起，或手动 `silverq serve ~/.config/silverq/nodes.yaml`。日志 `~/.local/state/silverq/serve.log`（daemon 主日志，`log-rotate` 每日轮转压缩）；poll 事件在 `poll.log`。
- **遗留**：`~/.config/sing-box/`（旧目录）和 `/usr/bin/sing-box`（旧二进制）尚未清理；本仓库已无 sing-box 任何引用。

### `agent/`

oh-my-pi 侧：**三栏**结构，不要混。

- `agent/omp/` → `~/.omp/agent/`（glob）；omp 主配置（`config.yml`、`models.yml` 等）
- `agent/skills/` → `~/.omp/agent/skills/`（glob）；omp 技能
- `agent/plugins/` **不走** dotbot；本地插件 fork，`~/.omp/plugins/package.json` 用 `file:` 指过来（如 patched `pi-cache-optimizer`）

仓库根 `AGENTS.md` 管本仓库；`agent/omp/AGENTS.md` 管 omp 会话行为，别混。

### LLM 渠道链路（wildtoken / api-hub / omp 模型配置）

omp 的模型全部经**本地 OpenAI 兼容端点**接入，两层调用链：

```
omp (agent/omp/models.yml, config.yml)
  → wildtoken  http://localhost:3100/v1   （~/repo/wildtoken，Go，admin=API key）
  → api-hub    http://localhost:17892     （bin/api-hub，单文件 Python，按 model 字段路由）
    → 上游 (api.b.ai 走代理 / sensenova 直连 …)
```

各层配置（按链路顺序）：

- **omp → 端点**：`agent/omp/models.yml`（providers/models 列表）+ `agent/omp/config.yml`（llmBaseUrl 等）**入库**；不放 key，引用环境变量名 `WILDTOKEN_API_KEY`
- **api-hub → 上游**：`config/api-hub/config.toml.example`（最小模板）与 `config/api-hub/config.snapshot.toml`（生产结构快照：路由/分层/权重/重试，key 换占位）**入库**；真实 `config.toml` **gitignored**，key 只在 `~/.config/api-hub/config.toml`
- **key 本体**：`config/fish/conf.d/api_key.fish`（gitignored）；fish `set -gx WILDTOKEN_API_KEY` 等

日常规则：

- **服务管理**：wildtoken 由 poll-jobs 拉活（wildtoken 180s，见 `bin/conf/poll-jobs.conf`）；手动启动 = `~/.local/bin/poll/wildtoken-check`（幂等，活着就退出），停止 = `pkill -x wildtoken`，日志 `~/.local/state/wildtoken.log`。api-hub 30s / bai-proxy 30s 同理由 poll-jobs 拉活。
- **改上游/路由**：编辑 `~/.config/api-hub/config.toml`，重启 api-hub（poll 30s 会自拉，或手动 `~/.local/bin/api-hub`）；改完**同步更新 `config/api-hub/config.snapshot.toml`**（只换 key 为占位，其余原样）以便结构入库。
- **路由格式**：`[routes]` 值可为字符串（单上游）或 `{ upstreams=[...], priority=[...], weight=[...] }`（priority 数字大者优先，同层按 weight 概率分流；层内全失败才降层，4xx 且还有后续候选也降层，最后一个候选的错误原样透传）。候选写成 `"上游名:该上游认的model名"` 可按候选改写 `body.model`（各渠道命名不同时用，如 `"yw-glm:GLM-5.3-Flash"` 配 bai 的 `glm-5.3-flash`）。别名映射走 `[rename]`。回归测试 `tests/test-api-hub-routing.sh`。
- **野王类临时节点**：配 `[retry."<model>"] max = 0` 让它一次不成立刻降层，别烧退避；这些节点明文 HTTP，upstream 要 `tls = false`；渠道不支持统一注入的参数时用 `[inject."<上游model名>"] drop = [...]` 摘掉（否则每次先白打一发 400 再降层）。
- 改 models.yml 前注意：`apiKey:` 字段填的是**环境变量名**，不是真实 key——别把 key 写进入库文件。

### `fonts/` / `rime/`

- `fonts/` 字体（dotbot glob 映射到 `~/.local/share/fonts/`）
- `rime/` fcitx5 rime 词库/方案（dotbot glob 映射到 `~/.local/share/fcitx5/rime/`）

## 修改与完成标准

- 配置改动用 **`edit` / `write`** 写进仓库，再视需要 `dotbot`。
- **删除任何文件前必须先问用户**，得到明确同意再删（`gio trash`，不要 `rm -rf`）。
- 小改动；不碰无关路径；不把语言依赖、大缓存、密钥推进 git。
- 推送前：无重复映射、diff 干净 → `git pull --rebase && git push`。
- commit 标题格式：git-commit skill（conventional commits）。

## 敏感文件清单（改动前必须停下来确认）

以下文件改动影响范围大、易踩坑或含敏感信息。agent **不得**擅自改动；如需动，先 `ask` 用户。

### 5.1 凭据 / 隐私（多数已 gitignore，本地存在）

- `config/fish/conf.d/api_key.fish` API key 模板（已 gitignore）
- `config/api-hub/config.toml` 本地 API hub upstream key / 路由（已 gitignore）
- `config/herdr/config.toml` OMP 监视器行为
- `config/mango/config.conf` 合成器 / `exec-once` 启动项
- `config/matugen/config.toml` 主题色生成源
- `~/.omp/agent/extensions/herdr-omp-agent-state.ts` herdr 集成安装产物
- 任何 `*.key` / `*.pem` / `*.env` / `*credential*` 默认按凭据对待；改动前必问

规则：

- 这些文件 agent **不读不写不打印** 内容到日志/回复/外部工具（`read` / `grep` / `cat` 都避免）。
- 即使用户要求展示，也要先确认是否要脱敏（用 `***` 替代密钥段）。
- 误提交敏感信息 → 立刻停手，提示用户轮换密钥，不要"补救式 commit"。

### 5.2 系统级 / 不可逆改动

- `os/systemd/**` 需要 root `rsync` 到 `/etc/systemd/system/`；影响所有会话/服务
- `os/pacman.d/hooks/**` pacman 钩子，改坏可能让包管理失败
- `config/fontconfig/**` 字体全局观感；现有约定"默认别乱动"
- `/etc/**`（dotfiles 仓库外）不属本仓库；需 root；改前必问
- 任何用 `sudo` / `setcap` / `systemctl` 的命令 影响系统全局，先确认

### 5.3 对外暴露的服务配置（误改会断网 / 断服务）

- `install.conf.yaml` dotbot 映射源；改映射 = `~/.config/` 链接关系整体重排
- `bin/dotpkg` 唯一构建/安装入口；改错 = 包系统损坏

### 5.4 不可入 dotbot 的运行时产物（只读）

- `agent/plugins/`（fork）由 `~/.omp/plugins/package.json` 通过 `file:` 引用；不走 dotbot
- `packages/**/src/`（构建产物）重建后会被覆盖；不要手工改
- `bin/__pycache__/` 构建产物；可再生
- `~/.omp/agent/extensions/herdr-omp-agent-state.ts` herdr 集成安装产物
- `~/.local/state/poll-jobs/**` 运行时日志；不要进仓库

### 5.5 触发规则

- agent 准备 `edit` / `write` / 删除 / `git add` 任一上述路径之前 → `ask` 用户确认改动范围。
- agent 准备 `read` / `grep` 5.1 任一文件 → 默认拒绝；用户明确要求且已脱敏才执行。
- 即使路径不在清单内，agent 也要避免"顺手"改动无关文件（surgical change 原则）。
- 清单是"高敏集合"而非白名单 —— 不在清单不代表可以乱改。
- **例外（重要）**：§1、§2 描述的例行流程（新增 home 文件加映射行、poll 任务增删）本身即用户已授权的常规操作，直接执行；只有**超出文档所述流程**的改动（如重写映射结构、删条目）才需要先 ask。

## 额外约定

- **dotpkg 工作流**：所有包构建只通过 `bin/dotpkg`，改 PKGBUILD 或 patch 后 `dotpkg -f <包名>` 刷新缓存。构建缓存 `~/.cache/pkg-build/<包名>` 可再生，绝不手动修改安装产物。
- **cpulimit（硬约束）**：CPU-heavy 命令（`dotpkg` 包构建、`makepkg`、`cargo build`、`npm`/`bun` 等）必须套 `cpulimit -l 65 -i --`；`git`、`grep`、文件读写等轻量命令不需要。脚本/PKGBUILD 的语法门已放 CI（`.github/workflows/ci.yml`：shellcheck + bash -n + py_compile），重型构建本地跑时才套 cpulimit。
- **LLM 模型配置**：`agent/omp/models.yml` 是唯一运行时模型入口（providers/models 列表），`agent/omp/config.yml` 只放 llmBaseUrl 等。改完后 `dotbot` 刷新 `/models`，无需重新编译 omp。
- **代码审查**：改动后必须通过 `git-commit` skill 提交，标题格式 `skill( scope ): description`。
- **安全底线**：敏感文件改动前必须 `ask` 用户确认；绝不擅自 `grep` 打印密钥或敏感内容。

**注意**：本新文档已按 `ls` 出来的真实目录结构、`install.conf.yaml` 实际映射规则完整重写；表格换成列表（每项以「路径 + 一句话目的」开头，便于 grep 命中）。用户请仔细检查后确认无误，再执行 `mv AGENTS.md.new AGENTS.md` 或 `edit` 合并更新到主文件。

## 发现处置纪律（gate / jev / review）

自动检查的每条 finding（gate `FAIL`/`WARN`、`jev` L3 发现、CRG / `ocr review` 意见）必须逐条处置：

1. **先读规范再改代码**：先读本仓规范（本文各节与 `docs/` 下的约定）确认要求，再动代码。判定不了就记为待裁决写进交付记录，不猜、不忽略。
2. **修根因**：让规则约束成立，不是让检查不再报。
3. **禁止糊弄式修复**：改/删 `.githooks/spec` 规则降严重度、`--no-verify`、`head`/`tail`/`grep -v` 截断输出、`#[allow(...)]`/`# noqa` 压制、空文件/空目录占位、`assert!(true)` 填数、拆分改名只为躲匹配范围——一律违规。
4. **逐条留痕**：修复写 `规则 ID → 根因 → 改法(file:line)`；驳回写 `规则 ID + 理由 + 依据` 交维护者裁决。落点 = PR 正文 `## Delivery record` 或 issue 交付评论。沉默即违规。
5. **WARN ≠ 可忽略**：与 FAIL 同等处置。

完整版与判例：canon `specs/agents/_discipline.md`；本仓 `AGENTS.md` 由 canon 维护并 agent-sync 下发，勿单独改。
