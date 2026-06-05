# g06.005 - DB Identifier And Schema Boundary Normalization

## Why

The Rust audit added typed SQL identifier primitives and routed the highest-risk
dynamic SQL paths through them.

That is the right first layer, but raw schema, table, and column strings still
exist as compatibility surface across database-adjacent crates.

## Goal

Normalize dynamic database identifier construction around `underlay-db` typed
identifier helpers and record which raw-string APIs remain compatibility
surface versus migration targets.

## Scope

In scope:

- audit dynamic schema, table, and column construction in Rust crates
- prefer `SqlIdentifier` and `QualifiedTableName` for new public APIs
- keep SQL values as bound parameters
- classify raw table-name helper APIs as stable compatibility, deprecation
  candidates, or internal-only
- prove audit, security-alert, existence, media, and test schema helper paths
  are covered

Out of scope:

- changing app database schemas
- moving consumer migrations
- broad repository trait redesign
- retiring raw table-name helpers before consumer proof exists

## Contract References

- `021`: database migration and schema workflow
- `023`: release and compatibility rollout
- `040`: storage, blob, and media systems
- `122`: Rust public API inventory

## Consumer Upgrade Impact

Impact classification: `additive` for typed identifier APIs.

Any raw-string helper retirement is `deprecation` and needs consumer proof
before landing.

## Acceptance Criteria

- dynamic identifier call sites in Underlay Rust crates are inventoried
- new DB-adjacent APIs prefer `SqlIdentifier` / `QualifiedTableName`
- values remain bound parameters rather than formatted SQL literals
- raw table-name compatibility surfaces are explicitly classified
- targeted Rust tests cover identifier parsing and representative dynamic SQL
  construction

## Inventory

Covered dynamic identifier paths:

- `underlay-db::ExistsCheck` quotes schema/table through
  `format_schema_table` and columns through `quote_sql_identifier`
- `underlay-db::value_exists_typed` and
  `value_exists_excluding_typed` provide typed helper paths for new string-value
  existence checks
- `underlay-db::drop_schemas` now parses schema names through
  `SqlIdentifier` and quotes before `DROP SCHEMA`
- `underlay-db::drop_schema_identifiers` provides the typed destructive schema
  helper for new reset tooling
- `underlay-audit` validates and quotes app-supplied audit table names through
  `underlay-db`
- `underlay-security-alerts` validates and quotes app-supplied alert/login table
  names through `underlay-db`
- `underlay-media` Postgres table config formats table names through
  `underlay-db`

Retained compatibility surfaces:

- `validate_schema_name`, `drop_schemas`, `ExistsCheck::new`, and raw
  `value_exists*` helpers remain source-compatible raw-string APIs
- consumer apps, especially `acowtancy/farmyard`, still call raw
  `ExistsCheck::new` and `value_exists`; this batch does not migrate or
  deprecate those callers
- future deprecation needs consumer proof under `023`

## Code Changes

- Added `parse_schema_name` and `drop_schema_identifiers`.
- Routed `drop_schemas` through typed schema parsing and quoted SQL.
- Added `value_exists_typed` and `value_exists_excluding_typed`.
- Added focused tests for typed schema parsing and typed existence-query SQL
  construction.
- Extended `021` with the dynamic identifier rule.

## Validation

- `cargo test -p underlay-db --all-features identifiers`
- `cargo test -p underlay-db --all-features schema`
- `cargo test -p underlay-db --all-features exists`
- `cargo clippy -p underlay-db --all-features --all-targets -- -D warnings`
- `git diff --check`

## Current State

`g06.005` is complete.

## Next Task

Execute `g06.006`: media repository contract and adapter split completion.
