# g06.037 Artifact - Typed DB Identifier Lane Closeout Audit

## Summary

The typed DB identifier lane is complete.

Shared Rust runtime SQL identifier construction is now behind typed or validated
boundaries. Remaining formatted SQL identifier sites are classified as
typed-safe, fixed-literal, or test-only.

## Scan Method

Scanned `rust/crates` for:

- `sqlx::query(&format!(...))`
- formatted SQL verbs
- `FROM {}`, `UPDATE {}`, `INSERT INTO {}`, and `DELETE FROM {}` slots
- `CREATE SCHEMA`, `DROP SCHEMA`, and `SET search_path`
- use of `QualifiedTableName`, `SqlIdentifier`, schema helpers, and `quoted()`

## Classification

| Surface | Classification | Evidence |
| --- | --- | --- |
| `underlay-db::TypedExistsCheck` and typed value helpers | typed-safe | Store `QualifiedTableName` and `SqlIdentifier`; values stay bound. |
| `underlay-db::drop_schema_identifiers` / `drop_schemas` | typed-safe | Raw schema names parse into `SqlIdentifier`; typed drop helper is available. |
| `underlay-audit` table query/write helpers | typed-safe | Public table config stores `QualifiedTableName`; SQL uses `AuditTable::quoted()`. |
| `underlay-security-alerts` table helpers | typed-safe | Public table config stores `QualifiedTableName`; SQL uses typed table wrappers. |
| `underlay-media-postgres::PostgresMediaConfig` | typed-safe | Stores `SqlIdentifier` and `QualifiedTableName`; SQL operation modules consume quoted typed tables. |
| `underlay-testing::TestDb` | typed-safe test helper | Stores generated schema as `SqlIdentifier`; creation/search-path/cleanup use typed quoting. |
| `underlay-devtools::reset_database` | fixed-literal + typed-safe | Drops schemas through `drop_schemas`; recreates literal `public` schema only. |
| `underlay-db` Postgres integration tests | test-only retained | Generate UUID-based schema names and validate them before raw test SQL. |

## Decision

Close the DB identifier lane.

No unclassified shared Rust runtime SQL identifier interpolation remains in the
current scan. The remaining formatted SQL is the expected shape where SQL cannot
bind identifiers and must interpolate quoted typed identifiers.

## Contract Update

Updated:

- `docs/contracts/122-rust-public-api-inventory.md`

## Validation

Validation passed:

- final dynamic identifier scan over `rust/crates`
- `effigy rust:check`
- `effigy qa:docs`
