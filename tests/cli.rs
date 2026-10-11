//! CLI 契约测试：argv 解析 + 真 spec 文件能加载。

use std::path::Path;

use canon::cli::{Cli, Commands};
use canon::gate::shared::load_yaml;
use clap::Parser;

#[test]
fn loads_real_spec_and_counts_required_headings() {
    // ponytail: the crate root IS the canon repo root, so the default
    // rules pack is just a subdir. No ancestor walk needed.
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("specs")
        .join("github")
        .join("github_issues.yaml");
    let v = load_yaml(path.to_str().expect("spec path is utf-8")).expect("spec yaml must parse");
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
    let cli = Cli::try_parse_from(["canon", "pre-push", "origin", "https://github.com/o/r.git"])
        .expect("git 的 pre-push 两个位置参数应被 CLI 接受");
    assert!(matches!(cli.command, Commands::PrePush { .. }));
}
