//! git 历史解析：把 `git log --name-only` 折成「提交 → 文件集」的通用建模。
//! 热点 / 变更耦合 / 缺陷历史三类规则共用这一层（Crime Scene 学派的最低成本实现）。

use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

/// 一次提交涉及（改写/新增/删除）的文件集 + 提交主题（缺陷关键词扫描用）。
#[derive(Debug)]
pub struct Commit {
    pub subject: String,
    pub files: Vec<String>,
}

pub type Churn = BTreeMap<String, usize>;

/// 取 `since`（git 相对时间，如 "6 months ago"）以来 name-only 的提交序列。
/// cargo / git 缺失、非 git 仓、命令失败 → None（调用方按情报缺失静默跳过）。
pub fn load(root: &Path, since: &str) -> Option<Vec<Commit>> {
    let output = Command::new("git")
        .current_dir(root)
        .args([
            "log",
            "--name-only",
            "--no-merges",
            "--pretty=format:\u{1e}%s",
            &format!("--since={since}"),
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mut commits = Vec::new();
    for block in text.split('\u{1e}') {
        let block = block.trim_start_matches(['\n', '\r']);
        if block.is_empty() {
            continue;
        }
        let (subject, files) = match block.split_once('\n') {
            Some((s, rest)) => (s.trim().to_string(), rest),
            None => (block.trim().to_string(), ""),
        };
        let files: Vec<String> = files
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect();
        commits.push(Commit { subject, files });
    }
    Some(commits)
}

/// 每文件变更次数（含删除事件——被删文件随 scope 过滤自然剔除）。
pub fn churn(commits: &[Commit]) -> Churn {
    let mut map = BTreeMap::new();
    for c in commits {
        for f in &c.files {
            *map.entry(f.clone()).or_insert(0) += 1;
        }
    }
    map
}

/// 时序耦合：同一提交里共现 ≥ `min` 次的文件对（对内按字典序去重）。
/// 返回 ((a, b), count)，count 降序。
pub fn coupling(commits: &[Commit], min: usize) -> Vec<((String, String), usize)> {
    let mut pairs: BTreeMap<(String, String), usize> = BTreeMap::new();
    for c in commits {
        let mut files = c.files.clone();
        files.sort();
        files.dedup();
        for (i, a) in files.iter().enumerate() {
            for b in &files[i + 1..] {
                *pairs.entry((a.clone(), (*b).clone())).or_insert(0) += 1;
            }
        }
    }
    let mut out: Vec<_> = pairs.into_iter().filter(|(_, n)| *n >= min).collect();
    out.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    out
}

/// 缺陷历史：subject 命中修复关键词的提交数，按文件聚合。
pub fn defect_counts(commits: &[Commit], keywords: &[&str]) -> Churn {
    let mut map = BTreeMap::new();
    for c in commits {
        let subject = c.subject.to_ascii_lowercase();
        let is_fix = keywords
            .iter()
            .any(|k| subject.contains(&k.to_ascii_lowercase()));
        if !is_fix {
            continue;
        }
        for f in &c.files {
            *map.entry(f.clone()).or_insert(0) += 1;
        }
    }
    map
}

/// 默认缺陷关键词（中英混合，commit subject 大小写不敏感）。
pub const DEFAULT_FIX_KEYWORDS: &[&str] = &[
    "fix",
    "bug",
    "revert",
    "regression",
    "修复",
    "回滚",
    "缺陷",
    "回归",
];
