use super::{trace_layer, UnderlayMakeSpan};
use crate::RequestId;
use http::Request;
use tower_http::trace::MakeSpan;
use tracing_subscriber::registry;

fn span_name_for_request(req: &Request<()>) -> Option<&'static str> {
    let subscriber = registry();
    let _guard = tracing::subscriber::set_default(subscriber);
    let mut make_span = UnderlayMakeSpan;
    let span = make_span.make_span(req);
    span.metadata().map(|meta| meta.name())
}

#[test]
fn make_span_includes_expected_span_name() {
    let mut req = Request::builder()
        .method("POST")
        .uri("/v1/example")
        .body(())
        .expect("request should build");
    req.extensions_mut().insert(RequestId::new());

    assert_eq!(span_name_for_request(&req), Some("http.request"));
}

#[test]
fn make_span_handles_missing_request_id_extension() {
    let req = Request::builder()
        .method("GET")
        .uri("/health")
        .body(())
        .expect("request should build");

    assert_eq!(span_name_for_request(&req), Some("http.request"));
}

#[test]
fn trace_layer_constructs_successfully() {
    let _layer = trace_layer();
}

#[cfg(feature = "opentelemetry")]
#[test]
fn make_span_accepts_traceparent_headers() {
    let req = Request::builder()
        .method("GET")
        .uri("/health")
        .header(
            crate::TRACEPARENT_HEADER,
            "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
        )
        .header(crate::TRACESTATE_HEADER, "vendor=value")
        .body(())
        .expect("request should build");

    assert_eq!(span_name_for_request(&req), Some("http.request"));
}
