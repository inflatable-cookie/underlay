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
fn retry_config_jitters_capped_exponential_delay() {
    let config = RetryConfig::default()
        .with_max_attempts(4)
        .with_base_delay(Duration::from_millis(100))
        .with_max_delay(Duration::from_millis(250))
        .with_retriable_kinds(default_retriable_error_kinds());

    assert_eq!(config.delay_for_attempt(0), Duration::ZERO);
    for (attempt, ceiling) in [(1, 100), (2, 200), (3, 250), (u32::MAX, 250)] {
        let ceiling = Duration::from_millis(ceiling);
        let bounds = (ceiling - ceiling / 4)..=ceiling;
        let first = config.delay_for_attempt(attempt);
        assert!(bounds.contains(&first), "attempt {attempt}: {first:?}");
        let mut varied = false;
        for _ in 0..31 {
            let delay = config.delay_for_attempt(attempt);
            assert!(bounds.contains(&delay), "attempt {attempt}: {delay:?}");
            varied |= delay != first;
        }
        assert!(varied, "attempt {attempt} always returned {first:?}");
    }
    assert_eq!(
        config.with_max_delay(Duration::ZERO).delay_for_attempt(1),
        Duration::ZERO
    );
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
        RetryConfig::default()
            .with_max_attempts(4)
            .with_base_delay(Duration::ZERO)
            .with_max_delay(Duration::ZERO)
            .with_retriable_kinds(default_retriable_error_kinds()),
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
