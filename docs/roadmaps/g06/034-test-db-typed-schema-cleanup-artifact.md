# g06.034 Artifact - Test DB Typed Schema Cleanup

## Summary

`underlay-testing::TestDb` now stores its generated schema as a
`SqlIdentifier`.

Changed internals:

- generated test schema names are parsed through the shared typed SQL
  identifier boundary
- `CREATE SCHEMA` uses `SqlIdentifier::quoted()`
- `SET search_path` uses `SqlIdentifier::quoted()`
- cleanup delegates to `underlay_db::drop_schema_identifiers`
- the ignored isolation test binds the inspected schema name instead of
  formatting it into SQL

Public API retained:

- `TestDb::new()`
- `TestDb::pool()`
- `TestDb::schema_name() -> &str`
- fixture, seeding, migration, migrator, and cleanup helpers

## Decision

This is an internal hardening cleanup, not a public test-harness redesign.

The public `schema_name()` accessor remains a string because consumers use it
for assertions, diagnostics, and bound query parameters. SQL construction inside
Underlay now uses the typed identifier value.

## Compatibility Classification

Impact: none expected.

No consumer app updates are required unless a consumer relied on undocumented
private internals, which Rust module privacy prevents.

## Docs And Contract Changes

Updated:

- `docs/contracts/120-tooling-testing-and-contract-artifacts.md`
- `docs/guides/130-testing.md`

## Validation

Validation passed:

- `cargo test -p underlay-testing --features db unique_test_schema`
- `effigy rust:check`
- `effigy qa:docs`
