# g06.022 Artifact - Postgres Runtime Adapter Isolation Batch

## Candidate Inventory

| Candidate | Current Postgres shape | Decision |
| --- | --- | --- |
| `underlay-auth` | One feature-gated `state` module for `AuthStateStore` over `auth.auth_state`. | Selected. Smallest concrete adapter seam with direct consumer proof. |
| `underlay-jobs` | Large feature-gated Postgres store, scheduled tasks, dead letters, outbox, rows, and runner wiring. | Deferred to a dedicated batch. |
| `underlay-audit` | SQLx-backed audit writer/query helpers in the main crate. | Deferred. It has no feature gate today and needs a clearer contract decision. |
| `underlay-security-alerts` | SQLx-backed alert store in the main crate. | Deferred. It is smaller than jobs but less direct than the active auth-state consumer proof. |

## Target

Selected target: `underlay-auth` Postgres state store.

Extraction shape:

- `underlay-auth` remains the app-facing auth contract crate.
- `underlay-auth-postgres` owns `AuthStateStore`, `AuthStateError`, and
  `AuthStateRow`.
- `underlay-auth` no longer has a `postgres` feature, `sqlx` dependency, or
  concrete auth-state table module.

## Consumer Matrix

| Consumer | Source impact | Cargo impact |
| --- | --- | --- |
| `underlay-reference` | `acme-auth` imports `AuthStateStore` / `AuthStateError` from `underlay_auth_postgres`. | Adds `underlay-auth-postgres`; removes `underlay-auth/postgres`. |
| `contact-patch` | `cp-auth` imports `AuthStateStore` / `AuthStateError` from `underlay_auth_postgres`. | Adds `underlay-auth-postgres`; removes `underlay-auth/postgres`. |
| `compli-me` | `compli-me-auth` imports `AuthStateStore` / `AuthStateError` from `underlay_auth_postgres`. | Adds `underlay-auth-postgres`; removes `underlay-auth/postgres`. |
| `acowtancy` | `farmyard-auth` imports `AuthStateStore` / `AuthStateError` from `underlay_auth_postgres`. | Adds `underlay-auth-postgres`; removes `underlay-auth/postgres`. |
| `songsprout` | none | Removes stale `underlay-auth/postgres`. |
| `loophole/composer` | none | Removes stale `underlay-auth/postgres`. |

## Underlay Changes

| File or crate | Change |
| --- | --- |
| `rust/crates/underlay-auth-postgres` | Added new adapter crate. |
| `rust/crates/underlay-auth/src/state.rs` | Moved to `underlay-auth-postgres/src/lib.rs`. |
| `rust/crates/underlay-auth/Cargo.toml` | Removed `postgres` feature and optional `sqlx` / `thiserror` dependencies. |
| `rust/crates/underlay-auth/src/lib.rs` | Removed `state` module and root re-exports. |

## Impact

Impact: breaking.

Known callers of `underlay_auth::state` were updated. Unknown callers must add
`underlay-auth-postgres` and import `AuthStateStore` / `AuthStateError` from
that crate.

No compatibility shim was added.

## Validation

- `cargo check -p underlay-auth --all-features --all-targets` passed.
- `cargo check -p underlay-auth-postgres --all-targets` passed.
- `effigy rust:check` passed.
- `cargo check -p acme-auth -p acme-api` passed in
  `underlay-reference/acme-api`.
- `cargo check -p cp-auth -p cp-api` passed in `contact-patch/cp-api`.
- `cargo check -p compli-me-auth -p compli-me-api` passed in `compli-me/api`.
- `cargo check -p farmyard-auth -p farmyard-api` passed in
  `acowtancy/farmyard`.
- `cargo check -p nursery-auth -p nursery-api` passed in `songsprout/nursery`.
- `cargo check -p composer-auth -p composer-api` passed in
  `loophole/composer/composer-api`.
- `effigy qa:docs` passed.
- `effigy qa:northstar` passed.
- `git diff --check` passed.
