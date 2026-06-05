# g06.035 - Remaining Dynamic Identifier Closeout Audit

## Why

The main runtime DB helper families now route through typed identifier
boundaries. Before opening another refactor lane, `g06` should prove whether
any meaningful dynamic SQL identifier construction remains in shared Rust code.

## Goal

Audit the remaining shared Rust dynamic SQL identifier construction sites and
decide whether `g06` can close the typed DB helper lane or needs one final
targeted cleanup.

## Scope

In scope:

- scan shared Rust crates for dynamic SQL identifier construction
- classify each finding as typed-safe, test-only retained, or needs cleanup
- update the Rust public API inventory if the lane is complete
- create follow-up only if a concrete cleanup remains

Out of scope:

- TypeScript/Svelte work
- broad SQL query refactors unrelated to dynamic identifiers
- release execution or publishing
- consumer rewrites unless the audit finds active shared API fallout

## Acceptance Criteria

- remaining dynamic identifier findings are listed and classified
- no unclassified shared Rust runtime identifier interpolation remains
- next task is either a concrete cleanup or the typed DB helper lane closeout

## Consumer Upgrade Impact

Expected impact: none unless the audit identifies a still-public raw helper.

## Current State

`g06.035` is next after `g06.034`.

## Next Task

Execute `g06.035`: remaining dynamic identifier closeout audit.
