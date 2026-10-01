# Justfile — canon 高频 CLI 封装
#
# 规范正本仓的常用动线。改规范的完整流程见 AGENTS.md「常用 CLI」。
# 安装 just: pacman -S just（或 cargo install just）

# 默认列出全部 recipe
default:
    @just --list

# ── build / test ────────────────────────────────────────────────

# 编译 release 二进制并落到 .githooks/canon（canon-sync 分发的是这个产物）
build:
    cargo build --release
    cp target/release/canon .githooks/canon
    chmod 755 .githooks/canon
    @echo "✓ built + installed to .githooks/canon"

# 全量测试
test:
    cargo test

# 格式检查（门禁会拦 fmt 违规，提交前先跑）
fmt-check:
    cargo fmt --check

# 装到 ~/.local/bin（MCP 客户端按绝对路径拉起 canon mcp）
install:
    cargo build --release
    # Atomic replace, not `cp`: the MCP client holds this exact path open as a
    # running server, and `cp` onto a busy executable fails with ETXTBSY.
    # `mv` is a rename, which the kernel allows over a running image.
    install -m 755 target/release/canon ~/.local/bin/.canon.new && mv ~/.local/bin/.canon.new ~/.local/bin/canon
    @echo "✓ canon installed to ~/.local/bin/canon"

# ── 分发到成员仓 ────────────────────────────────────────────────

# canon 同步到成员仓（custom/ 受保护不覆盖）
push project='':
    python3 scripts/canon-sync push {{project}} || just _push-all

_push-all:
    #!/usr/bin/env bash
    for p in algorchemy deskctl ferrite gugu kime new-api kymido silverq ui-kit; do
        python3 scripts/canon-sync push "$p" | tail -1
    done

# 漂移检查（无参数=全部成员仓）
status project='':
    #!/usr/bin/env bash
    if [ -n "{{project}}" ]; then
        python3 scripts/canon-sync status {{project}}
    else
        for p in algorchemy deskctl ferrite gugu kime new-api kymido silverq ui-kit; do
            python3 scripts/canon-sync status "$p" | grep -E "^==|DRIFT|ONLY-CANON" | head -4
        done
    fi

# ── spec 正本 ↔ 部署镜像 ────────────────────────────────────────

# specs/（正本）→ .githooks/spec/（canon 自用部署镜像）；custom/ 项目专有不覆盖
spec-sync:
    rsync -a --delete --exclude="custom" --exclude="__pycache__" --exclude="*.pyc" \
        specs/ .githooks/spec/
    @echo "✓ synced specs/ → .githooks/spec/"

# ── agent 文档 ──────────────────────────────────────────────────

# 重新组装 canon 自己的 AGENTS.md
agents-push:
    python3 scripts/agents push canon

# 全项目漂移检查（AGENTS.md 生成物 vs 源）
agents-status:
    python3 scripts/agents status

# agent-sync 分发任务书/规则到成员仓
agent-sync-push project:
    python3 scripts/agent-sync push {{project}}

# ── 自检 ────────────────────────────────────────────────────────

# canon 自检：跑本仓的全套 checklist
review:
    .githooks/canon check

# 提交前一站式：fmt + test + build + spec-sync
precommit: fmt-check test build spec-sync
    @echo "✓ ready to commit"
