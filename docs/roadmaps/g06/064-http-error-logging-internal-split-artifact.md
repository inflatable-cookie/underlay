# g06.064 Artifact - HTTP Error Logging Internal Split

## Summary

`underlay-http/src/error_logging.rs` is now a small public front door over
focused private modules.

New private module layout:

- `error_logging/row.rs`: `ErrorLogRow`
- `error_logging/filters.rs`: `ErrorLogFilters`, `ErrorLogStatusClass`, query
  filter helper
- `error_logging/queries.rs`: append, list, count, and get SQL helpers
- `error_logging/sink.rs`: `DbErrorLogSink`
- `error_logging/config.rs`: `ErrorLoggingConfig`
- `error_logging/middleware.rs`: `ERROR_CONTEXT_HEADER`,
  `error_logging_middleware`, handler-context helpers, and middleware tests

## Compatibility

The split preserves:

- the `error-logging` feature gate from `underlay-http/src/lib.rs`
- `underlay_http::error_logging::*`
- crate-root error-logging exports
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

This was a private module split only. The only visibility adjustment was making
`ErrorLoggingConfig.pool` `pub(crate)` so the sibling middleware module can
clone it. External visibility remains unchanged.

## Validation

- `cargo test -p underlay-http --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` failed on known structural backlog:
  `scan.attention-markers`, `scan.comment-ratio`, and `scan.god-files`

Structural movement:

- `underlay-http/src/error_logging.rs`: 532 lines to 13 lines
- `scan.god-files`: 56 findings to 55 findings

Next batch validation:

- targeted migration-core tests from `effigy test --plan` or a focused Cargo
  command
- `effigy rust:check`
- consumer checks only if public migration-core imports move
- `effigy qa:docs`
- `effigy qa:northstar`
