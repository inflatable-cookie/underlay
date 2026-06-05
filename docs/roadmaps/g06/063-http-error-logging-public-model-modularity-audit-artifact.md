# g06.063 Artifact - HTTP Error Logging Public Model Modularity Audit

## Summary

`underlay-http/src/error_logging.rs` is stable feature-gated operational
surface with a safe internal split shape if the `error-logging` feature gate
and crate-root exports stay intact.

The file currently groups:

- context header: `ERROR_CONTEXT_HEADER`
- row model: `ErrorLogRow`
- write helper: `append_error_log`
- query model: `ErrorLogFilters`, `ErrorLogStatusClass`
- query helpers: `list_error_logs`, `count_error_logs`,
  `get_error_log_by_id`
- DB sink adapter: `DbErrorLogSink`
- middleware config: `ErrorLoggingConfig`
- middleware: `error_logging_middleware`
- middleware context helpers: handler-context extraction and fallback context

## Consumer Evidence

Consumer usage is broad and mostly crate-root based:

- `underlay_http::ErrorLoggingConfig::new(...).with_source(...).with_client_errors(...).with_server_errors(...)`
- `underlay_http::error_logging_middleware`
- `underlay_http::ErrorLogRow`
- `underlay_http::{count_error_logs, get_error_log_by_id, list_error_logs}`
- `underlay_http::{ErrorLogFilters, ErrorLogStatusClass}`
- `underlay_http::ERROR_CONTEXT_HEADER`

Feature usage appears across the consumer family:

- Underlay Reference, Contact Patch, Compli-me, Songsprout Nursery,
  Acowtancy Farmyard, and Loophole Composer enable `underlay-http` with the
  `error-logging` feature.
- Error-log admin endpoints convert `ErrorLogRow` into app DTOs and use list,
  count, and get helpers.
- Startup code wires `ErrorLoggingConfig` and `error_logging_middleware`.
- Acowtancy also has app-local infra DB wrappers around error-log helpers, so
  SQL helper names and row/filter shapes remain migration-sensitive.

## Decision

Queue `g06.064` as an HTTP error logging internal split.

The split should preserve:

- the `error-logging` feature gate on all error logging exports
- crate-root exports from `underlay-http/src/lib.rs`
- `underlay_http::error_logging::*` compatibility
- `ErrorLogRow` field names and types
- `ErrorLogFilters` fields and defaults
- `ErrorLogStatusClass` variants
- list/count/get/append SQL helper names and behavior
- `DbErrorLogSink` construction and `ErrorLogSink` implementation behavior
- `ErrorLoggingConfig` public fields and builder methods
- middleware request/response behavior and async logging behavior
- `ERROR_CONTEXT_HEADER` name and handler-context fallback shape

## Public API Impact

Expected impact: none.

This should be a private module split only. If SQL schema assumptions,
feature-gated exports, row/filter shapes, middleware behavior, or root import
paths must change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-http --all-features`
- `effigy qa:docs`
- `effigy qa:northstar`

Next code batch validation:

- `cargo test -p underlay-http --all-features`
- `effigy rust:check`
- consumer checks only if public import paths or feature flags move
- `effigy qa:docs`
- `effigy qa:northstar`
