# jev 审查记录：组装后的 AGENTS.md

**时间**：2026-09-24
**范围**：16 份组装文档（不含 tolaria，已移除）
**rubric**：`verdict`（4 选 1：good / hollow / conflicting / misleading）+
`actionable`（0-4 评分）

## 结果

| 判定 | 数量 | 项目 |
|---|---|---|
| good（可执行度 4.0） | 15 | oh-my-pi, herdr, dotfiles, new-api, silverq, kime, ferrite, ainotation, kymido, gugu, deskctl, algorchemy, sentinel, novel, canon |
| hollow（可执行度 1.0） | 1 | claude-code |

## 升级核对：claude-code 的 hollow 已驳回

按 jev 契约，judge 输出是证据不是结论，需读原文核对。核对结果：

| 指标 | claude-code | good 组中位数 |
|---|---|---|
| 独有内容可执行行 / 纯描述行 | **3.27** | 2.95 |
| 可执行行绝对数 | **183**（全组最高） | — |

**结论：驳回 hollow 判定。** claude-code 的可执行密度高于 good 组中位数，
且可执行行绝对数全组最高。判定为 hollow 的真实原因是**篇幅**：它的独有内容
2509 词（4326 tokens）居 16 份之首，Architecture 一节就有 14 个子小节的背景
铺垫，拉低了模型对整份文档的表观可执行率。

判据本身可靠（good 组判定与内容质量一致），但**不适合单份超长文档**。
下次审查这类大仓应按节分批送 judge，而非整份一次。

## 另一处发现（已修）

组装脚本的标题层级有 bug：片段内 H1 被降一级后变成 H2，与章节标题同级，
破坏文档层级。claude-code 有 14 个 H1，问题最明显。
已修 `bin/agents` 的 `section()`：按片段自身最小层级归一化，整体下移到 H3 之下。
修复后 16/16 项目标题层级无违规。
