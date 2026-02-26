use super::{trace_layer, UnderlayMakeSpan};
use crate::RequestId;
use http::Request;
use tower_http::trace::MakeSpan;

#[test]
fn make_span_includes_expected_span_name() {
    let mut req = Request::builder()
        .method("POST")
        .uri("/v1/example")
        .body(())
        .expect("request should build");
    req.extensions_mut().insert(RequestId::new());

    let mut make_span = UnderlayMakeSpan;
    let span = make_span.make_span(&req);
    let name = span.metadata().map(|meta| meta.name());
    assert_eq!(name, Some("http.request"));
}

#[test]
fn make_span_handles_missing_request_id_extension() {
    let req = Request::builder()
        .method("GET")
        .uri("/health")
        .body(())
        .expect("request should build");

    let mut make_span = UnderlayMakeSpan;
    let span = make_span.make_span(&req);
    let name = span.metadata().map(|meta| meta.name());
    assert_eq!(name, Some("http.request"));
}

#[test]
fn trace_layer_constructs_successfully() {
    let _layer = trace_layer();
}
