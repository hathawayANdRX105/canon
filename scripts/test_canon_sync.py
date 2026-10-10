#!/usr/bin/env python3
"""canon-sync 分发判据的回归测试（无依赖，CI 直接 `python3` 跑）。

钉住三件容易悄悄坏掉的事：

  1. **适用面门**：dioxus 家族只能发给真有 dioxus 证据的仓。判据失效的失效形态是
     「所有仓都收到一堆永不命中的规则」或「dioxus 仓收不到规则」，两种都不会报错。
  2. **搬家清场**：engine 递归加载 spec/ 下全部 checklist，同一条规则两份 = 跑两遍。
     正本没落到目标位置时不许删（那是唯一副本）；内容不一致时不许删（要人裁决）。
  3. **custom/ 保护**：豁免名单是项目侧资产，清场逻辑不得越界。

canon-sync 是无扩展名脚本，用 SourceFileLoader 直接加载真身，不复制逻辑。
"""

from __future__ import annotations

import importlib.util
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_loader(
    "canon_sync", importlib.machinery.SourceFileLoader("canon_sync", str(HERE / "canon-sync")))
canon_sync = importlib.util.module_from_spec(spec)
sys.modules["canon_sync"] = canon_sync
spec.loader.exec_module(canon_sync)

MOVED = sorted(canon_sync.MOVED_RULES)[:1]  # 真表里取一条，不写死文件名


def git_repo(tmp: Path, files: dict[str, str]) -> str:
    for rel, body in files.items():
        p = tmp / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(body, encoding="utf-8")
    for cmd in (["init", "-q"], ["-c", "user.email=t@t", "-c", "user.name=t", "add", "-A"],
                ["-c", "user.email=t@t", "-c", "user.name=t", "commit", "-qm", "f"]):
        subprocess.run(["git", "-C", str(tmp), *cmd], check=True, capture_output=True)
    return str(tmp)


def check(label: str, got: object, want: object) -> int:
    if got == want:
        print(f"  ok  {label}")
        return 0
    print(f"FAIL  {label}\n  got : {got}\n  want: {want}")
    return 1


def main() -> int:
    failures = 0
    base = Path(tempfile.mkdtemp(prefix="canon-sync-"))

    dioxus = git_repo(base / "dioxus", {"Cargo.toml": "[dependencies]\ndioxus = \"0.6\"\n"})
    plain = git_repo(base / "plain", {"pyproject.toml": "[project]\nname = 'x'\n"})

    failures += check("dioxus 依赖 = 有证据", canon_sync.has_evidence(dioxus, "dioxus"), True)
    failures += check("无 dioxus 无 rsx! = 无证据", canon_sync.has_evidence(plain, "dioxus"), False)
    rsx_only = git_repo(base / "rsx", {"src/main.rs": "fn main() { let _ = rsx! {} }\n"})
    failures += check("rsx! 宏也算证据", canon_sync.has_evidence(rsx_only, "dioxus"), True)

    gated = "spec/dioxus/"
    d_rels = [r for r in canon_sync.owned_rels(dioxus) if r.startswith(gated)]
    p_rels = [r for r in canon_sync.owned_rels(plain) if r.startswith(gated)]
    failures += check("dioxus 仓拿到家族规则", bool(d_rels), True)
    failures += check("非 dioxus 仓一条都不拿", p_rels, [])
    failures += check("家族规则不得落进 custom/",
                      [r for r in canon_sync.owned_rels(dioxus) if "/custom/" in r], [])

    # —— 搬家清场：三种结局（一致删、不一致留并报、正本未落地不动）——
    name = MOVED[0]
    sub = canon_sync.MOVED_RULES[name]
    hooks = base / "hooks"
    spec_dir = hooks / "spec"
    (spec_dir / sub).mkdir(parents=True)
    (spec_dir / "quality").mkdir()
    (spec_dir / "custom").mkdir()
    keeper = spec_dir / sub / name
    keeper.write_text("keeper\n", encoding="utf-8")
    same_copy = spec_dir / "quality" / name
    same_copy.write_text("keeper\n", encoding="utf-8")
    diff_copy = spec_dir / "custom" / name
    diff_copy.write_text("project fork\n", encoding="utf-8")
    orphan_dir = spec_dir / "elsewhere"
    orphan_dir.mkdir()
    orphan = orphan_dir / f"unrelated_{name}"
    orphan.write_text("keeper\n", encoding="utf-8")

    dropped, conflicts = canon_sync.retire_moved(str(hooks))
    failures += check("内容一致的旧副本被清掉", same_copy.exists(), False)
    failures += check("custom/ 里的同名文件绝不碰（项目侧资产）", diff_copy.exists(), True)
    failures += check("非搬家表内的文件不碰", orphan.exists(), True)
    failures += check("清理结果有书面记录", dropped, [f"spec/quality/{name}"])

    same_copy.write_text("keeper\n", encoding="utf-8")   # 复原后测不一致分支
    keeper.write_text("keeper v2\n", encoding="utf-8")
    dropped, conflicts = canon_sync.retire_moved(str(hooks))
    failures += check("内容与正本不一致时不删", same_copy.exists(), True)
    failures += check("不一致必须报 conflict", len(conflicts), 1)

    keeper.unlink()
    dropped, conflicts = canon_sync.retire_moved(str(hooks))
    failures += check("正本未落地时不动唯一副本", same_copy.exists() and dropped == [], True)

    print("PASS" if failures == 0 else f"{failures} 组失败")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
