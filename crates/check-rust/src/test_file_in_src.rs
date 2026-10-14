//! 独立测试文件不得住在 crate 的 src/ 下。

use std::path::Path;

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

pub struct TestFileInSrc;

impl Rule for TestFileInSrc {
    fn id(&self) -> &str {
        "RUST.test-file-in-src"
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "测试文件不在 src/"
    }

    fn doc(&self) -> &str {
        "独立测试文件（*_test.rs）不应放在 crate 的 src/ 下——单测写进 \
         `#[cfg(test)]` mod，集成测试放同层 `tests/`；对应 canon \
         rust_tests_in_tests_dir / FR-04 的意图，改为 Rust 实现并修正了硬编码路径的假绿。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        ctx.scope_files()
            .iter()
            .filter(|rel| in_src_test_file(rel))
            .map(|rel| {
                Finding::new(
                    self.id(),
                    Severity::Warn,
                    rel.display().to_string(),
                    0,
                    "独立测试文件住在 src/；移到同层 tests/ 或合并为 #[cfg(test)] mod",
                )
                .with_suggestion("mv src/foo_test.rs tests/foo.rs，单测合并进 #[cfg(test)]")
            })
            .collect()
    }
}

fn in_src_test_file(rel: &Path) -> bool {
    let name = rel.file_name().and_then(|n| n.to_str()).unwrap_or_default();
    name.ends_with("_test.rs")
        && rel
            .components()
            .filter_map(|c| c.as_os_str().to_str())
            .any(|c| c == "src")
}
