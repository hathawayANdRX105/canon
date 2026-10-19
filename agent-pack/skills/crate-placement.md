---
name: crate-placement
description: "Author and use per-crate cratebooks before and after a code change. Use when planning a change that touches crates/** or bin/**, when deciding which crate or module should own a piece of code, or when a post-change diff must be checked against the crate placement spec. The spec is TOML, lives next to Cargo.toml, and is checked by cargo xtask-crate and the canon crate-placement rule."
license: MIT
---

# Crate placement

A placement contract answers two questions:

1. What does this crate own, and what must never be implemented here?
2. What is each file or module inside the crate responsible for?

The contract is machine-readable TOML. It is not a design document and not a test plan. Do not put `cargo check`, `cargo test`, or any other verification command in it; those belong to the build and CI layers.

## Spec location

Default: `crates/<package>/cratebook.toml`, next to `Cargo.toml`. The package name in the cratebook must match the cargo package name.

## Schema

```toml
schema = 1
crate = "relay"

responsibility = "Data-plane assembly library: gate chain, forwarding stage, pipeline router, egress proxy pool, and loopback serve."

belongs = [
  "Assembly order and wiring",
  "Gate chain and forwarding stage",
  "Egress proxy pool installation and probe policy",
  "Loopback serve construction",
]

[[does_not_belong]]
concern = "Directory snapshot loading"
to = "api-gateway-state"

[[does_not_belong]]
concern = "Options knob reading"
to = "api-observe"

# Optional. Defaults to ["src/**", "tests/**"]. An empty list disables
# the file-level responsibility check.
files_scope = ["src/**", "tests/**"]

# Ordered. The first matching entry wins. Put specific globs before broad ones.
[[files]]
path = "src/lib.rs"
responsibility = "Public surface: assemble(), serve(), and re-exports."
entry = true

[[files]]
path = "src/proxy/**"
responsibility = "Egress proxy pool installation, health probing, and policy. No database access."

[paths]
allow = ["src/**", "tests/**", "Cargo.toml", "cratebook.toml"]
deny = []
```

Rules:

- `responsibility` is one sentence. `belongs` is a short list, not a paragraph.
- `does_not_belong` is the highest-value section. Each entry names the concern and the crate or path that owns it.
- `files_scope` limits which changed files need a `[[files]]` entry.
- `[[files]]` is ordered and first-match-wins. `entry = true` marks the file a reviewer should open first; exactly one entry may set it, and its path must be exact (no glob).
- `paths.allow` and `paths.deny` are relative to the crate root. `deny` wins over `allow`.
- All values are English. CJK characters are reported by `lint-spec`.
- Keep the file short. If it grows beyond roughly 20 lines, the boundary is probably wrong or the detail belongs in `src/lib.rs` (`//!`) or an ADR.

## Before implementation

1. List the files you intend to add or change.
2. Run:

   ```bash
   cargo xtask-crate plan \
     --file crates/relay/src/assemble.rs \
     --file crates/contract/src/api/relay.rs
   ```

3. The command prints each crate's responsibility, its `does_not_belong` entries, the matching `[[files]]` rule, and any path violation.
4. Put the result into the implementation plan as a table:

   | Planned file | Crate | File responsibility | Verdict |
   |---|---|---|---|
   | `crates/relay/src/assemble.rs` | `relay` | `src/assemble.rs`: deterministic wiring | OK |
   | `crates/relay/src/db_loader.rs` | `relay` | no match; belongs to `api-control-plane` | MOVE |

5. If the verdict is `MOVE`, change the plan before writing code. Do not implement in the wrong crate and move it later.

## After implementation

1. Run:

   ```bash
   cargo xtask-crate verify --base origin/main
   ```

   The command reads `git diff` plus untracked files, maps each path to a crate with `cargo metadata`, and prints the matching spec and diagnostics.

2. Read every diagnostic. Fix the code or move the file. If a file genuinely belongs in the crate but has no responsibility entry, add a precise `[[files]]` entry.
3. Only change the spec when the boundary decision changed. Never edit the spec just to silence a finding.
4. In the pull request, include the placement table and any spec change with a one-line reason.

## Maintenance

- One real misplacement adds one `does_not_belong` entry or one `[[files]]` entry. Do not write a long explanation.
- A repeated misplacement becomes a `paths.deny` rule or a `check-crate` rule, not another paragraph.
- If the spec and the code disagree, decide which one is wrong first. The spec is the intended boundary; the code is the current state.
