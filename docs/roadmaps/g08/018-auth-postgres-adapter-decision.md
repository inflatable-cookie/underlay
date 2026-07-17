# g08.018 - Auth-Postgres Adapter Decision

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Resolve the misleading `underlay-auth-postgres` crate. `underlay-auth` defines
`UserRepository`/`CredentialRepository`/`SessionRepository`/`AuditLogRepository`,
but no crate implements them for Postgres; `underlay-auth-postgres` is only an
`AuthStateStore` (short-lived workflow state, hardcoded `auth.auth_state`). Every
consumer hand-writes ~30 SQL-backed trait methods, while the media and jobs
stacks both ship real `-postgres` adapters. Either ship the adapter or rename.

## Evidence

- traits `rust/crates/underlay-auth/src/repository.rs`
- crate scope `rust/crates/underlay-auth-postgres/src/lib.rs`
- schema authority `docs/architecture/050-auth-database-schema.md`

## Governing References

- [030 Auth and session systems](../../contracts/030-auth-and-session-systems.md)
- [021 Database migration and schema workflow](../../contracts/021-database-migration-and-schema-workflow.md)

## Planned Changes

- [x] **Decision: rename** (do not ship a centralized adapter). Renamed the
  crate `underlay-auth-postgres` -> `underlay-auth-state-postgres`.
- [x] Not shipping the full repo adapter; `g08.019` now covers the renamed
  auth-state crate. A table-name validation unit test was added.
- [x] Parameterized the hardcoded `auth.auth_state` table via
  `AuthStateStore::with_table` (validated `[A-Za-z0-9_.]+`, default
  `auth.auth_state`), so non-`auth`-schema consumers can point it at their own
  table.

## Consumer Upgrade Impact

Impact class: `behavioral` (shipping the adapter removes consumer boilerplate)
or `naming` (rename). Requires six-consumer proof per `023`.

## Validation

- [ ] if shipped: adapter passes repository contract tests against Postgres
- [ ] `cargo test -p underlay-auth-postgres`
- [ ] `effigy validate`

## Stop Conditions

This is partly a decision card; if consumers already have divergent auth-repo
implementations, surface that before centralizing.

## Decision & Evidence

**Rename, do not centralize.** The stop-condition survey found the consumer
auth-repo landscape is genuinely divergent, so a single Postgres adapter for
`UserRepository`/`CredentialRepository`/`SessionRepository`/`AuditLogRepository`
would fit none of them:

- Only **2 of 6** consumers implement the auth repos at all (acowtancy,
  songsprout). The other four (underlay-reference, contact-patch, compli-me,
  loophole) do not use these traits.
- The two that do are radically different: acowtancy uses `auth.*`
  (`auth.users` with a `role` column, single `auth.credentials`); songsprout
  uses `accounts.*` (`accounts.admin_users` keyed on `artist_id`, singular
  `accounts.session`/`accounts.credential`, a separate
  `accounts.totp_credential`, plus `accounts.login_attempt`).

So the crate is renamed to reflect what it actually is - an `AuthStateStore`,
nothing more - and its table is made configurable.

## Completion Notes

Completed 2026-07-17.
- Crate `underlay-auth-postgres` -> `underlay-auth-state-postgres` (dir +
  Cargo name + description). No underlay-internal crate depended on it.
- `AuthStateStore::with_table` added (validated name; default
  `auth.auth_state`); the five hardcoded table literals now interpolate the
  configured name. `AuthStateError` gained `InvalidTableName` and is now
  `#[non_exhaustive]` so future variants are not breaking. A validation unit
  test was added.
- Contract `030`, package map `010`, API inventory `122`, and guide `190`
  updated; `g08.019` re-pointed at the renamed crate. g06 archival logs left
  as-is per the roadmap evidence boundary.

## Consumer Rollout

The four consumers using the crate (`underlay-reference`, `contact-patch`,
`compli-me`, `acowtancy`; songsprout/loophole do not use it) were migrated:
workspace Cargo dep key + path, `crates/auth` dep, and the
`use underlay_auth_state_postgres::{...}` import. Each also gained a wildcard
arm in its `map_auth_state_error` match for the new `#[non_exhaustive]`
variant. All four consumer auth crates `cargo check` clean.

Validated: `cargo test -p underlay-auth-state-postgres` green;
`cargo test --workspace --all-features` green; four consumer auth crates
`cargo check` clean.

## Next Task

`g08.019` postgres adapter integration tests.
