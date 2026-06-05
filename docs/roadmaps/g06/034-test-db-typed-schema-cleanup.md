# g06.034 - Test DB Typed Schema Cleanup

## Why

The remaining typed DB helper inventory is mostly clean after the existence
helper removal.

`underlay-testing::TestDb` still constructs schema names around raw strings in a
test-only helper. This is lower risk than runtime SQL construction, but it is
still part of the shared Rust platform and should use the same identifier
boundary where practical.

## Goal

Move `TestDb` schema construction through the typed schema/identifier boundary
or explicitly document why the current shape is retained.

## Scope

In scope:

- inspect `underlay-testing::TestDb`
- replace raw schema construction where the shared schema parser fits
- update docs/contracts if the helper shape changes
- run targeted Rust checks

Out of scope:

- runtime media Postgres config refactor
- new public DB APIs
- consumer app rewrites unless a stale import appears
- release execution or publishing

## Acceptance Criteria

- `TestDb` schema handling is typed or explicitly retained with rationale
- no new raw SQL identifier interpolation is introduced
- targeted Rust checks pass or failures are classified

## Consumer Upgrade Impact

Expected impact: none. This is test-helper internals unless public constructor
types change.

## Current State

`g06.034` is complete.

Artifact:

- [034 artifact](./034-test-db-typed-schema-cleanup-artifact.md)

## Next Task

Execute `g06.035`: remaining dynamic identifier closeout audit.
