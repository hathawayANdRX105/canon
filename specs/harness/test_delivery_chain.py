#!/usr/bin/env python3
"""delivery_chain.py 的判定语义回归测试（unittest + mock，不碰网络/gh）。

样张挑的是**会决定这项检查存废的形态**，不是装饰性断言：

  - claims 抽取必须吃得下中英混排 bullet / 编号列表 / 一句话驳回段落，且段尾
    是下一个标题就停（`How to test` 里的字眼不能被当成声明）；
  - jev 的 noul 方向是 true=「证据不支持该声明」(HIGH=BAD)，p 过阈值才出
    finding —— 低 p 必须**无** finding（钉住它不是空转的噪声发生器）；
  - 基础设施失败（网络挂 / 无 PR body / 无 Delivery record）一律只降成单条
    INFO，永不阻断（DWJ 契约），也不抛异常；
  - 阈值真从 jev_questions_delivery.json 取（0.85/0.6；evidence_sufficient
    0.9/0.6），不是代码里写死的数。

进程形态：真进程 import delivery_chain 跑 main()，只用 fake urllib 换掉
urlopen —— 既钉住进程入口（stdio 协议），又彻底隔离网络与 gh。
"""

from __future__ import annotations

import importlib.util
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).parent
HARNESS = HERE / "delivery_chain.py"
MAX_STATE_CHARS = 24000

SAMPLE_BODY = """\
# What

body text.

## Delivery record

- 修复了 X 崩溃 (crash on exit)
1. 补了回归测试 test_alpha
   驳回: 该 finding 是误报，保留原写法

## How to test

cargo test -p alpha
"""

# 换掉 urlopen：把请求体落盘（供断言 state/questions），回放录好的 noul 概率。
PRELUDE = """\
import json, os, urllib.request


def fake_urlopen(req, timeout=None):
    with open(os.environ["DMC_CAPTURE"], "a", encoding="utf-8") as fh:
        fh.write(req.data.decode() + "\\n")
    if os.environ.get("DMC_NET_FAIL"):
        raise TimeoutError("simulated upstream reset")
    prob = json.loads(os.environ["DMC_PROB"])
    payload = {"answers": {q: {"type": "noul", "noul": p} for q, p in prob.items()}}

    class _Resp:
        def __enter__(self):
            return self

        def __exit__(self, *exc):
            return False

        def read(self):
            return json.dumps(payload).encode()

    return _Resp()


urllib.request.urlopen = fake_urlopen
"""

RUNNER = """\
import sys, os
sys.path.insert(0, {tmp!r})
import fakelocal  # noqa: F401
sys.path.insert(0, {workdir!r})
sys.stdin = open(os.environ["DMC_DIFF"], encoding="utf-8")
sys.argv = ["delivery_chain.py"]
import delivery_chain
sys.exit(delivery_chain.main())
"""


class DeliveryChainTest(unittest.TestCase):
    # ---------- 纯函数层：claims 抽取 ----------

    @classmethod
    def setUpClass(cls):
        spec = importlib.util.spec_from_file_location("delivery_chain", str(HARNESS))
        cls.mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.mod)

    def test_extracts_bullets_numbered_and_paragraph(self):
        self.assertEqual(
            self.mod.extract_claims(SAMPLE_BODY),
            [
                "修复了 X 崩溃 (crash on exit)",
                "补了回归测试 test_alpha",
                "驳回: 该 finding 是误报，保留原写法",
            ],
        )

    def test_section_stops_at_next_heading(self):
        self.assertNotIn("cargo test -p alpha", self.mod.extract_claims(SAMPLE_BODY))

    def test_no_delivery_section(self):
        self.assertEqual(self.mod.extract_claims("# What\nnothing here\n"), [])

    def test_twelve_cap_is_explicit_not_silent(self):
        body = "\n".join(["## Delivery record"] + [f"- 声明 {i}" for i in range(15)])
        self.assertEqual(len(self.mod.extract_claims(body)), 15)  # 抽取不截断
        out, _ = self.run_harness(body, {f"claim_{i}": 0.1 for i in range(15)})
        ids = [f["id"] for f in out]
        self.assertIn("DELIVERY-CLAIMS-TRUNCATED", ids)
        note = next(f for f in out if f["id"] == "DELIVERY-CLAIMS-TRUNCATED")
        self.assertEqual(note["severity"], "INFO")
        self.assertIn("15", note["message"])
        self.assertIn("12", note["message"])

    # ---------- 进程层：stdio 协议 + 判定 ----------

    def run_harness(self, body: str, prob: dict, diff: str = "diff --git a/x b/x\n+x",
                    net_fail: bool = False) -> tuple:
        with tempfile.TemporaryDirectory(prefix="dmc-") as tmp:
            def put(name, content):
                p = os.path.join(tmp, name)
                Path(p).write_text(content, encoding="utf-8")
                return p

            put("fakelocal.py", PRELUDE)
            env = dict(os.environ)
            env.pop("TYPESAFE_API_BASE", None)
            env.pop("JEV_MODEL", None)
            env.update({
                "TYPESAFE_API_KEY": "test-key",
                "GATE_PR_BODY_FILE": put("body.md", body),
                "DMC_DIFF": put("diff.txt", diff),
                "DMC_CAPTURE": os.path.join(tmp, "capture.jsonl"),
                "DMC_PROB": json.dumps(prob),
            })
            if net_fail:
                env["DMC_NET_FAIL"] = "1"
            code = RUNNER.format(tmp=tmp, workdir=str(HERE))
            p = subprocess.run([sys.executable, "-c", code],
                               capture_output=True, text=True, env=env, cwd=str(HERE),
                               check=False)
            self.assertEqual(p.returncode, 0, f"rc/stderr:\n{p.stderr}")
            cap_path = Path(env["DMC_CAPTURE"])
            capture = cap_path.read_text(encoding="utf-8").splitlines() if cap_path.exists() else []
            return json.loads(p.stdout), (json.loads(capture[-1]) if capture else None)

    def test_no_pr_body_single_info(self):
        out, req = self.run_harness("", {})
        self.assertEqual([(f["id"], f["severity"]) for f in out], [("DELIVERY-CLAIMS", "INFO")])
        self.assertIn("无 PR 上下文", out[0]["message"])
        self.assertIsNone(req)  # 没 PR 上下文就不该白发一次 jev 调用

    def test_no_delivery_section_single_info(self):
        out, req = self.run_harness("# What\n只有正文\n", {})
        self.assertEqual([(f["id"], f["severity"]) for f in out], [("DELIVERY-CLAIMS", "INFO")])
        self.assertIsNone(req)

    def test_single_claim_is_not_verifiable(self):
        body = "## Delivery record\n\n- 只有一条声明\n"
        out, _ = self.run_harness(body, {"claim_0": 0.9})
        self.assertEqual([(f["id"], f["severity"]) for f in out], [("DELIVERY-CLAIMS", "INFO")])

    def test_high_p_claim_is_fail_with_extras(self):
        out, req = self.run_harness(
            SAMPLE_BODY, {"claim_0": 0.9, "claim_1": 0.7, "claim_2": 0.2, "evidence_sufficient": 0.1})
        self.assertEqual([(f["id"], f["severity"]) for f in out],
                         [("DELIVERY-CLAIM-0", "FAIL"), ("DELIVERY-CLAIM-1", "WARN")])
        f0 = out[0]
        self.assertEqual(f0["tier"], "jev")
        self.assertEqual(f0["confidence"], "0.90")
        self.assertEqual(f0["claim"], "修复了 X 崩溃 (crash on exit)")
        self.assertEqual((f0["path"], f0["line"]), ("", 0))
        self.assertIn("声明未被证据支持", f0["message"])
        self.assertIn("修复了 X 崩溃", f0["message"])
        self.assertIn("0.90", f0["message"])
        # 低 p 的 claim_2 不出 finding（否则这条检查就是噪声发生器）
        self.assertNotIn("DELIVERY-CLAIM-2", [f["id"] for f in out])
        # state/题目都要真的带上这条声明与 diff 证据
        state = json.loads(req["state"])
        self.assertEqual(state["claims"][0], "修复了 X 崩溃 (crash on exit)")
        self.assertIn("+x", state["diff_evidence"])
        self.assertEqual(req["questions"]["claim_0"]["fail"], 0.85)
        self.assertEqual(req["questions"]["claim_0"]["warn"], 0.6)
        self.assertIn("evidence", req["questions"]["claim_0"]["instructions"])

    def test_low_p_produces_no_findings(self):
        out, _ = self.run_harness(
            SAMPLE_BODY, {"claim_0": 0.2, "claim_1": 0.2, "claim_2": 0.2, "evidence_sufficient": 0.1})
        self.assertEqual(out, [])

    def test_evidence_sufficient_high_p_finding(self):
        out, req = self.run_harness(
            SAMPLE_BODY, {"claim_0": 0.1, "claim_1": 0.1, "claim_2": 0.1, "evidence_sufficient": 0.95})
        self.assertEqual([(f["id"], f["severity"]) for f in out], [("DELIVERY-EVIDENCE", "FAIL")])
        self.assertEqual(out[0]["tier"], "jev")
        self.assertEqual(out[0]["confidence"], "0.95")
        self.assertIn("证据不足", out[0]["message"])
        self.assertEqual(req["questions"]["evidence_sufficient"]["fail"], 0.9)
        self.assertEqual(req["questions"]["evidence_sufficient"]["warn"], 0.6)

    def test_diff_evidence_keeps_tail(self):
        diff = "older hunk\n" + ("filler\n" * 6000) + "newest hunk\n"
        out, req = self.run_harness(
            SAMPLE_BODY, {"claim_0": 0.1, "claim_1": 0.1, "claim_2": 0.1}, diff=diff)
        self.assertEqual(out, [])
        evidence = json.loads(req["state"])["diff_evidence"]
        self.assertLessEqual(len(evidence), MAX_STATE_CHARS)
        self.assertTrue(evidence.endswith("newest hunk\n"))

    def test_network_error_degrades_to_single_info(self):
        out, _ = self.run_harness(SAMPLE_BODY, {}, net_fail=True)
        self.assertEqual([(f["id"], f["severity"]) for f in out], [("DELIVERY-CLAIMS", "INFO")])
        self.assertIn("jev 不可用", out[0]["message"])
        self.assertIn("降级", out[0]["message"])


if __name__ == "__main__":
    unittest.main(verbosity=2)
