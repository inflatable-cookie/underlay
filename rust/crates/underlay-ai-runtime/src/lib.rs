use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use async_trait::async_trait;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiErrorKind {
    Auth,
    RateLimit,
    Timeout,
    Provider,
    Validation,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRuntimeError {
    pub kind: AiErrorKind,
    pub message: String,
}

impl AiRuntimeError {
    pub fn new(kind: AiErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredOutputSpec {
    pub schema_version: String,
    pub strict: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    pub system_prompt: String,
    pub user_payload: Value,
    pub structured_output: StructuredOutputSpec,
    pub temperature: f32,
    pub max_output_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub text: Option<String>,
    pub structured_output: Value,
    pub usage: Option<TokenUsage>,
    pub latency_ms: u64,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedModelRoute {
    pub alias: String,
    pub provider_name: String,
    pub model_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelCapability {
    StructuredJson,
    ToolCalling,
    LongContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedModelRouteCandidate {
    pub route: ResolvedModelRoute,
    pub priority: u16,
    pub capabilities: HashSet<ModelCapability>,
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate_structured(
        &self,
        route: &ResolvedModelRoute,
        request: &LlmRequest,
    ) -> Result<LlmResponse, AiRuntimeError>;
}

#[derive(Clone, Default)]
pub struct ProviderRegistry {
    clients: HashMap<String, Arc<dyn LlmClient>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn register(&mut self, provider_key: impl Into<String>, client: Arc<dyn LlmClient>) {
        self.clients.insert(provider_key.into(), client);
    }

    pub fn get(&self, provider_key: &str) -> Option<Arc<dyn LlmClient>> {
        self.clients.get(provider_key).cloned()
    }
}

pub fn select_route_candidates(
    mut candidates: Vec<ResolvedModelRouteCandidate>,
    required_capabilities: &HashSet<ModelCapability>,
) -> Vec<ResolvedModelRouteCandidate> {
    candidates.retain(|candidate| {
        required_capabilities
            .iter()
            .all(|capability| candidate.capabilities.contains(capability))
    });

    candidates.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then_with(|| a.route.provider_name.cmp(&b.route.provider_name))
            .then_with(|| a.route.model_name.cmp(&b.route.model_name))
    });

    candidates
}

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

fn map_http_status_to_error_kind(status: StatusCode) -> AiErrorKind {
    match status {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => AiErrorKind::Auth,
        StatusCode::TOO_MANY_REQUESTS => AiErrorKind::RateLimit,
        StatusCode::REQUEST_TIMEOUT | StatusCode::GATEWAY_TIMEOUT => AiErrorKind::Timeout,
        _ if status.is_server_error() => AiErrorKind::Provider,
        _ => AiErrorKind::Unknown,
    }
}

fn safe_provider_metadata(metadata: Option<&Value>) -> Option<Value> {
    let Some(metadata) = metadata else {
        return None;
    };

    let Some(obj) = metadata.as_object() else {
        return None;
    };

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

#[derive(Debug, Clone, Default)]
pub struct StubLlmClient;

#[async_trait]
impl LlmClient for StubLlmClient {
    async fn generate_structured(
        &self,
        _route: &ResolvedModelRoute,
        request: &LlmRequest,
    ) -> Result<LlmResponse, AiRuntimeError> {
        Ok(LlmResponse {
            text: Some("stub-response".to_string()),
            structured_output: request.user_payload.clone(),
            usage: Some(TokenUsage {
                input_tokens: 0,
                output_tokens: 0,
            }),
            latency_ms: 1,
            finish_reason: Some("stop".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn route(provider: &str, model: &str, priority: u16) -> ResolvedModelRouteCandidate {
        ResolvedModelRouteCandidate {
            route: ResolvedModelRoute {
                alias: "authoring.default".to_string(),
                provider_name: provider.to_string(),
                model_name: model.to_string(),
                provider_metadata: None,
            },
            priority,
            capabilities: HashSet::from([ModelCapability::StructuredJson]),
        }
    }

    #[test]
    fn route_selection_is_deterministic_for_ties() {
        let selected = select_route_candidates(
            vec![
                route("router-b", "model-z", 10),
                route("router-a", "model-z", 10),
                route("router-a", "model-a", 10),
            ],
            &HashSet::from([ModelCapability::StructuredJson]),
        );

        assert_eq!(selected[0].route.provider_name, "router-a");
        assert_eq!(selected[0].route.model_name, "model-a");
        assert_eq!(selected[1].route.provider_name, "router-a");
        assert_eq!(selected[1].route.model_name, "model-z");
        assert_eq!(selected[2].route.provider_name, "router-b");
        assert_eq!(selected[2].route.model_name, "model-z");
    }

    #[test]
    fn route_selection_filters_by_capability() {
        let mut with_tools = route("router-a", "model-a", 1);
        with_tools.capabilities.insert(ModelCapability::ToolCalling);

        let selected = select_route_candidates(
            vec![with_tools, route("router-b", "model-b", 1)],
            &HashSet::from([
                ModelCapability::StructuredJson,
                ModelCapability::ToolCalling,
            ]),
        );

        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].route.provider_name, "router-a");
    }
}
