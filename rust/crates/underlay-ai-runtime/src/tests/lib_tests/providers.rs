use std::sync::Arc;

use super::super::{AiErrorKind, OpenAiCompatibleClient, ProviderRegistry, StubLlmClient};

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
fn openai_client_new_rejects_non_local_http() {
    let err = OpenAiCompatibleClient::new("http://api.example.com", "key")
        .expect_err("non-local http base url should fail");
    assert_eq!(err.kind, AiErrorKind::Validation);

    assert!(OpenAiCompatibleClient::new("http://localhost:11434", "key").is_ok());
    assert!(OpenAiCompatibleClient::new("http://127.0.0.1:11434", "key").is_ok());
}

#[test]
fn openai_client_debug_redacts_api_key() {
    let client = OpenAiCompatibleClient::new("https://api.example.com", "super-secret-key")
        .expect("valid config");
    let debug = format!("{client:?}");
    assert!(!debug.contains("super-secret-key"));
    assert!(debug.contains("[REDACTED]"));
}
