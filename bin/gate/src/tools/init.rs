//! gate init — install / uninstall the gate binary and configure git hooks.

use std::fs;
use std::path::{Path, PathBuf};

use super::git;

/// Hook scripts try PATH first, then the repo-local `.githooks/gate` copy —
/// fresh clones / CI machines may not have gate installed system-wide.
const HOOK_PRE_COMMIT: &str = "\
#!/usr/bin/env bash
# gate-managed hook — delegates to the gate binary
# 找二进制: 先 PATH 里的 gate (系统安装), 否则用仓库内 .githooks/gate
REPO=$(git rev-parse --show-toplevel 2>/dev/null)
if command -v gate >/dev/null 2>&1; then
  exec gate pre-commit
fi
if [ -x \"$REPO/.githooks/gate\" ]; then
  exec \"$REPO/.githooks/gate\" pre-commit
fi
echo \"gate binary not found (need: gate on PATH, or .githooks/gate)\" >&2
exit 1
";

const HOOK_PRE_PUSH: &str = "\
#!/usr/bin/env bash
# gate-managed hook — delegates to the gate binary
# 找二进制: 先 PATH 里的 gate (系统安装), 否则用仓库内 .githooks/gate
REPO=$(git rev-parse --show-toplevel 2>/dev/null)
if command -v gate >/dev/null 2>&1; then
  exec gate pre-push
fi
if [ -x \"$REPO/.githooks/gate\" ]; then
  exec \"$REPO/.githooks/gate\" pre-push
fi
echo \"gate binary not found (need: gate on PATH, or .githooks/gate)\" >&2
exit 1
";

const HOOK_MERGE: &str = "\
#!/usr/bin/env bash
# gate-managed hook — delegates to the gate binary
# 找二进制: 先 PATH 里的 gate (系统安装), 否则用仓库内 .githooks/gate
REPO=$(git rev-parse --show-toplevel 2>/dev/null)
if command -v gate >/dev/null 2>&1; then
  exec gate merge
fi
if [ -x \"$REPO/.githooks/gate\" ]; then
  exec \"$REPO/.githooks/gate\" merge
fi
echo \"gate binary not found (need: gate on PATH, or .githooks/gate)\" >&2
exit 1
";

/// `gate init` — copy current binary to `~/.local/bin/gate` (+`gh`), configure
/// `core.hooksPath`, and write hook templates to `.githooks/hooks/`.
pub fn install(rules_dir: Option<&Path>) -> anyhow::Result<()> {
    let home = std::env::var("HOME").map_err(|_| anyhow::anyhow!("HOME not set"))?;
    let install_dir = PathBuf::from(&home).join(".local").join("bin");
    let gate_target = install_dir.join("gate");
    let gh_target = install_dir.join("gh");

    fs::create_dir_all(&install_dir)?;

    let current_exe = std::env::current_exe()?;
    for target in [&gate_target, &gh_target] {
        // current_exe() canonicalizes symlinks; compare against canonical target
        // so a symlinked HOME/.local/bin doesn't mismatch and self-truncate.
        let already_installed = match fs::canonicalize(target) {
            Ok(t) => t == current_exe,
            Err(_) => false,
        };
        if !already_installed {
            // Broken symlink or non-canonicalizable target: fs::copy would follow
            // the dangling link and leave it behind — remove it first so the real
            // binary replaces the link.
            if target
                .symlink_metadata()
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false)
            {
                fs::remove_file(target)?;
            }
            // A busy target (ETXTBSY: the shim is executing in another session)
            // must not abort init — the stale copy keeps working; warn and go on.
            match fs::copy(&current_exe, target) {
                Ok(_) => {
                    chmod_755(target);
                    println!("✓ Installed {}", target.display());
                }
                Err(e) => eprintln!(
                    "⚠️  跳过 {}（{e}）— 旧副本仍可用，稍后重跑 gate init 更新",
                    target.display()
                ),
            }
        } else {
            println!("  Already installed: {}", target.display());
        }
    }

    // git config core.hooksPath .githooks/hooks
    let rc = std::process::Command::new("git")
        .args(["config", "core.hooksPath", ".githooks/hooks"])
        .status()?;
    if !rc.success() {
        anyhow::bail!("git config core.hooksPath failed");
    }

    write_hook_templates()?;

    println!("  git hooksPath → .githooks/hooks");

    // Verify install
    let rc = std::process::Command::new(&gate_target)
        .arg("--version")
        .output();
    if let Ok(o) = rc
        && o.status.success()
    {
        println!("  ✓ {}", String::from_utf8_lossy(&o.stdout).trim());
    }

    // Seed the default rules pack (never overwrites — user edits survive
    // re-init). canon/rules/gate is the source of truth; found by walking up
    // from the binary location (target/debug → bin/gate → bin → canon) or cwd.
    let pack = match rules_dir {
        Some(dir) => Some(dir.to_path_buf()),
        None => find_rules_pack(),
    };
    match pack {
        Some(dir) => {
            let githooks = ensure_githooks_dir()?;
            let spec_dir = githooks.join("spec");
            let n = seed_rules(&dir, &spec_dir)?;
            println!("  rules: {} new file(s) from {}", n, dir.display());
        }
        None => println!(
            "  rules: pack not found — point --rules-dir at canon/rules/gate, or run agent-sync"
        ),
    }

    Ok(())
}

/// A dir is a rules pack when it holds at least one `checklist_*.yaml`.
fn is_pack(dir: &Path) -> bool {
    fs::read_dir(dir).is_ok_and(|mut it| {
        it.any(|e| {
            e.ok()
                .is_some_and(|e| e.file_name().to_string_lossy().starts_with("checklist_"))
        })
    })
}

/// Look for `rules/gate` near the binary (canon checkout: target/debug →
/// bin/gate → bin → canon) then near cwd.
fn find_rules_pack() -> Option<PathBuf> {
    let mut starts = Vec::new();
    if let Ok(exe) = std::env::current_exe()
        && let Some(parent) = exe.parent()
    {
        starts.push(parent.to_path_buf());
    }
    if let Ok(cwd) = std::env::current_dir() {
        starts.push(cwd);
    }
    for start in starts {
        let mut dir = start;
        loop {
            let cand = dir.join("rules/gate");
            if is_pack(&cand) {
                return Some(cand);
            }
            if !dir.pop() {
                break;
            }
        }
    }
    None
}

fn seed_rules(pack: &Path, spec_dir: &Path) -> anyhow::Result<usize> {
    fs::create_dir_all(spec_dir)?;
    let mut n = 0;
    for entry in fs::read_dir(pack)? {
        let path = entry?.path();
        let ext = path.extension().and_then(|s| s.to_str());
        if !matches!(ext, Some("yaml") | Some("md")) {
            continue;
        }
        let dest = spec_dir.join(
            path.file_name()
                .ok_or_else(|| anyhow::anyhow!("bad pack entry name"))?,
        );
        if dest.exists() {
            continue; // never clobber user edits
        }
        fs::copy(&path, &dest)?;
        n += 1;
    }
    // docs/ subtree (protocol + overview + demo) — same never-clobber rule
    let docs_src = pack.join("docs");
    if docs_src.is_dir() {
        let docs_dst = spec_dir.join("docs");
        fs::create_dir_all(&docs_dst)?;
        for entry in fs::read_dir(&docs_src)?.flatten() {
            let dest = docs_dst.join(entry.file_name());
            if dest.exists() {
                continue;
            }
            fs::copy(entry.path(), &dest)?;
            n += 1;
        }
    }
    Ok(n)
}

/// `gate init --uninstall` — remove `~/.local/bin/gate`/`gh` and unset hooksPath.
pub fn uninstall() -> anyhow::Result<()> {
    let home = std::env::var("HOME").map_err(|_| anyhow::anyhow!("HOME not set"))?;
    let bin_dir = PathBuf::from(&home).join(".local").join("bin");
    let gate_target = bin_dir.join("gate");
    let gh_target = bin_dir.join("gh");

    for target in [&gate_target, &gh_target] {
        if target.symlink_metadata().is_ok() {
            fs::remove_file(target)?;
            println!("✓ Removed {}", target.display());
        } else {
            println!("  Not installed: {}", target.display());
        }
    }

    // Restore default hooksPath (ignore failure if not set).
    let _ = std::process::Command::new("git")
        .args(["config", "--unset", "core.hooksPath"])
        .status();

    println!("  git hooksPath unset");
    Ok(())
}

/// Find `.githooks/` walking up from cwd; bootstrap `./.githooks/` in fresh
/// repos that don't have one yet (init must work on first adoption).
fn ensure_githooks_dir() -> anyhow::Result<PathBuf> {
    if let Some(dir) = git::find_githooks_dir() {
        return Ok(dir);
    }
    fs::create_dir_all(".githooks")?;
    Ok(PathBuf::from(".githooks"))
}

/// Write hook template shell stubs to `.githooks/hooks/`.
fn write_hook_templates() -> anyhow::Result<()> {
    let githooks = ensure_githooks_dir()?;
    let hooks_dir = githooks.join("hooks");
    fs::create_dir_all(&hooks_dir)?;

    write_template(&hooks_dir.join("pre-commit"), HOOK_PRE_COMMIT)?;
    write_template(&hooks_dir.join("pre-push"), HOOK_PRE_PUSH)?;
    write_template(&hooks_dir.join("merge"), HOOK_MERGE)?;

    println!("  hook templates: pre-commit, pre-push, merge");
    Ok(())
}

fn write_template(path: &Path, content: &str) -> anyhow::Result<()> {
    fs::write(path, content)?;
    chmod_755(path);
    Ok(())
}
fn chmod_755(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o755));
    }
}

// ===========================================================================
// Tests
// ===========================================================================
