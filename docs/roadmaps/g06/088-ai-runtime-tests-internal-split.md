# g06.088 - AI Runtime Tests Internal Split

## Why

`g06.087` found that `underlay-ai-runtime/src/tests/lib_tests.rs` is test-only
but mixes route selection, provider/client behavior, metadata/status mapping,
retry/circuit middleware, and route-chain fallback behavior in one warning-size
file.

## Goal

Split AI runtime tests into focused test modules while preserving all
behavioral coverage and production APIs.

## Scope

In scope:

- extract shared route, scripted client, request, response, and error fixtures
  into test support
- split route selection tests into a focused module
- split provider registry and OpenAI-compatible client validation tests into a
  focused module
- split metadata and HTTP status mapping tests into a focused module
- split stub client behavior into a focused module
- split retry middleware and retry config behavior into a focused module
- split circuit breaker behavior into a focused module
- split route-chain fallback behavior into a focused module
- preserve test assertions and covered behavior

Out of scope:

- changing AI runtime public APIs
- changing provider/request/response semantics
- changing consumer apps

## Acceptance Criteria

- `lib_tests.rs` becomes a small test front door
- behavior groups live in focused test modules
- AI runtime tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only split. If production behavior or public APIs must change,
stop and re-enter planning.

## Current State

`g06.088` is next.

## Next Task

Execute `g06.088`: AI runtime tests internal split.
