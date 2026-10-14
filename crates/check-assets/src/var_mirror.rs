//! CSS token 镜像不变量（kind `css-var-mirror`）：`mirror_selector` 块必须是
//! `base_selector` 语义块的**同 token 集合**镜像。
//!
//! 迁移自 mono `dioxus_css_mirror`（CSS-LAYER-DARK-MIRROR）：theme.css 的 `.dark`
//! 段必须与语义 `:root` 块同内容；:root 增删 token 忘同步 .dark = 暗色主题静默破图，
//! 值得硬门（原规则实测 0 违规 → FAIL 零成本）。语义 `:root` 块 = 第一个含
//! `base_marker` 的 base_selector 块（缺则回落第一个）。

use std::path::Path;

use globset::GlobSet;
use serde::Deserialize;

use crate::css_blocks;

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

pub struct CssVarMirrorFactory;

impl check_core::kinds::KindFactory for CssVarMirrorFactory {
    fn kind(&self) -> &'static str {
        "css-var-mirror"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let parsed: CssVarMirrorParams =
            Deserialize::deserialize(toml::Value::Table(params.clone()))
                .map_err(|e| format!("bad params: {e}"))?;
        let include = compile_globs(&parsed.paths_include)?;
        Ok(Box::new(CssVarMirrorRule {
            id: format!("CSS-VAR-MIRROR.{name}"),
            include,
            base_selector: parsed.base_selector,
            mirror_selector: parsed.mirror_selector,
            base_marker: parsed.base_marker,
            message: parsed.message,
        }))
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CssVarMirrorParams {
    /// 参与检查的 CSS 文件 glob（相对仓库根）。
    paths_include: Vec<String>,
    /// 基准选择器（默认 `:root`）。
    #[serde(default = "default_base_selector")]
    base_selector: String,
    /// 镜像选择器（默认 `.dark`）。
    #[serde(default = "default_mirror_selector")]
    mirror_selector: String,
    /// 判定「语义 base 块」的标志 token（默认 `--background`）。
    #[serde(default = "default_base_marker")]
    base_marker: String,
    message: String,
}

fn default_base_selector() -> String {
    ":root".to_string()
}

fn default_mirror_selector() -> String {
    ".dark".to_string()
}

fn default_base_marker() -> String {
    "--background".to_string()
}

fn compile_globs(patterns: &[String]) -> Result<GlobSet, String> {
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

struct CssVarMirrorRule {
    id: String,
    include: GlobSet,
    base_selector: String,
    mirror_selector: String,
    base_marker: String,
    message: String,
}

impl Rule for CssVarMirrorRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "CSS token 镜像一致"
    }

    fn doc(&self) -> &str {
        "命中 paths_include 的 CSS 文件里，镜像选择器块（缺省 `.dark`）的 `--token` \
         集合必须与语义基准块（缺省含 `--background` 的 `:root`）完全一致；\
         缺/多 token 即 finding。无镜像块的文件自动安静。参数见 checks.toml 的 \
         [kinds.css-var-mirror.<name>]。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Fail
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        ctx.tracked_files()
            .iter()
            .filter(|rel| self.include.is_match(rel.as_path()))
            .filter_map(|rel| self.check_file(rel, ctx))
            .collect()
    }
}

impl CssVarMirrorRule {
    fn check_file(&self, rel: &Path, ctx: &Ctx) -> Option<Finding> {
        let text = ctx.read(rel).ok()?;
        let blocks = css_blocks::top_level_blocks(&text);
        let base = css_blocks::semantic_block(&blocks, &self.base_selector, &self.base_marker)?;
        let mirror = css_blocks::first_block(&blocks, &self.mirror_selector)?;
        let base_vars = css_blocks::custom_properties(base);
        let mirror_vars = css_blocks::custom_properties(mirror);
        let missing = count_missing(&base_vars, &mirror_vars);
        let extra = count_missing(&mirror_vars, &base_vars);
        if missing == 0 && extra == 0 {
            return None;
        }
        Some(Finding::new(
            self.id.as_str(),
            Severity::Fail,
            rel.display().to_string(),
            u32::try_from(mirror.start_line).unwrap_or(u32::MAX),
            format!(
                "{}（{} 缺 {missing} 个 / 多 {extra} 个 token）",
                self.message, self.mirror_selector
            ),
        ))
    }
}

/// left 有而 right 没有的 token 个数。
fn count_missing(left: &[String], right: &[String]) -> usize {
    left.iter().filter(|token| !right.contains(token)).count()
}
