# g06.059 Artifact - HTTP Query Public Model Modularity Audit

## Summary

`underlay-http/src/query.rs` is stable app-facing HTTP helper surface with a
safe internal split shape if both public front doors stay intact.

The file currently groups:

- sort model and parsing: `SortDirection`, `SortField`, `parse_sort_string`
- filter model: `FilterOperator`, `FilterField`
- query extraction: `QueryParams`
- SQL filter builder: `WhereBuilder`
- field mapping helper: `FieldMapping`
- macro helper: `field_mapping!`

## Consumer Evidence

Consumer usage is broad and uses both public paths:

- `underlay_http::query::{FieldMapping, QueryParams, WhereBuilder}`
- `underlay_http::{FieldMapping, SortDirection, WhereBuilder}`
- `underlay_http::{..., QueryParams}`
- `underlay_http::query::SortField::asc(...)`

The `field_mapping!` macro also appears in docs/contracts, and its exported
path depends on `$crate::query::FieldMapping`.

## Decision

Queue `g06.060` as an HTTP query internal split.

The split should preserve:

- `underlay_http::query::*` compatibility
- crate-root query exports from `underlay-http/src/lib.rs`
- `field_mapping!` macro behavior and `$crate::query::FieldMapping` path
- sort parsing and default-direction behavior
- filter bracket parsing behavior
- SQL operator strings and SQL fragment behavior
- `FieldMapping` map/sort-only/filter-only behavior

## Public API Impact

Expected impact: none.

This should be a private module split only. If query parsing semantics, macro
paths, SQL fragment behavior, or public import paths must change, stop and
re-enter planning.

## Validation

- `cargo test -p underlay-http --all-features`
- `effigy qa:docs`
- `effigy qa:northstar`

Next code batch validation:

- `cargo test -p underlay-http --all-features`
- `effigy rust:check`
- consumer checks only if public import paths move
- `effigy qa:docs`
- `effigy qa:northstar`
