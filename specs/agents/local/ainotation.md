# Ainotation

框架无关的 DOM 标注与原生动画检查工具，通过 Script/SDK 接入开发者项目。
以下选型已确定，无需在后续会话中重复确认。

## 技术栈

- 核心：TypeScript（strict），不依赖宿主渲染框架。
- UI：Lit + Shadow DOM；原生 CSS / CSS Variables；Lucide 图标。
- 动画与捕获：Web Animations API、Screen Capture API 等原生浏览器能力，渐进增强。
- 截图编辑：位图底图 + SVG 标记；Canvas 负责裁剪与图片合成。暂不引入 Konva。
- 图片标注从 marker popover 的 Screenshot action 进入实时页面绘图，Option/Alt 临时穿透、松开恢复绘制；支持粘贴、拖入、选择 PNG/JPEG/WebP 图片。完成生成 PNG 附件，先存入当前标注草稿，随 feedback 保存。图片元数据进入反馈契约，位图使用 IndexedDB Blob 与独立的鉴权传输，不塞入 JSON 同步正文；带图导出为 JSON/Markdown/PNG 的 ZIP，MCP 通过 ainotation_get_image 按需读取。
- 数据契约：Zod + JSON Schema；反馈文档独立于 UI、SVG 和浏览器运行时对象。
- 本地存储：IndexedDB + idb，图片使用 Blob，不放入 localStorage。
- 工具链：Vite+；应用使用 `vp dev` / `vp build`，SDK 与 MCP 包使用 `vp pack`。
- 组件开发：Storybook + `@storybook/web-components-vite`。
- 测试：Vitest + Playwright；保留小型 Playground 验证真实宿主、动画和屏幕捕获。
- 包管理与发布：pnpm workspace + Changesets，结合 Vite+ 管理。
- Agent 集成：独立 Node.js LTS 进程 + 官方 MCP SDK；浏览器通过 HTTP/SSE 通信。

## 实现边界

- MCP 是首版正式能力，但基础标注不依赖其运行；未连接时可复制反馈、导出附件。两种出口共用数据契约。
- SDK 支持 `mcp: false` 的实例级仅本地模式：不读取或修改 MCP 凭据、不连接或同步、不显示 MCP 连接配置与状态，保留标注、图片、复制和导出；不能与 `development` 同时设置。官网演示使用该模式，默认和 Vite 自动配对行为保持兼容。
- 当前阶段仅开放持久化标注 CRUD 与批量复制/导出；多轮对话模型和持久化能力保留在内部，UI 与 MCP 工具不开放回复或处理状态工作流。
- 标注与草稿按项目和完整页面 URL 隔离；复制汇总当前项目所有已保存页面并按 URL 分组，JSON 导出与清除操作作用于当前页面。路由往返时重新校验目标身份并恢复对应页面的 marker。
- 支持正文文本选区标注与有边界的通用 DOM 上下文采集。Settings 提供 Compact / Standard / Detailed / Everything 四档 Markdown 输出，默认 Standard；档位不裁剪持久化、JSON 导出或 MCP 数据。React 组件链和源码定位不属于当前阶段。
- 标注面板提供 Feedback / Styles 页签。样式按页面目标共享，以 document.targetStyles 为唯一数据源，目标 ID / styleTargetId 引用，inline styleChanges 是兼容性投影；多个 Marker 指向同一真实 DOM 节点时共享草稿和已保存样式，不按选择符相同直接合并。工具栏全局预览默认开启，局部开关仅影响当前目标/多选范围；全局关闭还原整页并禁用局部开关，重开恢复局部选择。保存、关闭编辑器、收起工具保留整页预览，取消回到当前编辑范围的已保存值；离页/卸载清理覆盖且保留宿主新写入，样式漂移需确认，缺失/替换节点不重绑。Shift 多选默认批量编辑，支持混合值、相对键盘/滚轮微调、黄色圆点各自恢复、四边联动和原子撤销；局部预览支持部分选中态。共享草稿/预览设置按项目和完整 URL 隔离，删除最后一个 Marker 引用时清理共享样式；原始快照不变，贯通 Markdown/JSON/ZIP/MCP，父级导航保留修改且单标注最多 20 个目标。旧 MCP 服务缺少 styleSuggestions/sharedStyles 能力时暂停相关同步并保留本地数据。
- 核心逻辑、反馈文档、UI、集成分离；无需提前把每个模块拆成发布包。
- 选择元素即持续观察其子树中的动画；多选取并集，不扩大到共同祖先。观察不暂停页面，显式检查才接管动画。
- Inspector 展开为 toolbar 并持续选取，Shift 临时多选、Option/Alt 临时穿透真实页面交互；标注通过页面 marker/popover 就地增删改。Toolbar 提供复制、导出、清除当前页面全部标注、Settings 和关闭，连接配置在 Settings popover 中。收起隐藏页面标记并停止拾取，不删除持久化数据。
- 编辑器打开时普通外部点击只收起并保留草稿，下一次点击才选取；Shift 多选、文本选区和 Option/Alt 穿透保留。按 Marker、项目和完整 URL 记忆编辑页签、拖拽位置及目标范围，保存新 Marker 时继承草稿的编辑上下文；父子导航不隐式扩大批量范围。初始隐藏/未挂载目标出现后重试严格身份校验，已绑定节点被替换时不重绑。样式输入失焦不得同步重入正在移除节点的渲染过程。
- 标注编辑器不显示最小化按钮或拖拽把手，拖动非交互空白区域即可移动面板；输入、选择、按钮、标签、展开标题、定位文本及滚动条保留原有交互，触摸滚动区域优先滚动。外部点击收起编辑器；聚焦面板本身时可用方向键移动。拖拽结束不得误触按钮，离开、失去指针捕获和卸载时清理拖拽状态。
- Padding / Margin 分成独立间距组，默认各一个四边联动输入；两个模式按钮分别展开水平/垂直联动或四边独立输入，再次点击激活项回到默认。展开时保留四边总输入，切换模式不修改样式；混合值、相对微调、黄色圆点按范围各自还原及原子撤销同时覆盖各边与多选目标。
- 本地 editorViews 另存选择项及各自父子返回路径（最多 20 项、每项 64 层），与完整关联修改目标分离。保存/重开/刷新恢复导航结构，单条父子导航不转成并列多选；显式 Shift 多选保持独立。返回前验证真实节点身份及直接父子关系，旧记录/无效导航元数据或已改变的关联目标集合回退目标列表，不根据 DOM 嵌套推断操作历史。导航上下文不进入 Agent 反馈契约。
- Trigger 与 toolbar 共用移动锚点；位置和展开方向按项目持久化，卸载、重新挂载和刷新后以收起态恢复，并限制在可见视口内。
- Ainotation UI 支持 Light / Dark 主题，默认 Light，在 Settings 切换并按项目持久化；主题覆盖 toolbar、trigger、Settings 和 marker/popover，不修改宿主页面配色或反馈快照。
- UI 国际化使用 SDK 内部类型安全字典与实例级语言状态，不新增 i18n 运行时库。Settings 支持 zh-Hans / zh-Hant / en / ja / ko，首次匹配浏览器语言、回退英文，手动选择按项目持久化；切换不重建编辑器，不翻译用户反馈、页面原文、JSON/MCP 标识或 Markdown 交接结构。
- 暂不做浏览器扩展、任意 JS 动画倒放、录像/GIF、云端账号和多人协作；源码定位为可选增强。
- 工具运行时依赖由 SDK 自行提供，不要求宿主安装 Lit；重模块按需加载。
- 原生资源生命周期独立于 UI 渲染；销毁时清理监听器、动画控制和媒体流。
- MCP 服务默认仅监听 loopback，并实现本地鉴权、Origin 校验和会话隔离。
- MCP 使用数据目录外的私有 IPC 协调器保护共享服务归属；缺失运行信息、注册表和已加载反馈可从内存重建，损坏 JSON 保留后尝试备份。恢复同步通过存储代次、版本校验及缺失图片列表避免静默覆盖；Settings 支持重试、项目恢复和版本选择。`doctor` 只读，`repair` 默认保留数据，显式重置与外部备份恢复要求服务停机；详见 `RECOVERY.md`。
- 标准接入通过 Vite 插件 `ainotation({ name, id? })` 声明项目，不要求独立身份文件或 init。插件启动时注册本机项目；MCP 按 roots/目录限定工作区，通过 `ainotation_list_projects` 发现 app，每次调用用可选 `project` 参数按名称、稳定键或 UUID 选择。单项目自动选择，多项目或同名歧义返回候选；不使用全局“当前项目”，不越过工作区范围。显式目录匹配 app 时固定限定该 app。name 默认兼作稳定键，显式 id 允许重命名；旧 UUID 与旧配置入口保留兼容。

## 当前仓库

```text
packages/
  schema/              共享数据契约
  sdk/                 核心逻辑与 Lit UI
  mcp/                 独立 MCP 服务
  vite/                开发环境注入与项目自动配对插件
apps/
  playground/          SDK 集成开发与验证
  storybook/           UI 组件开发
  website/             单页产品官网，GitHub Pages 部署
pnpm-workspace.yaml    Workspace 配置与依赖版本 catalog
```

使用 `vp install` 安装依赖，`vp run dev` 启动 Playground，`vp run storybook` 启动组件开发环境；提交前运行 `vp run ready`。
`vp <命令>` 是工具链内置命令，`vp run <脚本>` 执行 workspace 脚本；新增功能应补对应测试。
