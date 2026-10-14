//! 行级 CSS 块扫描：列首选择器开块、列首 `}` 收块。
//!
//! 口径与既有 shell harness 一致（`index($0, ":root {") == 1` / `^\}`）：
//! 缩进的选择器与嵌套块不参与顶层扫描——theme.css 的 token 块都是列首写法。

use regex::Regex;

/// 一个顶层块：选择器行 + 块体（含起止行号，1 起）。
#[derive(Debug, Clone)]
pub struct CssBlock {
    pub selector: String,
    pub body: String,
    pub start_line: usize,
}

/// 取所有「列首 `选择器 {`」开头的块；`}` 必须列首才收块。
pub fn top_level_blocks(text: &str) -> Vec<CssBlock> {
    let mut blocks = Vec::new();
    let mut current: Option<(String, String, usize)> = None;
    for (idx, line) in text.lines().enumerate() {
        if let Some((_, body, _)) = current.as_mut() {
            if line.starts_with('}') {
                if let Some((selector, body, start)) = current.take() {
                    blocks.push(CssBlock {
                        selector,
                        body,
                        start_line: start,
                    });
                }
                continue;
            }
            body.push('\n');
            body.push_str(line);
            continue;
        }
        if let Some(selector) = open_selector(line) {
            current = Some((selector, String::new(), idx + 1));
        }
    }
    // 未闭合的块不产块（容忍残缺文件）
    blocks
}

/// 列首 `选择器 {` → 选择器（去掉尾部 `{`）；否则 None。
fn open_selector(line: &str) -> Option<String> {
    let trimmed = line.trim_end();
    let head = trimmed.strip_suffix('{')?;
    if head.trim().is_empty() {
        return None;
    }
    // 只认「列首」：原行不得有前导空白
    if line.starts_with(char::is_whitespace) {
        return None;
    }
    Some(head.trim().to_string())
}

/// 块体里出现的 `--token` 名字集合（去重、保持首次出现顺序）。
pub fn custom_properties(block: &CssBlock) -> Vec<String> {
    let re = Regex::new(r"--[A-Za-z0-9_-]+").expect("static regex");
    let mut seen: Vec<String> = Vec::new();
    for m in re.find_iter(&block.body) {
        let name = m.as_str().to_string();
        if !seen.contains(&name) {
            seen.push(name);
        }
    }
    seen
}

/// 按选择器前缀找第一个块（精确匹配选择器文本）。
pub fn first_block<'a>(blocks: &'a [CssBlock], selector: &str) -> Option<&'a CssBlock> {
    blocks.iter().find(|b| b.selector == selector)
}

/// 语义 `:root` 块：第一个块体含 `marker` 的 `:root` 块，缺则回落第一个 `:root`。
pub fn semantic_block<'a>(
    blocks: &'a [CssBlock],
    selector: &str,
    marker: &str,
) -> Option<&'a CssBlock> {
    let candidates: Vec<&CssBlock> = blocks.iter().filter(|b| b.selector == selector).collect();
    candidates
        .iter()
        .find(|b| b.body.contains(marker))
        .copied()
        .or_else(|| candidates.first().copied())
}
