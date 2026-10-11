---
name: canon-check
description: >
  canon 消费侧手册：怎么查规范、读规则、被拦了怎么办，以及规范本身有缺陷时怎么修正本。三个工具 spec_catalog / spec_explain / preflight（MCP）+ canon check（CLI），六个实测陷阱，完整的「检查 → 修复（含修复规范正本）→ 分发」循环。凡是「这个仓有什么规范」「这条规则为什么拦我」「我要提交会不会被拦」「被拦了怎么修」「canon 怎么用」「这条规则/文档过时了怎么改」——提交前自查、commit 被拒、想按规范而不是靠猜来改代码、发现死配置/误导注释/退役引用要改正本，都用本技能。新增规则/写新 checklist/自定义 spec 走 gate-spec；豁免开关全表与拦截点全清单读 references/exemptions-and-overrides.md。
allowed-tools: Bash, Read, Grep, Glob, Edit, Write
license: MIT
---

# canon-check：怎么查规范、被拦了怎么办

canon 是**规则消费侧**的操作手册。规则本体在 `.githooks/spec/`，每个仓一份，由
canon 正本仓 `scripts/canon-sync push <项目>` 分发。

**边界**：本文件讲两件事——「按现有约束干活」，以及**规范本身有缺陷时怎么修**
（第六节：改正本 → 验证 → 分发）。**新增**规则 / 写新 checklist / 自定义 spec
走 `gate-spec`；**豁免开关全表 / 拦截点全清单**见 `references/exemptions-and-overrides.md`。

## 附属参考（按需读，不必每次通读）

- `references/exemptions-and-overrides.md` — 严重度怎么裁决、豁免开关全表、26 处拦截点全清单

---

## 一、先决条件：不满足，canon 根本不跑

```bash
git config core.hooksPath                    # 空 → 没有钩子，见陷阱 5
ls .githooks/spec/quality/checklist_*.yaml    # 空 → gate 报 setup 错
canon --version
```

**`core.hooksPath` 没设就修它**，别绕过去：

```bash
git config core.hooksPath .githooks/hooks
```

`core.hooksPath` 是**本地 config，不随 clone 走**。新克隆的仓库、CI 里的临时
checkout 默认都是「无钩子」状态——此时 `git commit` 静默通过，**不能**据此认为代码
合规。这种情况下唯一的真检查是手动跑 `canon check` 或 `preflight`。

---

## 二、三个工具

| 想知道 | 用 |
|---|---|
| 这个仓有哪些规则、每条什么严重度、挂哪个钩子 | `spec_catalog` |
| 这条规则为什么存在、怎么检测、拦了我该怎么修 | `spec_explain(rule_id)` |
| 我这份改动会不会被拦 | `preflight(sla, focus, target)` |

`spec_explain` 回答的是**为什么**，不是「怎么绕过去」。每条 finding 都必须处置：
修，或书面驳回并写明依据。

### finding id ≠ rule id：`spec_explain` 只认后者

被拦时你手上是 **finding id**，`spec_explain` 要的是 **rule id**。两个命名空间，
直接传 finding id 会报 `no rule named X`。取**第二段**：

```text
checklist.rust_no_process_cmd.HTTP-VIA-SUBPROCESS
                  └────── rule id ──────┘  └── finding id（harness 自己起的）
```

```text
spec_explain(rule_id: "rust_no_process_cmd")   ✅
spec_explain(rule_id: "HTTP-VIA-SUBPROCESS")   ❌ no rule named ...
```

rule id 就是 checklist 文件名去掉 `checklist_` 前缀和 `.yaml` 后缀
（`checklist_rust_no_process_cmd.yaml` → `rust_no_process_cmd`）。不确定就先
`spec_catalog` 查。

### preflight 的关键参数

```
sla:    L1 | L2 | L3        # 最高跑到哪层，默认 L1
focus:  test|refactor|security|style|size|docs|lint 或任意子串（"ccn" 也行）
target: { rev | commit | branch | paths[] | scope }
        scope: pre-commit | unstaged | branch
include_warn: bool
```

**定向提问**是它相对 `canon check` 的主要价值：

```jsonc
// 我改的这些文件，有没有 refactor 类问题？
{"paths": ["crates/web/src"], "focus": "refactor"}
// 这次提交会不会被拦？
{"commit": "HEAD"}
// 这个分支相对基线呢？
{"branch": "my-feature"}
// 某次提交里这两个文件改了什么？
{"target": {"commit": "HEAD", "paths": ["src/a.rs", "src/b.rs"]}}
```

三个坑：

- `target` 和顶层 `paths`/`commit` 都认，但 **rev / commit / branch 互斥**，
  同时给两个直接报错，不猜。
- `focus` 是**子串**匹配规则 id：预设名（`test`）或裸子串（`ccn`）都行。
- **`focus` 没匹配到任何规则时，返回的是「没跑」，不是「通过」。** 输出里的
  `note` 会明说这件事。看到 `rules_run` 缺失就别当绿灯。

### 什么时候用哪个

`canon check` 只看**工作树现状**（暂存区 / HEAD 相对态），改动多时给的是全仓噪声。
问「我这份 diff 干不干净」用 preflight。

```bash
canon check                       # 无参 = 列出可用规则，不执行
canon check clippy file_size      # 只跑指定规则
canon check --sla l3 --json       # 机器可读，带 score/confidence
```

---

## 三、被拦了长什么样

```
checklist.rust_no_process_cmd.HTTP-VIA-SUBPROCESS FAIL L2	probe/bad.rs: HTTP 调用用 reqwest 不要 subprocess curl/wget: ... (L2)
RESULT: FAIL
```

字段：`checklist.<规则文件>.<finding id> <严重度> <层>  <路径>: <说明> (<层>)`。
`RESULT: FAIL` 且存在 FAIL → 退出码 1 → `git commit` 被拒，**提交不会产生**。

### 严重度：拦不拦，和你必须做什么

| 严重度 | 拦不拦 commit | 你必须做什么 |
|---|---|---|
| **FAIL** | **拦** | 修，或拆 PR。不修就过不去 |
| **WARN** | 不拦 | **逐条处置**：修，或书面驳回写进 PR 记录 |
| **INFO** | 不拦 | 参考，需要时说明取舍 |

WARN 不拦 ≠ 可以无视。静默忽略 WARN 是违规——它和 FAIL 一样需要你给出结论。

**L3（jev / LLM 判层）一律不阻断**，无论判成什么严重度都只给参考分。L3 的作用是
补充 L1/L2 抓不到的东西，不是它们的替代。

**「书面驳回」怎么写**：PR 正文的 `## Delivery record` 段，一条一行——
`规则 ID → 驳回理由 → 依据`，例如
`rust_tests_in_tests_dir → 驳回 → 架构偏好规则，本仓 tests/ 存量布局不适用`。
只写「已知悉」不算处置。

### 修，不要绕

被拦时唯一的正确反应是**按规则修代码**。以下都是违规，不是「解决方案」：

| 别做 | 为什么 |
|---|---|
| `--no-verify` | 绕过整条门禁链 |
| 就地改消费侧 `.githooks/spec/`（或只为让本次 diff 变绿而改正本） | 把约束改没了，不是修问题；副本改动还会被 `canon-sync push` 覆盖 |
| 降 `fail_severity` | 只改 yaml 无效，见陷阱 4；真要降级直接找用户裁决 |
| 加 `#[allow(dead_code)]` / `# noqa` | 压制信号而非解决 |
| 拆文件/改名只为躲开 `paths_include` | 破坏结构换绿灯 |
| 造空文件骗过结构类规则 | 结构噪音 |

真觉得规则本身有问题 → **按第六节修 canon 正本**（正本改动要用户点头，走 PR）；
动判定逻辑的（严重度 / 豁免开关 / 规则退役）一律交用户裁决，不自己拍。

---

## 四、六个实测陷阱

以下每条都在真实仓库的临时克隆里复现过，不是推测。

### 陷阱 1：gate 是下界，`ALL PASS` 不是合规证明

规则是**正则**实现，不是语义理解。`rust_no_process_cmd` 的正则是
`std::process::Command.*"(curl|wget|httpie)"`：

```rust
Command::new("sh").arg("curl");                        // ✅ 被拦
Command::new("sh").args(["curl", "-s", "https://x"]);  // ❌ 放行（数组形式匹配不上）
```

所以 `ALL PASS` 只能证明「没踩到已实现的检测」，**不能**证明代码干净。想知道某条
规则到底怎么检测，去 `spec_explain(rule_id)` 读它的正则与路径过滤。

**但这不构成自我豁免的理由**：如果你明知自己干了规则禁止的事，就算这次没被抓到，
也照样要改。正则漏了形态是规则作者该修的问题（上报用户裁决），不是你在提交时跳过它的
许可证。**判断依据是你做了什么，不是检查器有没有报。**

### 陷阱 2：`mode: grep` 的规则无视你的 target

grep 模式规则自己扫全仓（`git ls-files`），**你给的 diff/path 范围对它无效**。
preflight 对非 hook 范围会提示：这类规则的 finding 是全仓结论，不是你范围的结论。
别把「我只改了 2 个文件所以只有 2 条 finding」当保证。

### 陷阱 3：规则的 `hooks:` 决定它什么时候跑

`rust_todo_needs_issue` 挂在 `hooks: [merge]`，**pre-commit 阶段根本不跑**。
所以 commit 通过 ≠ TODO 合规。要完整把关跑 `canon check --sla l2` 或走合并检查。
`spec_catalog` 的 `hooks` 字段直接告诉你每条什么时候跑。

### 陷阱 4：降级只改 yaml 无效

每条 checklist finding 的最终严重度 = `min(yaml fail_severity, harness 报告的 severity)`。
harness 输出 JSON 里写着 `severity: "FAIL"` 时，把 yaml 改成 WARN **依然硬拦**。
要降级必须同时改两处——这属于改约束，交用户裁决，不在消费侧做。

### 陷阱 5：无钩子状态会假绿

`core.hooksPath` 没设 / 新克隆 / CI checkout → commit 全过。**修法是设上它**
（见第一节），别用「反正没拦」当通过依据。

### 陷阱 6：`focus` 空结果不是通过

匹配不到规则时返回 note「没跑 ≠ 通过」。`rules_run` 缺失就是没跑。

---

## 五、一次完整的检查 → 修复循环

被拦之后照这个走，**不需要再读别的文件**：

```text
1. 确认没在无钩子状态（陷阱 5）；是就先设 core.hooksPath
2. 问「我这份改动会不会被拦」
     preflight: {"include_warn": true}      ← 不给 target = 查暂存区
     注意：{"commit": "HEAD"} 查的是上一次提交，不是你还没提交的改动
3. 有 FAIL → 先读规则为什么这么定
     spec_explain(rule_id)                   ← rule id 取 finding 的第二段
4. 按根因改代码，不是让检查闭嘴
     自问：这条约束在本仓还成立吗？下次同类改动还会不会触发？
     陷阱 1：明知违规但没被抓到，也要改
4b. 根因在**规范本身**（描述过时 / 死配置 / 注释与实现相反 / 退役引用 / 命名过时）
     → 走第六节改正本，不要就地改副本、更不要改到让这条不再报
5. 再跑一遍第 2 步，确认转绿
6. 剩下的 WARN 逐条处置：修，或书面驳回（写进 PR 的 Delivery record）
```

**第 3 步不能跳。** 直接猜「怎么改能让它过」就是在为提交乱弄，而不是在修问题。
**第 4b 步同理不能跳**：发现规范缺陷就修到正本，别攒着等用户提醒。

### 什么时候需要更多

- 想知道**某个具体开关怎么配**（降严重度、关某条检查）→ `references/exemptions-and-overrides.md`
- 想知道**哪一步会拦你**（26 处拦截点）→ `references/exemptions-and-overrides.md`
- 想**加规则 / 写 checklist / 自定义 spec** → `gate-spec` skill

---

## 六、规范本身有问题的修法（改正本，不是改副本）

检查跑出来的 finding，有时根因不在代码，在**规范本身**：描述过时、注释与实现相反、
配置键代码根本不读、文档还在教一条退役命令。这类要**改正本**，不是改那份副本。

### 三层结构：改哪一层决定改动是否持久

| 层 | 位置 | 谁写 |
|---|---|---|
| 消费侧副本 | 成员仓 `.githooks/spec/**` | `canon-sync push <项目>` 分发（**手改会被覆盖**） |
| **正本** | canon 仓 `specs/**`（规则 + 文档）；`agent-pack/**`（AGENTS.md 片段 / 分发规则 / 模板 / 技能） | 按本节流程改 |
| 分发 | `scripts/canon-sync push <项目>` → `.githooks/spec`；`scripts/agents push <项目>` → `AGENTS.md` | 由正本生成 |

成员仓里看到某条规则不对 → **去 canon 改正本**。就地改副本 = 白改。

### 什么算规范缺陷

- **死配置** —— yaml 的键在代码里零引用（先证明：`grep -rn '<key>' crates/ scripts/` 为 0）
- **误导注释 / 文档** —— 描述方向与实现相反（键名是历史遗留时尤其常见）
- **退役引用** —— 文档 / 模板还在教已卸载的命令、已删除的函数
- **文档漂移** —— 总览 / 布局文档与目录、规则表对不上
- **命名过时** —— 规则族名指向已退役的工具或机制
- **规则与现实冲突** —— 约束在本仓不成立，或存量结构根本不适用

### 修法对照

| 缺陷 | 修法 |
|---|---|
| 死配置 | 证明 0 引用 → 从正本删（连带描述它的注释）→ 镜像同步 |
| 误导注释 | 只改注释说清方向；**键名不动**（键名是代码读取面，改名属改判定逻辑，另立变更） |
| 命名过时 | `git mv` 文件 + 改载荷配置键 + 改全部引用 + 镜像同步 + **旧名加进 `canon-sync` 的 `RETIRED_RULES`**，成员仓 push 时自动清旧名 |
| 退役引用 | 改指现役机制（例：`ocr review` → `canon check --sla l2`）；**不删文件、不删内容** |
| 文档漂移 | 以 yaml / 代码为准刷新文档；总览表与规则表同步 |

### 验证（改正本后必跑）

```bash
# 1. 解析：yaml / json 能加载
python3 -c "import yaml,glob;[yaml.safe_load(open(f)) for f in glob.glob('specs/**/*.yaml',recursive=True)]"
# 2. 冒烟：规则列得出来、跑得动
canon check --sla l2
# 3. 残留：旧词在新内容里零命中（`canon-sync` 的 RETIRED_RULES 旧名清单除外）
grep -rn -i '<旧词>' specs/ .githooks/spec/ agent-pack/
# 4. 镜像：canon 自用的 .githooks/spec 是 specs/ 的 rsync 镜像
just spec-sync
```

改完正本要**想分发**：`canon-sync push <项目>`（规则）与 `agents push <项目>`（AGENTS.md）。
**分发在正本合并之后跑**——push 读的是已合并内容。

### 边界：这不是「改规则让检查闭嘴」

| 是修复 | 是违规 |
|---|---|
| 改正本，**所有仓的约束一致地、有意识地**变了（或只是描述 / 命名变，判定逻辑没动） | 就地改本仓副本，让**这一次** diff 变绿 |
| 改注释 / 死键 / 命名，判定逻辑不变 | 降 `fail_severity`、关开关（陷阱 4，用户裁决） |
| 记录缺陷 → 修法 → 验证 → 走 PR | `--no-verify` / 删 spec 文件 / 拆文件躲匹配 |

一句话判据：**改正本、所有仓一起变 → 修复；只让本仓本次过关 → 违规。**

### 正本改动的授权与流程

- 正本用**工作树 + 分支 + PR**：canon 自己的规矩是「要合就走 PR」，squash merge 需用户确认
- 影响面先数清楚：片段（`agent-pack/agents/fragments/**`）被所有项目共用，改一句 =
  全部项目 AGENTS.md 待重生成；规则（`specs/**`）经 `canon-sync` 分发到每个成员仓
- 改名 / 退役这类**动标识符**的改动，必须把旧名登记进 `RETIRED_RULES`，
  否则成员仓新旧两份同时加载、同一规则跑两遍
- 降 `fail_severity`、关开关、规则退役——**动判定逻辑的一律先问用户**，不做「顺手改」

---

规则清单不在本文档里——它会漂移。要清单用 `spec_catalog`。
