#!/usr/bin/env python3
"""canon-sync 分发判据的回归测试（无依赖，CI 直接 `python3` 跑）。

钉住四件容易悄悄坏掉的事：

  1. **适用面门**：dioxus 家族只能发给真有 dioxus 证据的仓。判据失效的失效形态是
     「所有仓都收到一堆永不命中的规则」或「dioxus 仓收不到规则」，两种都不会报错。
  2. **搬家清场**：engine 递归加载 spec/ 下全部 checklist，同一条规则两份 = 跑两遍。
     正本没落到目标位置时不许删（那是唯一副本）；内容不一致时不许删（要人裁决）。
  3. **custom/ 保护**：豁免名单是项目侧资产，清场逻辑不得越界。
  4. **项目根解析**：安装根只认清单里的 `root:` 映射（ferrite → mono）。映射目标不在、
     两份清单打架、或压根没写映射，都必须报错 —— 按项目名拼目录正是把种子写进同名
     陈旧检出的原因。三个入口（agents / agent-sync / canon-sync）共用 scripts/canon_roots.py。

canon-sync 是无扩展名脚本，用 SourceFileLoader 直接加载真身，不复制逻辑。
"""

from __future__ import annotations

import contextlib
import importlib.util
import io
import os
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
canon_roots = sys.modules["canon_roots"]  # canon-sync 从同目录导入的共享解析实现

MOVED = sorted(canon_sync.MOVED_RULES)[:1]  # 真表里取一条，不写死文件名


def sh(*args: str) -> str:
    r = subprocess.run(args, capture_output=True, text=True, check=True)
    return r.stdout.strip()


@contextlib.contextmanager
def environ(**values: str | None):
    """临时改环境变量（值为 None = 删除），退出时还原。"""
    old = {k: os.environ.get(k) for k in values}
    for key, value in values.items():
        if value is None:
            os.environ.pop(key, None)
        else:
            os.environ[key] = value
    try:
        yield
    finally:
        for key, value in old.items():
            if value is None:
                os.environ.pop(key, None)
            else:
                os.environ[key] = value


def run_resolution(project: str, canon_dir: Path, base: Path) -> tuple:
    """跑真身 project_root（受控清单 + 受控 base）；解析失败返回 (None, 报错文本)。

    报错有两种出口：canon_roots 直接写 stderr 后退出，canon-sync 把文案交给
    `sys.exit(msg)`（被捕获时不会打印），两条都要收进来。
    """
    stderr = io.StringIO()
    environ_switch = environ(CANON_PROJECTS_BASE=str(base), CANON_TARGET_ROOT=None)
    with environ_switch, contextlib.redirect_stderr(stderr):
        try:
            return canon_sync.project_root(project, canon=str(canon_dir)), ""
        except SystemExit as e:
            return None, stderr.getvalue() + (e.code if isinstance(e.code, str) else "")


def run_cli(args: list, env: dict) -> tuple:
    """跑 scripts/ 下的真脚本（不是导入的副本），返回 (退出码, stdout + stderr)。"""
    cmd = [sys.executable, str(HERE / args[0]), *args[1:]]
    r = subprocess.run(cmd, capture_output=True, text=True, env=env, check=False)
    return r.returncode, r.stdout + r.stderr


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


def check_project_root(base: Path) -> int:
    """安装根解析：`root:` 映射是唯一依据，映射缺失/打架/目标不在都必须报错。"""
    failures = 0
    target = base / "targets"
    (target / "mono").mkdir(parents=True)
    (target / "ferrite").mkdir()  # 同名陈旧检出：停在旧提交的独立检出，不得被选中
    failures += check("测试前提：映射目标与同名目录并存",
                      (target / "mono").is_dir() and (target / "ferrite").is_dir(), True)

    mapped = base / "manifests-mapped"
    mapped.mkdir()
    (mapped / "agent-sync.yaml").write_text(
        "projects:\n  ferrite:\n    root: mono\n", encoding="utf-8")
    got, err = run_resolution("ferrite", mapped, target)
    failures += check("映射存在 → 解析到映射目录", got, str(target / "mono"))
    failures += check("映射存在 → 不报错", err, "")

    gone = base / "manifests-gone"
    gone.mkdir()
    (gone / "agent-sync.yaml").write_text(
        "projects:\n  ferrite:\n    root: mono-gone\n", encoding="utf-8")
    got, err = run_resolution("ferrite", gone, target)
    failures += check("映射目标不存在 → 解析失败，不静默回退同名目录", got, None)
    failures += check("报错点名映射目标", str(target / "mono-gone") in err, True)
    failures += check("报错点名未被采用的同名目录", str(target / "ferrite") in err, True)

    unmapped = base / "manifests-unmapped"
    unmapped.mkdir()
    (unmapped / "agent-sync.yaml").write_text(
        "projects:\n  ferrite:\n    files: []\n", encoding="utf-8")
    got, err = run_resolution("ferrite", unmapped, target)
    failures += check("没有 root: 映射 → 解析失败", got, None)
    failures += check("报错说明不按项目名猜目录", "不按项目名猜" in err, True)

    split = base / "manifests-split"
    split.mkdir()
    (split / "agent-sync.yaml").write_text(
        "projects:\n  ferrite:\n    root: mono\n", encoding="utf-8")
    (split / "agents.yaml").write_text(
        "projects:\n  ferrite:\n    root: ferrite\n", encoding="utf-8")
    got, err = run_resolution("ferrite", split, target)
    failures += check("两份清单映射打架 → 解析失败", got, None)
    failures += check("打架报错列出两份出处",
                      "agent-sync.yaml" in err and "agents.yaml" in err, True)

    absolute = base / "manifests-absolute"
    absolute.mkdir()
    (absolute / "agent-sync.yaml").write_text(
        f"projects:\n  ferrite:\n    root: {target / 'mono'}\n", encoding="utf-8")
    got, _ = run_resolution("ferrite", absolute, base / "other-base")
    failures += check("绝对 root: 原样使用（不受 base 影响）", got, str(target / "mono"))

    with environ(CANON_PROJECTS_BASE=None):
        real = canon_roots.mapped_root(str(HERE.parent), "ferrite")
        failures += check("真清单：ferrite → mono", real,
                          os.path.join(canon_roots.projects_base(), "mono"))
    return failures


def check_cli_resolution(base: Path) -> int:
    """三个入口同一套解析：跑真脚本，避免「共享模块对了、入口忘接线」。"""
    failures = 0
    root = base / "cli"
    (root / "mono").mkdir(parents=True)
    decoy = root / "ferrite"  # 按项目名拼出来的同名目录，入口不得用它
    (decoy / ".agent").mkdir(parents=True)
    env = {**os.environ, "CANON_PROJECTS_BASE": str(root)}
    env.pop("CANON_TARGET_ROOT", None)

    _, out = run_cli(["agents", "show", "ferrite"], env)
    failures += check("agents show 指到映射目录", str(root / "mono") in out, True)

    _, out = run_cli(["canon-sync", "status", "ferrite"], env)
    failures += check("canon-sync status 报出的目标根是映射目录",
                      f"目标根: {root / 'mono'}" in out, True)

    # 同名目录里放一份「与 canon HEAD 逐字节一致」的文档：真按名字解析的话这条会显示
    # in-sync，而正确解析（目标是空的 mono）只能是 missing。
    head = sh("git", "-C", str(HERE.parent), "rev-parse", "--short", "HEAD")
    pristine = sh("git", "-C", str(HERE.parent), "show", f"{head}:agent-pack/rules/agent-readme.md")
    (decoy / ".agent" / "README.md").write_text(
        f"<!-- canon: test @ {head} (synced 2020-01-01) -->\n{pristine}\n", encoding="utf-8")
    _, out = run_cli(["agent-sync", "status", "ferrite"], env)
    line = next((ln for ln in out.splitlines() if ln.endswith(".agent/README.md")), "")
    failures += check("agent-sync 不认同名目录里的副本（用映射目标）", "missing" in line, True)
    return failures


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

    failures += check_project_root(base)
    failures += check_cli_resolution(base)

    print("PASS" if failures == 0 else f"{failures} 组失败")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
