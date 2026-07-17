# 2026-07-17 - g08.017 pagination collision resolution

## Context

Two public `PaginationParams` types with the same name and different semantics:
`underlay-http` (page/limit offset) and `underlay-db::pagination`
(cursor/keyset + page hybrid). A consumer using both got an import collision and
two pagination philosophies. The card also flagged the dynamic-SQL query seam
(`WhereBuilder`/`FieldMapping` in the http crate) as split in the wrong
direction.

## Changes

- **Renamed to distinct types:** `underlay_http::PagePaginationParams` (offset)
  and `underlay_db::pagination::CursorPaginationParams` (cursor). Each old name
  is kept as a `#[deprecated]` type alias with a `g09` retirement note, giving
  consumers a migration window per contract `023`.
- **`add_raw` hardened:** new `WhereBuilder::add_raw_indexed(value, |idx| ...)`
  composes the `$N` placeholder from an explicit index callback; the fragile
  `{}`-substitution `add_raw` is deprecated.
- **Query-seam relocation deferred to `g08.017b`:** `WhereBuilder` consumes
  http's `FilterField` wire model and consumers' db crates import it from
  `underlay-http`. Moving SQL-gen to `underlay-db` while keeping wire parsing in
  http is a real architecture decision (split `FilterField`, or a new
  `underlay-query` crate). Surfaced per the stop condition, not forced here.

## Six-Consumer Rollout

Every consumer importing either type was migrated to the new names using an
import-only `... as PaginationParams` alias - the app's internal name stays
`PaginationParams`, zero usage-site churn, and the underlying type is the
renamed one. acowtancy's central `crates/api/src/pagination.rs` re-export was
aliased to `CursorPaginationParams` so its `-D warnings` clippy gate stays clean
(no deprecated-alias use). `underlay-reference` had no direct imports.

Note on method: an initial word-boundary rename of usage sites broke
glob-imported leaf modules (`use super::*`); reverted and redone as
import-line-only aliasing, which is the correct minimal-churn migration.

## Validation

- `cargo check --workspace --all-features` and
  `cargo test --workspace --all-features`: green (73 suites, 0 failures).
- All six consumer api crates `cargo check` clean (acowtancy `--workspace`,
  including its strict clippy posture).

## Consumer Upgrade Notes

Impact class **behavioral** (type rename). Consumers import
`PagePaginationParams` (http) or `CursorPaginationParams` (db) directly, or keep
their local name via `... as PaginationParams`. The deprecated `PaginationParams`
aliases compile with a warning and retire in `g09`. `add_raw` callers should
move to `add_raw_indexed` (no in-tree callers existed).

## Next

`g08.017b` query-seam relocation decision (recommend an `underlay-query` crate),
then `g08.018` auth-postgres adapter decision.
