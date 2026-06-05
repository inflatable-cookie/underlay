# g06.060 Artifact - HTTP Query Internal Split

## Summary

`underlay-http/src/query.rs` is now a small public front door over focused
private modules.

New private module layout:

- `query/sort.rs`: `SortDirection`, `SortField`, `parse_sort_string`
- `query/filter.rs`: `FilterOperator`, `FilterField`
- `query/params.rs`: `QueryParams` and filter extraction
- `query/where_builder.rs`: `WhereBuilder`
- `query/field_mapping.rs`: `FieldMapping`

## Compatibility

The split preserves:

- `underlay_http::query::*`
- crate-root exports from `underlay-http/src/lib.rs`
- `field_mapping!` macro export
- `$crate::query::FieldMapping` macro path
- sort parsing default behavior
- filter bracket parsing behavior
- SQL operator strings and SQL fragment behavior
- `FieldMapping` map/sort-only/filter-only behavior

## Public API Impact

Expected impact: none.

This was a private module split. The only visibility adjustment was making the
deserialized raw filter map `pub(crate)` so crate-local tests can continue to
construct `QueryParams` with struct update syntax after the type moved into a
private child module. External visibility remains unchanged.

## Validation

- `cargo test -p underlay-http --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` failed on known structural backlog:
  `scan.attention-markers`, `scan.comment-ratio`, and `scan.god-files`

Structural movement:

- `underlay-http/src/query.rs`: 593 lines to 68 lines
- `scan.god-files`: 58 findings to 57 findings

Next batch validation:

- `cargo test -p underlay-http --all-features`
- `effigy rust:check`
- consumer checks only if public import paths move
- `effigy qa:docs`
- `effigy qa:northstar`
