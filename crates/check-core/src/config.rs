//! checks.toml 的类型化读取。deny_unknown_fields：拼错键硬报，禁静默开关。

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

use crate::finding::Severity;

/// 成员仓策略文件；文件缺失视为空配置（纯缺省）。
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChecksConfig {
    /// semantic 类所需的决策端点（laya / 托管 jev 同协议）；缺省 = semantic 全部 notChecked。
    #[serde(default)]
    pub semantic: Option<SemanticEndpoint>,
    /// 按规则 id 覆盖 severity / enabled。
    #[serde(default)]
    pub rules: BTreeMap<String, RuleOverride>,
    /// 参数化实例：kind -> 实例名 -> 参数表（D9：同一实现，多实例）。
    #[serde(default)]
    pub kinds: BTreeMap<String, BTreeMap<String, toml::Table>>,
    /// 语义路径表（配置特制/泛化拆分）：仓内布局的单一事实源。
    /// [rules.<id>] 的 scope_include/exclude 条目写 `@frontend`，
    /// load() 时展开为 domains.frontend 的 glob 列表——仓库改名只改这里。
    #[serde(default)]
    pub domains: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticEndpoint {
    pub endpoint: String,
    #[serde(default = "default_model")]
    pub model: String,
    /// 决策请求带上的 token 预算；laya 默认窗口远小于此，不带会静默截断。
    #[serde(default = "default_max_len")]
    pub max_len: u32,
}

fn default_model() -> String {
    "jev-latest".to_string()
}

fn default_max_len() -> u32 {
    8192
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuleOverride {
    pub severity: Option<Severity>,
    /// 缺省即启用；只有显式 `enabled = false` 才停用（禁静默开关）。
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// 策略层 scope 覆盖：只跑这些目录下的文件（glob，相对仓库根）。
    #[serde(default)]
    pub scope_include: Vec<String>,
    /// 策略层 scope 排除：命中这些 glob 的文件从检查集剔除。
    #[serde(default)]
    pub scope_exclude: Vec<String>,
}

fn default_true() -> bool {
    true
}

/// 配置错误：check / list / explain 一律 exit 2。
#[derive(Debug)]
pub struct ConfigError {
    pub path: std::path::PathBuf,
    pub message: String,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.message)
    }
}

/// 读取 checks.toml；文件缺失 = 空配置，解析失败 = 错误。
pub fn load(path: &Path) -> Result<ChecksConfig, ConfigError> {
    match std::fs::read_to_string(path) {
        Ok(text) => {
            let mut cfg: ChecksConfig = toml::from_str(&text).map_err(|e| ConfigError {
                path: path.to_path_buf(),
                message: e.to_string(),
            })?;
            expand_domains(&mut cfg).map_err(|message| ConfigError {
                path: path.to_path_buf(),
                message,
            })?;
            Ok(cfg)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(ChecksConfig::default()),
        Err(e) => Err(ConfigError {
            path: path.to_path_buf(),
            message: e.to_string(),
        }),
    }
}

/// 展开 [rules.<id>] scope_* 里的 `@<domain>` 标记为 [domains.<domain>] 的
/// glob 列表。单层展开（展开结果里的 `@` 不再递归——嵌套是配置错误，会以
/// 零命中形式被 gate 的 SCOPE-EMPTY 捕获）。未知标记硬报：配置笔误不能
/// 静默变成零命中。
fn expand_domains(cfg: &mut ChecksConfig) -> Result<(), String> {
    let mut expanded: BTreeMap<String, RuleOverride> = BTreeMap::new();
    for (id, rule) in std::mem::take(&mut cfg.rules) {
        let mut rule = rule;
        let map = |entry: String| -> Result<Vec<String>, String> {
            if let Some(marker) = entry.strip_prefix('@') {
                let patterns = cfg.domains.get(marker).ok_or_else(|| {
                    format!(
                        "规则 {id} 引用未定义的路径域 @{marker}——补 [domains.{marker}] 或改回字面 glob"
                    )
                })?;
                Ok(patterns.clone())
            } else {
                Ok(vec![entry])
            }
        };
        let mut inc = Vec::new();
        for e in std::mem::take(&mut rule.scope_include) {
            inc.extend(map(e)?);
        }
        let mut exc = Vec::new();
        for e in std::mem::take(&mut rule.scope_exclude) {
            exc.extend(map(e)?);
        }
        rule.scope_include = inc;
        rule.scope_exclude = exc;
        expanded.insert(id, rule);
    }
    cfg.rules = expanded;
    Ok(())
}
