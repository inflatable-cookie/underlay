use super::*;
use serde_json::json;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

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

#[test]
fn provider_registry_register_and_get() {
    let mut registry = ProviderRegistry::new();
    registry.register("openai", Arc::new(StubLlmClient));

    assert!(registry.get("openai").is_some());
    assert!(registry.get("missing").is_none());
}

#[test]
fn openai_client_new_validates_inputs() {
    let empty_base =
        OpenAiCompatibleClient::new("  ", "key").expect_err("empty base url should fail");
    assert_eq!(empty_base.kind, AiErrorKind::Validation);

    let empty_key = OpenAiCompatibleClient::new("https://api.example.com", " ")
        .expect_err("empty api key should fail");
    assert_eq!(empty_key.kind, AiErrorKind::Validation);

    let ok = OpenAiCompatibleClient::new("https://api.example.com/", "secret");
    assert!(ok.is_ok(), "valid config should construct client");
}

#[test]
fn safe_provider_metadata_filters_allowed_keys() {
    assert_eq!(safe_provider_metadata(None), None);
    assert_eq!(safe_provider_metadata(Some(&json!("not-an-object"))), None);

    let filtered = safe_provider_metadata(Some(&json!({
        "provider": "openrouter",
        "routing": {"strategy": "latency"},
        "order": 1,
        "secret": "should-not-pass"
    })))
    .expect("whitelisted keys should be retained");

    assert_eq!(filtered.get("provider"), Some(&json!("openrouter")));
    assert_eq!(
        filtered.get("routing"),
        Some(&json!({"strategy": "latency"}))
    );
    assert_eq!(filtered.get("order"), Some(&json!(1)));
    assert!(filtered.get("secret").is_none());
}

#[test]
fn status_code_mapping_covers_expected_classes() {
    assert_eq!(
        map_http_status_to_error_kind(StatusCode::UNAUTHORIZED),
        AiErrorKind::Auth
    );
    assert_eq!(
        map_http_status_to_error_kind(StatusCode::FORBIDDEN),
        AiErrorKind::Auth
    );
    assert_eq!(
        map_http_status_to_error_kind(StatusCode::TOO_MANY_REQUESTS),
        AiErrorKind::RateLimit
    );
    assert_eq!(
        map_http_status_to_error_kind(StatusCode::REQUEST_TIMEOUT),
        AiErrorKind::Timeout
    );
    assert_eq!(
        map_http_status_to_error_kind(StatusCode::GATEWAY_TIMEOUT),
        AiErrorKind::Timeout
    );
    assert_eq!(
        map_http_status_to_error_kind(StatusCode::BAD_GATEWAY),
        AiErrorKind::Provider
    );
    assert_eq!(
        map_http_status_to_error_kind(StatusCode::BAD_REQUEST),
        AiErrorKind::Unknown
    );
}

#[tokio::test]
async fn stub_client_echoes_structured_output() {
    let client = StubLlmClient;
    let route = ResolvedModelRoute {
        alias: "authoring.default".to_string(),
        provider_name: "stub".to_string(),
        model_name: "stub-model".to_string(),
        provider_metadata: None,
    };
    let request = LlmRequest {
        system_prompt: "system".to_string(),
        user_payload: json!({"hello": "world"}),
        structured_output: StructuredOutputSpec {
            schema_version: "v1".to_string(),
            strict: true,
        },
        temperature: 0.2,
        max_output_tokens: 256,
    };

    let response = client
        .generate_structured(&route, &request)
        .await
        .expect("stub should always succeed");
    assert_eq!(response.structured_output, request.user_payload);
    assert_eq!(response.finish_reason.as_deref(), Some("stop"));
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
struct SharedScriptedLlmClient(Arc<ScriptedLlmClient>);

impl SharedScriptedLlmClient {
    fn new(responses: Vec<Result<LlmResponse, AiRuntimeError>>) -> Self {
        Self(Arc::new(ScriptedLlmClient::new(responses)))
    }

    fn call_count(&self) -> usize {
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

fn sample_request() -> LlmRequest {
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

fn sample_route(provider: &str) -> ResolvedModelRoute {
    ResolvedModelRoute {
        alias: "authoring.default".to_string(),
        provider_name: provider.to_string(),
        model_name: format!("{provider}-model"),
        provider_metadata: None,
    }
}

fn ok_response(provider: &str) -> LlmResponse {
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

fn err(kind: AiErrorKind, message: &str) -> Result<LlmResponse, AiRuntimeError> {
    Err(AiRuntimeError::new(kind, message))
}

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

#[tokio::test]
async fn circuit_breaker_opens_then_allows_half_open_recovery() {
    let client = SharedScriptedLlmClient::new(vec![
        err(AiErrorKind::Provider, "provider down"),
        err(AiErrorKind::Provider, "still down"),
        Ok(ok_response("recovered")),
    ]);
    let middleware = CircuitBreakerMiddleware::new(
        client.clone(),
        CircuitBreakerConfig {
            failure_threshold: 2,
            window_duration: Duration::from_secs(30),
            reset_timeout: Duration::from_millis(5),
        },
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
