    use super::*;
    use serde_json::json;

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
        let empty_base = OpenAiCompatibleClient::new("  ", "key")
            .expect_err("empty base url should fail");
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
        assert_eq!(filtered.get("routing"), Some(&json!({"strategy": "latency"})));
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