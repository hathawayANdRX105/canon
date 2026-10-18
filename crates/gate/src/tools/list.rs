//! `canon list` — 规则清单：每条规范「查什么 / 多严重 / 拦在哪个 hook / 哪档 SLA」。
//!
//! 数据来自 [`crate::catalog::load`]，与 `canon check`、MCP `spec_catalog` 同一
//! 条加载路径；本模块只做筛选与排版，不重扫目录。清单与实际执行面若各走各的
//! 扫描，就会长出「列得出、跑不到」的死规则——这正是本仓最忌讳的分叉。
//!
//! `--json` 面向 agent：拿到 id 就能喂给 `canon check <id>`，拿到 severity /
//! hooks 就能自己判断要不要先修再提交。

use std::path::PathBuf;

use crate::catalog::{self, Rule};
use crate::engine::SlaLevel;

/// 头注没有描述时的占位文案。人读表格里留白比空列更难看出「本来就没有」。
const NO_DESCRIPTION: &str = "（无描述）";

/// 表格里「查什么」列的显示宽度上限（半角列宽，CJK 按 2 计）。
const DESC_COLS: usize = 52;

/// 加载规则目录并按 SLA 上限筛选。`max_sla` 的口径与 `canon check --sla`
/// 一致（`l1` = 只看 l1），未声明 sla 的规则按引擎默认值 l1 计入。
pub fn collect(max_sla: Option<SlaLevel>) -> Result<(PathBuf, Vec<Rule>), String> {
    let dir = crate::engine::resolve_spec_dir().ok_or_else(|| {
        "找不到规范目录：项目仓需要 .githooks/spec（canon init 播种），\
         canon 检出需要 specs/"
            .to_string()
    })?;
    let rules = filter_by_sla(catalog::load(&dir), max_sla);
    Ok((dir, rules))
}

/// 按 SLA 上限收窄规则集。抽成独立函数是为了让「收窄只减不增、结果全在档内」这条
/// 契约能被测试直接钉住——它藏在 `collect` 的目录发现后面就没法单独喂输入。
pub fn filter_by_sla(rules: Vec<Rule>, max_sla: Option<SlaLevel>) -> Vec<Rule> {
    rules
        .into_iter()
        .filter(|r| max_sla.is_none_or(|max| sla_of(r) <= max))
        .collect()
}

/// 一条规则的 SLA 档位；yaml 未声明时取引擎的缺省档 l1。
fn sla_of(rule: &Rule) -> SlaLevel {
    rule.sla.as_deref().map(SlaLevel::parse).unwrap_or_default()
}

/// 一条规则的一行说明：yaml `description:` 优先，否则取头注第一句。
pub fn description_of(rule: &Rule) -> String {
    let explicit = rule.description.trim();
    if !explicit.is_empty() {
        return explicit.to_string();
    }
    first_sentence(&rule.why).unwrap_or_else(|| NO_DESCRIPTION.to_string())
}

/// 头注是多行散文（「为什么 / 检测 / 阈值…」），清单只要第一句。
fn first_sentence(why: &str) -> Option<String> {
    let line = why.lines().map(str::trim).find(|l| !l.is_empty())?;
    let cut = line
        .find('。')
        .map(|i| i + '。'.len_utf8())
        .unwrap_or(line.len());
    let head = line[..cut].trim();
    (!head.is_empty()).then(|| head.to_string())
}

/// 终端列宽：CJK 与全角标点占两列，按 1 算会把表格右边缘挤歪。
fn display_width(s: &str) -> usize {
    s.chars().map(|c| if is_wide(c) { 2 } else { 1 }).sum()
}

fn is_wide(c: char) -> bool {
    matches!(c as u32,
        0x1100..=0x115F | 0x2E80..=0xA4CF | 0xAC00..=0xD7A3
        | 0xF900..=0xFAFF | 0xFE30..=0xFE6F | 0xFF00..=0xFF60 | 0xFFE0..=0xFFE6)
}

/// 按显示宽度截断（不是按字节，`truncate_utf8` 之外表格还要补省略号）。
fn clip(s: &str, cols: usize) -> String {
    if display_width(s) <= cols {
        return s.to_string();
    }
    let mut out = String::new();
    let mut width = 0;
    for c in s.chars() {
        let w = if is_wide(c) { 2 } else { 1 };
        if width + w > cols.saturating_sub(1) {
            break;
        }
        out.push(c);
        width += w;
    }
    out.push('…');
    out
}

fn pad(s: &str, cols: usize) -> String {
    let w = display_width(s);
    format!("{s}{}", " ".repeat(cols.saturating_sub(w)))
}

/// 人读表格：ID / 查什么 / 级别 / HOOKS / SLA。
pub fn render_table(dir: &std::path::Path, rules: &[Rule], max_sla: Option<SlaLevel>) -> String {
    let id_w = rules
        .iter()
        .map(|r| display_width(&r.id))
        .chain(std::iter::once("ID".len()))
        .max()
        .unwrap_or(2);
    let hook_w = rules
        .iter()
        .map(|r| display_width(&r.hooks.join(",")))
        .chain(std::iter::once("HOOKS".len()))
        .max()
        .unwrap_or(5);

    let mut out = String::new();
    out.push_str(&format!("规范目录: {}\n", dir.display()));
    out.push_str(&format!(
        "{} 条规范 · SLA {}\n\n",
        rules.len(),
        max_sla.map_or_else(|| "全部".to_string(), |s| format!("≤ {}", sla_label(s)))
    ));
    out.push_str(&format!(
        "{:<id_w$}  {}{}  {}{}  {}\n",
        "ID",
        pad("查什么", DESC_COLS + 2),
        "  ",
        pad("级别", 6),
        pad("HOOKS", hook_w + 2),
        "SLA",
        id_w = id_w,
    ));
    for r in rules {
        out.push_str(&format!(
            "{:<id_w$}  {}{}  {}{}  {}\n",
            r.id,
            pad(&clip(&description_of(r), DESC_COLS), DESC_COLS + 2),
            "  ",
            pad(r.severity.as_str(), 6),
            pad(&r.hooks.join(","), hook_w + 2),
            r.sla.as_deref().unwrap_or("-"),
            id_w = id_w,
        ));
    }
    out.push_str(
        "\n👉 下一步: canon check <ID> 试跑 · canon list --json 机器读 · \
         canon pr check --body-file <正文> 预检 PR 正文\n",
    );
    out
}

fn sla_label(sla: SlaLevel) -> &'static str {
    match sla {
        SlaLevel::L1 => "l1",
        SlaLevel::L2 => "l2",
        SlaLevel::L3 => "l3",
    }
}

/// agent 口径的机器可读清单：每条至少带 id / description / severity / hooks / sla。
pub fn render_json(rules: &[Rule]) -> String {
    let rows: Vec<serde_json::Value> = rules
        .iter()
        .map(|r| {
            serde_json::json!({
                "id": r.id,
                "description": description_of(r),
                "severity": r.severity.as_str(),
                "hooks": r.hooks,
                "sla": r.sla,
                "source": r.source,
            })
        })
        .collect();
    serde_json::to_string_pretty(&rows).unwrap_or_else(|_| "[]".to_string())
}

/// CLI 入口：打印清单，规则目录缺失才算失败（空目录不是「没有规范」）。
pub fn run(json: bool, sla: Option<&str>) -> i32 {
    let max_sla = sla.map(SlaLevel::parse);
    let (dir, rules) = match collect(max_sla) {
        Ok(hit) => hit,
        Err(e) => {
            eprintln!("canon list: {e}");
            return 1;
        }
    };
    if json {
        println!("{}", render_json(&rules));
    } else {
        print!("{}", render_table(&dir, &rules, max_sla));
    }
    0
}
