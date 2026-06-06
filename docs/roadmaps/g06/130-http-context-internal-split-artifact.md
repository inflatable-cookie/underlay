# g06.130 Artifact - HTTP Context Internal Split

## Summary

HTTP context now lives under a focused `context/` module directory instead of
one large `context.rs` file.

Changed files:

- `underlay-http/src/context.rs` removed
- `underlay-http/src/context/mod.rs`
- `underlay-http/src/context/headers.rs`
- `underlay-http/src/context/model.rs`
- `underlay-http/src/context/error.rs`
- `underlay-http/src/context/extractors.rs`
- `underlay-http/src/context/parse.rs`
- `underlay-http/src/context/tracing.rs`

## Module Shape

- `mod.rs`: public context front door, re-exports, and test module declaration
- `headers.rs`: request header constants
- `model.rs`: request/authenticated context models and authenticated user
  extension type
- `error.rs`: context extraction errors and response mapping
- `extractors.rs`: Axum `FromRequestParts` implementations
- `parse.rs`: request ID and client IP parsing helpers
- `tracing.rs`: feature-gated span helpers

The existing `underlay_http::context::{...}` imports and root
`underlay_http::{...}` re-exports remain stable.

## Behavior Preserved

The split keeps existing HTTP context behavior:

- `x-request-id` extraction
- request ID generation when the header is missing
- client IP priority order: `cf-connecting-ip`, `x-real-ip`,
  `x-forwarded-for`
- first-IP extraction from `x-forwarded-for`
- authenticated user extension lookup
- authenticated extractor `401` behavior
- context error response status/message mapping
- OpenTelemetry trace extraction/injection when enabled
- tracing span field recording when enabled

## Validation

Passed:

- `cargo test -p underlay-http context --all-features`
  - 15 tests passed
  - 1 Docker-backed error-logging test ignored
- `cargo test -p underlay-http --all-features`
  - 121 unit tests passed
  - 1 Docker-backed error-logging test ignored
  - 11 doc-tests passed
  - 13 doc-tests ignored
- `effigy rust:check`

Known backlog:

- `effigy doctor` still fails on the existing structural scan backlog:
  attention markers, comment ratio, and god-files.
- God-file findings dropped from 24 to 23 after this split.
- The next Rust warning-level target is
  `underlay-http/src/tests/cookies_tests.rs`.

## Public API Impact

None.

This was an internal module split. No public HTTP context API, root export,
header constant, status code, extraction behavior, tracing behavior, or
consumer import path changed.
*** Add File: docs/roadmaps/g06/131-http-cookies-tests-modularity-audit.md
# g06.131 - HTTP Cookies Tests Modularity Audit

## Why

After `g06.130`, the next Rust warning-level file in the god-file report is
`underlay-http/src/tests/cookies_tests.rs`.

Cookie tests cover security-sensitive auth and CSRF cookie behavior. They
should be split from evidence about test fixture families, not from file size
alone.

## Goal

Classify the HTTP cookies test surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-http/src/tests/cookies_tests.rs` by behavior family
- identify auth cookie, CSRF cookie, extraction, validation, builder, and
  clearing test boundaries
- identify security-sensitive test behavior that must remain covered
- decide whether the next batch should split internal test modules, extract
  helper files, or defer behind a broader cookie test checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing cookie public APIs
- changing auth or CSRF cookie behavior
- changing cookie defaults
- changing consumer apps

## Acceptance Criteria

- cookie test responsibilities are grouped by stable behavior family
- security-sensitive behavior coverage is recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only structure audit. If preserving tests requires changing
cookie behavior, stop and re-enter planning.

## Current State

`g06.131` is ready.

## Next Task

Execute `g06.131`: HTTP cookies tests modularity audit.
