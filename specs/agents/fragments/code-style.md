# 代码风格

- 函数名动宾结构、见名知目的（`parse_channel_config` 而不是 `do_config`）。
- 公共 API 写文档注释（用途、参数、错误、示例），模块头写 `//!`。
- 不留 AI 味注释（`// Step 1:` / `// This function` / `// 该函数…`）。注释只写"为什么"，不复述代码。
- 未实现的函数或 trait 用语言原生宏 + issue 号：`todo!("TODO(#123): …")` / `unimplemented!("…")`。
- TODO / FIXME 注释必须带 issue 号：`// TODO(#123): …`。
- 命名、缩进、格式化交给项目工具（`cargo fmt` / `gofmt` / `ruff format` / `prettier`），不手工对齐。
- 优先复用已有实现：先找同仓同类代码与已装依赖，再考虑新写。
- 删除优于新增：不留兼容垫片、旧别名、废弃分支。
