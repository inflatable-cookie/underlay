use std::collections::{HashSet, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::json;

use super::super::{
    AiErrorKind, AiRuntimeError, LlmClient, LlmRequest, LlmResponse, ModelCapability,
    ResolvedModelRoute, ResolvedModelRouteCandidate, StructuredOutputSpec, TokenUsage,
};

pub(crate) fn route(provider: &str, model: &str, priority: u16) -> ResolvedModelRouteCandidate {
    ResolvedModelRouteCandidate {
        route: ResolvedModelRoute {
            alias: "openai.gpt-5.4-mini".to_string(),
            provider_name: provider.to_string(),
            model_name: model.to_string(),
            provider_metadata: None,
        },
        priority,
        capabilities: HashSet::from([ModelCapability::StructuredJson]),
    }
}

#[derive(Debug)]
struct ScriptedLlmClient {
    calls: AtomicUsize,
    responses: Mutex<VecDeque<Result<LlmResponse, AiRuntimeError>>>,
}

impl ScriptedLlmClient {
    fn new(responses: Vec<Result<LlmResponse, AiRuntimeError>>) -> Self {
        Self {
            calls: AtomicUsize::new(0),
            responses: Mutex::new(VecDeque::from(responses)),
        }
    }

    fn call_count(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SharedScriptedLlmClient(Arc<ScriptedLlmClient>);

impl SharedScriptedLlmClient {
    pub(crate) fn new(responses: Vec<Result<LlmResponse, AiRuntimeError>>) -> Self {
        Self(Arc::new(ScriptedLlmClient::new(responses)))
    }

    pub(crate) fn call_count(&self) -> usize {
        self.0.call_count()
    }
}

#[async_trait]
impl LlmClient for SharedScriptedLlmClient {
    async fn generate_structured(
        &self,
        _route: &ResolvedModelRoute,
        _request: &LlmRequest,
    ) -> Result<LlmResponse, AiRuntimeError> {
        self.0.calls.fetch_add(1, Ordering::SeqCst);
        self.0
            .responses
            .lock()
            .expect("scripted client mutex poisoned")
            .pop_front()
            .unwrap_or_else(|| {
                Err(AiRuntimeError::new(
                    AiErrorKind::Unknown,
                    "No scripted response remaining.",
                ))
            })
    }
}

pub(crate) fn sample_request() -> LlmRequest {
    LlmRequest {
        system_prompt: "system".to_string(),
        user_payload: json!({"hello": "world"}),
        structured_output: StructuredOutputSpec {
            schema_version: "v1".to_string(),
            strict: true,
        },
        temperature: 0.2,
        max_output_tokens: 256,
    }
}

pub(crate) fn sample_route(provider: &str) -> ResolvedModelRoute {
    ResolvedModelRoute {
        alias: "openai.gpt-5.4-mini".to_string(),
        provider_name: provider.to_string(),
        model_name: format!("{provider}-model"),
        provider_metadata: None,
    }
}

pub(crate) fn ok_response(provider: &str) -> LlmResponse {
    LlmResponse {
        text: Some(format!("{provider}-response")),
        structured_output: json!({ "provider": provider }),
        usage: Some(TokenUsage {
            input_tokens: 10,
            output_tokens: 20,
        }),
        latency_ms: 5,
        finish_reason: Some("stop".to_string()),
    }
}

pub(crate) fn err(kind: AiErrorKind, message: &str) -> Result<LlmResponse, AiRuntimeError> {
    Err(AiRuntimeError::new(kind, message))
}
