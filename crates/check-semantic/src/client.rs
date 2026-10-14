//! jev / laya 决策端点客户端：`POST {base}/v1/systemone`（wire 协议对齐
//! TypeSafe Jev；laya 为其本地实现）。
//!
//! 只做一件事：把 `state` + `questions` 发出去，把 `answers` 收回来。
//! 阈值判定在 [`questions::evaluate`]（纯函数、可测）；无端点/请求失败由
//! 调用方（kind 的 run_outcome）转成 notChecked——绝不伪装成 finding。

use std::time::Duration;

use serde_json::{Value, json};

/// 端点调用结果：`answers` 原始 JSON（{qid: {type, noul|score|choice, probabilities}}）。
pub struct Ask<'a> {
    pub base: String,
    pub key: Option<String>,
    pub model: String,
    pub state: String,
    pub questions: Value,
    pub timeout_secs: u64,
    pub _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Ask<'a> {
    pub fn new(base: impl Into<String>, state: impl Into<String>, questions: Value) -> Self {
        Self {
            base: base.into(),
            key: None,
            model: "jev-latest".to_string(),
            state: state.into(),
            questions,
            timeout_secs: 60,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn key(mut self, key: Option<String>) -> Self {
        self.key = key;
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// 发一次请求；返回 `answers` 节点。失败（不可达 / 非 2xx / 非 JSON）返回 Err(reason)。
    pub fn send(self) -> Result<Value, String> {
        let url = format!("{}/v1/systemone", self.base.trim_end_matches('/'));
        let mut req = ureq::post(&url)
            .timeout(Duration::from_secs(self.timeout_secs))
            .set("content-type", "application/json");
        if let Some(key) = &self.key {
            req = req
                .set("authorization", &format!("Bearer {key}"))
                .set("x-api-key", key);
        }
        let body = json!({
            "state": self.state,
            "model": self.model,
            "questions": self.questions,
        });
        match req.send_json(body) {
            Ok(resp) => match resp.into_json::<Value>() {
                Ok(v) => Ok(v.get("answers").cloned().unwrap_or(Value::Null)),
                Err(e) => Err(format!("response not json: {e}")),
            },
            Err(ureq::Error::Status(code, resp)) => {
                let detail = resp.into_string().unwrap_or_default();
                Err(format!(
                    "http {code}: {}",
                    detail.chars().take(200).collect::<String>()
                ))
            }
            Err(ureq::Error::Transport(t)) => Err(format!("unreachable: {t}")),
        }
    }
}

/// 从环境解析 API key：JEV_API_KEY 优先，回落 TYPESAFE_API_KEY（与 canon
/// harness 的 jev_env 同序）；都没有 = None（本地 laya 无需 key）。
pub fn resolve_key() -> Option<String> {
    std::env::var("JEV_API_KEY")
        .ok()
        .or_else(|| std::env::var("TYPESAFE_API_KEY").ok())
        .filter(|k| !k.trim().is_empty())
}
