# g06.058 Artifact - DB Pagination Internal Split

## Summary

`underlay-db/src/pagination.rs` is now a small public front door over private
pagination modules.

New internal modules:

- `pagination/params.rs`
- `pagination/response.rs`
- `pagination/errors.rs`
- `pagination/cursor.rs`
- `pagination/builder.rs`
- `pagination/typed_cursors.rs`

The split preserves existing `underlay_db::pagination::*` compatibility.

## Public API Impact

Impact: none expected.

No cursor encoding, response shape, params shape, SQL helper behavior, or
public item name changed.

The only test update made the `Uuid` import explicit in
`pagination_tests.rs`; that name was previously inherited from the monolithic
parent module.

## Structural Impact

`pagination.rs` moved from a 598-line monolith to a 45-line front door.

Largest new pagination modules:

- `pagination/builder.rs`: 183 lines
- `pagination/cursor.rs`: 137 lines
- `pagination/params.rs`: 106 lines
- `pagination/response.rs`: 72 lines

`effigy doctor` still fails on the known structural backlog, but the god-file
scan moved from 59 findings and 15 errors after `g06.056` to 58 findings and
15 errors.

## Validation

- `cargo test -p underlay-db --all-features`
- `effigy rust:check`
- `effigy doctor` - expected structural backlog failure, with one fewer
  god-file finding
- `effigy qa:docs`
- `effigy qa:northstar`
