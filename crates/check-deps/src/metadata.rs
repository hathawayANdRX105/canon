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
    kind: Option<Vec<String>>,
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
            if dep.path.is_some() && dep.kind.is_none() {
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
