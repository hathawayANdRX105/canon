pub mod issues;
pub mod pull_requests;
pub mod reviews;

#[cfg(test)]
pub(crate) fn repo_spec_path(name: &str) -> std::path::PathBuf {
    // gate 现在住 crates/gate，specs/ 在仓根。从 CARGO_MANIFEST_DIR 向上找
    // 第一个含 specs/github/ 的目录，把 spec 文件定位出来。
    let mut dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    loop {
        let cand = dir.join("specs").join("github").join(name);
        if cand.is_file() {
            return cand;
        }
        match dir.parent() {
            Some(p) => dir = p.to_path_buf(),
            None => panic!(
                "specs/github/{name} not found walking up from {}",
                env!("CARGO_MANIFEST_DIR")
            ),
        }
    }
}
