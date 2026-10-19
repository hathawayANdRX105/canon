#!/usr/bin/env python3
"""项目安装根（`root:`）解析的唯一实现 —— agents.yaml / agent-sync.yaml / canon-sync 共用。

canon 的约定（见 AGENTS.md）：两份清单的 `root:` 与 canon-sync 用**同一套解析规则**，
三处保持一致。规则只应有一份实现：此前 `scripts/agents` 与 `scripts/agent-sync`
各抄一份，`scripts/canon-sync` 却根本没读映射、只按项目名拼
`CANON_PROJECTS_BASE/<项目>` —— `ferrite: root: mono` 在这条路径上无人读，种子于是
被写进同名的陈旧检出。

规则：
  - 绝对路径原样用；`~/x` 展开 home；其余相对路径落到 CANON_PROJECTS_BASE
    （缺省 `~/projects`）下；
  - 没写 `root:` 就报错，绝不拿项目名猜目录；
  - 两份清单都声明了同一项目而解析结果不同 → 报错：不一致就是「改一处漏另一处」。

改映射只改清单里的 `root:` 值；本模块不认任何具体项目名。
"""

import os
import sys
from typing import NamedTuple

import yaml

MANIFESTS = ("agent-sync.yaml", "agents.yaml")


class Declaration(NamedTuple):
    """一份清单对某项目 `root:` 的声明。"""

    manifest: str
    value: str


def die(msg: str) -> None:
    print(f"✗ {msg}", file=sys.stderr)
    sys.exit(1)


def projects_base() -> str:
    """相对 `root:` 的基准目录；CANON_PROJECTS_BASE 覆盖，缺省 ~/projects。"""
    return os.environ.get("CANON_PROJECTS_BASE") or os.path.join(
        os.path.expanduser("~"), "projects"
    )


def absolute_path(value) -> str:
    """`root:` 值 → 绝对路径。不查存在性：门禁与 CI 里目标仓常常不在本机。"""
    path = os.path.expanduser(str(value).strip())
    if not os.path.isabs(path):
        path = os.path.join(projects_base(), path)
    return os.path.normpath(path)


def load_projects(canon: str, manifest: str) -> dict:
    """读清单的 `projects:` 段；清单不存在当空（另一份可能才是该项目的正本）。"""
    path = os.path.join(canon, manifest)
    if not os.path.isfile(path):
        return {}
    with open(path) as f:
        try:
            data = yaml.safe_load(f) or {}
        except yaml.YAMLError as e:
            die(f"{path} 解析失败: {e}")
    projects = data.get("projects") or {}
    return projects if isinstance(projects, dict) else {}


def root_declarations(canon: str, project: str) -> list:
    """两份清单里该项目 `root:` 的全部声明，顺序同 MANIFESTS。"""
    out = []
    for manifest in MANIFESTS:
        spec = load_projects(canon, manifest).get(project)
        if not isinstance(spec, dict):
            continue
        value = spec.get("root")
        if value is None or not str(value).strip():
            continue
        out.append(Declaration(manifest, str(value).strip()))
    return out


def describe(canon: str, project: str) -> str:
    """映射来源摘要：报错与 status 都靠它指路「这个目录是谁说的」。"""
    decls = root_declarations(canon, project)
    if not decls:
        return "两份清单都没写该项目的 root:"
    return "；".join(f"{d.manifest} root: {d.value}" for d in decls)


def _reject_disagreement(project: str, decls: list) -> None:
    """两份清单对同一项目解析出不同目录 = 改一处漏了一处；报错，不挑一个用。"""
    if len({absolute_path(d.value) for d in decls}) < 2:
        return
    detail = "；".join(f"{d.manifest} root: {d.value} → {absolute_path(d.value)}"
                       for d in decls)
    die(f"{project} 的 root 映射不一致：{detail}\n"
        f"  两份清单对同一项目必须解析到同一目录，这里不替你挑一个用；"
        f"改一处不要漏另一处。")


def resolve_root(value, project: str, source: str, canon: str = "") -> str:
    """清单里的一个 `root:` 值 → 绝对路径。

    source 为该值所在清单名（诊断用）。canon 给定时顺带与另一份清单交叉校验：
    两份清单解析结果不同即报错，而不是让两个工具各自静默用自己那份。
    """
    if value is None or not str(value).strip():
        die(f"{source} 的 {project} 未配置 root")
    if canon:
        decls = root_declarations(canon, project)
        if not any(d.manifest == source for d in decls):
            decls.append(Declaration(source, str(value).strip()))
        _reject_disagreement(project, decls)
    return absolute_path(value)


def mapped_root(canon: str, project: str) -> str:
    """canon-sync 用：只认清单里的 `root:` 映射，绝不按项目名猜目录。"""
    decls = root_declarations(canon, project)
    if not decls:
        die(f"{project} 没有安装根映射：{'、'.join(MANIFESTS)} 都没写该项目的 root:。\n"
            f"  canon 不按项目名猜目录 —— 那正是种子被写进同名陈旧检出的原因。\n"
            f"  给两份清单补上 root:，或用 CANON_PROJECTS_BASE 指向正确的 checkout 根。")
    _reject_disagreement(project, decls)
    return absolute_path(decls[0].value)
