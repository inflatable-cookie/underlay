# g06.132 - HTTP Cookies Tests Internal Split

## Why

`g06.131` found that `underlay-http/src/tests/cookies_tests.rs` mixes cookie
builder, extraction, config, typed value, validation failure, and header
append/clear behavior tests in one flat file.

The next split should make cookie behavior coverage easier to scan while
preserving all security-sensitive tests.

## Goal

Split HTTP cookie tests into focused internal test modules without changing
cookie APIs or behavior.

## Scope

In scope:

- replace `tests/cookies_tests.rs` with a `tests/cookies_tests/` module
  directory
- update the parent test path to `tests/cookies_tests/mod.rs`
- keep shared imports in the module front door
- move cookie builder tests into `builders.rs`
- move token extraction tests into `extractors.rs`
- move config, typed value, and validation tests into `config.rs`
- move set/clear header append tests into `headers.rs`
- preserve existing cookie tests

Out of scope:

- changing cookie public APIs
- changing auth or CSRF cookie behavior
- changing cookie defaults
- changing consumer apps

## Acceptance Criteria

- the old oversized cookie test file is replaced by focused test modules
- existing cookie behavior coverage remains intact
- focused cookie tests pass
- full HTTP tests pass
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only split. If cookie behavior must change, stop and re-enter
planning.

## Current State

`g06.132` is complete.

Artifact:

- [132 artifact](./132-http-cookies-tests-internal-split-artifact.md)

## Next Task

Execute `g06.133`: migration-core drift modularity audit.
