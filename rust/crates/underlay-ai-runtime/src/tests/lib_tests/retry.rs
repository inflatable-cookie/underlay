use std::time::Duration;

use serde_json::json;

use super::super::{
    default_retriable_error_kinds, AiErrorKind, AiRuntimeError, LlmClient, RetryConfig,
    RetryMiddleware,
};
use super::support::{err, ok_response, sample_request, sample_route, SharedScriptedLlmClient};

#[test]
fn ai_runtime_error_helpers_match_default_policies() {
    assert!(AiRuntimeError::new(AiErrorKind::Timeout, "timeout").is_retriable());
    assert!(AiRuntimeError::new(AiErrorKind::CircuitOpen, "open").allows_fallback());
    assert!(!AiRuntimeError::new(AiErrorKind::Validation, "bad payload").is_retriable());
    assert!(!AiRuntimeError::new(AiErrorKind::Validation, "bad payload").allows_fallback());
}

#[test]
fn retry_config_uses_bounded_exponential_backoff() {
    let config = RetryConfig {
        max_attempts: 4,
        base_delay: Duration::from_millis(100),
        max_delay: Duration::from_millis(250),
        retriable_kinds: default_retriable_error_kinds(),
    };

    assert_eq!(config.delay_for_attempt(0), Duration::ZERO);
    assert_eq!(config.delay_for_attempt(1), Duration::from_millis(100));
    assert_eq!(config.delay_for_attempt(2), Duration::from_millis(200));
    assert_eq!(config.delay_for_attempt(3), Duration::from_millis(250));
    assert_eq!(config.delay_for_attempt(4), Duration::from_millis(250));
}

#[tokio::test]
async fn retry_middleware_retries_transient_errors_then_succeeds() {
    let client = SharedScriptedLlmClient::new(vec![
        err(AiErrorKind::Timeout, "timeout"),
        err(AiErrorKind::RateLimit, "rate limit"),
        Ok(ok_response("retry-ok")),
    ]);
    let middleware = RetryMiddleware::new(
        client.clone(),
        RetryConfig {
            max_attempts: 4,
            base_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
            retriable_kinds: default_retriable_error_kinds(),
        },
    );

    let response = middleware
        .generate_structured(&sample_route("openai"), &sample_request())
        .await
        .expect("retry middleware should eventually succeed");

    assert_eq!(
        response.structured_output,
        json!({ "provider": "retry-ok" })
    );
    assert_eq!(client.call_count(), 3);
}

#[tokio::test]
async fn retry_middleware_stops_on_terminal_errors() {
    let client = SharedScriptedLlmClient::new(vec![
        err(AiErrorKind::Validation, "bad payload"),
        Ok(ok_response("should-not-run")),
    ]);
    let middleware = RetryMiddleware::new(client.clone(), RetryConfig::default());

    let error = middleware
        .generate_structured(&sample_route("openai"), &sample_request())
        .await
        .expect_err("validation errors should not retry");

    assert_eq!(error.kind, AiErrorKind::Validation);
    assert_eq!(client.call_count(), 1);
}
