# g06.021 Artifact - Media Postgres Adapter Extraction Proof

## Target

Selected target: `underlay-media` Postgres adapter.

Extraction shape:

- `underlay-media` remains the contract crate.
- `underlay-media-postgres` owns `PostgresMediaRepository`,
  `PostgresMediaConfig`, SQL operation modules, and database row mappings.
- `underlay-media` no longer has a `postgres` feature, `sqlx` dependency,
  `postgres` module, or Postgres row internals.

## Consumer Matrix

| Consumer | Direct `underlay_media::postgres` usage | Cargo impact |
| --- | --- | --- |
| `underlay-reference` | none | No source update needed. Existing `underlay-media` workspace dependency remains valid. |
| `contact-patch` | none | No source update needed. |
| `compli-me` | none | No source update needed. |
| `acowtancy` | none | No source update needed. Its `underlay-media` `full` feature now means `renditions` + `nightfire`, not Postgres. |
| `songsprout` | none | No source update needed. |
| `loophole/composer` | none | No source update needed. |

## Underlay Changes

| File or crate | Change |
| --- | --- |
| `rust/crates/underlay-media-postgres` | Added new adapter crate. |
| `rust/crates/underlay-media/src/postgres.rs` | Moved to `underlay-media-postgres/src/lib.rs`. |
| `rust/crates/underlay-media/src/postgres/` | Moved to `underlay-media-postgres/src/`. |
| `rust/crates/underlay-media/src/postgres_rows.rs` | Moved to `underlay-media-postgres/src/postgres_rows.rs`. |
| `rust/crates/underlay-media/Cargo.toml` | Removed `postgres` feature and optional `sqlx` dependency. |
| `rust/crates/underlay-media/src/lib.rs` | Removed Postgres module declarations. |
| `rust/crates/underlay-media/src/error.rs` | Removed `sqlx::Error` conversion from the contract crate. |

## Impact

Impact: breaking for unknown callers of `underlay_media::postgres` or the
`underlay-media/postgres` feature.

No compatibility shim was added. The six known consumers do not import that
module.

## Validation

- `cargo check -p underlay-media --all-features --all-targets` passed.
- `cargo check -p underlay-media-postgres --all-targets` passed.
- `cargo check -p underlay-devtools --all-features --all-targets` passed.
- `effigy rust:check` passed.
- `cargo check -p acme-db -p acme-api` passed in
  `underlay-reference/acme-api`.
- `cargo check -p farmyard-domain -p nightfire` passed in
  `acowtancy/farmyard`.
- `cargo check -p composer-db` passed in
  `loophole/composer/composer-api`.
- `effigy qa:docs` passed.
- `effigy qa:northstar` passed.
- `git diff --check` passed.
