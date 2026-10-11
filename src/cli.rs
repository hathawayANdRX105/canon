//! `canon` 的命令行表面：clap 定义。
//!
//! 住在 lib 而非 bin，是为了让 `tests/cli.rs` 能直接解析 argv —— CLI 的
//! 契约（子命令名、位置参数、flag）本身就是被测行为，bin 里的私有类型
//! 集成测试看不见。

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "canon",
    version,
    about = "spec-driven quality gate + MCP spec server — yaml rules, external harnesses, zero built-in detection"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
}

#[derive(clap::Args, Debug)]
pub struct MergeArgs {
    /// owner/repo
    pub repo: String,
    /// PR number
    pub pr: u32,
    /// Plan only, no squash
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(clap::Args, Debug)]
pub struct AuditArgs {
    /// owner/repo (defaults to git remote origin)
    pub repo: Option<String>,
    /// Scan issues/PRs created in the last N days
    #[arg(long)]
    pub recent: Option<u32>,
    /// Limit number of items to scan (0 = unlimited)
    #[arg(long, default_value = "0")]
    pub limit: u32,
    /// Number of concurrent workers
    #[arg(long, default_value = "5")]
    pub workers: u32,
    /// Specific issue/PR numbers to check
    #[arg(long)]
    pub issues: Option<String>,
}
