//! `gate::tools::init` 单元测试：hook 包装脚本必须原样转发调用方参数。

use canon::gate::tools::init::{HOOK_COMMIT_MSG, HOOK_MERGE, HOOK_PRE_COMMIT, HOOK_PRE_PUSH};

/// 每个包装脚本必须把调用方参数原样转给 canon 子命令
/// （`canon merge <owner/repo> <pr> --dry-run` 走的就是包装层）。
#[test]
fn hook_templates_forward_caller_args() {
    for (name, tmpl, subcmd) in [
        ("pre-commit", HOOK_PRE_COMMIT, "pre-commit"),
        ("commit-msg", HOOK_COMMIT_MSG, "commit-msg"),
        ("pre-push", HOOK_PRE_PUSH, "pre-push"),
        ("merge", HOOK_MERGE, "merge"),
    ] {
        // PATH 分支与 repo-local 分支都要转发
        assert!(
            tmpl.contains(&format!("exec canon {subcmd} \"$@\"\n")),
            "{name}: PATH 分支未转发 \"$@\""
        );
        assert!(
            tmpl.contains(&format!("exec \"$REPO/.githooks/canon\" {subcmd} \"$@\"\n")),
            "{name}: repo-local 分支未转发 \"$@\""
        );
        // 不允许残留不带参数的裸 exec（历史 bug：调用方参数被整段丢弃）
        assert!(
            !tmpl.contains(&format!("exec canon {subcmd}\n")),
            "{name}: 仍有丢弃参数的裸 exec canon {subcmd}"
        );
    }
}
