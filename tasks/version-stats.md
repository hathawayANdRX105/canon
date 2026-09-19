# 版本统计任务书（通用骨架）

> 这是**通用骨架**：版本三段的口径、功能域判定、执行流程、发布核对是共享的。
> **项目特有**内容（候选集枚举命令、排除表、真相源文件、发版分支、tag 格式、产物清单）在本项目自己的 `versioning.md` 里。
> 两者冲突**以项目 `versioning.md` 为准**——它绑定了真实路径，本文件只定流程。

## 版本号三段（来源各不同，别混）

- **major = 用户确认**。breaking change 由用户拍板，不自动算。执行前先问用户当前 major。
- **minor = 功能域数**。用户/前端能直接感知的能力，逐项清点。
- **patch = fix 类型 commit 累计数**。机数：只数主题行以 `fix` 开头的提交，`perf` 不算 fix。

## 判定「功能域 vs 管道」（minor 记不记，就这一条）

该能力若被移除，**用户/前端是否察觉**？

- 察觉（候选消失 / 上屏没了 / 某键行为没了）→ **功能域**，计入 minor。
- 不察觉（内部存储 / 解析 / 解码 / 状态机 / 配置 / 调度，行为由别的模块对外体现）→ **管道**，不计。

候选集 = 顶层公开能力（`pub mod` / 顶层 API / 平台前端通路，枚举命令见项目 `versioning.md`）。
新增模块：重跑枚举，逐个过这条判定，再对照项目排除表核对。

## 执行步骤

### 1. 确认 major

问用户当前 major 值（无 breaking 保持 0；有 breaking 用户确认后 +1）。

### 2. 清点 / 核对 minor

对照项目的**真相源清单**（`VERSIONING.md` / ROADMAP 的功能域表）与代码现状：

- 新增用户可感知能力 → 清单加一行，minor +1。
- 只在既有功能域内修 bug / 内部重构 → minor 不变。
- 枚举命令与固定排除表见项目 `versioning.md`；清单项数必须 == minor。

### 3. 统计 patch

```bash
git log <发版分支> --no-merges --format="%s" | grep -cE "^fix"
```

`--format="%s"` 取纯主题行（无 hash、无前导空格），`^fix` 才能匹配。
发版分支名见项目 `versioning.md`；本地 `feat/*` 分支上的 fix 合入前不计——patch 跟随发版分支。

### 4. 写入版本号

先存 shell 变量再代入（别在 sed 里写占位）：

```bash
MAJOR=<步骤1的数字>   MINOR=<步骤2合计>   PATCH=<步骤3输出>
sed -i "s/^version = .*/version = \"\$MAJOR.\$MINOR.\$PATCH\"/" Cargo.toml
cargo check   # 刷新 Cargo.lock
```

### 5. 同步真相源文档

更新项目的 `VERSIONING.md` / ROADMAP 快照节与功能域清单（清单项数必须 == minor）。

### 6. 提交 + tag

```bash
git add Cargo.toml Cargo.lock <真相源文档>
git commit -m "chore(release): v$MAJOR.$MINOR.$PATCH"
git tag "v$MAJOR.$MINOR.$PATCH"
git push origin <发版分支> "v$MAJOR.$MINOR.$PATCH"
```

### 7. 发布产物核对（push tag 后）

**workflow 绿 ≠ 产物对**（历史上标题坏了三个版本才发现）。发布后必须回读：

- **Release 标题**：GitHub Actions 的 `with:` 参数是字面字符串，`$VAR` **不插值**（只有 `run:` 步骤插值）——必须用 `${{ github.ref_name }}` 表达式，否则标题变成字面量。
- **Release notes**：若开 `generate_release_notes: true`，它会在每条 PR 后追加「by @作者 in #N/URL」尾巴，需要 strip 步骤剥掉（kime / silverq 都踩过）。
- **Assets 齐全**：数量与命名按项目产物清单核对（见项目 `versioning.md`）。
- **Draft 清理**：手动触发会留 stale draft，`gh release list` 扫一眼删掉。

## 校验（全过才算完成）

- [ ] major = 用户确认值
- [ ] 真相源清单项数 == Cargo.toml 的 minor
- [ ] 步骤 3 命令输出 == Cargo.toml 的 patch
- [ ] `cargo check` 通过
- [ ] tag 已推送，CI success
- [ ] Release 标题正确（非字面变量），notes 无「by @作者」尾巴
- [ ] Release assets 按项目清单齐全
