use std::sync::Arc;

use super::super::{AiErrorKind, ProviderRegistry, RouteChainExecutor};
use super::support::{err, ok_response, sample_request, sample_route, SharedScriptedLlmClient};

#[tokio::test]
async fn route_chain_executor_falls_back_to_the_next_route() {
    let mut registry = ProviderRegistry::new();
    registry.register(
        "openai",
        Arc::new(SharedScriptedLlmClient::new(vec![err(
            AiErrorKind::Timeout,
            "primary timed out",
        )])),
    );
    registry.register(
        "anthropic",
        Arc::new(SharedScriptedLlmClient::new(vec![Ok(ok_response(
            "anthropic",
        ))])),
    );

    let executor = RouteChainExecutor::new(registry);
    let routes = vec![sample_route("openai"), sample_route("anthropic")];
    let result = executor
        .execute_with_fallback(&routes, &sample_request())
        .await
        .expect("secondary route should succeed");

    assert_eq!(result.route.provider_name, "anthropic");
    assert_eq!(result.route_index, 1);
    assert_eq!(result.attempts.len(), 2);
    assert_eq!(
        result.attempts[0].error.as_ref().map(|error| &error.kind),
        Some(&AiErrorKind::Timeout)
    );
    assert!(result.attempts[1].error.is_none());
}

#[tokio::test]
async fn route_chain_executor_stops_on_validation_errors() {
    let mut registry = ProviderRegistry::new();
    registry.register(
        "openai",
        Arc::new(SharedScriptedLlmClient::new(vec![err(
            AiErrorKind::Validation,
            "invalid response schema",
        )])),
    );
    registry.register(
        "anthropic",
        Arc::new(SharedScriptedLlmClient::new(vec![Ok(ok_response(
            "anthropic",
        ))])),
    );

    let executor = RouteChainExecutor::new(registry);
    let routes = vec![sample_route("openai"), sample_route("anthropic")];
    let failure = executor
        .execute_with_fallback(&routes, &sample_request())
        .await
        .expect_err("validation errors should stop the chain");

    assert_eq!(failure.error.kind, AiErrorKind::Validation);
    assert_eq!(failure.attempts.len(), 1);
}
