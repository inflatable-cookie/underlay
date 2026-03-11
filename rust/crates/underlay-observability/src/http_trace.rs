use http::Request;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::{MakeSpan, TraceLayer};

use crate::RequestId;
#[cfg(feature = "opentelemetry")]
use crate::TraceContext;

#[derive(Debug, Clone, Copy)]
pub struct UnderlayMakeSpan;

impl<B> MakeSpan<B> for UnderlayMakeSpan {
    fn make_span(&mut self, req: &Request<B>) -> tracing::Span {
        let request_id = req
            .extensions()
            .get::<RequestId>()
            .map(|id| id.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        #[cfg(feature = "opentelemetry")]
        let trace_context = TraceContext::from_headers(req.headers());

        #[cfg(feature = "opentelemetry")]
        let span = tracing::info_span!(
            "http.request",
            request_id = %request_id,
            method = %req.method(),
            uri = %req.uri(),
            trace_id = tracing::field::Empty,
            parent_span_id = tracing::field::Empty,
            trace_flags = tracing::field::Empty,
            tracestate = tracing::field::Empty,
        );

        #[cfg(not(feature = "opentelemetry"))]
        let span = tracing::info_span!(
            "http.request",
            request_id = %request_id,
            method = %req.method(),
            uri = %req.uri()
        );

        #[cfg(feature = "opentelemetry")]
        if let Some(trace_context) = trace_context.as_ref() {
            span.record("trace_id", trace_context.trace_id());
            span.record("parent_span_id", trace_context.parent_span_id());
            span.record("trace_flags", trace_context.trace_flags());
            if let Some(tracestate) = trace_context.tracestate() {
                span.record("tracestate", tracestate);
            }
        }

        span
    }
}

pub fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, UnderlayMakeSpan> {
    TraceLayer::new_for_http().make_span_with(UnderlayMakeSpan)
}

#[cfg(test)]
#[path = "tests/http_trace_tests.rs"]
mod tests;
