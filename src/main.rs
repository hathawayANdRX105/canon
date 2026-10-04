//! canon — spec-driven quality gate + MCP spec server, one binary.
//!
//! The engine (`engine` module) contains zero detection logic: every rule
//! lives in `checklist_*.yaml` under `<repo>/.githooks/spec/` and runs an
//! external harness command. `specs/` is the source of truth for the default
//! rules pack; `canon init` seeds a repo with it. The `tools`/`rules` modules
//! carry the gh-workflow policy layer (issue/PR compliance, merge
//! orchestration, gh command interception), and `mcp` exposes the rule
//! catalog plus a pre-commit preflight over MCP stdio.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use gate::{engine, shared, tools};

#[derive(Parser)]
#[command(
    name = "canon",
    version,
    about = "spec-driven quality gate + MCP spec server — yaml rules, external harnesses, zero built-in detection"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install canon: copy binary to ~/.local/bin (canon + gh), set
    /// core.hooksPath, write hook scripts, and seed .githooks/spec/ with
    /// the default rules pack (never overwrites existing files).
    Init {
        /// Rules pack dir (auto-detected near the binary / canon checkout if omitted)
        #[arg(long)]
        rules_dir: Option<PathBuf>,
    },
    /// Run pre-commit hooks
    PreCommit,
    /// Validate the commit message git is about to commit (CM-01/02/03);
    /// git passes the message file as $1
    CommitMsg { path: PathBuf },
    /// Run pre-push hooks (git 调 pre-push 钩子时带 remote name 与 URL，
    /// 包装脚本按 issue #21 统一转发 "$@"，两个位置参数 canon 只接收不消费)
    PrePush {
        #[arg(hide = true)]
        remote_name: Option<String>,
        #[arg(hide = true)]
        remote_url: Option<String>,
    },
    /// Run merge checks: `canon merge <owner/repo> <pr_number> [--dry-run]`
    Merge(MergeArgs),
    Check {
        /// Checklist names (file name minus `checklist_` prefix and `.yaml`)
        names: Vec<String>,
        /// Maximum SLA tier to run (l1/l2/l3, default l1)
        #[arg(long, default_value = "l1")]
        sla: String,
        /// Output machine-readable JSON with all finding extras (score,
        /// confidence, evidence, ...). For dev agents to consume.
        #[arg(long)]
        json: bool,
    },
    /// Validate issues
    Issue,
    /// Audit issues/PRs for checkbox & linkage compliance
    Audit(AuditArgs),
    /// Validate issues
    Pr,
    /// Serve the rule catalog and a preflight run over MCP stdio
    Mcp,
    /// Run the local flow board: REST + WebSocket push over std::net.
    /// Routes reuse the 16 flow tools; `GET /api/stream` is the WS push.
    /// Default port 10081 (web UI via `dx serve` runs on 10080).
    Serve {
        /// Listen port (10000+ range; default 10081)
        #[arg(long, default_value_t = 10081)]
        port: u16,
    },
}

#[derive(clap::Args)]
struct MergeArgs {
    /// owner/repo
    repo: String,
    /// PR number
    pr: u32,
    /// Plan only, no squash
    #[arg(long)]
    dry_run: bool,
}

#[derive(clap::Args)]
struct AuditArgs {
    /// owner/repo (defaults to git remote origin)
    repo: Option<String>,
    /// Scan issues/PRs created in the last N days
    #[arg(long)]
    recent: Option<u32>,
    /// Limit number of items to scan (0 = unlimited)
    #[arg(long, default_value = "0")]
    limit: u32,
    /// Number of concurrent workers
    #[arg(long, default_value = "5")]
    workers: u32,
    /// Specific issue/PR numbers to check
    #[arg(long)]
    issues: Option<String>,
}

fn main() -> ExitCode {
    // gh-mode: installed as ~/.local/bin/gh → intercept issue/pr commands
    if let Some(arg0) = std::env::args().next() {
        let base = std::path::Path::new(&arg0)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        if base == "gh" || base == "gh.exe" {
            let args: Vec<String> = std::env::args().skip(1).collect();
            let rc = tools::gh_wrap::dispatch(&args);
            return ExitCode::from(rc as u8);
        }
    }

    let cli = Cli::parse();
    match cli.command {
        Commands::Init { rules_dir } => match tools::init::install(rules_dir.as_deref()) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("canon init 失败: {e}");
                ExitCode::FAILURE
            }
        },
        Commands::PreCommit => ExitCode::from(tools::pre_commit::run() as u8),
        Commands::CommitMsg { path } => {
            ExitCode::from(tools::pre_commit::run_commit_msg(path.to_str().unwrap_or("")) as u8)
        }
        Commands::PrePush { .. } => ExitCode::from(tools::pre_push::run() as u8),
        Commands::Merge(args) => {
            let mut arg_vec = vec![args.repo, args.pr.to_string()];
            if args.dry_run {
                arg_vec.push("--dry-run".to_string());
            }
            ExitCode::from(tools::merge::run(&arg_vec) as u8)
        }
        Commands::Issue => ExitCode::from(tools::gh_wrap::intercept_issue_create(&[]) as u8),
        Commands::Pr => ExitCode::from(tools::gh_wrap::intercept_pr_create(&[]) as u8),
        Commands::Check { names, sla, json } => {
            let max_sla = engine::SlaLevel::parse(&sla);
            if !json {
                eprintln!("══════════════════════════════════════════════════════");
                eprintln!("⚠️  L3 质量关卡: 不阻断 push, 但 finding 必须逐条处置, 禁止静默忽略");
                eprintln!(
                    "    WARN = 修复(默认) 或 书面驳回(证据写进 PR 审查记录); FAIL = 必须修复或拆 PR 才能继续"
                );
                eprintln!("══════════════════════════════════════════════════════");
            }
            let mut findings = engine::run_named(&names, max_sla);
            shared::apply_global_overrides(&mut findings);
            if json {
                let arr = serde_json::Value::Array(findings.iter().map(|f| f.to_json()).collect());
                println!("{}", serde_json::to_string_pretty(&arr).unwrap_or_default());
            } else {
                shared::print_findings(&findings);
                eprintln!("══════════════════════════════════════════════════════");
                eprintln!(
                    "ℹ️  L3 finding 不阻断, 但每条 WARN/FAIL 必须处置: 修复(默认) 或 书面驳回记入 PR 审查记录; 静默忽略 = 违规"
                );
                eprintln!(
                    "    L1+L2 FAIL = 硬门槛, 必须修复. 深度语义审查由 merge 钩子的 L2 checklist 承接."
                );
                eprintln!("══════════════════════════════════════════════════════");
            }
            ExitCode::from(shared::exit_code(&findings) as u8)
        }
        Commands::Serve { port } => match flow::serve::run(port) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("canon serve: {e}");
                ExitCode::FAILURE
            }
        },
        Commands::Mcp => match run_mcp_stdio() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("canon mcp: {e}");
                ExitCode::FAILURE
            }
        },
        Commands::Audit(args) => {
            let args_vec: Vec<String> = build_audit_args(&args);
            let rc = tools::audit::run(&args_vec);
            ExitCode::from(rc as u8)
        }
    }
}

/// Newline-delimited JSON-RPC on stdin/stdout; stderr is free for the
/// engine's progress output, which is why it must never touch stdout.
fn run_mcp_stdio() -> std::io::Result<()> {
    use std::io::{BufRead, Write};
    let stdin = std::io::stdin();
    let mut out = std::io::stdout().lock();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        if let Some(reply) = mcp::handle_line(&line) {
            writeln!(out, "{reply}")?;
            out.flush()?;
        }
    }
    Ok(())
}

fn build_audit_args(args: &AuditArgs) -> Vec<String> {
    let mut vec = Vec::new();
    let repo = args.repo.clone().unwrap_or_else(tools::audit::derive_repo);
    if repo.is_empty() {
        eprintln!("无法确定 repo (git remote get-url origin 失败)");
        return vec![];
    }
    vec.push(repo);
    if let Some(days) = args.recent {
        vec.push(format!("--recent={days}"));
    }
    if args.limit > 0 {
        vec.push(format!("--limit={}", args.limit));
    }
    if args.workers != 5 {
        vec.push(format!("--workers={}", args.workers));
    }
    if let Some(issues) = &args.issues {
        vec.push(format!("--issues={issues}"));
    }
    vec
}

#[cfg(test)]
mod tests {
    use crate::{Cli, Commands};
    use clap::Parser;
    use gate::shared::load_yaml;
    use std::path::Path;

    #[test]
    fn loads_real_spec_and_counts_required_headings() {
        // ponytail: the crate root IS the canon repo root, so the default
        // rules pack is just a subdir. No ancestor walk needed.
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("specs")
            .join("github")
            .join("github_issues.yaml");
        let v =
            load_yaml(path.to_str().expect("spec path is utf-8")).expect("spec yaml must parse");
        let headings = v
            .get("required_headings")
            .expect("required_headings key present")
            .as_sequence()
            .expect("required_headings is a sequence");
        assert_eq!(headings.len(), 6, "expected 6 required headings");
        let names: Vec<&str> = headings.iter().filter_map(|h| h.as_str()).collect();
        assert!(names.contains(&"Goal"));
        assert!(names.contains(&"Out of scope"));
    }

    // issue #21：钩子包装统一转发 "$@" 后，git 给 pre-push 的 remote name/URL
    // 必须能被 CLI 原样接收，而不是报 unexpected argument
    #[test]
    fn pre_push_tolerates_git_forwarded_args() {
        let cli =
            Cli::try_parse_from(["canon", "pre-push", "origin", "https://github.com/o/r.git"])
                .expect("git 的 pre-push 两个位置参数应被 CLI 接受");
        assert!(matches!(cli.command, Commands::PrePush { .. }));
    }
}
