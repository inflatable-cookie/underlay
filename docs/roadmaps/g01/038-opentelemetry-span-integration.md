# 038 - OpenTelemetry Span Integration

Status: Complete
Owner: Platform
Created: 2026-03-11
Depends on: 031

## Overview

Add feature-gated OpenTelemetry trace-context support across `underlay-http` and `underlay-observability` so consuming apps can correlate request logs with distributed traces without taking on app-local propagation glue.

## Research Basis

- `docs/roadmaps/backlog/opentelemetry-integration.md`
- `docs/guides/070-api-handlers.md`
- `docs/guides/078-error-logging.md`
- `rust/crates/underlay-http/src/context.rs`
- `rust/crates/underlay-observability/src/http_trace.rs`

## Likely Implementation Surface

- `rust/crates/underlay-http/src/context.rs`
- `rust/crates/underlay-http/src/lib.rs`
- `rust/crates/underlay-observability/src/`
- `docs/guides/070-api-handlers.md`
- `docs/guides/078-error-logging.md`

## Phase 38.1 - Trace Context in RequestContext

- [x] Extend `RequestContext` with feature-gated trace/span context accessors without making OpenTelemetry a default dependency path.
- [x] Parse W3C `traceparent` and `tracestate` headers on inbound requests and expose helpers for injecting them into outgoing headers.
- [x] Add targeted tests for extraction, round-tripping, and missing-header fallback behavior.

## Phase 38.2 - Observability Layer and Correlation

- [x] Add an opt-in tracing layer or helper surface in `underlay-observability` that creates request spans with trace correlation fields.
- [x] Keep request IDs and trace identifiers aligned so apps can join logs and traces without duplicating middleware.
- [x] Document what stays app-owned in the first batch, especially OTLP exporter setup and backend-specific sampling policy.

## Phase 38.3 - Consumer Rollout and Documentation

- [x] Add an upgrade note entry in `docs/guides/190-upgrade-compatibility.md`.
- [x] Update `docs/guides/070-api-handlers.md` and `docs/guides/078-error-logging.md` with feature flags, setup order, and propagation examples.
- [x] Make the opt-in default explicit so existing apps do not change runtime behavior just by upgrading Underlay.

## Deferred

- Shared OTLP exporter bootstrap for every deployment environment.
- Metrics and baggage propagation beyond request tracing.
- Automatic outgoing client middleware for every HTTP client abstraction in the repo.

## Consumer Upgrade Impact

- Expected impact class: `additive`.
- OpenTelemetry support should be feature-gated and opt-in so existing HTTP stacks do not gain new dependencies or runtime behavior by default.
- Upgrade guidance must call out any new crate features, middleware ordering requirements, and trace-header propagation expectations.
- Exporter endpoint configuration, sampling policy, and backend credentials remain app-owned unless a later roadmap explicitly centralizes them.

## Validation

```bash
cargo check -p underlay-http --all-features
cargo test -p underlay-http --all-features
cargo check -p underlay-observability --all-features
cargo test -p underlay-observability --all-features
effigy validate --repo .
```

## Completion

Current active roadmap set is complete. Promote the next backlog item into `g01` only when the next reusable batch is ready for execution.
