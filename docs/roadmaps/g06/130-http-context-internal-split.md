# g06.130 - HTTP Context Internal Split

## Why

`g06.129` found that `underlay-http/src/context.rs` mixes public context
models, authenticated extension state, extractor implementations, header
parsing, error response mapping, OpenTelemetry helpers, and tracing span
helpers in one module.

The next split should make the HTTP context boundary easier to reason about
without changing public imports or request extraction behavior.

## Goal

Split HTTP context into focused internal modules while preserving public context
APIs and runtime behavior.

## Scope

In scope:

- replace `context.rs` with a `context/` module directory
- keep current `underlay_http::context::{...}` imports working
- keep current root `underlay_http::{...}` re-exports working
- move header constants into `headers.rs`
- move public context models into `model.rs`
- move `ContextError` into `error.rs`
- move Axum extractors into `extractors.rs`
- move header parsing helpers into `parse.rs`
- move feature-gated tracing helpers into `tracing.rs`
- preserve existing context tests

Out of scope:

- changing HTTP public APIs
- changing request context extraction semantics
- changing auth/session behavior
- changing tracing/OpenTelemetry behavior
- changing consumer apps

## Acceptance Criteria

- the old oversized context file is replaced by focused modules
- public context imports remain stable
- context tests pass with all features
- full HTTP tests pass with all features
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal module split. If public context APIs or runtime behavior
must change, stop and re-enter planning.

## Current State

`g06.130` is ready.

## Next Task

Execute `g06.130`: HTTP context internal split.
