# 代码约定：UI、Rust、调查与审查

**什么时候读这份文档**：写前端界面（Dioxus）、写 Rust 公共接口、或者要调查代码 / 审查代码的时候。

**这份文档解决什么**：三类容易做错的约定——前端 UI 的验证方式、Rust 代码的写法要求、
以及调查和审查代码该用什么工具。

---

## 一、遇到什么问题（含曾经踩过的坑）

| 你看到的现象 | 真正的原因 | 怎么办 |
|---|---|---|
| 前端改动靠截图判断"看起来对"，但验收时说不清楚 | 只做了肉眼判断，没有可自动验证的契约 | 给交互元素加 `data-testid`，每页写一份 spec 文件 |
| PR 验收时用 class 选择器找元素，页面改样式就失效 | 用 class 做定位 | 用 `role` 加 `aria-label`，以及 `data-testid` |
| 写完公共函数没写文档注释，几个月后没人知道参数含义 | 没写 rust doc | 公共 API 必须写 `///` 文档，模块头写 `//!` |
| 一次性把整个仓库喂给审查工具，结果被限流或输出一堆噪音 | 没分批 | 审查按模块分批喂，只喂这次改动的 diff |
| 逐文件翻代码找调用关系，花了很久还找不全 | 没有用代码图谱 | 先建图谱再查调用关系 |

---

## 二、维护者希望做什么事

### UI 验证（Dioxus 前端）

- 交互元素都要加 `data-testid`，值用元素的 `name` 属性值。
- 容器元素要加 `role` 和 `aria-label`。
- 每个页面写一份契约文件，位置是**该页面所在 crate 里的** `specs/ui/<页面名>.yaml`
  （例如 `crates/web/admin-page-admin/specs/ui/aliases.yaml`），列出 role、name、testid、action。
  现成的例子：`crates/web/admin-page-admin/specs/ui/` 下有 5 份，照着写。
- PR 的冒烟验证用 `tab.ariaSnapshot()` 检查 role、name、testid 是否正确。
  这是 Playwright 的 API：`tab` 是会话工具 `browser` 打开的页签对象，
  用法示例在 `.agent/skills/ui-validation/SKILL.md` 里有完整代码。
- 截图只作为辅助手段（看视觉风格和品牌效果），失败的时候附上截图。

**禁区**：只用截图肉眼判断、用 class 选择器定位元素、没写 ui-spec 文件就直接提 PR。

详细操作在项目级技能文件里：`.agent/skills/ui-validation/SKILL.md`

### Rust 代码风格

- 函数名用动宾结构，见名知目的。写成 `parse_channel_config`，不要写 `do_config`。
  类型和结构体名要能说清自己的角色。
- 公共 API 必须写 rust doc（`///`），说明：用途、参数含义、错误情况、示例。
  模块头部写 `//!` 说明职责。**写文档注释是交付的一部分，不是可选的装饰。**

### 调查与审查代码

- **调查代码**：先用 `code-review-graph update` 建立增量图谱（命令在 `~/.local/bin/`，已在 PATH，
  图谱数据存在仓库根的 `.code-review-graph/` 目录），再通过图谱查调用关系和整体结构。
  不要直接逐个文件翻（本地没有 LSP，查调用方只能靠图谱）。
- **审查代码**分两层：
  1. 结构层：`code-review-graph detect-changes`，看改动的影响面。
  2. 规范层：`ocr review`（`ocr` 装在 `/usr/bin/ocr`，用法 `ocr --help` 可查），
     按文件或模块分批跑，**禁止一次性把整个仓库喂进去**（会触发限流）。

**重要区分**：`OCR` 是图片文字识别；本文说的 `ocr` 命令是**代码审查工具**（OpenCodeReview）。
在审查语境下提到 ocr，指的是后者。

---

## 三、可能的情况

| 你要做的事 | 该怎么做 |
|---|---|
| 新增或改动交互元素 | 加 `data-testid`，并更新该页面的 `specs/ui/<页面名>.yaml` |
| 改动整个页面 | 跑 `ariaSnapshot` 断言，不要只靠截图 |
| 检查视觉回归（配色、布局） | 截图辅助对比 |
| 新增公共函数或类型 | 先写 rust doc 再写实现 |
| 重命名公共符号 | 保持动宾结构，同时改所有调用方 |
| 改公共符号之前 | 用代码图谱查所有调用方，列出全部要改的地方 |
| PR 收尾审查 | 先 CRG（带 `--base <base_sha>`），再分批跑 ocr |
| 只读了某个模块 | ocr 只喂该模块的 diff |

---

## 四、约束事项（简略）

- UI 禁区：只用截图肉眼判断、用 class 选择器、没写 ui-spec 文件就提 PR。
- 公共 API 没有 rust doc 视为不完整交付。
- ocr 审查禁止一次性喂整个仓库（会限流）；审查结论和修复记录要写进 PR 评论
  （闸门的 `pr_crg_review` 检查项要求）。
