# g06.082 - Auth OAuth Tests Internal Split

## Why

`g06.081` found that `underlay-auth-oauth/src/tests/lib_tests.rs` is test-only
but mixes env setup, login URL behavior, callback behavior, repository fixtures,
and disconnect behavior in one high-error file.

## Goal

Split auth OAuth tests into focused test modules while preserving all
behavioral coverage and production APIs.

## Scope

In scope:

- extract shared env helpers, stub provider, in-memory repository, and common
  fixtures into test support
- split env/config behavior into a focused module
- split login URL/state/PKCE behavior into a focused module
- split callback success and callback rejection behavior into a focused module
- split disconnect behavior into a focused module
- preserve test assertions and covered behavior
- adjust imports only as needed for module-local test support

Out of scope:

- changing auth OAuth public APIs
- changing redirect/callback semantics
- changing token/session security behavior
- changing consumer apps

## Acceptance Criteria

- `lib_tests.rs` becomes a small test front door
- behavior groups live in focused test modules
- auth OAuth tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only split. If production behavior or public APIs must change,
stop and re-enter planning.

## Current State

`g06.082` is next.

## Next Task

Execute `g06.082`: auth OAuth tests internal split.
