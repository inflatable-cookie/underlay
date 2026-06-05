# g06.036 Artifact - Postgres Media Config Typed Identifier Cleanup

## Summary

`underlay-media-postgres::PostgresMediaConfig` now stores typed schema/table
identifiers internally.

Changed:

- raw public config fields became private typed fields
- schema storage moved to `SqlIdentifier`
- table storage moved to `QualifiedTableName`
- `with_schema` now validates immediately and panics on invalid identifiers
- `try_with_schema` remains the fallible path for external config
- `try_with_tables` validates table identifiers at config construction
- SQL operation modules consume already-typed, already-quoted table names
- table accessors expose typed identifiers for inspection

Retained:

- `PostgresMediaConfig::default()`
- `PostgresMediaConfig::with_schema(...)`
- `PostgresMediaConfig::try_with_schema(...)`
- `PostgresMediaConfig::try_with_tables(...)`
- `PostgresMediaRepository::with_config(...)`
- `PostgresMediaRepository::config()`

## Compatibility Classification

Impact: narrow breaking public struct cleanup.

The direct raw fields are no longer public. Current six-consumer scan found no
custom `PostgresMediaConfig` construction or field access, so no consumer code
updates were required.

## Consumer Evidence

Scanned current consumer family:

- `underlay-reference`
- `contact-patch`
- `compli-me`
- `acowtancy`
- `songsprout`
- `loophole/composer`

Result:

- no `PostgresMediaConfig` custom construction
- no direct media config table field access
- all six Rust workspaces still pass `cargo check --workspace`

## Docs And Contract Changes

Updated:

- `docs/contracts/040-storage-blob-and-media-systems.md`
- `docs/contracts/122-rust-public-api-inventory.md`

## Validation

Validation passed:

- `cargo test -p underlay-media-postgres --all-targets`
- `effigy rust:check`
- `effigy qa:docs`
- `cargo check --workspace` in `underlay-reference/acme-api`
- `cargo check --workspace` in `contact-patch/cp-api`
- `cargo check --workspace` in `compli-me/api`
- `cargo check --workspace` in `acowtancy/farmyard`
- `cargo check --workspace` in `songsprout/nursery`
- `cargo check --workspace` in `loophole/composer/composer-api`

Known non-blocking output:

- Farmyard emitted an existing dead-code warning in `farmyard-migration`.
