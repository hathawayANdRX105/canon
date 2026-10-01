# 暂存：wf 问句质量契约 + jev 前置决策（未部署）

**状态**：✅ 已部署（dotfiles `3b41d0b` / `b741236`）。补丁留档作变更证据，勿重复应用。
的原因：`~/.omp/agent/skills/wf-*` 软链直连 dotfiles，改文件即刻进正在运行的会话）。
**来源问题**：用户在 claude-code 项目的 `/wf-architect` 会话里遇到两个体验问题。

## 问题 1：问句缺信息，术语无定义

实际生成的问句（会话记录 `01a0d372`）：

> **Q**: 这份契约**这一刀**管到哪？
> **header**: 这一刀
> - 只定交互终端 :: 后台会话和远程控制的规则留到它们自己的 spec。这份只定多个交互终端共用一个进程。
> - 三种都写进这份 :: …后面两**刀**不再单独做契约。

问题：
- **"这一刀"是模型自造行话**，`grep 刀` 在全部 wf 技能零命中，skill 从未定义。读者第一次见只能猜。
- **主语缺失**："这份契约"指哪份？需回溯整段对话。
- **选项描述继续用未定义术语**（"后面两刀"），猜错就选错。
- 用户原话：「我一开始都不理解什么意思，问的问题很多缺少信息，我不好回答」。

根因：`internal/design-conversation.md` 的 Mechanics 段规定了**问什么维度**，但没规定
**问句本身必须携带什么信息**，模型自由发挥无人拦。

## 问题 2：jev 只在收尾 calibration，问前完全不调

`design-conversation.md` 全文仅两处提及 jev（"for calibration on critical judgments"），
推荐方案全由主模型自己拍。用户要求：**询问之前先让决策模型判，推荐方案要有质量**。

## 改动内容

### A. `wf-architect/internal/design-conversation.md`

**A1. Mechanics 段新增问句信息契约**（在现有 4 条 mechanics 之后追加）：

每个问句必须自带判断所需信息，读者不看上文也能答：

| 要素 | 要求 | 反例 → 正例 |
|---|---|---|
| **主语具体** | 点名具体对象，不用"这份/这个" | "这份契约" → "交互终端 ↔ 进程通信契约" |
| **位置/背景** | 一句话说清正在定什么、为什么现在定 | 只写 "管到哪" → "这份 spec 只覆盖交互终端，还是三种进程模式一次写完？" |
| **选项含取舍** | 每个选项写清代价，不只写名字 | "只定交互终端" → 加上"后台/远程控制留到各自 spec" |
| **推荐有理由** | `(recommended)` 后跟一句为什么 | 已要求，补：理由须指向本项目的具体约束 |

**A2. 术语纪律**（新增硬规则）：

- 禁用未在 skill 正文定义过的比喻、行话、内部简称（"这一刀""那一刀"等）。
- 每个术语首次出现必须当场定义，或换成直白表述。
- 写完问句自查一遍：把上文遮住，只看问句，能否独立读懂并作出选择？不能 → 重写。
- 现有 skill 未定义任何"刀"类术语，故此类表达一律视为违规。

**A3. jev 前置为强制步骤**（改写第 57 行）：

原文：`Use jev … for calibration on critical judgments (per contract for Beta/GA)。`

改为：jev 在**提问之前**先判，不是收尾校准。每个 Stage 的每个推荐方案，必须先经
`judge()`（见 `jev` skill）评估：

```
state  = 该维度的完整决策上下文（本项目的栈/约束/已定的前序答案 + 各选项的完整取舍）
question = { type: "choice", criteria: { 选项A: "什么证据让它赢", … } }
```

- judge 的选择 → 面板的 `(recommended)`，**并把概率写进理由**（如"决策模型判定 A 更优
  (p=0.72)"），让工程师知道推荐不是随口给的。
- 概率 < 0.6 → 老实说"这一项我不确定"，别硬推。
- 全部选项都要过 judge，不只推荐项 —— 否则"推荐"只是模型偏好，不是评估结果。
- jev 不可用（无 API key / 独立场景）→ 降级为主模型判断，并在面板里说明"未经决策模型评估"。

### B. 同一契约进 `wf-scope/modes/plan.md`

scope 的问题面板最密集（每轮最多 4 个 panel），同样受四要素问句契约 + 推荐先过 jev 约束。
`wf-check` **不改**：它只问一个问题（确认审查模型），无推荐决策，不适用。

## 部署步骤（有空时做）

1. 确认无在跑的 wf-* 任务。
2. 在 `~/projects/dotfiles` 应用补丁（普通 diff，非 mail patch，用 `git apply`）：
   ```bash
   git apply ~/projects/canon/specs/pending-wf-decision-quality/0001-ask-contract-jev-predecision.patch
   git commit -am "refactor(wf): ask-quality contract + mandatory jev pre-decision before asking"
   git push origin dev
   ```
3. 新开会话验证：跑一次 `/wf-architect`，看问句是否自带主语/背景/取舍，推荐是否带
   决策模型概率。
