# 017 - Auth And Session Assessment

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.016` repaired the lower transport error normalization failure, so the
shared foundation and transport boundary is now stable enough for the next
assessment wave.

The next system family in sequence is auth. `030` spans the Rust provider,
extractor, session, credential, MFA, passkey, OAuth, and retained browser auth
shell surfaces that many higher features depend on.

## Goals

- assess the live auth and session implementation against `030`
- separate true contract failures from schema/doc drift and packaging residue
- identify the smallest honest repair set for the shared auth boundary
- leave explicit findings and a bounded next lane instead of broad auth churn

## Non-Goals

- executing broad auth refactors in the same batch
- jumping ahead to storage or higher layers before auth findings are explicit
- product-specific account UX redesign

## Inputs

- [docs/contracts/030-auth-and-session-systems.md](/Users/tom/Dev/projects/underlay/docs/contracts/030-auth-and-session-systems.md)
- `rust/crates/underlay-auth*/**`
- `ts/src/client/auth.ts`
- `ts/src/client/useAuth.ts`
- `ts/src/runtime/auth.ts`
- `ts/src/patterns/auth.ts`
- `ts/src/patterns/auth-workflows.ts`
- `ts/src/patterns/passkey.svelte.ts`
- `ts/src/utils/webauthn.ts`
- [docs/architecture/050-auth-database-schema.md](/Users/tom/Dev/projects/underlay/docs/architecture/050-auth-database-schema.md)
- [docs/architecture/055-account-database-schema.md](/Users/tom/Dev/projects/underlay/docs/architecture/055-account-database-schema.md)

## Exit Criteria

- the live auth/session implementation is reviewed against `030`
- the real findings are documented in severity order
- the next repair step is expressed as one bounded roadmap lane or a small
  repair set
- storage/media assessment can start without ambiguity about the shared auth
  boundary

## Findings

### 1. `underlay-auth-jwt` still exposes a second public auth error-code vocabulary

Severity: medium

The shared contract says the stable public auth error vocabulary lives on
`AuthError` and the retained TS auth type surface. The JWT layer still exposes
its own public `.code()` values such as `auth.jwt_config_error`,
`auth.jwt_key_error`, `auth.token_expired`, and `auth.token_replay`, while the
rest of the shared auth surface collapses JWT failures into `AuthError`.

Evidence:

- [rust/crates/underlay-auth/src/errors.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth/src/errors.rs:1)
- [rust/crates/underlay-auth-jwt/src/error.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth-jwt/src/error.rs:1)
- [ts/src/client/types.ts](/Users/tom/Dev/projects/underlay/ts/src/client/types.ts:125)
- [rust/crates/underlay-auth-jwt/src/tests/service_tests.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth-jwt/src/tests/service_tests.rs:623)

Impact:

- Rust exposes two overlapping public code namespaces for the same auth stack
- the JWT-specific codes are not represented in the retained TS auth type
  surface or the `030` contract
- callers have to guess whether `JwtError::code()` is internal detail or public
  compatibility surface

### 2. Auth/account schema authority docs are materially stale against the live migration and shared types

Severity: medium

The live migration and shared types no longer match the architecture docs in
several important places. This was already suspected by `030`, and the
assessment confirms it.

Examples:

- [docs/architecture/050-auth-database-schema.md](/Users/tom/Dev/projects/underlay/docs/architecture/050-auth-database-schema.md:1)
  still documents `auth.users.display_name` as required, but the live shared
  migration has no user-level `display_name` column and the shared `User` type
  makes `display_name` optional
- the doc still describes a simpler session table, while the live migration
  includes `roles`, `is_active`, `refresh_token_id`, and
  `refresh_token_version`
- the migration itself is now the real authority:
  [rust/crates/underlay-auth/migrations/0001_create_auth_tables.sql](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth/migrations/0001_create_auth_tables.sql:1)

Impact:

- the contract is correct to treat these docs as evidence rather than active
  authority
- maintainers reading the architecture docs today get the wrong schema model
  for shared auth

### 3. `runtime/auth.ts` still acts as a compatibility barrel rather than a real runtime owner

Severity: low

This is not breaking behavior, but the assessment confirms the contract’s
packaging drift note. The runtime auth entrypoint is still just a curated
re-export over pattern-owned auth helpers.

Evidence:

- [ts/src/runtime/auth.ts](/Users/tom/Dev/projects/underlay/ts/src/runtime/auth.ts:1)
- [ts/src/patterns/auth.ts](/Users/tom/Dev/projects/underlay/ts/src/patterns/auth.ts:1)
- [ts/src/patterns/auth-workflows.ts](/Users/tom/Dev/projects/underlay/ts/src/patterns/auth-workflows.ts:1)

Impact:

- public authority and implementation ownership remain split
- this is a packaging clarity issue, not a reason to stop the auth assessment
  wave

## Assessment Result

The shared auth mechanics are not the problem. The provider seam, extractor
status mapping, token/session checks, and refresh replay guards all looked
substantively aligned enough to move on.

The next bounded repair lane should focus on:

- collapsing the public auth error-code story back onto one shared contract
- repairing the stale auth/account schema authority docs
- leaving the runtime-vs-pattern auth split explicit without forcing a larger
  packaging refactor yet

## Next Task

Execute `g04.018`: repair the auth authority and public error-vocabulary
drift.
