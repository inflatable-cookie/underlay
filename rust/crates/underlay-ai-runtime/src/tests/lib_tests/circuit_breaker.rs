use std::time::Duration;

use serde_json::json;

use super::super::{
    AiErrorKind, CircuitBreakerConfig, CircuitBreakerMiddleware, CircuitState, LlmClient,
};
use super::support::{SharedScriptedLlmClient, err, ok_response, sample_request, sample_route};

#[tokio::test]
async fn circuit_breaker_opens_then_allows_half_open_recovery() {
    let client = SharedScriptedLlmClient::new(vec![
        err(AiErrorKind::Provider, "provider down"),
        err(AiErrorKind::Provider, "still down"),
        Ok(ok_response("recovered")),
    ]);
    let middleware = CircuitBreakerMiddleware::new(
        client.clone(),
        CircuitBreakerConfig::default()
            .with_failure_threshold(2)
            .with_window_duration(Duration::from_secs(30))
            .with_reset_timeout(Duration::from_millis(5)),
    );
    let route = sample_route("openai");
    let request = sample_request();

    let first = middleware
        .generate_structured(&route, &request)
        .await
        .expect_err("first provider error should fail");
    assert_eq!(first.kind, AiErrorKind::Provider);
    assert_eq!(middleware.provider_state("openai"), CircuitState::Closed);

    let second = middleware
        .generate_structured(&route, &request)
        .await
        .expect_err("second provider error should fail");
    assert_eq!(second.kind, AiErrorKind::Provider);
    assert_eq!(middleware.provider_state("openai"), CircuitState::Open);

    let open_error = middleware
        .generate_structured(&route, &request)
        .await
        .expect_err("open circuit should reject without hitting inner client");
    assert_eq!(open_error.kind, AiErrorKind::CircuitOpen);
    assert_eq!(client.call_count(), 2);

    tokio::time::sleep(Duration::from_millis(10)).await;
    assert_eq!(middleware.provider_state("openai"), CircuitState::HalfOpen);

    let response = middleware
        .generate_structured(&route, &request)
        .await
        .expect("half-open request should succeed and close the circuit");
    assert_eq!(
        response.structured_output,
        json!({ "provider": "recovered" })
    );
    assert_eq!(middleware.provider_state("openai"), CircuitState::Closed);
    assert_eq!(client.call_count(), 3);
}

#[tokio::test]
async fn circuit_breaker_recovers_from_poisoned_state_lock() {
    let client = SharedScriptedLlmClient::new(vec![Ok(ok_response("ok"))]);
    let middleware = CircuitBreakerMiddleware::new(client, CircuitBreakerConfig::default());
    let route = sample_route("openai");
    let request = sample_request();

    middleware.test_poison_state_lock();

    assert_eq!(middleware.provider_state("openai"), CircuitState::Closed);
    let response = middleware
        .generate_structured(&route, &request)
        .await
        .expect("poisoned state lock should be recovered");
    assert_eq!(response.structured_output, json!({ "provider": "ok" }));
}
