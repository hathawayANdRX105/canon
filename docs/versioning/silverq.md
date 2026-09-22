# 版本统计任务书（silverq）

> 本文件是版本口径的唯一真相源：流程 + 功能域清单 + 快照都在这里。
> 根目录 `VERSIONING.md` 已删除（2026-09-21），内容并入本文件。

## 任务

统计 silverq 的功能数与 fix 提交数，产出新版本号并更新 Cargo.toml 与本文件快照。

## 版本号三段（来源各不同，别混）

- **major** = **用户确认**。breaking change 由用户拍板，不自动算。执行前先问用户当前 major（当前 0）。
- **minor** = 功能域数（本文件功能域清单逐项清点；新增模块时按「移除后用户是否察觉」判定，察觉=计入）。
- **patch** = **fix 类型 commit 累计数**（基于发版分支 master：`git log master --no-merges --format="%s" | grep -cE "^fix"`；`perf` 不算 fix）。当前 25（2026-09-21 v0.12.25 发版重跑值：机械计数 26，其中 #11「drop UPX」已被 #12 回滚，实际生效 25）。

## 执行步骤

### 1. 确认 major

问用户当前 major 值（无 breaking 保持 0；有 breaking 用户确认后 +1）。

### 2. 核对 minor（功能域）

对照本文件功能域清单（当前 12 项）与代码现状：

```bash
# 候选集机械枚举：新增 pub 模块/文件要过一遍判定
find src -name '*.rs' | sort
grep -E "^(pub )?(mod|fn|struct|enum) " src/lib.rs
```

- 新增用户可感知能力 → 在本文件功能域清单加一行，minor +1。
- 改动只在既有功能域内（bug 修复/内部重构）→ minor 不变。
- 排除管道：`config`、`proxy/factory`、`scheduler/node` 等内部层不计。

> **2026-09-20 待决**：本文件排除表里 `dataplane/tun` 的理由是「未实现占位」——TUN 已于 #4 实现并在 master（用户主动关闭不用）。下次发版需重新判定：按「移除后用户是否察觉」口径，用户既已决定不用 TUN，倾向仍计管道；但这是用户确认项，别默默沿用旧理由。

### 3. 统计 patch

```bash
git log master --no-merges --format="%s" | grep -cE "^fix"
```

注意：本地开发分支（如 feat/*）上的 fix 在合入 master 前不计——patch 跟随发版分支。

### 4. 更新 Cargo.toml

```bash
MAJOR=<步骤1> MINOR=<步骤2> PATCH=<步骤3>
sed -i "s/^version = .*/version = \"\$MAJOR.\$MINOR.\$PATCH\"/" Cargo.toml
cargo check   # 刷新 Cargo.lock
```

### 5. 同步本文件快照节

更新「## 快照」的三行数字；功能域清单有增删时同步上表。

### 6. 提交 + tag

```bash
git add Cargo.toml Cargo.lock .agent/tasks/versioning.md
git commit -m "chore(release): v$MAJOR.$MINOR.$PATCH"
git tag "v$MAJOR.$MINOR.$PATCH"
git push origin master "v$MAJOR.$MINOR.$PATCH"
```

### 7. 发布产物核对（push tag 后）

tag 触发 `release.yml`：tag↔Cargo.toml 一致性校验 → 双变体构建（plain / meow-tun）→ UPX `--best --lzma` → 双二进制冒烟（--version）→ 打包发布。**workflow 绿 ≠ 产物对，发布后必须回读**：

- **Release 标题**：GitHub Actions 的 `with:` 参数是字面字符串，`$VAR` 不插值（只 `run:` 步骤插值），必须用 `${{ github.ref_name }}`（当前已正确，改动时别退回 `$VAR` 写法）。
- **Release notes**：当前是 workflow 内手写 body（无 generate_release_notes，所以没有「by @作者」尾巴）。若改用自动 notes，注意它会追加「by @作者 in #N/URL」，需加 strip 步骤（参考 kime 仓库 `.agent/tasks/versioning.md`）。
- **Assets 4 个齐全**：`silverq-X.Y.Z-x86_64-linux.tar.gz` + `.sha256`、`silverq-tun-X.Y.Z-x86_64-linux.tar.gz` + `.sha256`。
- **Draft 清理**：调试期手动触发会留 draft（历史上残留过 v0.13.6 / v0.18.16），发布后 `gh release list` 扫一眼，stale draft 删掉。

## 校验（全过才算完成）

- [ ] major = 用户确认值
- [ ] 本文件功能域清单项数 == Cargo.toml 的 minor
- [ ] 步骤 3 命令输出 == Cargo.toml 的 patch
- [ ] `cargo check` 通过
- [ ] tag 已推送，CI success
- [ ] Release list 标题正确（`silverq vX.Y.Z`），notes 与 workflow body 一致
- [ ] Release assets 4 个齐全（plain/tun 各 tar.gz + sha256）
