use super::*;
use axum::http::HeaderValue;

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
