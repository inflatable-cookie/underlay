use async_trait::async_trait;

use crate::{AiRuntimeError, LlmClient, LlmRequest, LlmResponse, ResolvedModelRoute, TokenUsage};

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
