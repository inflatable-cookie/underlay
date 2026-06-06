use super::model::RequestContext;

/// Create a tracing span with request context fields.
///
/// This is useful for structured logging with request context.
///
/// # Example
///
/// ```ignore
/// use underlay_http::context::{RequestContext, make_request_span};
///
/// async fn my_handler(ctx: RequestContext) -> Json<String> {
///     let span = make_request_span(&ctx);
///     let _guard = span.enter();
///
///     tracing::info!("Processing request");
///     Json("ok".to_string())
/// }
/// ```
pub fn make_request_span(ctx: &RequestContext) -> tracing::Span {
    #[cfg(feature = "opentelemetry")]
    let span = tracing::info_span!(
        "request",
        request_id = %ctx.request_id(),
        user_id = ?ctx.user_id(),
        ip = ?ctx.ip_address(),
        trace_id = tracing::field::Empty,
        parent_span_id = tracing::field::Empty,
        trace_flags = tracing::field::Empty,
        tracestate = tracing::field::Empty,
    );

    #[cfg(not(feature = "opentelemetry"))]
    let span = tracing::info_span!(
        "request",
        request_id = %ctx.request_id(),
        user_id = ?ctx.user_id(),
        ip = ?ctx.ip_address(),
    );

    #[cfg(feature = "opentelemetry")]
    if let Some(trace_context) = ctx.trace_context() {
        span.record("trace_id", trace_context.trace_id());
        span.record("parent_span_id", trace_context.parent_span_id());
        span.record("trace_flags", trace_context.trace_flags());
        if let Some(tracestate) = trace_context.tracestate() {
            span.record("tracestate", tracestate);
        }
    }

    span
}

impl RequestContext {
    /// Record context fields to the current tracing span.
    ///
    /// Call this to add request context to structured logs.
    pub fn record_to_span(&self, span: &tracing::Span) {
        span.record("request_id", self.request_id());
        if let Some(user_id) = self.user_id() {
            span.record("user_id", tracing::field::display(user_id));
        }
        if let Some(ip) = self.ip_address() {
            span.record("ip", tracing::field::display(ip));
        }
        #[cfg(feature = "opentelemetry")]
        if let Some(trace_context) = self.trace_context() {
            span.record("trace_id", trace_context.trace_id());
            span.record("parent_span_id", trace_context.parent_span_id());
            span.record("trace_flags", trace_context.trace_flags());
            if let Some(tracestate) = trace_context.tracestate() {
                span.record("tracestate", tracestate);
            }
        }
    }
}
