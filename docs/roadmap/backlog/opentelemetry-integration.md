# Backlog: OpenTelemetry Span Integration

**Status**: Backlog  
**Priority**: Medium  
**Estimated Effort**: 4-6 hours  
**Source**: Deferred from roadmap 009 (Quick Wins)

---

## Problem Statement

The `RequestContext` in `underlay-http` currently provides structured logging integration via `make_request_span` and `record_to_span`, but lacks native OpenTelemetry (OTLP) span integration. This means:

- Distributed traces don't automatically include request context
- Context propagation across service boundaries requires manual setup
- No automatic span creation for HTTP handlers
- Missing correlation between logs and traces

---

## Proposed Solution

Extend `underlay-http` and `underlay-observability` to provide seamless OpenTelemetry integration:

### 1. RequestContext OTLP Extension

```rust
use underlay_http::context::RequestContext;

impl RequestContext {
    /// Get the current trace ID (from incoming headers or generated)
    pub fn trace_id(&self) -> Option<TraceId>;
    
    /// Get the current span ID
    pub fn span_id(&self) -> Option<SpanId>;
    
    /// Inject context into outgoing request headers
    pub fn inject_into(&self, headers: &mut HeaderMap);
    
    /// Create a child span for this request
    pub fn child_span(&self, name: &str) -> Span;
}
```

### 2. Tower Layer for Automatic Tracing

```rust
use underlay_http::tracing::OtelLayer;

let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(OtelLayer::new());
```

### 3. Context Propagation

Support W3C Trace Context propagation:
- Extract `traceparent` and `tracestate` from incoming requests
- Inject into outgoing HTTP client requests
- Compatible with Jaeger, Zipkin, and other OTLP backends

---

## Dependencies

- `opentelemetry` crate (0.21+)
- `opentelemetry-otlp` for OTLP export
- `tracing-opentelemetry` for tracing integration
- Existing `underlay-observability` crate

---

## Success Criteria

- [ ] RequestContext includes trace/span IDs
- [ ] Automatic span creation for all HTTP handlers
- [ ] W3C Trace Context propagation works end-to-end
- [ ] Traces visible in Jaeger/Grafana Tempo
- [ ] Zero-config setup with reasonable defaults
- [ ] Feature-gated to avoid bloating non-OTLP users

---

## Risks & Considerations

- **Bundle size**: OpenTelemetry adds significant dependencies
- **Performance**: Span creation has overhead (measure impact)
- **Configuration**: OTLP endpoints vary by environment
- **Feature flags**: Need careful gating to keep core lightweight

---

## Related

- `underlay-http/src/context.rs` - RequestContext implementation
- `underlay-observability/` - Existing observability utilities
- [OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust)
- [W3C Trace Context](https://www.w3.org/TR/trace-context/)

---

**Created**: 2026-01-12
