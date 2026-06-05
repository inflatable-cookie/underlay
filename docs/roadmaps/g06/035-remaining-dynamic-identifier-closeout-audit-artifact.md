# g06.035 Artifact - Remaining Dynamic Identifier Closeout Audit

## Summary

The shared Rust dynamic SQL identifier lane is almost closed.

Most runtime paths now route table, schema, and column identifiers through
`underlay-db::SqlIdentifier`, `QualifiedTableName`, typed wrapper types, or
schema helpers before SQL construction.

One meaningful runtime cleanup remains:

- `underlay-media-postgres::PostgresMediaConfig`

## Audit Method

Scanned shared Rust crates for:

- `sqlx::query(&format!(...))`
- formatted SQL verbs
- `CREATE SCHEMA`, `DROP SCHEMA`, and `SET search_path`
- `FROM {}`, `UPDATE {}`, `INSERT INTO {}`, `DELETE FROM {}` patterns
- uses of `quoted()`, `format_schema_table`, `quote_sql_identifier`,
  `QualifiedTableName`, `SqlIdentifier`, and schema helpers

## Classification

| Surface | Classification | Rationale |
| --- | --- | --- |
| `underlay-db::TypedExistsCheck` and typed value helpers | typed-safe | Store `QualifiedTableName` and `SqlIdentifier`; runtime values remain bound. |
| `underlay-db::drop_schema_identifiers` / `drop_schemas` | typed-safe | Public raw helper parses into `SqlIdentifier`; typed helper is available and used by `TestDb`. |
| `underlay-audit::AuditTable` query/write helpers | typed-safe | Public table config stores `QualifiedTableName`; raw wrappers were retired earlier. |
| `underlay-security-alerts` table helpers | typed-safe | Public table configs store `QualifiedTableName`; SQL uses quoted table names. |
| `underlay-testing::TestDb` | typed-safe test helper | Stores generated schema as `SqlIdentifier`; creation, search path, and cleanup use typed quoting. |
| `underlay-devtools::reset_database` | typed-safe tooling | Delegates schema dropping to `drop_schemas`; `CREATE SCHEMA public` is a fixed literal. |
| `underlay-db` Postgres integration tests | test-only retained | Generate schema names from UUIDs and validate before raw test SQL. This is not app-facing runtime surface. |
| `underlay-media-postgres::PostgresMediaConfig` | needs cleanup | `try_with_schema` / `try_with_tables` validate, and query-time formatting quotes, but the public config still stores raw string fields and `with_schema` accepts unchecked raw input. |

## Decision

Open one final runtime cleanup batch for `PostgresMediaConfig`.

This is not a live SQL injection issue because query-time formatting validates
and quotes identifiers before SQL construction. It remains the weaker reference
shape because identifier validation is late and public fields can be mutated
after construction.

## Consumer Evidence

Prior six-consumer scans found no current custom `PostgresMediaConfig` table
configuration usage. Expected consumer impact is low, but the config is public,
so the next batch should classify any field visibility or constructor changes
under the compatibility contract and check current consumers.

## Contract Update

Updated:

- `docs/contracts/122-rust-public-api-inventory.md`

## Validation

Validation passed:

- dynamic identifier scan over `rust/crates`
- `effigy qa:docs`
