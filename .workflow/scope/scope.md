# Scope: gate 合并收尾审查与修复

gate 在合并收尾时把问题一次查全：代码类只拦截并给出带概率和改法的建议，由会话 agent 决定怎么改；GitHub 类（issue、PR、review）由 gate 直接改。两边都产出一份透明报告，让 agent 和 reviewer 看到同一份事实。

**Build approach:** Tracer Bullet（按 finding 类型逐类打透，每一类先跑通判定、改法、落地、报告这条整链）。
**Workflow:** GA（/wf-develop 之后依次跑 /wf-check verify、/wf-test、换模型 /wf-check review、/wf-document）。这是项目默认的最严档：gate 会替人改 GitHub 上的对象，还要决定拦不拦合并。
入口只有合并收尾一处（`gate merge`），会话 agent 想提前看结果也调同一个命令。

_下面都是建议，不是要求。跳过任何不适合的：已经知道怎么做的 feature 可以直接 /wf-develop 跳过 /wf-architect。本 scope 里每个 planned 的 feature 都以设计 spec 为第一步，因为每一项都带一个还没做的决定。_

## At a glance

| # | Feature | Phase | Status |
|---|---------|-------|--------|
| A | 规范驱动的检查引擎 | 现状 | existing |
| B | 语义判定 harness | 现状 | existing |
| C | GitHub 对象规则校验 | 现状 | existing |
| D | 合并收尾与其余只读检查 | 现状 | existing |
| E | 安装与分发 | 现状 | existing |
| 1 | 合并收尾一次性完整审查 | Foundation | planned |
| 2 | 判定与改法两步流水线 | Foundation | planned |
| 3 | 透明报告与升级通道 | Foundation | planned |
| 4 | 代码类 finding：拦截加建议 | Slice 1 | planned |
| 5 | issue 对象自动修复 | Slice 2 | planned |
| 6 | PR 对象自动修复 | Slice 2 | planned |
| 7 | review 评论类问题处置 | Slice 2 | planned |

## 现状（pre-workflow，gate 已经有的东西）

### A. 规范驱动的检查引擎 · existing
checklist yaml 描述每条规则（匹配路径、fail_severity、sla、超时、harness 命令），Rust 引擎跑 harness、收集 finding、合并严重级、打印到 stderr、按有没有 Fail 决定退出码。code in `src/engine.rs`、`src/shared.rs`

### B. 语义判定 harness · existing
正则先出候选，再把候选交给 jev 判（noul / choice / score 三种问题，返回标签加概率），低于阈值的当误报丢掉。缺 key 或请求失败时降级成原样 WARN，召回不丢。code in `specs/harness/`

### C. GitHub 对象规则校验 · existing
issue、PR、review 三套规则 yaml 加三个 Rust 校验器：必填 heading、标题语言、禁用前缀与全角括号、type label、正文关键词、乱码、checkbox 禁用、CRG Review 存在性。code in `src/rules/`、`specs/github/`

### D. 合并收尾与其余只读检查 · existing
`gate merge` 串起 PR 校验、review 校验、cleanup、checklist、CRG；docs_hygiene、tests_check、done_when、code 都是只出 finding，不改文件。只有 `gate cleanup` 会删分支，且必须显式开 dry_run 的反面才动手。code in `src/tools/`

### E. 安装与分发 · existing
`gate init` 装二进制、写钩子、种规则包；`scripts/gate-sync` 把 gate 与 spec 推到 8 个成员仓（custom 受保护）。新能力要进 8 个仓，靠这条链。code in `src/tools/init.rs`、`scripts/`

## Foundation

### 1. 合并收尾一次性完整审查
把 `gate merge` 从现在这种只跑 merge 阶段规则、遇到问题就停，改成一轮跑齐该跑的全部规则、一轮出完整清单。目的：让会话 agent 一次拿到全部问题，而不是被逐条叫醒。
**Done when:** 一次 `gate merge` 跑出本次改动涉及的全部适用规则的完整 finding 清单（不提前中断、不截断、不漏文件），任一 FAIL 拦住合并并返回非零退出码，WARN 进清单但不拦，同一个问题只出现一次。
- [ ] Design it (spec): `/wf-architect 合并收尾一次性完整审查`

### 2. 判定与改法两步流水线
每条 finding 先由 jev 判（只出标签和概率），过阈值的再由第二个模型写出改法。两个角色、两次模型调用，输出成结构化结果给下游用：代码类拿建议，GitHub 类拿去落地。
**Done when:** 每条 finding 都带 jev 概率和一个改法建议（或明确标注需人工决策），阈值写在规则里可调；jev 不可用时全部降级成需人工决策并保留原 finding，不静默放过；第二个模型不可用时只出概率不出改法，且不阻塞其余 finding。
- [ ] Design it (spec): `/wf-architect 判定与改法两步流水线`

### 3. 透明报告与升级通道
gate 做的每一件事和没做的每一件事都留一份人能读的记录，会话 agent 与 reviewer 看同一份。这是"抛提示给 agent 自己决策"的落点。
**Done when:** `gate merge` 结束时终端打印一段报告，每条 finding 一行（规则 ID、文件与行、jev 概率、gate 改了哪里或为什么没改），同时在同一路径写一份报告文件供事后回查，两处内容一致；报告文件不进版本库（跑完 `git status` 干净）。
- [ ] Design it (spec): `/wf-architect 透明报告与升级通道`

## Slice 1: 代码类 finding

### 4. 代码类 finding：拦截加建议
代码问题 gate 一律不碰。它只负责一轮查全、拦下合并、把带概率和改法的清单交出去，改不改、怎么改由会话 agent 决定。
**Done when:** 代码类 FAIL 让 `gate merge` 失败，并逐条给出文件与行、规则 ID、jev 概率、具体改法、还有哪些别的改法可选；跑完之后仓库文件一个字节没变（`git diff` 与跑之前一致）；agent 按报告改完重跑一次即通过。
- [ ] Design it (spec): `/wf-architect 代码类 finding 拦截加建议`

## Slice 2: GitHub 类自动修复

### 5. issue 对象自动修复
issue 上机器可改的那几项（label 缺失或不对、标题全角括号、标题禁用前缀、正文字面 \n 与乱码字符）由 gate 直接改，改完读回来确认。
**Done when:** 命中上述几类且 jev 概率过阈值的 issue，gate 用 gh 改完再读回确认，各项都符合规则；置信度不够的只进报告不改；报告里每一处改动有改前与改后；gh 调用失败时按原 finding 拦合并，并在报告里写明失败原因。
- [ ] Design it (spec): `/wf-architect issue 对象自动修复`

### 6. PR 对象自动修复
PR 上的同类问题（label、标题、body 结构、Fixes 关联）由 gate 直接改。改不了的（CI 未过、review 未 approve）不假装能改。
**Done when:** 与 issue 同类的 PR 元数据问题在概率过阈值时被 gate 改完并读回确认；CI 未过、review 未 approve 这类只进报告并按原严重级拦截；body 缺段落时 gate 插入骨架，同时在报告里标出该段内容仍待补，空段落不算已修。
- [ ] Design it (spec): `/wf-architect PR 对象自动修复`

### 7. review 评论类问题处置
review 评论里的格式问题分清两类：能通过 API 改的直接改，改不了的（缺 CRG Review、inline finding 没有回复）只给具体指引，不伪造内容冒充已修。
**Done when:** 每条 RV 规则都标了"gate 可改"或"只能提示"；可改的那类在概率过阈值时被改完并读回确认；不可改的按原严重级拦截，报告里给出该怎么做（补哪条评论、怎么写）的具体指引。
- [ ] Design it (spec): `/wf-architect review 评论类问题处置`

## Deferred

留在计划里是为了让这份计划不撒谎，不在这一轮做。
- **规则误报率度量**：记每条规则的命中数、误报数、降级次数，按数据改最差的那几条。你说过这不是这一轮要实现的东西。
- **pre-commit 阶段的自动修复**：评估过，否决。修复放到合并收尾，会话 agent 在场。
- **代码类自动改写**：评估过，否决。代码由 agent 改，gate 只给建议。
- **自动 commit 或 push 修复结果**：评估过，否决。GitHub 类的修复落在 GitHub 对象上，不进仓库。

## Legend

**决策框**：每个 feature 恰好一个 label 以 `(spec)` 结尾的子项，它就是入口命令。其他框都是执行框，/wf-architect 一个都不勾。

**Feature 状态**：`planned` → `in-progress` → `done`，另有 `existing`（早于这套工作流，/wf-develop 与 /wf-sync 不动它）和 `dropped`（移出计划，保留历史，不删行）。

**下一步** = 第一个没勾的框，永远是一条命令。

**Workflow 档位**（项目默认在文件头 `**Workflow:**` 那行）：`Prototype` 只到 /wf-develop；`Alpha` 加 /wf-check verify；`Beta` 再加 /wf-test；`GA` 再加换模型 /wf-check review 与 /wf-document。单个 feature 想更严或更松，在标题后加 `· GA` 这样的标签。

**指针行**（`spec <n> · code in <path>`）：spec 链接由 /wf-architect 在 capture 时加，代码路径由 /wf-develop 加。没有就不写。
