# g06.074 - Auth JWT Service Tests Internal Split

## Why

`g06.073` found that `underlay-auth-jwt/src/tests/service_tests.rs` is
test-only but covers security-sensitive JWT and refresh-rotation behavior in
one high-error file.

## Goal

Split auth JWT service tests into focused test modules while preserving all
behavioral coverage and production APIs.

## Scope

In scope:

- extract shared `MemoryStore` and helper setup into test support
- split session lifecycle and refresh-rotation tests into a focused module
- split key generation, config, token issuance, token validation, fingerprint,
  and error mapping tests into focused modules
- preserve test assertions and covered behavior
- adjust imports only as needed for module-local test support

Out of scope:

- changing production auth/session APIs
- changing token semantics
- changing refresh rotation behavior
- changing JWT error codes
- changing consumer apps

## Acceptance Criteria

- `service_tests.rs` becomes a small test front door
- behavior groups live in focused test modules
- auth JWT tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only split. If production behavior or public APIs must change,
stop and re-enter planning.

## Current State

`g06.074` is next.

## Next Task

Execute `g06.074`: auth JWT service tests internal split.
