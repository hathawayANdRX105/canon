#!/usr/bin/env python3
"""css_token_guard.py — dioxus/css 语义 token 两支柱检查（l1 确定性，零 LLM）。

把 ferrite 两次样式收敛（PR #30 页面侧 + PR #45 组件侧）的验收契约泛化成
项目无关的 lint：

  mode=rsx     rsx class 字符串字面量只许语义类（ui-*/role-*/宿主注册类/
               data-* 标记/动态插值/文档化覆盖槽）——泛化 scripts/check-style-classes.mjs
               的「页面类必须已登记」契约，但不依赖项目自己的语料清单（那部分
               仍是项目侧构建守卫；本规则拦的是语法面上可判的裸 Tailwind 工具类）。
  mode=css     kit/assets 语义 css 的 @layer components 规则体禁止新增裸 Tailwind
               utility 行与裸色板值（#hex / 原始色板类），组合必须走 var(--*) token
               或语义类 @apply——泛化 style_contract.rs 契约 3 与 PR#30 的「css 是
               token 唯一真源」支柱。theme 镜像不变量由 css_mirror 规则另行覆盖。

用法（checklist yaml 的 harness）:
  python3 .githooks/spec/harness/css_token_guard.py --mode rsx
  python3 .githooks/spec/harness/css_token_guard.py --mode css

输出: findings JSON 数组（stdout），退出码恒 0（严重度由 yaml fail_severity 定）。

豁免口径（与 ferrite 两次收敛一致的文档化例外，命中时跳过）:
  - 动态插值: 字符串含 `{`（rsx format/信号插值）
  - data-* / aria-* / testid 属性值
  - 宿主基座类（poster-flip*/card-frame/card-tilt/… 等 kit 注册的物理类名——
    名单可经 --allow-file 传入一行一个；无名单时只拦明确的 Tailwind utility 语法）
  - 单字符/纯数字 token（w-0、gap-0 等边界值不误报——本规则只拦
用法（checklist yaml 的 harness）:
  python3 .githooks/spec/harness/css_token_guard.py --mode rsx --corpus-glob "crates/web/ui-kit/assets/*.css"
  python3 .githooks/spec/harness/css_token_guard.py --mode css

可移植性（自动安静）: --mode rsx 需要 --corpus-glob 命中至少一个 kit 语义 CSS 文件
  ——没有 kit 语料的仓（纯后端/其他前端栈）恒输出 []，门禁绿不代表查过（同
  css_mirror 规则的「静默=未验证」口径）。
"""

import argparse
import json
import os
import re
import subprocess
import sys

# Tailwind 已知 utility 语法（形式判定，非白名单）：前缀-值 / 纯值 / 任意值方括号。
# 刻意保守：只拦「确定是 Tailwind utility」的形态，宁可漏报不可误报（gate 下界原则）。
TAILWIND_TOKEN = re.compile(
    r"^(?:(?:hover|focus|focus-visible|active|disabled|group-hover|peer-(?:checked|hover)|"
    r"dark|sm|md|lg|xl|2xl|first|last|odd|even|visited|checked|selection|marker|file|"
    r"placeholder|before|after|data-\[[^\]]+\]|aria-\[[^\]]+\]):)*"
    r"(?:[a-z][a-z0-9-]*(?:\[[^\]]+\]|/\d{1,3})?|-\[[^\]]+\])$"
)
KNOWN_UTILITY_ROOTS = re.compile(
    r"^(container|sr-only|not-sr-only|flex|inline-flex|grid|inline-grid|block|inline|hidden|"
    r"table|contents|isolation|absolute|relative|fixed|sticky|static|inset|top|right|bottom|left|"
    r"z|order|col|row|float|clear|object|overflow|overscroll|truncate|basis|grow|shrink|"
    r"border|rounded|shadow|ring|opacity|mix-blend|bg|from|via|to|filter|blur|brightness|"
    r"contrast|grayscale|hue-rotate|invert|saturate|sepia|backdrop|transition|duration|ease|"
    r"delay|animate|scale|rotate|translate|skew|origin|accent|appearance|cursor|caret|"
    r"resize|scroll|snap|touch|select|will-change|fill|stroke|outline|resize|appearance|"
    r"pointer-events|visible|invisible|collapse|whitespace|break|text|font|tracking|leading|"
    r"list|placeholder|decoration|underline|uppercase|lowercase|capitalize|normal-case|"
    r"italic|not-italic|antialiased|tabular-nums|align|justify|items|content|self|place|"
    r"gap|space|divide|p|px|py|pt|pb|pl|pr|m|mx|my|mt|mb|ml|mr|w|h|min-w|min-h|max-w|max-h|"
    r"size|aspect|columns|aspect-ratio|leading|tracking|underline|no-underline|line-through)"
    r"(?:-|$)"
)
NON_UTILITY_OK = re.compile(
    r"^(ui-|role-|chat-|dm-|poster|card-|row-|scroll-|demo-|preview-|ts-|cov-|proj-|widgets-|"
    r"swatch|bubble-action|sidebar|dual-nav|group$|is-flipped$)"
)
STATE_ATTR = re.compile(r"^(data-|aria-|id$|for$|name$|type$|value$|href$|src$|alt$|style$|testid)")

RSX_CLASS_RE = re.compile(r'class:\s*"([^"]+)"')
CSS_CLASS_DEF_RE = re.compile(r"^\s*(\.[A-Za-z][\w-]*(?:\s*,\s*\.[A-Za-z][\w-]*)*)\s*\{")
HEX_COLOR_RE = re.compile(r"#[0-9a-fA-F]{3,8}\b")
PALETTE_CLASS_RE = re.compile(
    r"@apply[^;]*\b(?:bg|text|border|ring|from|via|to)-(?:zinc|slate|gray|red|orange|amber|"
    r"yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose)-\d{2,3}"
)
LAYER_OPEN_RE = re.compile(r"@layer\s+components\s*\{")


def git_ls_files(root: str, patterns: list[str]) -> list[str]:
    out: list[str] = []
    for pat in patterns:
        r = subprocess.run(["git", "ls-files", "--", pat], cwd=root, capture_output=True, text=True)
        out.extend(l for l in r.stdout.splitlines() if l.strip())
    return sorted(set(out))


def load_allowlist(root: str, path: str | None) -> set[str]:
    if not path:
        return set()
    p = os.path.join(root, path)
    if not os.path.isfile(p):
        return set()
    with open(p, encoding="utf-8") as f:
        return {line.strip() for line in f if line.strip() and not line.startswith("#")}


def is_exempt_token(tok: str, allow: set[str]) -> bool:
    if tok in allow:
        return True
    if NON_UTILITY_OK.match(tok):
        return True
    if "{" in tok or "}":  # 动态插值
        return True
    if not TAILWIND_TOKEN.match(tok):
        return True  # 不是 Tailwind 形态（自定义物理类等）→ 本规则不判
    root_m = TAILWIND_TOKEN.match(tok).group(0).split(":")[-1]
    root_m = root_m.split("/")[0].split("[")[0]
    return not KNOWN_UTILITY_ROOTS.match(root_m + "-")
def glob_has_hit(root: str, pattern: str) -> bool:
    return bool(git_ls_files(root, [pattern]))


def scan_rsx(root: str, findings: list[dict], allow: set[str]) -> None:
    files = git_ls_files(root, ["crates/web/**/*.rs", "apps/*/src/**/*.rs"])
    for rel in files:
        path = os.path.join(root, rel)
        try:
            src = open(path, encoding="utf-8").read()
        except (OSError, UnicodeDecodeError):
            continue
        for lineno, line in enumerate(src.splitlines(), 1):
            for m in RSX_CLASS_RE.finditer(line):
                for tok in m.group(1).split():
                    if STATE_ATTR.match(tok) or is_exempt_token(tok, allow):
                        continue
                    findings.append({
                        "id": "RSX-RAW-UTILITY",
                        "severity": "WARN",
                        "path": rel,
                        "line": lineno,
                        "message": f"rsx class 字面量引用裸 Tailwind utility「{tok}」：样式真身必须"
                                   f"住 kit 资产 CSS 的语义配方（ui-*），rsx 只发射语义类 + data-* 状态。"
                                   f"确认是文档化覆盖槽/动态插值误报时，加入 allowlist 文件并注明理由。",
                    })
def is_exempt_token(tok: str, allow: set[str]) -> bool:
    if tok in allow:
        return True
    if NON_UTILITY_OK.match(tok):
        return True
    if "{" in tok or "}" in tok:  # 动态插值
        return True
    if not TAILWIND_TOKEN.match(tok):
        return True  # 不是 Tailwind 形态（自定义物理类等）→ 本规则不判
    root_m = TAILWIND_TOKEN.match(tok).group(0).split(":")[-1]
    root_m = root_m.split("/")[0].split("[")[0]
    return not KNOWN_UTILITY_ROOTS.match(root_m + "-")
def scan_css(root: str, findings: list[dict]) -> None:
    files = [f for f in git_ls_files(root, ["crates/web/ui-kit/assets/*.css", "apps/*/assets/*.css"])
             if "tailwind.out" not in f and ".min." not in f]  # 生成产物不判
    for rel in files:
        path = os.path.join(root, rel)
        try:
            src = open(path, encoding="utf-8").read()
        except (OSError, UnicodeDecodeError):
            continue
        in_components = 0
        for lineno, line in enumerate(src.splitlines(), 1):
            stripped = line.strip()
            if LAYER_OPEN_RE.search(line):
                in_components += 1
                continue
            if in_components and stripped == "}":
                in_components -= 1
                continue
            if not in_components:
                continue
            body = stripped
            if body.startswith("/*") or body.startswith("*") or body.startswith("//"):
                continue
            if PALETTE_CLASS_RE.search(body):
                findings.append({
                    "id": "CSS-RAW-PALETTE-APPLY",
                    "severity": "WARN",
                    "path": rel,
                    "line": lineno,
                    "message": "components 层 @apply 引用原始色板类（zinc/emerald/…-NNN）："
                               "必须换语义 token（--success/--warning/--foreground …）或语义类。",
                })
            if HEX_COLOR_RE.search(body) and "--" not in body and "gradient" not in body:
                findings.append({
                    "id": "CSS-HARDCODED-HEX",
                    "severity": "WARN",
                    "path": rel,
                    "line": lineno,
                    "message": "components 层裸 #hex 色值：色值必须绑定 theme.css 语义 token"
                               "（var(--*)）；演示样张/第三方快照文件请移出 components 层。",
                })


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--mode", choices=["rsx", "css"], required=True)
    ap.add_argument("--corpus-glob", default=None,
                    help="仓根相对 glob：kit 语义 CSS 语料；无命中 = 本仓无该契约，恒输出 []")
    ap.add_argument("--allow-file", default=None,
                    help="仓根相对路径：每行一个豁免 token（# 注释），rsx 模式用")
    args = ap.parse_args()

    root = subprocess.run(["git", "rev-parse", "--show-toplevel"],
                          capture_output=True, text=True).stdout.strip()
    if not root:
        print("[]")
        return 0

    if args.corpus_glob and not glob_has_hit(root, args.corpus_glob):
        print("[]")  # 无 kit 语料 → 可移植静默（静默=未验证，与 css_mirror 同口径）
        return 0

    findings: list[dict] = []
    allow = load_allowlist(root, args.allow_file) if args.mode == "rsx" else set()
    if args.mode == "rsx":
        scan_rsx(root, findings, allow)
    else:
        scan_css(root, findings)

    print(json.dumps(findings, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    sys.exit(main())
