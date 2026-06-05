# g06.078 - Auth Email TOTP Service Tests Internal Split

## Why

`g06.077` found that
`underlay-auth-email-totp/src/tests/service_tests.rs` is test-only but mixes
mock repository setup, request-code behavior, verify-code failure/success
behavior, code-only verification, and session delegation in one high-error
file.

## Goal

Split auth email TOTP service tests into focused test modules while preserving
all behavioral coverage and production APIs.

## Scope

In scope:

- extract shared mock repositories, mock sender, and helper functions into test
  support
- split request-code rate-limit/storage/send tests into a focused module
- split verify-code missing/expired/exhausted/invalid tests into a focused
  module
- split verify-code success and code-only verification tests into a focused
  module
- split consume/get session delegation tests into a focused module
- preserve test assertions and covered behavior
- adjust imports only as needed for module-local test support

Out of scope:

- changing auth public APIs
- changing email TOTP token/code semantics
- changing recovery, verification, or session behavior
- changing consumer apps

## Acceptance Criteria

- `service_tests.rs` becomes a small test front door
- behavior groups live in focused test modules
- auth email TOTP tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only split. If production behavior or public APIs must change,
stop and re-enter planning.

## Current State

`g06.078` is complete.

Artifact:

- [078 artifact](./078-auth-email-totp-service-tests-internal-split-artifact.md)

## Next Task

Execute `g06.079`: devtools migration-bundle tests modularity audit.
