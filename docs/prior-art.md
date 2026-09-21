# 先行调研(Prior Art)— gate 设计的横向对照

调查日期 2026-09-21。回答一个问题:**这个领域现成方案怎么解,我们哪些拍对了、哪些拍弱了。**

## 五类候选

### 1. Hook 管理器(只管挂载,不管规则)
- **husky**(npm): hooks 写进 package.json,零规则概念,串行执行,Node 生态绑定。
- **lefthook**(Go, Evil Martians): 单个 `lefthook.yml`,job + glob 过滤(`{staged_files}`),`parallel: true`,安装即写 `.git/hooks/` 小脚本转发。**机制上最接近我们的 dispatch.yaml**,但规则体是命令行而非数据。
- **cargo-husky**: `cargo test` 专用,无配置面。

### 2. 规则即数据的分析器(最接近的先行者)
- **pre-commit**(Python): `.pre-commit-config.yaml` 引用「规则仓库 + 版本」,每 hook 独立 venv,`pre-commit run --all-files`,autoupdate。**启发:规则包可版本化、可复用、可锁定版本** —— 我们的 manifest→seed 分发对应它,但没有版本化。
- **semgrep**: yaml rule schema(`id/pattern/message/severity/languages`),registry 社区分发。**我们 checklist yaml 的直系同类**。差异:它用 AST 模式 DSL,我们用「外部 harness + stdin/stdout JSON 协议」——更钝,但零依赖、语言无关、LLM 可直接执行。
- **gitleaks**: TOML 规则(secret 正则)+ pre-commit 集成。**hardcoded_secret 的现成替代**;其规则库是社区维护的事实标准。

### 3. Policy-as-code(声明式 deny)
- **OPA / conftest**: Rego 对结构化 JSON 写 `deny[msg]`,`conftest test file.yaml`,自带 `conftest verify` 规则单测框架。**概念同构**:输入→JSON→规则判定→输出。差异:Rego 学习曲线陡(社区公认),我们刻意选了「yaml + 任意 harness」的钝方案。
- **启发(我们的 gap):规则本身要能被单测** —— gate spec 目前没有等价物。

### 4. AI slop 专门检测(直接竞品,2026 活跃赛道)
- **AI-SLOP-Detector**(GitHub): 静态分析「结构像但功能空/断连/误导」——幻觉包、幽灵依赖、stale API。
- **sloppoke**: 提交时 <10ms 判定,声称学习个人风格。
- **slopscan**: PR/commit message/注释空洞检测。
- 我们的差异化 = **棘轮**(只对增量记账)+ 本地静态、零外部服务。

## 复评表

| 决策点 | 现成方案 | 我们的拍板 | 复评 |
|---|---|---|---|
| 规则载体 | semgrep yaml / conftest Rego | checklist yaml + JSON harness | 拍对了:比 Rego 钝,零学习成本,LLM 友好 |
| 规则分发 | pre-commit repo+venv+版本 | manifest→seed 复制 | **拍弱了:无版本化/autoupdate**,roadmap 候选 |
| hook 挂载 | lefthook glob 路由 | dispatch.yaml 主题路由 | 等价,稍钝,够用 |
| secret 检测 | gitleaks 规则库 | 自写 5 语言 PCRE | **建议向 gitleaks 规则库对齐**,不重造 |
| 规则自测 | conftest verify | 无 | **gap**:`gate check --self-test` 候选 |
| slop 检测 | AI-SLOP-Detector / sloppoke | slop_comment + 棘轮 | 棘轮记账是差异化,保留 |

## 为什么仍自建

1. **棘轮记账**(复杂度/LOC 只管增量、存量冻结)没有现成工具做;
2. 本地、静态、无云、单二进制(lefthook 同级依赖水平);
3. spec 契约要与 omenic 的 UI 契约体系(7 份 yaml)协同,外部工具无法感知。

## action items(自调研导出)

- [ ] hardcoded_secret 规则库向 gitleaks TOML 对齐(翻译其社区规则,不重造)
- [ ] `gate check --self-test`:每份 spec 自带最小 positive/negative 样例,gate 自验
- [ ] 规则包版本化:manifest 加 `version:` 字段 + `gate init --upgrade` 提示更新(对标 pre-commit autoupdate 的降级版)
