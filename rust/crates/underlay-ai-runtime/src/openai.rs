use async_trait::async_trait;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    AiErrorKind, AiRuntimeError, LlmClient, LlmRequest, LlmResponse, ResolvedModelRoute, TokenUsage,
};

#[derive(Debug, Clone)]
pub struct OpenAiCompatibleClient {
    http: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl OpenAiCompatibleClient {
    pub fn new(base_url: &str, api_key: &str) -> Result<Self, AiRuntimeError> {
        let base_url = base_url.trim().trim_end_matches('/').to_string();
        if base_url.is_empty() {
            return Err(AiRuntimeError::new(
                AiErrorKind::Validation,
                "base_url is required",
            ));
        }

        let api_key = api_key.trim().to_string();
        if api_key.is_empty() {
            return Err(AiRuntimeError::new(
                AiErrorKind::Validation,
                "api_key is required",
            ));
        }

        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(45))
            .build()
            .map_err(|e| {
                AiRuntimeError::new(
                    AiErrorKind::Unknown,
                    format!("HTTP client init failed: {e}"),
                )
            })?;

        Ok(Self {
            http,
            base_url,
            api_key,
        })
    }
}

#[derive(Debug, Serialize)]
struct ChatCompletionsRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
    temperature: f32,
    max_tokens: u32,
    response_format: ChatResponseFormat<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<Value>,
}

#[derive(Debug, Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatResponseFormat<'a> {
    #[serde(rename = "type")]
    kind: &'a str,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionsResponse {
    choices: Vec<ChatChoice>,
    usage: Option<ChatUsage>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    finish_reason: Option<String>,
    message: ChatChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct ChatChoiceMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatUsage {
    prompt_tokens: Option<u32>,
    completion_tokens: Option<u32>,
}

#[async_trait]
impl LlmClient for OpenAiCompatibleClient {
    async fn generate_structured(
        &self,
        route: &ResolvedModelRoute,
        request: &LlmRequest,
    ) -> Result<LlmResponse, AiRuntimeError> {
        let payload = ChatCompletionsRequest {
            model: route.model_name.as_str(),
            messages: vec![
                ChatMessage {
                    role: "system",
                    content: request.system_prompt.clone(),
                },
                ChatMessage {
                    role: "user",
                    content: serde_json::to_string_pretty(&request.user_payload).map_err(|e| {
                        AiRuntimeError::new(
                            AiErrorKind::Validation,
                            format!("Failed to serialize user payload: {e}"),
                        )
                    })?,
                },
            ],
            temperature: request.temperature,
            max_tokens: request.max_output_tokens,
            response_format: ChatResponseFormat {
                kind: "json_object",
            },
            provider: safe_provider_metadata(route.provider_metadata.as_ref()),
        };

        let endpoint = format!("{}/chat/completions", self.base_url);
        let start = std::time::Instant::now();
        let response = self
            .http
            .post(&endpoint)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    AiRuntimeError::new(AiErrorKind::Timeout, format!("AI request timed out: {e}"))
                } else {
                    AiRuntimeError::new(AiErrorKind::Provider, format!("AI request failed: {e}"))
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            let kind = map_http_status_to_error_kind(status);
            let request_id = response
                .headers()
                .get("x-request-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());
            return Err(AiRuntimeError::new(
                kind,
                match request_id {
                    Some(id) => format!(
                        "AI provider returned {} (request_id: {}). Response body is redacted by default.",
                        status, id
                    ),
                    None => format!(
                        "AI provider returned {}. Response body is redacted by default.",
                        status
                    ),
                },
            ));
        }

        let parsed: ChatCompletionsResponse = response.json().await.map_err(|e| {
            AiRuntimeError::new(
                AiErrorKind::Validation,
                format!("Failed to parse AI response JSON: {e}"),
            )
        })?;

        let choice = parsed.choices.first().ok_or_else(|| {
            AiRuntimeError::new(AiErrorKind::Validation, "AI response contained no choices")
        })?;
        let content = choice.message.content.clone().ok_or_else(|| {
            AiRuntimeError::new(AiErrorKind::Validation, "AI response content missing")
        })?;

        let structured_output: Value = serde_json::from_str(&content).map_err(|e| {
            AiRuntimeError::new(
                AiErrorKind::Validation,
                format!("AI structured output is not valid JSON: {e}"),
            )
        })?;

        Ok(LlmResponse {
            text: Some(content),
            structured_output,
            usage: parsed.usage.map(|u| TokenUsage {
                input_tokens: u.prompt_tokens.unwrap_or(0),
                output_tokens: u.completion_tokens.unwrap_or(0),
            }),
            latency_ms: start.elapsed().as_millis() as u64,
            finish_reason: choice.finish_reason.clone(),
        })
    }
}

pub(crate) fn map_http_status_to_error_kind(status: StatusCode) -> AiErrorKind {
    match status {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => AiErrorKind::Auth,
        StatusCode::TOO_MANY_REQUESTS => AiErrorKind::RateLimit,
        StatusCode::REQUEST_TIMEOUT | StatusCode::GATEWAY_TIMEOUT => AiErrorKind::Timeout,
        _ if status.is_server_error() => AiErrorKind::Provider,
        _ => AiErrorKind::Unknown,
    }
}

pub(crate) fn safe_provider_metadata(metadata: Option<&Value>) -> Option<Value> {
    let metadata = metadata?;
    let obj = metadata.as_object()?;

    let allowed_keys = ["provider", "routing", "transforms", "order"];
    let mut out = serde_json::Map::new();
    for key in allowed_keys {
        if let Some(value) = obj.get(key) {
            out.insert(key.to_string(), value.clone());
        }
    }

    if out.is_empty() {
        None
    } else {
        Some(Value::Object(out))
    }
}
