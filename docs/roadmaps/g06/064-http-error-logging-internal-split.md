# g06.064 - HTTP Error Logging Internal Split

## Why

`g06.063` found that `underlay-http/src/error_logging.rs` is stable
feature-gated operational surface with a safe internal split shape.

Consumers use crate-root exports heavily, and the module is enabled by the
`error-logging` feature across the consumer family.

## Goal

Split `underlay-http/src/error_logging.rs` into focused private modules while
preserving feature gates, row/filter shapes, SQL helper behavior, middleware
behavior, and public exports.

## Scope

In scope:

- split row model and constants
- split filter model and query-filter helper
- split SQL write/list/count/get helpers
- split `DbErrorLogSink`
- split `ErrorLoggingConfig`
- split middleware and context helper internals
- preserve `underlay_http::error_logging::*` compatibility
- preserve crate-root error-logging exports
- preserve the `error-logging` feature gate
- update tests only where module parent imports need to become explicit

Out of scope:

- changing error log table schema or SQL behavior
- changing middleware request/response behavior
- changing async logging behavior
- changing `ERROR_CONTEXT_HEADER`
- changing feature flags
- changing consumer admin endpoint DTOs or routes

## Acceptance Criteria

- `error_logging.rs` becomes a small feature-gated module front door
- public exports remain source-compatible
- `underlay-http` tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal module split. If public imports, feature flags, row/filter
shapes, SQL behavior, or middleware behavior must move, stop and re-enter
planning.

## Current State

`g06.064` is complete.

Artifact:

- [064 artifact](./064-http-error-logging-internal-split-artifact.md)

## Next Task

Execute `g06.065`: migration-core orchestrator public model modularity audit.
