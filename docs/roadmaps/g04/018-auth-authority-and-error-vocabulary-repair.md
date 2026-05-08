# 018 - Auth Authority And Error Vocabulary Repair

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.017` assessed the live auth/session implementation against `030`.

The core auth mechanics are broadly aligned: provider/extractor seams exist,
session rotation and replay checks are real, and the retained browser auth
store/workflow shell is still coherent.

The real drift is at the boundary:

- `underlay-auth-jwt` still exposes a second public auth error-code vocabulary
  that does not match the shared `AuthError` contract or TS auth type surface
- the auth/account schema docs are materially stale against the live migration
  and shared types
- `runtime/auth.ts` is still only a curated barrel over pattern-owned auth
  helpers

## Goals

- collapse the public auth error-code story back onto the shared `AuthError`
  contract
- repair the stale auth/account schema authority docs to match the live shared
  migration and type surfaces
- leave the runtime-vs-pattern auth split explicitly documented and bounded
  without forcing a bigger packaging refactor in the same batch

## Non-Goals

- broad auth feature redesign
- storage/media assessment in the same batch
- rewriting the retained TS auth shell surface

## Inputs

- [docs/roadmaps/g04/017-auth-and-session-assessment.md](/Users/tom/Dev/projects/underlay/docs/roadmaps/g04/017-auth-and-session-assessment.md)
- [docs/contracts/030-auth-and-session-systems.md](/Users/tom/Dev/projects/underlay/docs/contracts/030-auth-and-session-systems.md)
- `rust/crates/underlay-auth/src/errors.rs`
- `rust/crates/underlay-auth-jwt/src/error.rs`
- `rust/crates/underlay-auth/migrations/0001_create_auth_tables.sql`
- [docs/architecture/050-auth-database-schema.md](/Users/tom/Dev/projects/underlay/docs/architecture/050-auth-database-schema.md)
- [docs/architecture/055-account-database-schema.md](/Users/tom/Dev/projects/underlay/docs/architecture/055-account-database-schema.md)
- `ts/src/runtime/auth.ts`
- `ts/src/patterns/auth.ts`

## Exit Criteria

- the shared public auth error-code story is consistent across Rust and TS
- stale auth/account schema authority docs are repaired
- the next higher assessment lane can treat auth as a stable lower dependency

## Result

Completed.

Implemented:

- `underlay-auth-jwt` no longer exposes a second public auth error-code dialect
  through `JwtError::code()`. It now collapses onto the shared `AuthError`
  vocabulary.
- the stale schema docs were rewritten to match the live shared migration and
  type surfaces:
  - [docs/architecture/050-auth-database-schema.md](/Users/tom/Dev/projects/underlay/docs/architecture/050-auth-database-schema.md:1)
  - [docs/architecture/055-account-database-schema.md](/Users/tom/Dev/projects/underlay/docs/architecture/055-account-database-schema.md:1)

Validated with:

- `cargo test -p underlay-auth-jwt`

Remaining auth drift:

- `runtime/auth.ts` is still a curated compatibility barrel over pattern-owned
  auth helpers, but that is packaging clarity work rather than a blocking
  contract mismatch

## Next Task

Execute `g04.019`: assess the live storage and media implementation against
`040` and `050`.
