---
name: jev
description: >
  用 TypeSafe Jev（System One 决策模型）做校准判断：审阅筛选、路由分类、打分、
  事实验证——不生成文本。覆盖三种问题类型（noul/choice/score）、如何把复杂判断
  拆成原子问题、如何用置信度设阈值分流，以及本仓 harness（review_chain.py）的
  接入方式。写代码调 Jev API、设计问题集、或想用"快速结构化判断"替代
  LLM prompt-and-parse 时用本 skill。随附零依赖调用脚本（jev-ask.py）。
---

# jev — System One 决策模型使用

Jev 是 TypeSafe 的 System One 模型：输入 **state**（文本/JSON）+ 一组**类型化问题**，
一次并行返回带校准概率的结构化答案。**不生成文本、不解释、不写代码**——它是软件里的
"聪明 if 语句"，不是聊天/编码 LLM。控制流、算术、副作用全在代码（或 agent）手里。

**官方文档（真源，页面加 `.md` 可取 markdown）**：<https://docs.typesafe.ai/llms.txt>

## 何时用 / 何时不用

| 用 | 不用 |
|---|---|
| 高频重复、答案空间有界的判断（路由/分类/打分/验证/guardrail） | 写代码、写文案、开放生成 |
| 毫秒级在线判断（~100ms，$0.042/MTok 输入，输出免费） | 需要书面理由的审计（Jev 只给概率不给 rationale） |
| 替代"LLM 返 JSON 再 parse"的脆弱环节 | 一次性的多步复杂推理（用正常 LLM） |

## 调用（变量 + 脚本 + 裸 HTTP）

**环境变量两个**（判断走哪里、用哪把钥匙）：

| 变量 | 值 | 说明 |
|---|---|---|
| `TYPESAFE_API_KEY` | `sk-...`（本机已配在 `~/.config/fish/conf.d/api_key.fish`） | 官方 key 或网关 key |
| `TYPESAFE_API_BASE` | `https://api.knox.chat`（网关）/ 不设则默认 `https://api.typesafe.ai` | knox 网关只授权 jev 模型 |

**模型名不走环境变量，直接写死在脚本常量里**：请求体 `"model": "jev-latest"`（别名，当前指向
`jev-1.13.0`；调过阈值要可复现就写死版本号）。

> **模型不可用怎么办**：jev 下线/改名/被网关拒绝时，**ask.py 会自动抓最新模型重试**——
> `GET $TYPESAFE_API_BASE/v1/models`，从返回里挑最新的 `jev-*`（优先 `jev-latest`
> 别名，否则取版本号最大的 `jev-1.x.y`），把 model 字段换掉重发。注意两种返回形状：
> 官方 `models[]`、网关（OpenRouter 风格）`data[]`；两种都按 `id ?? name` 取——
> `id` 是机器名（优先），`name` 可能是展示名。
> **`jev-ask.py` 已内置这步**：HTTP 400/404 或报错提 model 时自动抓最新重试一次，
> stderr 会打 `改用抓到的最新模型 jev-x.y.z 重试`。

新机器配置（fish）：`set -gx TYPESAFE_API_KEY sk-...; set -gx TYPESAFE_API_BASE https://...`，重开 shell。bash 用 `export` 同名两个。验证：`echo $TYPESAFE_API_KEY` 非空。

**推荐走脚本**（随本 skill 分发：canon 在 `specs/skills/jev-ask.py`，装到项目后是
`.agent/skills/jev/ask.py`；stdlib 零依赖，自动双发 Bearer+x-api-key 兼容网关）：

```bash
# 1) 验连通鉴权（顺手排除 env 问题）
python3 .agent/skills/jev/ask.py --check          # ✓ jev-1.13.0 连通正常

# 2) 写问题集 questions.json（见下节"如何问问题"）
# 3) state 从文件或 stdin 进，answers JSON 出：
echo "$diff" | python3 .agent/skills/jev/ask.py questions.json
python3 .agent/skills/jev/ask.py questions.json state.json
```

**裸 HTTP**（没有脚本时）：

```bash
curl -s "$TYPESAFE_API_BASE/v1/systemone" \
  -H "Authorization: Bearer $TYPESAFE_API_KEY" -H "x-api-key: $TYPESAFE_API_KEY" \
  -H "Content-Type: application/json" -d '{
    "model": "jev-latest",
    "state": "Help! My payouts have been failing for 3 days.",
    "questions": {"is_urgent": {"type": "noul", "instructions": "Does this convey urgency?"}}
  }'
```

**认证坑**：官方 api.typesafe.ai 认 `Authorization: Bearer`；中转网关（knox.chat 等）认
`x-api-key`，只发 Bearer 会 401 Invalid token。脚本两头都发，手动 curl 也建议双发。

**一次调用长这样**（真实输出，三类型混发）：

```json
{"is_doc": {"type": "noul", "noul": 0.89},
 "kind":  {"type": "choice", "choice": "prose", "confidence": 0.98,
           "probabilities": {"prose": 0.98, "code": 0.01, "config": 0.01}},
 "quality": {"type": "score", "score": 0.67, "confidence": 0.32,
             "legend": {"0": "rough", "1": "acceptable", "2": "polished"}}}
```

**限制**（jev-1.13.0）：单请求 64k tokens，**state + 最长问题 ≤ 32k**；1200 req/min；
250k tokens/s；choice ≤255 项，score 2–10 级。超限 429/400。
state 只放本题需要的材料——**无关内容拉低准确率（context rot）**。本仓实测 269KB state 直接 400，
经验上限 24000 字符——**ask.py 已内置本地预检**，超限直接报错并给拆分建议，不会白打一次远端。

## 三种问题类型

| 类型 | 问什么 | 返回 |
|---|---|---|
| `noul` | 是非题 | `noul` 0–1（是的概率）。**没有 confidence 字段** |
| `choice` | 固定选项挑一个 | `choice` + `probabilities`（全选项分布）+ `confidence` |
| `score` | 有序等级打分 | `score`（概率加权，可落在级间）+ `legend` + `probabilities` + `confidence` |

选型：是非→noul；无序分类→choice；程度/频谱→score（levels 必须是具体情境描述，每级自足可判——不参照其它级也能判断）。
noul 0.5 = "是/否各半"，**不是**"中等程度"——要测程度用 score。

## 如何问问题（写 instructions 的规矩）

1. **一题一个"专家看一眼就能答"的判断**。"这条消息急吗？"好；"分析并给出最佳行动"坏——那是
   System 2，拆开。
2. **字面执行**。Jev 照字面答，不猜意图。范围词、否定、隐含条件全写明。你发现答错后想解释的
   那句话，就是该写进 instructions 的另一半。
3. **完整写进 instructions**。问题 ID（key）不发给模型，ID 里有的信息 instructions 里也要有。
4. **criteria 与 instructions 对齐**，别说反（true 映射"否"会显著变差）。边界情况写进 criteria。
5. **不问模型能精确算的**：计数、算术、日期先后/间隔、hex 比较——代码算好喂结论或命名分桶。
   抽取（哪年、几号）可以问，比较（谁早）留给代码。
6. instructions/criteria/state 都可以是结构化 JSON（对象放定义、问题放一个字段、数据按名引用
   `` `field.path` ``），比塞长字符串清楚。

7. **选项集必须覆盖时序/边界变体，不只是理想路径**。同一个提交入口有三种调用时序：agent 空闲、
   流式中、需排队；state 里只给理想形态，jev 会给低置信选择，而正确答案在没列出的变体里
   （实测：注入机制三选一 0.53，漏掉 followUp 排队变体，集成测试抓出 AgentBusyError 竞态）。
8. **state 必须写进每个选项的副作用事实**。漏一个会改变判断的事实（如"戳插在 shebang 前
   会让内核 exec 失效"），jev 就在不完整选项空间里给你 0.31 置信的错误选择——它只能在你
   给的选项里挑。
9. **state 放真实踩坑证据，不放抽象描述**：具体报错文本（`SyntaxError: invalid decimal
   literal`）、实测数字（269KB state → 400）、复现步骤。有证据的判断校准得明显更准。

## 拆分问题（原子化 + 组合）

- **独立维度各问一题，一次调用全带上**。同 state 的所有问题并行、互相不可见：
  "给这份简历打分" → python_depth / leadership / system_design 三道 score。
- **组合在代码里**：权重、阈值、排序归代码（composite scoring）。改权重不用重跑推理。
- **只有依赖才发第二次调用**（第一次的答案决定要取的证据/下一批选项时）。
- **投机扇出**：把"可能用得上"的问题也一起发，代码里挑着消费；多余问题只多花 token，不串答案。
- "有没有"和"是哪个"分开：choice 是相对比较（哪个更优），noul 是绝对判断（可以全都低）。
- **方案选择两轮法**：第一轮 choice 粗评架构选项（每选项一段实现摘要进 state）；第二轮只对
  胜者发**实现级原子题**（注入机制/守卫设计/插入点/回滚），每题带具体代码级选项。粗评定方向，
  细评定路线，别混在一张卷子里。
- **keyed 批量筛选**：候选编号 C1..Cn，每题一条独立 noul，instructions 按名引用候选并共用
  criteria 模板；一次调用全并行、互不可见。组合权重与阈值在代码里做。
- **测试覆盖评审**：把行为面（输入维度 × 分支 × 边界）枚举成候选场景，state 里带上**现有
  测试清单**（每条一行契约），逐题问"是否守住现有测试没覆盖的可观察契约"——同路径冗余会被
  自然否决（实测 15 选 13，新增测试当场抓出一个真实竞态）。

## 置信度 → 行为

- confidence 度量选项概率分布的集中度（choice/score 才有）：越集中越高，摊平越低。低 confidence 常意味着
  选项本身歧义或 state 不够。
- **三段式**：高→自动执行；中→确认/复核/再取证；低→不动作，转人工或回退到常规 LLM 推理。
- **阈值随风险走**，不是一个数：只读操作 0.6 就能动，破坏性操作要 0.9+。起保守值，用自己的数据调。
- **阈值不跨类型搬运**：noul 上调好的阈值换到 choice 不通用（见官方 structural invariants 反例）。
- **choice 低置信（≤0.6）≠ 重新问一次**：两条路——把漏掉的事实补进 state 再问，或先用
  测试/实验复核选中分支，再继续实现（实测：0.53 的选择直接继续实现，集成测试当场抓出竞态）。
- **choice 两选项概率接近对半（各 ≈0.5）**：选 diff 更小且与现有语义一致的那个，别在对半
  结果上发明新的防护机制。
- 只挑最优项时不需要阈值——直接取 confidence/probability 最高的；要做质量门控才设阈值。

## jev-1.13 已知短板（写问题前过一遍）

1. 字面读题（否定/范围词照字面）→ 条件写死，必要时拆成两道字面题
2. 不会数数、不会算术 → 代码算
3. 日期是文本不是有序量 → 抽取给模型（choice 枚举），先后/间隔代码算
4. 多跳间接问法掉准确率 → 减少间接层，按名指到 state 字段
5. state 大而杂 → 代码先过滤；过滤本身可用一道 noul 做
6. 对抗性内容会带偏它 → criteria 写明，上线前测边界
7. instructions 与 criteria 矛盾会糊涂 → 两者当一体的措辞写
8. 别指望结构不变量（noul 与等价 choice 的概率不必一致、正反题之和可≠1）→ 阈值别跨问法搬
9. 不能生成文本 → 候选用 regex/LLM 先列，Jev 挑

## 实战案例

三个端到端案例，交叉引用上面的规则；state/问题集原文见各次会话记录。

1. **架构方案三选一（两轮法）**：续跑机制去留。第一轮 3 选项粗评（每选项含实现摘要与丢失面），
   C_lift confidence 1.0 胜出；第二轮只对它发 7 道实现题（注入形态/防循环守卫/插入点/error
   策略/timer 退役/handle 范围/默认值），每题带代码级选项。两道低置信题（0.53/0.51）按
   「置信度→行为」处置：一道经集成测试否决后补列 followUp 变体再重选，一道按最小 diff 定案。
2. **测试覆盖筛选（keyed 批量）**：15 个候选场景 + 现有 11 条测试契约进 state，15 题独立
   noul 一发筛完（13 通过 2 冗余）。补上的测试第一个就抓出真实竞态——覆盖筛选不是仪式，
   是用 jev 的概率给"哪块测试缺失最危险"排序。
3. **元-jev（skill 自审）**：用 jev 评审 jev skill 自身——候选改进 + 真实踩坑证据
   （SyntaxError/Permission denied/269KB 400 的原文）进 state，noul×score 双题定优先级。
   自审也要遵守规则 8：漏掉"stamp 会插到 shebang 前"这个事实，实现方式的 choice 就给了
   0.31 置信的错误答案。

## 本仓接入（gate 的 l3 语义层）

`rules/harness/review_chain.py`：读问题 yaml → 调 jev → 按每题 `fail`/`warn` 阈值出
FAIL/WARN/INFO finding；jev 不可用时降级 `REVIEW_LLM_BASE_URL`/`REVIEW_LLM_MODEL` 的小 LLM
问同一套题。问题文件即 Python dict：`{qid: {type, instructions, criteria?, fail?, warn?}}`；
choice 题 bad 选项以 `_bad` 结尾命名，harness 汇总其概率为 p(issue)。

```bash
TYPESAFE_API_KEY=... TYPESAFE_API_BASE=https://api.knox.chat JEV_MODEL=jev-latest \
  python3 rules/harness/review_chain.py <questions.py> <fail阈值> <warn阈值> < state.json
```

## 语言

英文最准；中日韩可用但准确率下降——中文材料上格外盯 confidence，低置信走复核。
