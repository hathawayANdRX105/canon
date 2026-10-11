pub mod issues;
pub mod pull_requests;
pub mod reviews;

#[cfg(test)]
pub(crate) fn repo_spec_path(name: &str) -> std::path::PathBuf {
    // 单 crate 布局：CARGO_MANIFEST_DIR 就是仓根，specs/ 在其下。
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("specs")
        .join("github")
        .join(name)
}
