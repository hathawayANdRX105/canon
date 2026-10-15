//! git 依赖钉版：`branch=` 钉版的 git 依赖即 WARN（branch 漂移 → 构建不可复现，
//! 且上游 push 会静默改变依赖内容）；tag / rev 才是可复现钉版。
//!
//! 典型场景：成员仓依赖 canon 规则库（D6 决策「tag 钉版，不用分支」）。
//! advisory 定位——既有仓可能有正当的 branch 依赖（内部跟随仓），由
//! checks.toml 的 scope / severity 覆盖链收口。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

use crate::metadata;

pub struct GitDepPin;

impl Rule for GitDepPin {
    fn id(&self) -> &str {
        "DEPS.git-dep-branch"
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "git 依赖不得用 branch 钉版"
    }

    fn doc(&self) -> &str {
        "Cargo.toml 里 git 源依赖用 branch= 钉版即 WARN：branch 漂移会让构建\
         不可复现，且上游 push 静默改变依赖内容。tag= / rev= 才是可复现钉版\
         （D6：规则库依赖一律 tag）。参数：无（全量判定，scope 走 checks.toml \
         覆盖链）。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let Some(deps) = metadata::git_deps(ctx.root()) else {
            return Vec::new();
        };
        deps.into_iter()
            .filter(|d| !d.pin.is_pinned())
            .map(|d| {
                Finding::new(
                    self.id(),
                    Severity::Warn,
                    format!("{}/Cargo.toml", d.package_dir),
                    0,
                    format!(
                        "git 依赖 {} → {} 用 {} 钉版（branch 漂移）；改用 tag 或 rev",
                        d.dep,
                        d.url,
                        d.pin.describe()
                    ),
                )
            })
            .collect()
    }
}
