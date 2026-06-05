use reqwest::StatusCode;
use serde_json::json;

use super::super::{map_http_status_to_error_kind, safe_provider_metadata, AiErrorKind};

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
