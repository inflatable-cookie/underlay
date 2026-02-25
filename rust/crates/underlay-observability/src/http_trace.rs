use http::Request;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::{MakeSpan, TraceLayer};

use crate::RequestId;

#[derive(Debug, Clone, Copy)]
pub struct UnderlayMakeSpan;

impl<B> MakeSpan<B> for UnderlayMakeSpan {
    fn make_span(&mut self, req: &Request<B>) -> tracing::Span {
        let request_id = req
            .extensions()
            .get::<RequestId>()
            .map(|id| id.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        tracing::info_span!(
            "http.request",
            request_id = %request_id,
            method = %req.method(),
            uri = %req.uri()
        )
    }
}

pub fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, UnderlayMakeSpan> {
    TraceLayer::new_for_http().make_span_with(UnderlayMakeSpan)
}

#[cfg(test)]
#[path = "tests/http_trace_tests.rs"]
mod tests;
