//! 参数化规则种类（D9）：同一 Rust 实现，checks.toml 多实例。
//! TOML 只做参数，不做逻辑——复杂语义写专用 Rust 规则（域 crate），不塞配置。

use std::collections::BTreeSet;
use std::path::Path;

use serde::Deserialize;

use crate::context::Ctx;
use crate::finding::{Class, Finding, Severity};
use crate::rule::Rule;

/// kind 工厂：把 checks.toml 里的参数表实例化成规则。
/// 参数键由各实现的 serde 类型裁决（deny_unknown_fields），未知键实例化即报错。
pub trait KindFactory: Send + Sync {
    fn kind(&self) -> &'static str;

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String>;
}

/// 内置 kind；其余 kind 随所属域 crate（T1–T8）落地。
pub fn builtin_factories() -> Vec<Box<dyn KindFactory>> {
    vec![
        Box::new(PathForbiddenFactory),
        Box::new(TextPatternFactory),
        Box::new(NamePatternFactory),
        Box::new(DuplicateFileFactory),
    ]
}

// ---------------------------------------------------------------------------
// path-forbidden：命中的跟踪路径存在即违规（全仓不变量，line=0）。

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PathForbiddenParams {
    /// 相对仓库根的 glob；`*` 不跨目录分隔符，`**` 跨。
    paths: Vec<String>,
    message: String,
}

struct PathForbiddenRule {
    id: String,
    patterns: globset::GlobSet,
    message: String,
}

impl Rule for PathForbiddenRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "禁止出现的路径存在"
    }

    fn doc(&self) -> &str {
        "跟踪文件命中 paths glob 即违规；全仓不变量，无视变更 scope。\
         参数见 checks.toml 的 [kinds.path-forbidden.<name>]。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Fail
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        ctx.tracked_files()
            .iter()
            .filter(|rel| self.patterns.is_match(rel.as_path()))
            .map(|rel| {
                Finding::new(
                    self.id.as_str(),
                    Severity::Fail,
                    rel.display().to_string(),
                    0,
                    self.message.clone(),
                )
            })
            .collect()
    }
}

struct PathForbiddenFactory;

impl KindFactory for PathForbiddenFactory {
    fn kind(&self) -> &'static str {
        "path-forbidden"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: PathForbiddenParams =
            PathForbiddenParams::deserialize(toml::Value::Table(params.clone()))
                .map_err(|e| format!("bad params: {e}"))?;
        let mut builder = globset::GlobSet::builder();
        for pattern in &parsed.paths {
            let glob = globset::GlobBuilder::new(pattern)
                .literal_separator(true)
                .build()
                .map_err(|e| format!("bad glob {pattern:?}: {e}"))?;
            builder.add(glob);
        }
        let patterns = builder.build().map_err(|e| format!("bad globs: {e}"))?;
        Ok(Box::new(PathForbiddenRule {
            id: format!("PATH-FORBIDDEN.{name}"),
            patterns,
            message: parsed.message,
        }))
    }
}

// ---------------------------------------------------------------------------
// text-pattern：scope 文件逐行匹配拒绝正则，命中即行级 finding；
// `patterns_allow` 命中同一行时该行豁免（占位符/示例抑制）。

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TextPatternParams {
    #[serde(default)]
    paths_include: Vec<String>,
    #[serde(default)]
    paths_exclude: Vec<String>,
    /// 逐行匹配；一行只报一次（首个命中的模式）。
    #[serde(default)]
    patterns_deny: Vec<String>,
    /// 行级豁免：deny 命中但本组任一模式也命中该行 → 不报
    /// （典型用法：真实凭据召回 + 占位符/示例词抑制）。
    #[serde(default)]
    patterns_allow: Vec<String>,
    message: String,
}

struct TextPatternRule {
    id: String,
    include: globset::GlobSet,
    exclude: globset::GlobSet,
    patterns: Vec<regex::Regex>,
    allow: Vec<regex::Regex>,
    message: String,
}

impl Rule for TextPatternRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "文本模式拒绝"
    }

    fn doc(&self) -> &str {
        "scope 文件逐行匹配 patterns_deny，命中即行级 finding；\
         `patterns_allow` 任一模式命中同一行时该行豁免（占位符/示例抑制）。\
         参数见 checks.toml 的 [kinds.text-pattern.<name>]。"
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let mut findings = Vec::new();
        for rel in ctx.scope_files() {
            if !self.applies(rel) {
                continue;
            }
            findings.extend(self.scan(rel, ctx));
        }
        findings
    }
}

impl TextPatternRule {
    fn applies(&self, rel: &Path) -> bool {
        (self.include.is_empty() || self.include.is_match(rel)) && !self.exclude.is_match(rel)
    }

    fn scan(&self, rel: &Path, ctx: &Ctx) -> Vec<Finding> {
        let Ok(text) = ctx.read(rel) else {
            return Vec::new();
        };
        let mut findings = Vec::new();
        for (idx, line) in text.lines().enumerate() {
            if !self.patterns.iter().any(|re| re.is_match(line)) {
                continue;
            }
            if self.allow.iter().any(|re| re.is_match(line)) {
                continue;
            }
            findings.push(Finding::new(
                self.id.as_str(),
                Severity::Warn,
                rel.display().to_string(),
                u32::try_from(idx + 1).unwrap_or(u32::MAX),
                format!("{}: {}", self.message, line.trim()),
            ));
        }
        findings
    }
}

struct TextPatternFactory;

impl KindFactory for TextPatternFactory {
    fn kind(&self) -> &'static str {
        "text-pattern"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: TextPatternParams =
            TextPatternParams::deserialize(toml::Value::Table(params.clone()))
                .map_err(|e| format!("bad params: {e}"))?;
        let include = compile_globs(&parsed.paths_include)?;
        let exclude = compile_globs(&parsed.paths_exclude)?;
        let mut patterns = Vec::new();
        for source in &parsed.patterns_deny {
            let re = regex::Regex::new(source).map_err(|e| format!("bad regex {source:?}: {e}"))?;
            patterns.push(re);
        }
        let mut allow = Vec::new();
        for source in &parsed.patterns_allow {
            let re = regex::Regex::new(source).map_err(|e| format!("bad regex {source:?}: {e}"))?;
            allow.push(re);
        }
        Ok(Box::new(TextPatternRule {
            id: format!("TEXT-PATTERN.{name}"),
            include,
            exclude,
            patterns,
            allow,
            message: parsed.message,
        }))
    }
}

fn compile_globs(patterns: &[String]) -> Result<globset::GlobSet, String> {
    let mut builder = globset::GlobSet::builder();
    for pattern in patterns {
        let glob = globset::GlobBuilder::new(pattern)
            .literal_separator(true)
            .build()
            .map_err(|e| format!("bad glob {pattern:?}: {e}"))?;
        builder.add(glob);
    }
    builder.build().map_err(|e| format!("bad globs: {e}"))
}

// ---------------------------------------------------------------------------
// name-pattern：跟踪文件里，命中 paths_include 的**文件名**必须匹配 pattern
// （全仓不变量，line=0）。用于文件命名规范（snake_case、*_test.rs 等）。

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NamePatternParams {
    /// 参与命名的跟踪文件 glob（相对仓库根）。
    paths_include: Vec<String>,
    #[serde(default)]
    paths_exclude: Vec<String>,
    /// 文件名（不含目录）必须匹配的正则。
    pattern: String,
    message: String,
}

struct NamePatternRule {
    id: String,
    include: globset::GlobSet,
    exclude: globset::GlobSet,
    pattern: regex::Regex,
    message: String,
}

impl Rule for NamePatternRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "文件命名规范"
    }

    fn doc(&self) -> &str {
        "跟踪文件命中 paths_include（且不在 paths_exclude）时，文件名必须匹配 \
         pattern 正则，否则 finding；全仓不变量。参数见 checks.toml 的 \
         [kinds.name-pattern.<name>]。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Fail
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        ctx.tracked_files()
            .iter()
            .filter(|rel| self.applies(rel))
            .filter_map(|rel| self.check_name(rel))
            .collect()
    }
}

impl NamePatternRule {
    fn applies(&self, rel: &Path) -> bool {
        self.include.is_match(rel) && !self.exclude.is_match(rel)
    }

    fn check_name(&self, rel: &Path) -> Option<Finding> {
        let name = rel.file_name()?.to_str()?;
        if self.pattern.is_match(name) {
            return None;
        }
        Some(Finding::new(
            self.id.as_str(),
            Severity::Fail,
            rel.display().to_string(),
            0,
            format!(
                "{}（文件名 {name} 不匹配 {}）",
                self.message,
                self.pattern.as_str()
            ),
        ))
    }
}

struct NamePatternFactory;

impl KindFactory for NamePatternFactory {
    fn kind(&self) -> &'static str {
        "name-pattern"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: NamePatternParams =
            NamePatternParams::deserialize(toml::Value::Table(params.clone()))
                .map_err(|e| format!("bad params: {e}"))?;
        let include = compile_globs(&parsed.paths_include)?;
        let exclude = compile_globs(&parsed.paths_exclude)?;
        let pattern = regex::Regex::new(&parsed.pattern)
            .map_err(|e| format!("bad regex {:?}: {e}", parsed.pattern))?;
        Ok(Box::new(NamePatternRule {
            id: format!("NAME-PATTERN.{name}"),
            include,
            exclude,
            pattern,
            message: parsed.message,
        }))
    }
}

// ---------------------------------------------------------------------------
// duplicate-file：变更文件与全仓既有文件的 token 集合相似度（Jaccard）≥ 阈值
// → 一条 advisory（复用/复制粘贴提醒）。口径与限制见 doc()；默认 WARN。

/// 默认停用词：Rust 关键字、基本类型与各仓都在用的通用名。
/// 不含这些 token，两个无关 Rust 文件也会被 `pub fn self` 顶到虚高相似度。
const DEFAULT_STOPWORDS: &[&str] = &[
    "async", "await", "bool", "box", "break", "char", "clone", "collect", "const", "continue",
    "crate", "debug", "dyn", "else", "enum", "err", "expect", "extern", "false", "f32", "f64",
    "filter", "fmt", "fn", "for", "from", "i128", "i16", "i32", "i64", "i8", "if", "impl", "in",
    "insert", "into", "isize", "iter", "len", "let", "loop", "map", "match", "mod", "move", "mut",
    "name", "new", "none", "ok", "option", "path", "pub", "push", "ref", "result", "return",
    "self", "some", "static", "str", "string", "struct", "super", "this", "trait", "true", "type",
    "u128", "u16", "u32", "u64", "u8", "unsafe", "unwrap", "use", "usize", "value", "vec", "where",
    "while", "with",
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DuplicateFileParams {
    #[serde(default)]
    paths_include: Vec<String>,
    #[serde(default)]
    paths_exclude: Vec<String>,
    /// 少于该 token 数的文件不参评（小文件/样板噪声）。
    #[serde(default = "default_dup_min_tokens")]
    min_tokens: usize,
    /// Jaccard 相似度阈值，(0,1]。
    #[serde(default = "default_dup_min_similarity")]
    min_similarity: f64,
    /// 每个 scope 文件最多报几条（按相似度降序）。
    #[serde(default = "default_dup_limit")]
    limit: usize,
    /// 覆盖默认停用词表。
    #[serde(default)]
    stopwords: Option<Vec<String>>,
    message: String,
}

fn default_dup_min_tokens() -> usize {
    80
}

fn default_dup_min_similarity() -> f64 {
    0.85
}

fn default_dup_limit() -> usize {
    5
}

struct DuplicateFileRule {
    id: String,
    include: globset::GlobSet,
    exclude: globset::GlobSet,
    min_tokens: usize,
    min_similarity: f64,
    limit: usize,
    stopwords: BTreeSet<String>,
    message: String,
}

impl Rule for DuplicateFileRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "变更文件与既有文件 token 相似度过高"
    }

    fn doc(&self) -> &str {
        "把 scope 文件与全仓既有文件都切成 3+ 字符的标识符 token 集合（去停用词），\
         Jaccard 相似度 ≥ min_similarity 即报 WARN，附最相似的既有文件路径。\
         只做提示不做拦截：命中≠必须改，但复用既有实现通常更优。\
         参数见 checks.toml 的 [kinds.duplicate-file.<name>]（min_tokens / \
         min_similarity / limit / stopwords）。"
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let corpus = self.corpus(ctx);
        let mut findings = Vec::new();
        for rel in ctx.scope_files() {
            if !self.applies(rel) {
                continue;
            }
            findings.extend(self.scan_one(rel, ctx, &corpus));
        }
        findings
    }
}

impl DuplicateFileRule {
    fn applies(&self, rel: &Path) -> bool {
        (self.include.is_empty() || self.include.is_match(rel)) && !self.exclude.is_match(rel)
    }

    /// 全仓语料：每个可评文件的 token 集合。
    fn corpus(&self, ctx: &Ctx) -> Vec<(std::path::PathBuf, BTreeSet<String>)> {
        ctx.tracked_files()
            .iter()
            .filter(|rel| self.applies(rel))
            .filter_map(|rel| {
                let text = ctx.read(rel).ok()?;
                let tokens = self.prune(text);
                (tokens.len() >= self.min_tokens).then(|| (rel.clone(), tokens))
            })
            .collect()
    }

    fn scan_one(
        &self,
        rel: &Path,
        ctx: &Ctx,
        corpus: &[(std::path::PathBuf, BTreeSet<String>)],
    ) -> Vec<Finding> {
        let Ok(text) = ctx.read(rel) else {
            return Vec::new();
        };
        let tokens = self.prune(text);
        if tokens.len() < self.min_tokens {
            return Vec::new();
        }
        let mut hits: Vec<(f64, &Path)> = corpus
            .iter()
            .filter(|(other, _)| other != rel)
            .map(|(other, set)| (jaccard(&tokens, set), other.as_path()))
            .filter(|(sim, _)| *sim >= self.min_similarity)
            .collect();
        hits.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        hits.truncate(self.limit);
        hits.into_iter()
            .map(|(sim, other)| {
                Finding::new(
                    self.id.as_str(),
                    Severity::Warn,
                    rel.display().to_string(),
                    0,
                    format!(
                        "{}：与 {} 相似度 {:.2}（{} tokens）",
                        self.message,
                        other.display(),
                        sim,
                        tokens.len()
                    ),
                )
            })
            .collect()
    }

    /// 标识符 token 集合，去停用词与短 token。
    fn prune(&self, text: String) -> BTreeSet<String> {
        let mut set = BTreeSet::new();
        let mut cur = String::new();
        for ch in text.chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                cur.push(ch.to_ascii_lowercase());
            } else {
                flush_token(&mut cur, &mut set);
            }
        }
        flush_token(&mut cur, &mut set);
        set.retain(|t| !self.stopwords.contains(t));
        set
    }
}

fn flush_token(cur: &mut String, set: &mut BTreeSet<String>) {
    if cur.len() >= 3 && !cur.starts_with(|c: char| c.is_ascii_digit()) {
        set.insert(std::mem::take(cur));
    } else {
        cur.clear();
    }
}

fn jaccard(a: &BTreeSet<String>, b: &BTreeSet<String>) -> f64 {
    let inter = a.intersection(b).count();
    let union = a.len() + b.len() - inter;
    if union == 0 {
        return 0.0;
    }
    inter as f64 / union as f64
}

struct DuplicateFileFactory;

impl KindFactory for DuplicateFileFactory {
    fn kind(&self) -> &'static str {
        "duplicate-file"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: DuplicateFileParams =
            DuplicateFileParams::deserialize(toml::Value::Table(params.clone()))
                .map_err(|e| format!("bad params: {e}"))?;
        if parsed.min_similarity <= 0.0 || parsed.min_similarity > 1.0 {
            return Err(format!(
                "min_similarity 必须在 (0,1]，实得 {}",
                parsed.min_similarity
            ));
        }
        if parsed.min_tokens == 0 {
            return Err("min_tokens 必须 ≥ 1".to_string());
        }
        let include = compile_globs(&parsed.paths_include)?;
        let exclude = compile_globs(&parsed.paths_exclude)?;
        let stopwords = parsed
            .stopwords
            .map(|words| words.into_iter().map(|w| w.to_ascii_lowercase()).collect())
            .unwrap_or_else(|| DEFAULT_STOPWORDS.iter().map(|w| (*w).to_string()).collect());
        Ok(Box::new(DuplicateFileRule {
            id: format!("DUPLICATE-FILE.{name}"),
            include,
            exclude,
            min_tokens: parsed.min_tokens,
            min_similarity: parsed.min_similarity,
            limit: parsed.limit,
            stopwords,
            message: parsed.message,
        }))
    }
}
