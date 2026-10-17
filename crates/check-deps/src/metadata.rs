//! `cargo metadata --no-deps` 的最小建模：够判「谁依赖谁」即可，零正式依赖。

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

/// (包名 → Cargo.toml 相对目录) 映射 + workspace 内 normal path 依赖边（源包名 → 目标包名）。
#[derive(Debug)]
pub struct Graph {
    pub package_dirs: BTreeMap<String, String>,
    pub edges: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    #[serde(default)]
    packages: Vec<PackageJson>,
}

#[derive(Debug, Deserialize)]
struct PackageJson {
    name: String,
    manifest_path: String,
    #[serde(default)]
    dependencies: Vec<DepJson>,
}

#[derive(Debug, Deserialize)]
struct DepJson {
    name: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    kind: Option<DepKindJson>,
}

/// cargo metadata serializes `kind` as `null` for normal edges and as a string
/// (`"dev"` / `"build"`) for the rest. The array shape never appears in real
/// output; it is kept tolerated only because the old array-only model made the
/// whole metadata fail to deserialize whenever any dev/build dependency existed,
/// silently turning the dependent rules into no-ops.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DepKindJson {
    One(String),
    Many(Vec<String>),
}

impl DepJson {
    /// Whether this is a workspace-internal normal edge: `kind` is `null`
    /// (or an explicit `"normal"`). Dev/build edges do not count; the array
    /// shape counts as normal only when every element is `"normal"`.
    fn is_normal_edge(&self) -> bool {
        match &self.kind {
            None => true,
            Some(DepKindJson::One(kind)) => kind == "normal",
            Some(DepKindJson::Many(kinds)) => kinds.iter().all(|kind| kind == "normal"),
        }
    }
}

/// cargo 缺失 / metadata 失败 = None，调用方静默跳过（加固型规则的情报缺失语义）。
pub fn load(root: &Path) -> Option<Graph> {
    let output = std::process::Command::new("cargo")
        .current_dir(root)
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let metadata: Metadata = serde_json::from_slice(&output.stdout).ok()?;

    let mut package_dirs = BTreeMap::new();
    for package in &metadata.packages {
        if let Some(rel) = rel_dir(root, &package.manifest_path) {
            package_dirs.insert(package.name.clone(), rel);
        }
    }

    let mut edges = Vec::new();
    for package in &metadata.packages {
        for dep in &package.dependencies {
            if dep.path.is_some() && dep.is_normal_edge() {
                edges.push((package.name.clone(), dep.name.clone()));
            }
        }
    }
    Some(Graph {
        package_dirs,
        edges,
    })
}

/// Cargo.toml 绝对路径 → 相对仓库根的目录串；仓库外（路径依赖越界）返回 None。
fn rel_dir(root: &Path, manifest_path: &str) -> Option<String> {
    let dir = Path::new(manifest_path).parent()?;
    let rel = dir.strip_prefix(root).ok()?;
    Some(rel.display().to_string())
}

/// 一条 git 依赖（Cargo.toml `dependencies` 里的 git 源）的钉版信息。
#[derive(Debug)]
pub struct GitDep {
    /// 消费方包名（workspace 内）。
    pub package: String,
    /// 消费方 Cargo.toml 相对目录。
    pub package_dir: String,
    /// 依赖重命名后的键（`check-core = { git = … }` → `check-core`）。
    pub dep: String,
    /// git 仓库 URL。
    pub url: String,
    /// 钉版方式：tag / branch / rev 三选一。
    pub pin: GitPin,
}

#[derive(Debug)]
pub enum GitPin {
    Tag(String),
    Branch(String),
    Rev(String),
}

impl GitPin {
    /// 「是否钉死」：tag 与 rev 是可复现钉版；branch 会漂移。
    pub fn is_pinned(&self) -> bool {
        !matches!(self, GitPin::Branch(_))
    }

    /// 人类可读的钉版描述。
    pub fn describe(&self) -> String {
        match self {
            GitPin::Tag(t) => format!("tag={t}"),
            GitPin::Branch(b) => format!("branch={b}"),
            GitPin::Rev(r) => format!("rev={}", &r[..r.len().min(12)]),
        }
    }
}

/// 从 `cargo metadata` 的完整 JSON 里抽 git 依赖。
///
/// `--no-deps` 不够：git 依赖本身不进 packages（除非被 resolve），但其
/// `spec` 在依赖声明里。为避免引 cargo_metadata crate，直接跑不带
/// `--no-deps` 的 metadata 并按 `dependencies[].source` 过滤（零正式依赖的
/// 代价是多解析一层 JSON——serde 已在依赖树里）。
pub fn git_deps(root: &Path) -> Option<Vec<GitDep>> {
    let output = std::process::Command::new("cargo")
        .current_dir(root)
        .args(["metadata", "--format-version", "1"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    #[derive(Deserialize)]
    struct Full {
        #[serde(default)]
        packages: Vec<FullPkg>,
    }
    #[derive(Deserialize)]
    struct FullPkg {
        name: String,
        manifest_path: String,
        #[serde(default)]
        dependencies: Vec<FullDep>,
    }
    #[derive(Deserialize)]
    struct FullDep {
        name: String,
        #[serde(default)]
        source: Option<String>,
        #[serde(default, rename = "req")]
        _req: Option<String>,
    }

    let metadata: Full = serde_json::from_slice(&output.stdout).ok()?;

    let mut out = Vec::new();
    for pkg in &metadata.packages {
        for dep in &pkg.dependencies {
            let Some(source) = &dep.source else {
                continue;
            };
            // source 形如 "git+https://…?tag=check-v0.1.5#dd812df…" 或 "?branch=main#rev"
            let Some(rest) = source.strip_prefix("git+") else {
                continue;
            };
            let (url, query) = match rest.split_once('?') {
                Some((u, q)) => (u, q),
                None => (rest, ""),
            };
            let url = url.trim_end_matches('#').to_string();
            let pin = parse_pin(query);
            let Some(pin) = pin else { continue };
            out.push(GitDep {
                package: pkg.name.clone(),
                package_dir: rel_dir(root, &pkg.manifest_path)?,
                dep: dep.name.clone(),
                url,
                pin,
            });
        }
    }
    Some(out)
}

/// 从 metadata source 的 query 串解析钉版（tag=/branch=/rev=）。
fn parse_pin(query: &str) -> Option<GitPin> {
    for pair in query.split('&') {
        let pair = pair.split('#').next().unwrap_or(pair);
        if let Some(t) = pair.strip_prefix("tag=") {
            return Some(GitPin::Tag(t.to_string()));
        }
        if let Some(b) = pair.strip_prefix("branch=") {
            return Some(GitPin::Branch(b.to_string()));
        }
        if let Some(r) = pair.strip_prefix("rev=") {
            return Some(GitPin::Rev(r.to_string()));
        }
    }
    None
}
