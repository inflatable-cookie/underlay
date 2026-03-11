use super::{TraceContext, TRACEPARENT_HEADER, TRACESTATE_HEADER};
use http::HeaderMap;

#[test]
fn trace_context_parses_valid_headers() {
    let context = TraceContext::parse(
        "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
        Some("vendor=value"),
    )
    .expect("trace context should parse");

    assert_eq!(context.version(), "00");
    assert_eq!(context.trace_id(), "4bf92f3577b34da6a3ce929d0e0e4736");
    assert_eq!(context.parent_span_id(), "00f067aa0ba902b7");
    assert_eq!(context.trace_flags(), "01");
    assert_eq!(context.tracestate(), Some("vendor=value"));
}

#[test]
fn trace_context_rejects_invalid_values() {
    assert!(TraceContext::parse(
        "00-00000000000000000000000000000000-00f067aa0ba902b7-01",
        None,
    )
    .is_none());
    assert!(TraceContext::parse(
        "00-4bf92f3577b34da6a3ce929d0e0e4736-0000000000000000-01",
        None,
    )
    .is_none());
    assert!(TraceContext::parse("zz-invalid", None).is_none());
}

#[test]
fn trace_context_round_trips_through_headers() {
    let context = TraceContext::parse(
        "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
        Some("vendor=value"),
    )
    .expect("trace context should parse");
    let mut headers = HeaderMap::new();

    context.inject_into(&mut headers);

    assert_eq!(
        headers.get(TRACEPARENT_HEADER).and_then(|value| value.to_str().ok()),
        Some("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01")
    );
    assert_eq!(
        headers.get(TRACESTATE_HEADER).and_then(|value| value.to_str().ok()),
        Some("vendor=value")
    );
    assert_eq!(TraceContext::from_headers(&headers), Some(context));
}

#[test]
fn inject_removes_empty_tracestate() {
    let context = TraceContext::parse(
        "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
        None,
    )
    .expect("trace context should parse");
    let mut headers = HeaderMap::new();
    headers.insert(TRACESTATE_HEADER, "stale=value".parse().unwrap());

    context.inject_into(&mut headers);

    assert!(headers.get(TRACESTATE_HEADER).is_none());
}
