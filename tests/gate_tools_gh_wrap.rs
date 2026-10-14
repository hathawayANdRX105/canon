//! `gate::tools::gh_wrap` 单元测试：merge 方式拦截。

use canon::gate::tools::gh_wrap::merge_method_is_squash;

fn args(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}
#[test]
fn squash_flag_alone_is_accepted() {
    assert!(merge_method_is_squash(&args(&[
        "pr", "merge", "12", "--squash", "--body", "x"
    ])));
}
#[test]
fn no_merge_method_is_rejected() {
    // gh 不带 flag 时默认走 merge commit，交互式选择同理 —— 非 squash。
    assert!(!merge_method_is_squash(&args(&["pr", "merge", "12"])));
    assert!(!merge_method_is_squash(&args(&[
        "pr", "merge", "12", "--body", "x"
    ])));
}
#[test]
fn explicit_conflicting_methods_are_rejected() {
    for flag in [
        "--merge",
        "--rebase",
        "--auto",
        "--ff-only",
        "--no-squash",
        "-m",
        "-r",
        "-F",
    ] {
        assert!(
            !merge_method_is_squash(&args(&["pr", "merge", "12", flag])),
            "{flag} must be rejected"
        );
    }
}
#[test]
fn squash_plus_conflicting_flag_is_rejected() {
    // 同时带 --squash 和 --merge 语义矛盾，按拒绝处理（不猜用户意图）。
    assert!(!merge_method_is_squash(&args(&[
        "pr", "merge", "12", "--squash", "--merge"
    ])));
    assert!(!merge_method_is_squash(&args(&[
        "pr",
        "merge",
        "12",
        "--squash",
        "--no-squash"
    ])));
}
