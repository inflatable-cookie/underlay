# g06.080 - Devtools Migration-Bundle Tests Internal Split

## Why

`g06.079` found that
`underlay-devtools/src/tests/migration_bundle_tests.rs` is test-only but mixes
bundle local-store behavior, digest-ref parsing, media shard behavior, and
remote registry helpers in one high-error file.

## Goal

Split devtools migration-bundle tests into focused test modules while
preserving all behavioral coverage and production APIs.

## Scope

In scope:

- extract shared temp-dir and Docker registry helpers into test support
- split local build/publish/pull/run tests into a focused module
- split digest-ref and typed run-option tests into a focused module
- split media shard sanitization and deterministic shard build tests into a
  focused module
- split ignored remote registry round-trip test into a focused module
- preserve test assertions and covered behavior
- adjust imports only as needed for module-local test support

Out of scope:

- changing devtools public APIs
- changing migration-bundle semantics
- changing generated bundle formats
- changing consumer apps

## Acceptance Criteria

- `migration_bundle_tests.rs` becomes a small test front door
- behavior groups live in focused test modules
- focused migration-bundle tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only split. If production behavior or public APIs must change,
stop and re-enter planning.

## Current State

`g06.080` is next.

## Next Task

Execute `g06.080`: devtools migration-bundle tests internal split.
