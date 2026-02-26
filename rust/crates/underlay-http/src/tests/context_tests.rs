use super::*;
use axum::http::HeaderValue;

#[test]
fn test_extract_request_id_from_header() {
    let mut headers = HeaderMap::new();
    headers.insert(
        headers::X_REQUEST_ID,
        HeaderValue::from_static("test-request-id"),
    );

    let request_id = extract_request_id(&headers);
    assert_eq!(request_id, "test-request-id");
}

#[test]
fn test_extract_request_id_generates_uuid() {
    let headers = HeaderMap::new();
    let request_id = extract_request_id(&headers);

    // Should be a valid UUID
    assert!(Uuid::parse_str(&request_id).is_ok());
}

#[test]
fn test_extract_ip_from_cf_connecting_ip() {
    let mut headers = HeaderMap::new();
    headers.insert(
        headers::CF_CONNECTING_IP,
        HeaderValue::from_static("192.168.1.1"),
    );

    let ip = extract_ip_address(&headers);
    assert_eq!(ip, Some("192.168.1.1".parse().unwrap()));
}

#[test]
fn test_extract_ip_from_x_real_ip() {
    let mut headers = HeaderMap::new();
    headers.insert(headers::X_REAL_IP, HeaderValue::from_static("10.0.0.1"));

    let ip = extract_ip_address(&headers);
    assert_eq!(ip, Some("10.0.0.1".parse().unwrap()));
}

#[test]
fn test_extract_ip_from_x_forwarded_for() {
    let mut headers = HeaderMap::new();
    headers.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("203.0.113.195, 70.41.3.18, 150.172.238.178"),
    );

    let ip = extract_ip_address(&headers);
    assert_eq!(ip, Some("203.0.113.195".parse().unwrap()));
}

#[test]
fn test_extract_ip_priority() {
    let mut headers = HeaderMap::new();
    // CF-Connecting-IP should win
    headers.insert(
        headers::CF_CONNECTING_IP,
        HeaderValue::from_static("1.1.1.1"),
    );
    headers.insert(headers::X_REAL_IP, HeaderValue::from_static("2.2.2.2"));
    headers.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("3.3.3.3"),
    );

    let ip = extract_ip_address(&headers);
    assert_eq!(ip, Some("1.1.1.1".parse().unwrap()));
}

#[test]
fn test_extract_ip_none_when_missing() {
    let headers = HeaderMap::new();
    let ip = extract_ip_address(&headers);
    assert!(ip.is_none());
}

#[test]
fn test_request_context_methods() {
    let ctx = RequestContext::new(
        "req-123".to_string(),
        Some("192.168.1.1".parse().unwrap()),
        Some("Mozilla/5.0".to_string()),
        Some(Uuid::nil()),
    );

    assert_eq!(ctx.request_id(), "req-123");
    assert_eq!(ctx.ip_address(), Some("192.168.1.1".parse().unwrap()));
    assert_eq!(ctx.user_agent(), Some("Mozilla/5.0"));
    assert_eq!(ctx.user_id(), Some(Uuid::nil()));
    assert!(ctx.is_authenticated());
}

#[test]
fn test_request_context_unauthenticated() {
    let ctx = RequestContext::new("req-456".to_string(), None, None, None);

    assert!(!ctx.is_authenticated());
    assert!(ctx.user_id().is_none());
}
