use std::time::Duration;

use serde_json::json;

use super::super::{
    AiErrorKind, CircuitBreakerConfig, CircuitBreakerMiddleware, CircuitState, LlmClient,
};
use super::support::{err, ok_response, sample_request, sample_route, SharedScriptedLlmClient};

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
async fn circuit_breaker_does_not_count_validation_failures() {
    let client = SharedScriptedLlmClient::new(vec![
        err(AiErrorKind::Provider, "provider down"),
        err(AiErrorKind::Validation, "bad input"),
        err(AiErrorKind::Validation, "still bad input"),
        err(AiErrorKind::Provider, "provider still down"),
    ]);
    let middleware = CircuitBreakerMiddleware::new(
        client,
        CircuitBreakerConfig::default().with_failure_threshold(2),
    );
    let route = sample_route("openai");
    let request = sample_request();

    for expected in [
        AiErrorKind::Provider,
        AiErrorKind::Validation,
        AiErrorKind::Validation,
    ] {
        let error = middleware
            .generate_structured(&route, &request)
            .await
            .unwrap_err();
        assert_eq!(error.kind, expected);
        assert_eq!(middleware.provider_state("openai"), CircuitState::Closed);
    }
    let error = middleware
        .generate_structured(&route, &request)
        .await
        .unwrap_err();
    assert_eq!(error.kind, AiErrorKind::Provider);
    assert_eq!(middleware.provider_state("openai"), CircuitState::Open);
}

#[tokio::test]
async fn validation_releases_a_half_open_probe_for_the_next_request() {
    let client = SharedScriptedLlmClient::new(vec![
        err(AiErrorKind::Provider, "provider down"),
        err(AiErrorKind::Validation, "bad input"),
        Ok(ok_response("recovered")),
    ]);
    let middleware = CircuitBreakerMiddleware::new(
        client,
        CircuitBreakerConfig::default()
            .with_failure_threshold(1)
            .with_reset_timeout(Duration::ZERO),
    );
    let route = sample_route("openai");
    let request = sample_request();

    let _ = middleware
        .generate_structured(&route, &request)
        .await
        .unwrap_err();
    assert_eq!(middleware.provider_state("openai"), CircuitState::HalfOpen);
    let error = middleware
        .generate_structured(&route, &request)
        .await
        .unwrap_err();
    assert_eq!(error.kind, AiErrorKind::Validation);
    assert_eq!(middleware.provider_state("openai"), CircuitState::HalfOpen);
    middleware
        .generate_structured(&route, &request)
        .await
        .expect("next probe can recover");
    assert_eq!(middleware.provider_state("openai"), CircuitState::Closed);
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
