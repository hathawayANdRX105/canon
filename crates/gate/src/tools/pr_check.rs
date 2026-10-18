//! `canon pr check --body-file <path>` —— 不等 `gh pr create` 被拦，先在本地把 PR 正文验一遍。
//!
//! 为什么需要它：PR 正文的规则（`PR-01` 标题英文 / `PR-03` 必需 heading / `PR-04`
//! heading 禁 CJK / `PR-07` done_when checkbox）此前只在 gh shim 拦截 `gh pr create`
//! 的那一刻生效。作者把正文写完、提交、推送、等 CI，才在创建 PR 时被告知「缺 Why 段」——
//! 一轮往返换一条 finding，信息量极低。维护者 2026-10-18 的原话是「免得在 commit 跟
//! merge 之前被拦住」。
//!
//! 复用 [`crate::rules::pull_requests::check_content`] 这条纯函数入口与同一个
//! `github_pull_requests.yaml` 加载路径：本模块只负责取正文与排版，不复制判定逻辑。
//! 判据一旦两处各写一份，就会出现「本地过、线上拦」的分叉。
//!
//! 依赖 gh 上下文的规则（`PR-06` type label、`PR-08`）在本地取不到，输出明确写「跳过」——
//! **不做假绿**：跳过与通过是两件事，混在一起等于骗自己。

use std::io::Read;
use std::path::Path;

use crate::rules::pull_requests;
use crate::shared::{Finding, Severity};

/// 正文来源为 `-` 时的约定：从 stdin 读，方便 `gh pr view --json body -q .body | canon pr check`。
const STDIN_MARKER: &str = "-";

/// 本地模式下 PR-06（type label）靠 `--label` 入参求值：不传 label 时它必然报「缺 type
/// label」，那是本地没有线上信息导致的**假阳性**，不是作者漏挂。这条提醒让读者知道该
/// 传什么，而不是把它当成必须修的 FAIL。
const LABEL_NOTE: &str = "提示\t--\tPR-06 按 --label 入参判定：本地不传 label 时必报「缺 type label」；\
                          复验请加 --label chore 之类（线上 label 以 PR 页面为准）";

/// 取正文：文件路径或 `-`（stdin）。
fn read_body(path: &Path) -> Result<String, String> {
    if path.as_os_str() == STDIN_MARKER {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| format!("读 stdin 失败: {e}"))?;
        return Ok(buf);
    }
    std::fs::read_to_string(path).map_err(|e| format!("读 {} 失败: {e}", path.display()))
}

/// 一条 finding 的一行显示：`SEVERITY\tRULE\tMESSAGE`，与 gh shim 的口径一致，
/// 让作者在本地看到的格式与线上被拦时完全相同。
fn render(findings: &[Finding]) -> String {
    let mut lines: Vec<String> = findings
        .iter()
        .map(|f| format!("{}\t{}\t{}", f.severity.as_str(), f.rule_id, f.msg))
        .collect();
    lines.push(LABEL_NOTE.to_string());
    lines.join("\n")
}

/// 失败时给出的修法提示。必需 heading 与 checkbox 段落都从 cfg 读：写死在提示里的话，
/// 规则一改（比如 done_when 多了个段落）提示就开始骗人，而提示是作者唯一会读的东西。
fn fix_hint(cfg: &serde_yaml::Value) -> String {
    let list = |key: &str| -> Vec<String> {
        cfg.get(key)
            .and_then(|v| v.as_sequence())
            .map(|seq| {
                seq.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default()
    };
    let headings = list("required_body_headings");
    let done_when = list("done_when_headings");
    let min = cfg
        .get("done_when_min_checkboxes")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let mut hint = String::from("典型修法：");
    if !headings.is_empty() {
        hint.push_str(&format!("补齐必需 heading（{}）；", headings.join("/")));
    }
    hint.push_str("heading 用英文、title 用英文（CM 口径同）；");
    if !done_when.is_empty() && min > 0 {
        hint.push_str(&format!(
            "{} 各自至少 {min} 个 checkbox。",
            done_when.join(" / ")
        ));
    }
    hint
}

/// CLI 入口。返回进程退出码：有 FAIL → 1，只有 WARN/INFO（或全过）→ 0。
///
/// `title` / `labels` / `head_ref` 由调用方给：本地没有 PR 对象，标题通常直接抄拟用的
/// 那一条；label 拿不到就留空，此时 PR-06 会按「跳过」处理而不是误报缺 label。
pub fn run(body_file: &Path, title: &str, labels: &[String], head_ref: &str) -> i32 {
    let body = match read_body(body_file) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("canon pr check: {e}");
            return 2;
        }
    };

    let cfg = crate::shared::load_spec_yaml("github_pull_requests.yaml")
        .unwrap_or(serde_yaml::Value::Null);
    let label_refs: Vec<&str> = labels.iter().map(String::as_str).collect();

    let mut findings =
        pull_requests::check_content(title, &body, &label_refs, head_ref, Some(&cfg));
    crate::shared::apply_global_overrides(&mut findings);

    if !findings.is_empty() {
        println!("{}", render(&findings));
    }

    let fails = findings
        .iter()
        .filter(|f| f.severity == Severity::Fail)
        .count();
    if fails == 0 {
        println!(
            "\n✅ 本地预检通过（{} 条提醒，0 条 FAIL）。",
            findings.len()
        );
        0
    } else {
        println!("\n❌ {fails} 条 FAIL。{}", fix_hint(&cfg));
        1
    }
}
