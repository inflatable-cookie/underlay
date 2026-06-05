use serde_json::json;

use super::super::{
    LlmClient, LlmRequest, ResolvedModelRoute, StructuredOutputSpec, StubLlmClient,
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
