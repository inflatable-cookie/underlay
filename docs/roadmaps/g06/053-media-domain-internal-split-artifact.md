# g06.053 Artifact - Media Domain Internal Split

## Summary

`underlay-media/src/domain.rs` is now a small public front door over private
domain modules.

New internal modules:

- `domain/identifiers.rs`
- `domain/kinds.rs`
- `domain/rendition_types.rs`
- `domain/entities.rs`
- `domain/usage_edges.rs`
- `domain/migrated_attachments.rs`
- `domain/inputs.rs`

The split preserves existing `underlay_media` root exports and
`underlay_media::domain::*` compatibility.

## Public API Impact

Impact: none expected.

No repository trait, serialized model shape, object-key field type, or Postgres
adapter behavior changed.

The only test update made `Uuid` and `Utc` imports explicit in
`domain_tests.rs`; those names were previously inherited from the monolithic
parent module.

## Structural Impact

`domain.rs` moved from a 706-line monolith to a 25-line front door.

Largest new domain modules:

- `domain/entities.rs`: 191 lines
- `domain/usage_edges.rs`: 182 lines
- `domain/identifiers.rs`: 129 lines
- `domain/inputs.rs`: 95 lines

`effigy doctor` still fails on the known structural backlog, but the god-file
scan moved from 61 findings and 18 errors to 60 findings and 17 errors.

## Validation

- `cargo test -p underlay-media --all-features`
- `cargo test -p underlay-media-postgres --all-features`
- `effigy rust:check`
- `effigy doctor` - expected structural backlog failure, with one fewer
  god-file error
- `effigy qa:docs`
- `effigy qa:northstar`
