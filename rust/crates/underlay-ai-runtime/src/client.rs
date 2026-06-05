use async_trait::async_trait;

use crate::{AiRuntimeError, LlmRequest, LlmResponse, ResolvedModelRoute};

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate_structured(
        &self,
        route: &ResolvedModelRoute,
        request: &LlmRequest,
    ) -> Result<LlmResponse, AiRuntimeError>;
}
