# g06.187 Artifact - Typed ExistsCheck Soft-Delete Default

Status: complete
Owner: repo maintainers
Completed: 2026-06-06

## Purpose

Resolve the `040` drift hook where `underlay_db::TypedExistsCheck` assumed
`deleted_at IS NULL` unless callers opted out.

## Result

`TypedExistsCheck` is now neutral by default:

- no `deleted_at` predicate is added unless requested
- `.active_only()` opts into `deleted_at IS NULL` for tables using Underlay's
  soft-delete convention
- simple typed value helpers no longer assume a `deleted_at` column

This makes generic database helpers safe for tables that do not use soft delete
and keeps soft-delete behavior explicit at each call site.

## Consumer Upgrade Impact

Impact class: `breaking`.

Consumers that relied on the old implicit `deleted_at IS NULL` filter must add
`.active_only()`.

Current-family scan found direct `TypedExistsCheck` usage only in
`acowtancy/farmyard`. Farmyard was updated in this batch:

- content, pathway, and module uniqueness checks now call `.active_only()`
- area and activity checks no longer call `.include_deleted()` because neutral
  default behavior handles all-row checks

The other current consumers had no direct `TypedExistsCheck` call sites in the
scanned Rust source.

## Validation

- `cargo test -p underlay-db existence --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `cargo check -p farmyard-db -p farmyard-api`
- six-consumer source scan for `TypedExistsCheck` call sites

## Next Task

No active roadmap task remains. Continue with bounded drift repairs only, or
re-enter planning before opening a new Rust hardening lane.
