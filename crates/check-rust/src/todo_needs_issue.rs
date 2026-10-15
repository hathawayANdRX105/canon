//! TODO / FIXME 必须挂 issue 号：`// TODO(#123): …` 或 `todo!("TODO(#123): …")`。
//! 对应 canon rust_todo_needs_issue；注释本是文本（syn AST 不含注释），维持文本口径，
//! 但用 syn 字符串字面量区间过滤「字符串里的 TODO 文案」误报（旧版不滤，改进）。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::rule::Rule;

use crate::attrs;

const KEYWORDS: &[&str] = &["TODO", "FIXME", "todo!(", "unimplemented!("];

pub struct TodoNeedsIssue;

impl Rule for TodoNeedsIssue {
    fn id(&self) -> &str {
        "RUST.todo-needs-issue"
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "TODO/FIXME 必须挂 issue 号"
    }

    fn doc(&self) -> &str {
        "注释含 TODO/FIXME、或 todo!/unimplemented! 宏调用，而未带 `(#<数字>)` \
         可追溯标识即 WARN（例：`// TODO(#123): …`）。字符串字面量里的 TODO 文案\
         不误报（syn 字面量区间过滤）。tests/ 与 *_test.rs 由 checks.toml 覆盖链排除\
         （测试占位不追踪）。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let mut findings = Vec::new();
        for rel in ctx.scope_files() {
            if rel.extension().is_some_and(|e| e != "rs") {
                continue;
            }
            if rel
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with("_test.rs"))
            {
                continue;
            }
            let Some(text) = ctx.read(rel).ok() else {
                continue;
            };
            let lits = attrs::parse(&text).map(|file| attrs::str_lit_spans(&file));
            for (idx, line) in text.lines().enumerate() {
                let line_no = idx + 1;
                if !is_candidate(line) {
                    continue;
                }
                if let Some(lits) = &lits {
                    if let Some(col) = first_hit_col(line) {
                        if lits.iter().any(|s| s.contains(line_no, col)) {
                            continue; // 命中落在字符串字面量内
                        }
                    }
                }
                findings.push(Finding::new(
                    self.id(),
                    Severity::Warn,
                    rel.display().to_string(),
                    u32::try_from(line_no).unwrap_or(u32::MAX),
                    format!("TODO/FIXME 必须挂 issue 号：{}", line.trim()),
                ));
            }
        }
        findings
    }
}

/// 候选：注释里的 TODO/FIXME（无 `(#N)` 标识）或 todo!/unimplemented! 宏（同）。
fn is_candidate(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") && (contains_todo_kw(trimmed) && !has_issue_ref(trimmed)) {
        return true;
    }
    if (trimmed.contains("todo!(") || trimmed.contains("unimplemented!("))
        && !has_issue_ref(trimmed)
    {
        return true;
    }
    false
}

fn contains_todo_kw(s: &str) -> bool {
    KEYWORDS[..2].iter().any(|kw| s.contains(kw))
}

/// `TODO(#123)` / `FIXME(#42)` / `todo!("TODO(#7)…")` 形态算已挂标识。
fn has_issue_ref(s: &str) -> bool {
    for kw in ["TODO", "FIXME"] {
        if let Some(pos) = s.find(kw) {
            let rest = &s[pos + kw.len()..];
            if rest.starts_with("(#")
                && rest[2..].chars().next().is_some_and(|c| c.is_ascii_digit())
            {
                return true;
            }
        }
    }
    false
}

/// 行内最早出现的候选关键词列偏移（字面量区间过滤用）。
fn first_hit_col(line: &str) -> Option<usize> {
    KEYWORDS.iter().filter_map(|kw| line.find(kw)).min()
}
