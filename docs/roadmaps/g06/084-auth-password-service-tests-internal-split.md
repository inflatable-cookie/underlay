# g06.084 - Auth Password Service Tests Internal Split

## Why

`g06.083` found that
`underlay-auth-password/src/tests/service_tests.rs` is test-only but mixes
repository fixtures, login, lockout, rate-limit, password policy, password
change, reset, and normalization behavior in one high-error file.

## Goal

Split auth password service tests into focused test modules while preserving
all behavioral coverage and production APIs.

## Scope

In scope:

- extract shared in-memory repository and user/service setup helpers into test
  support
- split login success and email-normalization tests into a focused module
- split lockout and rate-limit behavior into a focused module
- split compromised-password policy behavior into a focused module
- split password change behavior into a focused module
- split password reset behavior into a focused module
- preserve test assertions and covered behavior
- adjust imports only as needed for module-local test support

Out of scope:

- changing auth password public APIs
- changing password hashing or verification semantics
- changing reset/session security behavior
- changing consumer apps

## Acceptance Criteria

- `service_tests.rs` becomes a small test front door
- behavior groups live in focused test modules
- auth password tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only split. If production behavior or public APIs must change,
stop and re-enter planning.

## Current State

`g06.084` is next.

## Next Task

Execute `g06.084`: auth password service tests internal split.
