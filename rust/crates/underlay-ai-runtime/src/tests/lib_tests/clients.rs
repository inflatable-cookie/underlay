use serde_json::json;

use super::super::{
    LlmClient, LlmRequest, OpenAiCompatibleClient, ResolvedModelRoute, StructuredOutputSpec,
    StubLlmClient,
};

#[tokio::test]
async fn stub_client_echoes_structured_output() {
    let client = StubLlmClient;
    let route = ResolvedModelRoute {
        alias: "openai.gpt-5.4-mini".to_string(),
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

#[test]
fn openai_client_validates_http_local_hosts_with_url_parser() {
    for allowed in [
        "https://api.openai.com/v1",
        "http://localhost:11434/v1",
        "http://LOCALHOST:11434/v1",
        "http://api.localhost:8080/v1",
        "http://127.0.0.1:8080/v1",
        "http://[::1]:8080/v1",
        "http://0.0.0.0:8080/v1",
    ] {
        assert!(
            OpenAiCompatibleClient::new(allowed, "test-key").is_ok(),
            "expected {allowed} to be accepted"
        );
    }

    for rejected in [
        "http://api.openai.com/v1",
        "http://localhost.evil.com/v1",
        "http://127.0.0.1.evil.com/v1",
        "http://not-a-valid-url:::/v1",
        "ftp://localhost/v1",
    ] {
        assert!(
            OpenAiCompatibleClient::new(rejected, "test-key").is_err(),
            "expected {rejected} to be rejected"
        );
    }
}
