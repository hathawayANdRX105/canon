//! Step/state 模板：内置三份（backend/frontend/general）+ 用户目录覆盖。
//!
//! 模板是**缺省**，不是约束：`project_create` 接受自定义 `states` 覆盖管道，
//! 项目级覆盖放 `~/.config/canon/templates/<name>.yaml`（不动目标仓）。

use std::path::PathBuf;

use serde::Deserialize;

use crate::flow::FlowError;

#[derive(Debug, Clone)]
pub struct TplStep {
    pub title: String,
    /// `work` / `spec`
    pub kind: String,
    pub spec_ref: Option<String>,
    /// 归属的 state（项目管道里没有该 state 时丢弃 hint，P2 归入当前栏）
    pub state: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    pub states: Vec<String>,
    pub steps: Vec<TplStep>,
}

#[derive(Deserialize)]
struct TemplateYaml {
    states: Vec<String>,
    steps: Vec<TplStepYaml>,
}

#[derive(Deserialize)]
struct TplStepYaml {
    title: String,
    #[serde(default = "default_kind")]
    kind: String,
    spec_ref: Option<String>,
    state: Option<String>,
}

fn default_kind() -> String {
    "work".into()
}

fn parse(name: &str, raw: &str) -> Template {
    let yaml: TemplateYaml =
        serde_yaml::from_str(raw).unwrap_or_else(|e| panic!("flow template {name} malformed: {e}"));
    let states = yaml.states;
    let steps = yaml
        .steps
        .into_iter()
        .map(|s| TplStep {
            title: s.title,
            kind: s.kind,
            spec_ref: s.spec_ref,
            state: s.state,
        })
        .collect();
    Template {
        name: name.to_string(),
        states,
        steps,
    }
}

/// kind → 内置模板（未知 kind 落 general）。
pub fn for_kind(kind: &str) -> Template {
    builtin_templates()
        .into_iter()
        .find(|t| t.name == kind)
        .unwrap_or_else(|| parse("general", include_str!("templates/general.yaml")))
}

pub fn builtin_templates() -> Vec<Template> {
    vec![
        parse("backend", include_str!("templates/backend.yaml")),
        parse("frontend", include_str!("templates/frontend.yaml")),
        parse("general", include_str!("templates/general.yaml")),
    ]
}

/// 用户模板目录（`CANON_FLOW_HOME` 可指别处，测试用）。
pub fn user_dir() -> PathBuf {
    if let Some(p) = std::env::var("CANON_FLOW_HOME")
        .ok()
        .filter(|s| !s.is_empty())
    {
        return PathBuf::from(p);
    }
    let base = std::env::var("XDG_CONFIG_HOME")
        .ok()
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or_else(|| home_dir().join(".config"));
    base.join("canon").join("templates")
}

/// 可用模板名：内置 + 用户目录 `<name>.yaml`。
pub fn available() -> Vec<String> {
    let mut names: Vec<String> = builtin_templates().into_iter().map(|t| t.name).collect();
    if let Ok(rd) = std::fs::read_dir(user_dir()) {
        for e in rd.flatten() {
            if let Some(ext) = e.file_name().to_str().and_then(|s| s.strip_suffix(".yaml")) {
                names.push(ext.to_string());
            }
        }
    }
    names
}

pub fn resolve(name: &str) -> Result<Template, FlowError> {
    if let Some(t) = builtin_templates().into_iter().find(|t| t.name == name) {
        return Ok(t);
    }
    let path = user_dir().join(format!("{name}.yaml"));
    let raw = std::fs::read_to_string(&path)
        .map_err(|_| FlowError::new("TEMPLATE_NOT_FOUND", format!("no template named {name}")))?;
    Ok(parse(name, &raw))
}

fn home_dir() -> PathBuf {
    std::env::var("HOME").map(PathBuf::from).unwrap_or_default()
}
