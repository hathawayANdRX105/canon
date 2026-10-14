//! 规则运行上下文：仓库根、跟踪文件与变更集，全规则一次构建、共享。

use std::path::{Path, PathBuf};

use globset::GlobSet;

fn compile_globs(patterns: &[String]) -> Result<GlobSet, globset::Error> {
    let mut builder = globset::GlobSet::builder();
    for pattern in patterns {
        let glob = globset::GlobBuilder::new(pattern)
            .literal_separator(true)
            .build()?;
        builder.add(glob);
    }
    builder.build()
}

#[derive(Debug, Clone)]
pub struct Ctx {
    root: PathBuf,
    tracked: Vec<PathBuf>,
    changed: Option<Vec<PathBuf>>,
    selected: Vec<PathBuf>,
    /// semantic 类的决策端点（checks.toml [semantic]）；由 Registry 注入。
    semantic: Option<crate::config::SemanticEndpoint>,
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
            semantic: None,
        }
    }

    /// 注入语义端点配置（Registry 在跑 semantic 类规则前调用）。
    pub fn with_semantic(&self, semantic: Option<crate::config::SemanticEndpoint>) -> Ctx {
        let mut next = self.clone();
        next.semantic = semantic;
        next
    }

    /// semantic 端点（endpoint/model/max_len）；None = 未配置。
    pub fn semantic(&self) -> Option<&crate::config::SemanticEndpoint> {
        self.semantic.as_ref()
    }

    /// 策略层 scope 覆盖：include glob 内且不在 exclude glob 的文件入选；
    /// 两组都空 = 原口径不变。tracked（全仓不变量口径）保持原样。
    pub fn scoped(&self, include: &[String], exclude: &[String]) -> Ctx {
        if include.is_empty() && exclude.is_empty() {
            return self.clone();
        }
        let (Ok(inc), Ok(exc)) = (compile_globs(include), compile_globs(exclude)) else {
            return self.clone();
        };
        let selected: Vec<PathBuf> = self
            .selected
            .iter()
            .filter(|p| {
                let hit = inc.is_empty() || inc.is_match(p.as_path());
                let excluded = !exc.is_empty() && exc.is_match(p.as_path());
                hit && !excluded
            })
            .cloned()
            .collect();
        Self {
            root: self.root.clone(),
            tracked: self.tracked.clone(),
            changed: self.changed.clone(),
            selected,
            semantic: self.semantic.clone(),
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
