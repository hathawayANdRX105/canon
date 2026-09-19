# canon — agent 共享文档正本

多项目共用的规范 / 任务书 / skill 的**单一正本**。各项目拿到的是**真文件**（非软链、非 submodule），文件头盖版本戳，`agent-sync` 按 commit SHA 比对分发。

## 为什么这样

- 软链 / submodule 在别人 clone、CI、跨机器时都会断，不能用。
- 纯手工复制 = 漂移（`versioning.md` 曾在 kime/silverq/ferrite 三份各不相同）。
- 本仓管**共享内容**；机器配置归 dotfiles；同步工具在 `bin/`（也算工具，随仓走）。

## 目录

```
canon/
├── bin/agent-sync      # 同步工具（status / push / backport / pull）
├── tasks/              # 共享任务书（通用骨架）
├── rules/              # 共享规范
├── skills/             # 共享 skill
└── projects/<name>.yaml  # 分发 manifest
```

## 内容拆分原则（重要）

一个主题要么**纯通用**（放 canon，可同步），要么**纯项目特有**（留项目，不同步）。混合内容按自然接缝拆成两份：

- 通用骨架 → `canon/tasks/version-stats.md`（三门口径、判定规则、执行流程）
- 项目特有（路径 / 清单 / tag 格式 / 产物）→ 各项目的 `.agent/tasks/versioning.md`

两份冲突时**以项目文件为准**——它绑死了真实路径。

## manifest 格式

`projects/<项目名>.yaml`：

```yaml
root: /abs/path/to/project   # 项目本地绝对路径
dest: .agent/tasks           # 文件落地的目录（可省，默认 .）

# <canon 源路径> -> <项目内相对 dest 的路径>
tasks/version-stats.md -> version-stats.md

# 可选 pin：项目暂时挂旧版本
# rules/foo.md -> foo.md @ abc1234
```

## 用法

```bash
agent-sync status              # 所有项目的漂移报告（只读）
agent-sync status kime         # 单个项目
agent-sync push kime [--commit]  # 同步 canon -> kime（只推 behind/missing/unstamped）
agent-sync backport kime tasks/version-stats.md   # kime 的本地修正回流到 canon
agent-sync pull                # canon 仓自更新
```

## 分类（status 的判定）

| 状态 | 含义 | push 行为 |
|---|---|---|
| `in-sync` | 本地 == 正本，戳 == HEAD | 无操作 |
| `behind` | 本地没动，canon 更新了 | **更新**（快进，安全） |
| `unstamped` | 有文件无戳，但内容 == HEAD | 只补戳 |
| `local-mod` | 本地改了，canon 没动 | 拒绝，建议 `backport` |
| `diverged` | 两边都改了 | **拒绝**，人工三方合并 |
| `unknown` | 有文件无戳且内容 != HEAD | 拒绝，先 backport 或 `--force` |

**安全底线**：本地有改动的文件绝不自动覆盖——那是丢数据。

## 版本戳

每个同步文件首行盖：

```
<!-- canon: hathawayANdRX105/canon @ abc1234 (synced 2026-09-19) -->
```

戳 = 「这份副本来自哪」。agent 在项目里看到它，就知道改文档去 canon，**别改本地副本**。
