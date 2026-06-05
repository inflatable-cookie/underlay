# g06.057 Artifact - DB Pagination Public Model Modularity Audit

## Summary

`underlay-db/src/pagination.rs` is stable app-facing helper surface with a safe
internal split shape if the public module front door stays intact.

The file currently groups:

- constants and params: `DEFAULT_PAGE_SIZE`, `MAX_PAGE_SIZE`,
  `PaginationDirection`, `PaginationParams`
- response wrapper: `PaginatedResponse<T>`
- cursor errors: `CursorError`
- generic cursor encoding/decoding: `Cursor`
- SQL/keyset helpers: `PaginationBuilder`
- typed cursor helpers: `WeightCursor`, `TimestampCursor`

## Consumer Evidence

Consumer usage is direct and broad. Current consumers import from
`underlay_db::pagination::{...}` rather than crate-root DB exports.

Observed imported names include:

- `Cursor`
- `CursorError`
- `PaginatedResponse`
- `PaginationBuilder`
- `PaginationDirection`
- `PaginationParams`
- `TimestampCursor`
- `WeightCursor`
- `DEFAULT_PAGE_SIZE`
- `MAX_PAGE_SIZE`

Acowtancy also re-exports the pagination family from an app-local API
pagination module, so item names and module-path compatibility matter.

## Decision

Queue `g06.058` as a DB pagination internal split.

The split should preserve:

- `underlay_db::pagination::*` compatibility
- all current public item names
- cursor encoding and decoding semantics
- `PaginatedResponse<T>` serialized shape
- `PaginationParams` defaults and serialized shape
- `PaginationBuilder` SQL fragment behavior
- typed cursor helper behavior

## Public API Impact

Expected impact: none.

This should be a private module split only. If cursor encoding, response shape,
or public import paths must change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-db --all-features`
- `effigy qa:docs`
- `effigy qa:northstar`

Next code batch validation:

- `cargo test -p underlay-db --all-features`
- `effigy rust:check`
- consumer checks only if public import paths move
- `effigy qa:docs`
- `effigy qa:northstar`
