//! 规则运行上下文：仓库根、跟踪文件与变更集，全规则一次构建、共享。

use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Ctx {
    root: PathBuf,
    tracked: Vec<PathBuf>,
    changed: Option<Vec<PathBuf>>,
    selected: Vec<PathBuf>,
}

impl Ctx {
    /// `tracked` / `changed` 均为相对 `root` 的路径。`changed = None` 即全量口径。
    pub fn new(
        root: impl Into<PathBuf>,
        tracked: Vec<PathBuf>,
        changed: Option<Vec<PathBuf>>,
    ) -> Self {
        let selected = changed.clone().unwrap_or_else(|| tracked.clone());
        Self {
            root: root.into(),
            tracked,
            changed,
            selected,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// 当前口径下应逐文件检查的文件集：changed 优先，缺省为全部跟踪文件。
    pub fn scope_files(&self) -> &[PathBuf] {
        &self.selected
    }

    /// 全部跟踪文件：全仓不变量类规则用，无视 scope。
    pub fn tracked_files(&self) -> &[PathBuf] {
        &self.tracked
    }

    /// 该文件是否属于变更集；全量口径恒真。
    pub fn is_changed(&self, rel: &Path) -> bool {
        match &self.changed {
            Some(set) => set.iter().any(|p| p == rel),
            None => true,
        }
    }

    pub fn read(&self, rel: &Path) -> std::io::Result<String> {
        std::fs::read_to_string(self.root.join(rel))
    }
}
