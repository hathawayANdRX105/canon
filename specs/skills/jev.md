---
name: jev
license: MIT
description: >
  Classify, rank, verify, and decide with Jev — TypeSafe's System One decision
  model — through the omp eval kernel's judge() / judge_batch(). Jev turns
  natural language plus state into typed answers with probabilities that code
  can act on. Use when the user says jevify, when ≥ ~20 homogeneous items need
  bucketing / yes-no / scoring (diffs, log lines, test names, search hits,
  review findings, checklist rows), or when a recommendation should be
  calibrated by a decision model before it reaches the engineer. This skill
  covers the rubric-first bulk discipline, the escalation rule, and the
  disposition contract that turns verdicts into action. Outside the omp
  kernel, the bundled zero-dependency jev-ask.py talks to the same API.
---

# Classify and decide with Jev

Jev answers narrow questions about a piece of **state** and returns typed
answers with probabilities. Your code (or agent workflow) owns the data and the
consequences; Jev supplies the judgment. In omp, the eval kernel exposes
`judge(state, questions)`; outside omp, `jev-ask.py` (same directory) calls the
HTTP API directly.

The live TypeSafe docs are the source of truth for semantics. A local clone of
the official skill lives at `canon/todo/ref/typesafe-skills`.

## Read the docs for the task at hand

| Task | Start here |
| --- | --- |
| Programming model | https://docs.typesafe.ai/concepts/system-one.md |
| Question primitives | https://docs.typesafe.ai/primitives.md |
| Uncertainty semantics | https://docs.typesafe.ai/confidence.md |
| State design | https://docs.typesafe.ai/concepts/state.md |
| HTTP API (for jev-ask.py) | https://docs.typesafe.ai/api.md |

Mintlify serves any page as Markdown by appending `.md`.

## Pick the primitive

| Kernel type | Official name | Returns | Use for |
| --- | --- | --- | --- |
| `choice` | Choice | `{choice, probabilities, confidence}` | One of a defined set; the distribution compares competing options |
| `bool` | Noul | `{bool: P(yes)}` | A yes/no condition. **≈0.5 means "yes and no are equally likely", not "medium degree"** — for degree, use `score` |
| `score` | Score | `{score, probabilities, confidence}` | Position on an ordered spectrum; levels must describe concrete situations that stand on their own ("low/medium/high" alone is not a level) |

One narrow, coherent judgment per question. Split independently useful
dimensions into separate questions — they run in parallel and cannot see each
other's answers. Ask independent questions over the same state **together in
one call**; a second call is warranted only when a later question needs an
earlier answer to fetch evidence or build new state.

Design rules (from the official docs, verified against our usage):

- **Give enough state.** Source text, identities, relationships, policies,
  current facts. Prefer named JSON fields when the state has several parts.
  A judge with thin state guesses; feed it like you would a new teammate.
- **Labels must be exhaustive.** The model cannot choose an omitted value.
  Include an explicit catch-all ("unrelated" / "other") and a "mixed" label
  when a unit can straddle.
- **Every criterion label is one sentence of observable evidence** — what a
  reader would check — not a verdict word ("bad", "suspicious").
- Put the judgment in `instructions`; define the answer space in `criteria`.
  Question ids are for your code only; carry the full meaning in the question.

## Bulk discipline: rubric first, judge the bulk, read only flags

For any list of ≥ ~20 homogeneous items with a bucket / yes-no / score
question. Under ~20 items, or when the question needs cross-item reasoning,
just read them.

1. **Decide** (one eval cell, before any data is loaded). Freeze as constants:
   the **unit** (what one `state` is — prefer the smallest unit that still
   carries enough context), the **questions** (independent, fixed ids),
   a deterministic **pre-filter** (path prefix, extension, size — log how many
   units it removes), an **escalation rule** (which verdicts and which
   uncertainty bands you will read yourself, e.g. top probability < 0.7), and
   a **cap** (max state size; truncate with a visible marker and count).
2. **Partition** — load every unit, apply the pre-filter, key by stable id.
3. **Judge** — one `judge()` per unit with all questions in that call, all
   handles fired in one cell. In omp: `judge_batch(states, questions,
   concurrency=N)` for large runs; `wait(handles, raise_errors=False)` keeps
   failures in their slot instead of raising.
4. **Escalate** — read only the units your escalation rule flags. Confirm or
   overturn each against the code (implementation contract, callers) rather
   than re-judging.
5. **Report** — counts per label, pre-filter removals, truncations, then
   flagged items with path + one-line evidence each.

<critical>
- NEVER read bulk items before the rubric is frozen. Rubric first, data second.
- NEVER hand-scan the bulk or delegate scanning to subagents. Judge classifies; you read only flagged items.
- Rubric changes mid-run invalidate every prior verdict: re-judge everything.
</critical>

## Interpret the answers (official semantics — do not improvise)

- `confidence` is **distribution concentration**, not overall correctness and
  not permission to act. Several acceptable alternatives can spread
  probability; low confidence need not invalidate a harmless preference
  choice. Set thresholds from the consequence of being wrong, evaluated on
  your data — not from the number alone.
- A Noul near 0.5 means similar probability for yes and no, not medium
  intensity.
- **Typed output guarantees the interface, not the truth.** Jev is trained for
  calibrated decisions, but validate performance in your target domain before
  thresholds drive automation. Cookbook thresholds are examples, not rules.
- Keep policy explicit and raw judgments reusable: weighted scores suit
  compensating preferences; an "any serious violation" rule needs separate
  conditions per violation.

## Turn verdicts into action (disposition contract)

In this workflow, jev findings feed gates and reviews. A verdict is evidence,
not a final ruling — escalate, then act:

- **FAIL (p ≥ fail threshold): blocks.** Fix, or split the change; it cannot ship as-is.
- **WARN: must be dispositioned.** Fix (default) or **reject in writing** —
  one-line reason + evidence, recorded in the PR review record (`.workflow/reviews/`
  artifact or the PR template's review section). Silence is a violation.
- **INFO: may be batched into a backlog.**
- Closing a task with undispositioned WARN/FAIL findings is a violation;
  closeout checks the disposition record.

## Example: bulk diff classification

```python
SHA = "abc123"
SUBJECT = "refactor: new tui framework"
QUESTIONS = {
    "verdict": {"type": "choice",
        "instructions": f"One file diff from commit '{SUBJECT}'. Does this change belong to that refactor?",
        "criteria": {
            "belongs": "Every hunk is required by or mechanically follows from the stated refactor.",
            "mixed": "Mostly the refactor, plus at least one hunk changing unrelated behavior.",
            "unrelated": "No hunk relates to the stated refactor.",
        }},
    "logic": {"type": "bool", "instructions": "Does any hunk change runtime behavior outside the refactor's subsystem (not renames/imports/types)?"},
}
CAP = 24_000
def prefilter(path): return path.startswith("packages/tui/") or path.endswith(".tsx")

import subprocess
def git(*args): return subprocess.run(["git", *args], capture_output=True, text=True, check=True).stdout
files = [f for f in git("show", "--name-only", "--format=", SHA).split() if not prefilter(f)]
diffs = {f: git("show", "--format=", SHA, "--", f) for f in files}
handles = {f: judge({"file": f, "subject": SUBJECT, "diff": d[:CAP] + ("\n…[truncated]" if len(d) > CAP else "")}, QUESTIONS) for f, d in diffs.items()}
results = wait(list(handles.values()), raise_errors=False)
rows = [(f, r) for f, r in zip(handles, results)]
flag = [f for f, r in rows if isinstance(r, Exception) or r["verdict"]["choice"] != "belongs" or r["verdict"]["probabilities"]["belongs"] < 0.7 or r["logic"]["bool"] >= 0.5]
```

Then print only `diffs[f]` for `f in flag`, confirm each against the code, and report.

## Example: calibrate a recommendation before asking the engineer

When a workflow (e.g. `/wf-architect`) must mark one option `(recommended)`,
run the judgment first and put the number in the panel:

```python
STATE = {
    "constraints": "single maintainer; nightly batch; existing Rust/tokio stack",
    "prior_decisions": ["storage: SQLite (WAL)", "auth: platform-managed"],
    "options": {
        "cron_in_process": "tokio cron inside the daemon; no new process; dies with daemon",
        "systemd_timer": "systemd units; survives daemon restarts; needs install step",
    },
}
Q = {"type": "choice",
     "instructions": "Given the constraints and prior decisions, which scheduling approach should we recommend?",
     "criteria": {
         "cron_in_process": "Simplicity outweighs restart-survival for this deployment shape.",
         "systemd_timer": "Restart-survival and ops visibility outweigh the install cost.",
     }}
r = judge(STATE, Q).wait()
# Panel: "(recommended) systemd_timer — decision model p=0.72; runner-up cron_in_process p=0.21"
# p < 0.6 → say so in the panel instead of pushing a pick.
```

Pure product judgments only the engineer knows (target users, monetization)
are not judgeable — ask those directly.

## Outside the kernel: jev-ask.py

`jev-ask.py` (this directory) is a zero-dependency HTTP client for the same
API — use it in scripts, hooks, or non-omp environments. Keep API credentials
server-side in web apps; never commit keys.

## Anti-patterns

- Reading the first N items "to get a feel" before writing the rubric.
- One question per `judge()` call when several independent questions share a state.
- Fanning items to `task` subagents to "review" them: they read; the judge classifies.
- Treating a judge verdict as final without reading the flagged unit.
- Dropping errored or truncated items silently instead of counting and escalating them.
- Vague labels ("bad", "suspicious") instead of one-sentence observable evidence.
- Reading a low `confidence` as "medium intensity" or as a ban on acting.
