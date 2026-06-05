# g06.031 Artifact - Remaining Typed DB Helper Migration Plan

## Summary

`g06.031` classified the remaining dynamic SQL identifier helper surface after
the audit/security-alert wrapper removal.

The next code batch should focus on `underlay-db::ExistsCheck`. Other inspected
surfaces are either already typed enough or test-only.

## Findings

### `underlay-db::ExistsCheck`

Status: retained raw builder, next execution target.

Current shape:

- `ExistsCheck::new(schema: &str, table: &str)`
- condition methods accept raw column names:
  - `value(column, value)`
  - `value_i32(column, value)`
  - `scope(column, value)`
  - `nullable_value(column, value)`
- execution validates and quotes identifiers before SQL construction
- typed helpers already exist for the simplest case:
  - `value_exists_typed()`
  - `value_exists_excluding_typed()`

Assessment:

- no immediate injection issue; identifiers are validated and quoted before SQL
- public shape still teaches raw schema/table/column strings
- current typed helpers are too narrow for the composite constraints that drove
  real consumer usage

Plan:

1. Add a typed builder, probably `TypedExistsCheck` or `ExistsCheck::from_table`.
2. Store `QualifiedTableName` and `SqlIdentifier` values internally.
3. Add typed condition methods for string, integer, UUID scope, nullable integer,
   exclusion column if needed, and soft-delete behavior.
4. Reimplement raw `ExistsCheck` as a compatibility wrapper or deprecate it only
   after consumer migration proof.
5. Migrate the six consumers, which currently means Farmyard call sites.

### Raw Existence Helper Functions

Status: compatibility candidates.

Current helpers:

- `value_exists()`
- `value_exists_excluding()`
- `value_exists_in_scope()`
- `value_exists_in_scope_excluding()`
- `number_exists_in_scope()`
- `number_exists_in_scope_excluding()`

Assessment:

- all route through `ExistsCheck`, so validation still happens
- they should not be removed before Farmyard migration
- they can become deprecated after `g06.032` proves typed call sites

### `underlay-media-postgres::PostgresMediaConfig`

Status: retained.

Current shape:

- `try_with_schema()` validates schema as `SqlIdentifier`
- `try_with_tables()` validates each table as `SqlIdentifier`
- query operations use quoted fully qualified names
- no scanned current consumer uses custom `PostgresMediaConfig` table APIs

Assessment:

- acceptable for now
- future improvement could store typed identifiers internally, but the public
  safe constructors already enforce the boundary
- no `g06.032` code needed

### `underlay-testing::TestDb`

Status: retained test helper.

Current shape:

- schema name is generated internally from UUIDv7 characters
- dynamic SQL uses the generated schema for `CREATE SCHEMA` and `SET search_path`
- not app supplied

Assessment:

- no app-facing injection issue
- can be cleaned up later by constructing `SqlIdentifier`, but this should not
  block the runtime helper migration

## Consumer Evidence

Six-consumer scan:

- `ExistsCheck` / raw existence helper usage appears in Farmyard only.
- No current scanned consumer uses `PostgresMediaConfig` custom table APIs.
- `TestDb` use appears in docs and local consumer test helpers, but not as an
  app runtime dynamic identifier surface.

Farmyard call-site families:

- content slug/key checks via `value_exists()` / `value_exists_excluding()`
- learning module, area, activity, and pathway uniqueness checks via
  `ExistsCheck::new(...)`

## Recommended Next Batch

Open `g06.032` as an additive code batch:

- add typed composite existence builder in `underlay-db`
- keep raw helpers initially
- migrate Farmyard
- run targeted Underlay and Farmyard checks
- then decide deprecation/removal posture for raw helpers

## Consumer Impact

Impact for `g06.031`: none, planning only.

Expected impact for `g06.032`: additive first, with possible deprecation after
Farmyard migration proof.
