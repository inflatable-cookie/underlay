# g08.017 - Pagination Collision Resolution

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Resolve two public `PaginationParams` types with the same name and different
semantics. `underlay-http/src/pagination.rs` is a page/limit offset model;
`underlay-db/src/pagination/params.rs` is a cursor/keyset model with direction
and include_total. Both are exported, so a consumer using both crates gets an
import collision and two pagination philosophies. Separately, dynamic SQL WHERE
building lives in the HTTP crate (`where_builder.rs`) while the DB crate owns
`PaginationBuilder`/identifier quoting, splitting the query seam in the wrong
direction.

## Evidence

- `rust/crates/underlay-http/src/pagination.rs`
- `rust/crates/underlay-db/src/pagination/params.rs`
- `rust/crates/underlay-http/src/query/where_builder.rs`

## Governing References

- [010 Foundation primitives and envelopes](../../contracts/010-foundation-primitives-and-envelopes.md)
- [116 Canonical collection routes and query profiles](../../contracts/116-canonical-collection-routes-and-query-profiles.md)

## Planned Changes

- [x] Renamed to distinct types: `underlay_http::PagePaginationParams` (offset)
  and `underlay_db::pagination::CursorPaginationParams` (cursor/keyset), each
  with a `#[deprecated]` `PaginationParams` alias for a g09 retirement window.
- [~] **Deferred to `g08.017b` (boundary decision).** `WhereBuilder` consumes
  http's `FilterField` wire model, and consumers' db-layer crates already
  import `WhereBuilder`/`FieldMapping` from `underlay-http` (the wrong-direction
  seam). Moving SQL generation to `underlay-db` while keeping wire parsing in
  http requires an explicit call on splitting `FilterField` or introducing an
  `underlay-query` crate - the stop-condition boundary decision. Split into
  `g08.017b` rather than forced here.
- [x] Replaced `add_raw`'s `{}` substitution with `add_raw_indexed`, an explicit
  placeholder-index callback (old `add_raw` deprecated). Typed-value enum is
  bundled with the relocation decision below.

## Consumer Upgrade Impact

Impact class: `behavioral`. Type names and query-builder location change.
Requires six-consumer proof per `023`.

## Validation

- [ ] no name collision on dual import; SQL generation lives in the DB layer
- [ ] `cargo test -p underlay-http -p underlay-db`
- [ ] `effigy validate`

## Stop Conditions

Stop if a `underlay-query` crate is warranted; that is a boundary decision worth
surfacing before the move.

## Completion Notes

Completed 2026-07-17 (collision + add_raw scope; relocation split to
`g08.017b`).
- The two same-named `PaginationParams` are now distinct:
  `underlay_http::PagePaginationParams` and
  `underlay_db::pagination::CursorPaginationParams`. Each old name remains a
  `#[deprecated]` type alias (retirement planned `g09`) so consumers migrate on
  a window.
- `WhereBuilder::add_raw_indexed(value, |idx| ...)` replaces the fragile `{}`
  string substitution; the old `add_raw` is deprecated.
- **Six-consumer rollout:** every consumer that imported either type was
  migrated to the new names via an import-only `... as PaginationParams` alias
  (keeps each app's internal name stable, zero usage-site churn). acowtancy's
  central `crates/api/src/pagination.rs` re-export was aliased to
  `CursorPaginationParams` so its `-D warnings` clippy gate stays clean.
  underlay-reference had no direct imports. All six api crates `cargo check`
  clean.
- The query-seam relocation (`WhereBuilder`/`FieldMapping` -> `underlay-db` vs
  a new `underlay-query` crate, plus typed `WhereBuilder` values) is a real
  architecture decision surfaced per the stop condition and carried into
  `g08.017b`.

Validated: `cargo check/test --workspace --all-features` green (73 suites, 0
failures); all six consumer api crates `cargo check` clean.

## Next Task

`g08.018` auth-postgres adapter decision.
