//! Finding 与分级词汇；JSON 形状与 canon checklist 引擎的 finding 契约同构。

use serde::{Deserialize, Serialize};

/// 执法类别（jev-code 命名法）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Class {
    /// 规则代码可判对错：AST / metadata / 文件系统证据。
    Deterministic,
    /// 需要语境或意图判断，由决策模型给出概率、阈值决定去留。
    Semantic,
    /// 人与外部流程的契约（PR / review），本工具明确不查。
    Process,
    /// 外部工具输出的翻译（clippy / machete / fmt / lizard）。
    Advisory,
}

impl std::str::FromStr for Class {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "deterministic" => Ok(Self::Deterministic),
            "semantic" => Ok(Self::Semantic),
            "process" => Ok(Self::Process),
            "advisory" => Ok(Self::Advisory),
            other => Err(format!(
                "unknown class {other:?} (deterministic|semantic|process|advisory)"
            )),
        }
    }
}

/// 建议严重度；生效值由成员仓策略与 canon dispatch 覆盖链决定。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warn,
    Fail,
}

/// 一条发现。id 形如 `TEXT-PATTERN.no-dbg`（kind 实例）或 `RSX.one-rsx`（专用规则）。
#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub id: String,
    pub severity: Severity,
    pub path: String,
    pub line: u32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

impl Finding {
    pub fn new(
        id: impl Into<String>,
        severity: Severity,
        path: impl Into<String>,
        line: u32,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            severity,
            path: path.into(),
            line,
            message: message.into(),
            suggestion: None,
        }
    }

    /// 附加修复建议（explain / 报告里展示给修复方）。
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}
