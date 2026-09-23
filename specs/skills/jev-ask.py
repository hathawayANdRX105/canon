#!/usr/bin/env python3
"""jev-ask — 调 TypeSafe System One (jev) 做结构化判断。零依赖，stdlib only。

用法:
  jev-ask.py questions.json state.txt      # state 为文件（文本或 JSON）
  cat diff.patch | jev-ask.py questions.json   # state 从 stdin
  jev-ask.py --check                       # 只验连通与鉴权（发一个 1-题 ping）

环境变量:
  TYPESAFE_API_KEY    必填。官方 key 或网关 key
  TYPESAFE_API_BASE   可选。默认 https://api.typesafe.ai；走网关填网关地址
  JEV_MODEL           可选。默认 jev-latest

questions.json 形如（type ∈ noul | choice | score）:
  {
    "has_secret": {"type": "noul",
                   "instructions": "Does the diff add a hardcoded secret?",
                   "criteria": {"true": "credential-like literal added",
                                 "false": "none added"}},
    "severity":   {"type": "score", "instructions": "...",
                   "criteria": ["info", "warn", "critical"]}
  }

输出: answers JSON（stdout），错误信息走 stderr 并以非零码退出。
"""
import json
import os
import sys
import urllib.error
import urllib.request

TIMEOUT = 60


def die(msg: str) -> None:
    print(f"jev-ask: {msg}", file=sys.stderr)
    sys.exit(1)


def main() -> int:
    base = os.environ.get("TYPESAFE_API_BASE", "https://api.typesafe.ai").rstrip("/")
    key = os.environ.get("TYPESAFE_API_KEY")
    if not key:
        die("TYPESAFE_API_KEY 未设置（fish: set -gx TYPESAFE_API_KEY sk-...; 重开 shell）")
    model = os.environ.get("JEV_MODEL", "jev-latest")
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {key}",  # 官方 api.typesafe.ai 认这个
        "x-api-key": key,                  # 中转网关（knox.chat 等）认这个；双发自动兼容
    }

    if len(sys.argv) == 2 and sys.argv[1] == "--check":
        body = {"model": model, "state": "ok",
                "questions": {"ping": {"type": "noul", "instructions": "Is the state non-empty?"}}}
    elif len(sys.argv) >= 2:
        try:
            with open(sys.argv[1]) as f:
                questions = json.load(f)
        except (OSError, json.JSONDecodeError) as e:
            die(f"问题文件读不了: {e}")
        if len(sys.argv) >= 3:
            with open(sys.argv[2]) as f:
                raw = f.read()
        elif not sys.stdin.isatty():
            raw = sys.stdin.read()
        else:
            die("缺 state：给文件参数或从 stdin 管道进来")
        try:
            state = json.loads(raw)  # JSON 对象/数组原样发，普通文本按字符串发
        except json.JSONDecodeError:
            state = raw
        if not str(state)[:1]:
            die("state 为空")
        body = {"model": model, "state": state, "questions": questions}
    else:
        die(__doc__)

    req = urllib.request.Request(f"{base}/v1/systemone",
                                 data=json.dumps(body).encode(), headers=headers)
    try:
        with urllib.request.urlopen(req, timeout=TIMEOUT) as resp:
            payload = json.loads(resp.read().decode())
    except urllib.error.HTTPError as e:
        detail = e.read().decode(errors="replace")[:500]
        die(f"HTTP {e.code}: {detail}")
    except Exception as e:  # noqa: BLE001 — 网络层错误统一透出
        die(f"请求失败: {e}")

    if len(sys.argv) == 2 and sys.argv[1] == "--check":
        print(f"✓ {payload.get('model', model)} 连通正常")
        return 0
    print(json.dumps(payload.get("answers", {}), ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
