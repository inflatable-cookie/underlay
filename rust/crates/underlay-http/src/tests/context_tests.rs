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

use crate::context::TrustedProxyConfig;
use std::net::IpAddr;

fn socket(ip: &str) -> Option<IpAddr> {
    Some(ip.parse().unwrap())
}

#[test]
fn test_default_config_ignores_all_forwarding_headers() {
    let mut headers = HeaderMap::new();
    headers.insert(
        headers::CF_CONNECTING_IP,
        HeaderValue::from_static("1.1.1.1"),
    );
    headers.insert(headers::X_REAL_IP, HeaderValue::from_static("2.2.2.2"));
    headers.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("3.3.3.3"),
    );

    let ip = resolve_ip_address(&headers, &TrustedProxyConfig::None, socket("9.9.9.9"));
    assert_eq!(ip, socket("9.9.9.9"));

    let no_socket = resolve_ip_address(&headers, &TrustedProxyConfig::None, None);
    assert!(no_socket.is_none());
}

#[test]
fn test_cloudflare_config_uses_cf_connecting_ip() {
    let mut headers = HeaderMap::new();
    headers.insert(
        headers::CF_CONNECTING_IP,
        HeaderValue::from_static("192.168.1.1"),
    );

    let ip = resolve_ip_address(
        &headers,
        &TrustedProxyConfig::CloudflareHeader,
        socket("9.9.9.9"),
    );
    assert_eq!(ip, Some("192.168.1.1".parse().unwrap()));

    let fallback = resolve_ip_address(
        &HeaderMap::new(),
        &TrustedProxyConfig::CloudflareHeader,
        socket("9.9.9.9"),
    );
    assert_eq!(fallback, socket("9.9.9.9"));
}

#[test]
fn test_real_ip_config_uses_x_real_ip() {
    let mut headers = HeaderMap::new();
    headers.insert(headers::X_REAL_IP, HeaderValue::from_static("10.0.0.1"));

    let ip = resolve_ip_address(&headers, &TrustedProxyConfig::RealIpHeader, None);
    assert_eq!(ip, Some("10.0.0.1".parse().unwrap()));
}

#[test]
fn test_forwarded_for_takes_rightmost_untrusted_hop() {
    let mut headers = HeaderMap::new();
    // client-supplied garbage, then the real client as appended by the
    // single trusted proxy
    headers.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("6.6.6.6, 203.0.113.195"),
    );

    let ip = resolve_ip_address(
        &headers,
        &TrustedProxyConfig::ForwardedFor { trusted_hops: 1 },
        socket("9.9.9.9"),
    );
    assert_eq!(ip, Some("203.0.113.195".parse().unwrap()));

    // two trusted hops: the second entry from the right is the client
    let ip2 = resolve_ip_address(
        &headers,
        &TrustedProxyConfig::ForwardedFor { trusted_hops: 2 },
        socket("9.9.9.9"),
    );
    assert_eq!(ip2, Some("6.6.6.6".parse().unwrap()));
}

#[test]
fn test_forwarded_for_spoofed_prefix_cannot_change_resolved_ip() {
    // Attacker sends a forged XFF; the trusted proxy appends the real peer.
    let mut spoofed = HeaderMap::new();
    spoofed.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("6.6.6.6, 7.7.7.7, 203.0.113.195"),
    );

    let mut clean = HeaderMap::new();
    clean.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("203.0.113.195"),
    );

    let config = TrustedProxyConfig::ForwardedFor { trusted_hops: 1 };
    assert_eq!(
        resolve_ip_address(&spoofed, &config, None),
        resolve_ip_address(&clean, &config, None),
    );
}

#[test]
fn test_forwarded_for_falls_back_to_socket_when_underpopulated() {
    let mut headers = HeaderMap::new();
    headers.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("203.0.113.195"),
    );

    let ip = resolve_ip_address(
        &headers,
        &TrustedProxyConfig::ForwardedFor { trusted_hops: 2 },
        socket("9.9.9.9"),
    );
    assert_eq!(ip, socket("9.9.9.9"));

    let zero_hops = resolve_ip_address(
        &headers,
        &TrustedProxyConfig::ForwardedFor { trusted_hops: 0 },
        socket("9.9.9.9"),
    );
    assert_eq!(zero_hops, socket("9.9.9.9"));
}

#[test]
fn test_public_resolve_client_ip_matches_extractor_resolution() {
    // The public wrapper must produce the same result the extractor would.
    let mut headers = HeaderMap::new();
    headers.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("6.6.6.6, 203.0.113.195"),
    );
    let config = TrustedProxyConfig::ForwardedFor { trusted_hops: 1 };

    assert_eq!(
        resolve_client_ip(&headers, &config, socket("9.9.9.9")),
        resolve_ip_address(&headers, &config, socket("9.9.9.9")),
    );
    // Fail-closed default ignores the spoofable header and uses the socket peer.
    assert_eq!(
        resolve_client_ip(&headers, &TrustedProxyConfig::None, socket("9.9.9.9")),
        socket("9.9.9.9"),
    );
}

#[test]
fn test_trusted_proxy_from_env_parsing() {
    let parse = TrustedProxyConfig::parse_env;

    // Unset / off / none / blank -> fail-closed default, all recognised.
    assert_eq!(parse(None, None), (TrustedProxyConfig::None, true));
    assert_eq!(parse(Some(""), None), (TrustedProxyConfig::None, true));
    assert_eq!(parse(Some("none"), None), (TrustedProxyConfig::None, true));
    assert_eq!(parse(Some("off"), None), (TrustedProxyConfig::None, true));

    // Case-insensitive, whitespace-trimmed aliases.
    assert_eq!(
        parse(Some("  CloudFlare "), None),
        (TrustedProxyConfig::CloudflareHeader, true)
    );
    assert_eq!(
        parse(Some("cf"), None),
        (TrustedProxyConfig::CloudflareHeader, true)
    );
    assert_eq!(
        parse(Some("X-Real-IP"), None),
        (TrustedProxyConfig::RealIpHeader, true)
    );

    // forwarded-for: hops default 1, honour override, ignore garbage.
    assert_eq!(
        parse(Some("forwarded-for"), None),
        (TrustedProxyConfig::ForwardedFor { trusted_hops: 1 }, true)
    );
    assert_eq!(
        parse(Some("xff"), Some("3")),
        (TrustedProxyConfig::ForwardedFor { trusted_hops: 3 }, true)
    );
    assert_eq!(
        parse(Some("forwarded-for"), Some("notanumber")),
        (TrustedProxyConfig::ForwardedFor { trusted_hops: 1 }, true)
    );

    // Unrecognised mode -> fail-closed None, flagged unrecognised for the warn.
    assert_eq!(
        parse(Some("cloudfare"), None),
        (TrustedProxyConfig::None, false)
    );
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

#[cfg(feature = "opentelemetry")]
#[test]
fn test_extract_trace_context_from_headers() {
    let mut headers = HeaderMap::new();
    headers.insert(
        underlay_observability::TRACEPARENT_HEADER,
        HeaderValue::from_static("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01"),
    );
    headers.insert(
        underlay_observability::TRACESTATE_HEADER,
        HeaderValue::from_static("vendor=value"),
    );

    let trace_context = underlay_observability::TraceContext::from_headers(&headers)
        .expect("trace context should be present");
    let ctx = RequestContext::new("req-otel".to_string(), None, None, None)
        .with_trace_context(trace_context.clone());

    assert_eq!(ctx.trace_id(), Some("4bf92f3577b34da6a3ce929d0e0e4736"));
    assert_eq!(ctx.parent_span_id(), Some("00f067aa0ba902b7"));
    assert_eq!(ctx.trace_context(), Some(&trace_context));
}

#[cfg(feature = "opentelemetry")]
#[test]
fn test_inject_trace_context_round_trips_headers() {
    let trace_context = underlay_observability::TraceContext::parse(
        "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
        Some("vendor=value"),
    )
    .expect("trace context should parse");
    let ctx = RequestContext::new("req-otel".to_string(), None, None, None)
        .with_trace_context(trace_context);
    let mut headers = HeaderMap::new();

    ctx.inject_trace_context(&mut headers);

    assert_eq!(
        headers
            .get(underlay_observability::TRACEPARENT_HEADER)
            .and_then(|value| value.to_str().ok()),
        Some("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01")
    );
    assert_eq!(
        headers
            .get(underlay_observability::TRACESTATE_HEADER)
            .and_then(|value| value.to_str().ok()),
        Some("vendor=value")
    );
}
