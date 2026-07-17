# 2026-07-17 - g08.018 auth-postgres adapter decision (rename)

## Context

`underlay-auth-postgres` was misleadingly named: `underlay-auth` defines
`UserRepository`/`CredentialRepository`/`SessionRepository`/`AuditLogRepository`,
but the crate only implements an `AuthStateStore` (short-lived workflow state,
hardcoded `auth.auth_state`). The card asked: ship the full Postgres repo
adapter, or rename?

## Decision: rename (do not centralize)

The stop-condition survey settled it - consumer auth schemas are genuinely
divergent, so one adapter fits none:

- **2 of 6** consumers implement the auth repos (acowtancy, songsprout); the
  other four don't use them.
- acowtancy: `auth.*` schema (`auth.users` + `role`, single `auth.credentials`).
  songsprout: `accounts.*` (`accounts.admin_users` keyed on `artist_id`,
  singular `accounts.session`/`accounts.credential`, separate
  `accounts.totp_credential`, `accounts.login_attempt`).

## Changes

- Renamed crate `underlay-auth-postgres` -> `underlay-auth-state-postgres`
  (dir, Cargo name, description). No underlay-internal crate depended on it.
- `AuthStateStore::with_table` (validated `[A-Za-z0-9_.]+`, default
  `auth.auth_state`) replaces the five hardcoded table literals, so
  non-`auth`-schema consumers can configure it. `AuthStateError` gained
  `InvalidTableName` and is now `#[non_exhaustive]`. Validation unit test
  added.
- Docs: contract `030` (records the no-central-adapter rationale + rename),
  package map `010`, API inventory `122`, guide `190`, and `g08.019`
  re-pointed. g06 archival logs left per the evidence boundary.

## Consumer Rollout

The four users (`underlay-reference`, `contact-patch`, `compli-me`,
`acowtancy`; songsprout/loophole don't use it) migrated: Cargo dep key + path,
`crates/auth` dep, and the `underlay_auth_state_postgres` import. Each also
gained a wildcard arm in `map_auth_state_error` for the new `#[non_exhaustive]`
variant. All four consumer auth crates `cargo check` clean.

## Validation

- `cargo test -p underlay-auth-state-postgres`: green (table-name validation).
- `cargo test --workspace --all-features`: green.
- Four consumer auth crates (`acme-auth`, `cp-auth`, `compli-me-auth`,
  `farmyard-auth`): `cargo check` clean.

## Consumer Upgrade Notes

Impact class **naming** (breaking). Consumers using the crate rename the dep
`underlay-auth-postgres` -> `underlay-auth-state-postgres` and the import
`underlay_auth_postgres` -> `underlay_auth_state_postgres`. `AuthStateError`
is now `#[non_exhaustive]`, so a `match` on it needs a wildcard arm.
`AuthStateStore::with_table` is available for non-`auth`-schema tables.
Underlay intentionally ships no Postgres adapter for the user/credential/
session/audit repos; apps keep their own.

## Next

`g08.019` postgres adapter integration tests.
