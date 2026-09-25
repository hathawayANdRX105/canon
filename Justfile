# Justfile — canon 高频 CLI 封装
#
# 规范正本仓的常用动线。改规范的完整流程见 AGENTS.md「常用 CLI」。
# 安装 just: pacman -S just（或 cargo install just）

# 默认列出全部 recipe
default:
    @just --list

# ── gate（canon 仓本身即 Rust crate）────────────────────────────

# 编译 release 二进制并落到 .githooks/gate（gate-sync 分发的是这个产物）
gate-build:
    cargo build --release
    cp target/release/gate .githooks/gate
    chmod 755 .githooks/gate
    @echo "✓ built + installed to .githooks/gate"

# gate crate 全量测试
gate-test:
    cargo test

# 格式检查（门禁会拦 fmt 违规，提交前先跑）
gate-fmt:
    cargo fmt --check

# gate 同步到成员仓（custom/ 受保护不覆盖）
gate-push project='':
    python3 scripts/gate-sync push {{project}} || just _gate-push-all

_gate-push-all:
    #!/usr/bin/env bash
    for p in algorchemy deskctl ferrite gugu kime new-api omenic silverq; do
        python3 scripts/gate-sync push "$p" | tail -1
    done

# 漂移检查（无参数=全部成员仓）
gate-status project='':
    #!/usr/bin/env bash
    if [ -n "{{project}}" ]; then
        python3 scripts/gate-sync status {{project}}
    else
        for p in algorchemy deskctl ferrite gugu kime new-api omenic silverq; do
            python3 scripts/gate-sync status "$p" | grep -E "^==|DRIFT|ONLY-CANON" | head -4
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

# gate 自检：跑本仓的全套 checklist
review:
    .githooks/gate check

# 提交前一站式：fmt + test + build + spec-sync
precommit: gate-fmt gate-test gate-build spec-sync
    @echo "✓ ready to commit"
