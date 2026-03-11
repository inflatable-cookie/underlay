# 033 - AI Runtime Resilience Middleware

Status: Complete
Owner: Platform
Created: 2026-03-11
Depends on: 031

## Overview

Add opt-in resilience primitives to `underlay-ai-runtime` so consuming apps can use shared retry, circuit breaker, and route-chain behavior instead of maintaining custom orchestration.

## Research Basis

- `docs/research/implementation-decision-records/idr-ai-runtime-resilience.md`
- `docs/research/value-tracks/ai-runtime-patterns.md`
- `docs/research/specimen-dossiers/litellm.md`
- `docs/research/specimen-dossiers/portkey.md`
- `docs/guides/176-ai-runtime-routing.md`

## Likely Implementation Surface

- `rust/crates/underlay-ai-runtime/src/lib.rs`
- new middleware modules for retry, circuit breaking, and route chaining
- docs and examples in `docs/guides/176-ai-runtime-routing.md`

## Phase 33.1 - Retry Policy and Error Taxonomy

- [x] Confirm which `AiRuntimeError` variants are retriable and how that is configured.
- [x] Add bounded retry middleware with exponential backoff.
- [x] Add unit and integration coverage for success, transient failure, and terminal failure cases.

## Phase 33.2 - Circuit Breaker and Fallback Chains

- [x] Add circuit-breaker middleware with in-memory state as the initial default.
- [x] Add route-chain fallback execution with route metadata in the result path.
- [x] Document non-goals for the first batch, especially cost tracking and dead-letter ownership.

## Phase 33.3 - Consumer Rollout and Documentation

- [x] Add an upgrade note entry in `docs/guides/190-upgrade-compatibility.md`.
- [x] Update `docs/guides/176-ai-runtime-routing.md` with composition examples and rollout guidance.
- [x] Provide migration notes for apps replacing custom retry or fallback implementations.

## Deferred

- Cumulative cost tracking and budget enforcement.
- Shared dead-letter queue ownership for AI actions.
- Streaming-specific fallback behavior unless the initial executor model proves compatible.

## Consumer Upgrade Impact

- Expected impact class: `additive`.
- The new middleware should be opt-in so existing apps do not change runtime behavior by upgrading alone.
- Any migration guide must explain how to replace app-local retry and circuit-breaker code without changing provider configuration ownership.
- If later defaults change, that follow-on batch must include concrete before/after runtime behavior examples.

## Validation

```bash
cargo check -p underlay-ai-runtime --all-features
cargo test -p underlay-ai-runtime --all-features
effigy validate --repo .
```

## Next Task

Roadmap complete on 2026-03-11. Next broad batch: implement `g01.035` for job retry safety, dead letters, and lifecycle events.
