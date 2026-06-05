# g06.054 Artifact - Media Renditions Internal Split

## Summary

`underlay-media/src/renditions.rs` is now a small public front door over
private rendition modules.

New internal modules:

- `renditions/config.rs`
- `renditions/result.rs`
- `renditions/keys.rs`
- `renditions/processing.rs`
- `renditions/service.rs`

The split preserves existing `underlay_media::renditions::*` exports.

## Public API Impact

Impact: none expected.

No feature flag, repository trait, generated object-key format, raw-string
wrapper behavior, or typed result-key behavior changed.

## Structural Impact

`renditions.rs` moved from a 633-line service file to an 18-line front door.

Largest new rendition modules:

- `renditions/service.rs`: 432 lines
- `renditions/config.rs`: 109 lines
- `renditions/processing.rs`: 57 lines

The source-read, image-generation, and blob-write helper is now isolated in
`processing.rs`, and object-key parsing is isolated in `keys.rs`.

`effigy doctor` still fails on the known structural backlog, but the god-file
scan moved from 60 findings and 17 errors after `g06.053` to 60 findings and
16 errors.

## Validation

- `cargo test -p underlay-media --all-features`
- `cargo test -p underlay-media-postgres --all-features`
- `effigy rust:check`
- `effigy doctor` - expected structural backlog failure, with one fewer
  god-file error
- `effigy qa:docs`
- `effigy qa:northstar`
