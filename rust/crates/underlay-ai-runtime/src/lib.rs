mod chain;
mod circuit_breaker;
mod client;
mod error;
mod openai;
mod registry;
mod retry;
mod routing;
mod stub;
mod types;

pub use crate::chain::{
    default_fallback_error_kinds, RouteChainAttempt, RouteChainConfig, RouteChainExecutor,
    RouteChainFailure, RouteChainResult,
};
pub use crate::circuit_breaker::{CircuitBreakerConfig, CircuitBreakerMiddleware, CircuitState};
pub use crate::client::LlmClient;
pub use crate::error::{AiErrorKind, AiRuntimeError};
pub use crate::openai::OpenAiCompatibleClient;
pub use crate::registry::ProviderRegistry;
pub use crate::retry::{default_retriable_error_kinds, RetryConfig, RetryMiddleware};
pub use crate::routing::select_route_candidates;
pub use crate::stub::StubLlmClient;
pub use crate::types::{
    LlmRequest, LlmResponse, ModelCapability, ResolvedModelRoute, ResolvedModelRouteCandidate,
    StructuredOutputSpec, TokenUsage,
};

#[cfg(test)]
pub(crate) use crate::openai::{map_http_status_to_error_kind, safe_provider_metadata};

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
