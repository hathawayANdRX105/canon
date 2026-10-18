//! Integration tests — the read-only entrypoints (`canon list` / `canon pr check`).
//!
//! 这两条命令存在的理由是「别让作者靠撞闸门才知道规矩」，所以测试钉的是同一件事的
//! 两端：清单**列得出**（且与执行面同源），本地预检**与线上判定一致**（同一条
//! `check_content`，不另写一套判据）。一旦有人把它们改成第二次目录扫描或简化版判据，
//! 就会出现「本地过、线上拦」——那比没有这条命令更坏。

use std::io::Write;
use std::path::PathBuf;

use gate::tools::{list, pr_check};

/// 一份满足全部必需 heading 的 PR 正文（模板口径：六段 + Construction plan 两个 checkbox）。
const COMPLETE_BODY: &str = "\
## What

改了 A。

## Why

不修就会 B。

## Issue

Related: —

## Construction plan

- [x] 第一步
- [x] 第二步

## Delivery record

- Delivered: A
- Verification: 跑了测试
- Follow-up: none

## How to test

1. 跑 `just test-fast`

## Checklist

- [x] 已打类型 label
- [x] 已跑本地预检
";

fn write_temp(name: &str, body: &str) -> PathBuf {
    let path = std::env::temp_dir().join(name);
    let mut f = std::fs::File::create(&path).expect("create temp body");
    f.write_all(body.as_bytes()).expect("write temp body");
    path
}

/// 清单必须列出真实存在的规则，且字段齐备——`canon list` 是作者的入口，
/// 列不出来等于那条命令没有意义。
#[test]
fn list_carries_the_full_catalog_with_required_fields() {
    let (_, rules) = list::collect(None).expect("规范目录必须可发现（.githooks/spec 或 specs/）");
    assert!(
        rules.len() > 40,
        "规则包应有数十条规则，实得 {}",
        rules.len()
    );

    // 每条规则的「查什么 / 级别 / hooks」都不能整列为空：读者靠这三列决定要不要
    // 先修再提交，空列会让人以为那条规则不需要理。
    for rule in &rules {
        assert!(!rule.id.is_empty(), "规则缺 id: {}", rule.source);
        assert!(
            !list::description_of(rule).is_empty(),
            "规则 {} 无描述（yaml description 与头注都空）",
            rule.id
        );
    }
    assert!(
        rules.iter().any(|r| !r.hooks.is_empty()),
        "至少应有规则声明了 hooks，否则清单无法回答「拦在哪个阶段」"
    );
}

/// `--sla` 是收窄器，不是过滤器：它只能让集合变小，且结果全在该档以内。
#[test]
fn list_sla_filter_narrows_and_stays_within_tier() {
    let (_, all) = list::collect(None).expect("规范目录必须可发现");
    let l1 = list::filter_by_sla(all.clone(), Some(gate::engine::SlaLevel::L1));

    assert!(l1.len() <= all.len(), "l1 视图不应比全量还多");
    assert!(!l1.is_empty(), "l1 是默认档，不该为空");
    for rule in &l1 {
        let sla = rule.sla.as_deref().unwrap_or("l1");
        assert_eq!(sla, "l1", "规则 {} 不该出现在 l1 视图", rule.id);
    }
}

/// 本地预检要与线上同判：缺必需 heading 必须报 FAIL（这是 PR-03 的契约），
/// 且一次列全——作者最怕的是「改一条报一条」。
#[test]
fn pr_check_reports_every_missing_heading_at_once() {
    let body = write_temp("canon-pr-check-incomplete.md", "## What\n\n只有一段。\n");
    let code = pr_check::run(&body, "chore(ci): x", &["chore".to_string()], "main");
    assert_eq!(code, 1, "缺 heading 应判 FAIL（退出码 1）");
}

/// 满足模板口径的正文必须干净通过：这条命令若连合格正文都拦，作者就会学会忽略它。
#[test]
fn pr_check_passes_a_template_conformant_body() {
    let body = write_temp("canon-pr-check-complete.md", COMPLETE_BODY);
    let code = pr_check::run(
        &body,
        "chore(ci): add the local preflight",
        &["chore".to_string()],
        "main",
    );
    assert_eq!(code, 0, "模板口径的完整正文不该有 FAIL");
}
