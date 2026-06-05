# g06.104 Artifact - Media Renditions Service Internal Split

## Summary

`underlay-media/src/renditions/service.rs` is now a small module front door.
The former mixed service implementation was split into focused internal
modules.

New module shape:

- `service.rs`: front door and stable `RenditionService` re-export
- `service/core.rs`: `RenditionService` type, construction, accessors, and
  `Clone`
- `service/generate.rs`: thumbnail, preview, and raw-byte rendition generation
- `service/delete.rs`: single-rendition and version-rendition blob deletion
- `service/version.rs`: legacy and standardized version rendition generation

## Public API Impact

None expected.

The public `underlay_media::renditions::RenditionService` export, service
constructors, accessors, generation methods, deletion methods, clone behavior,
object-key parsing, generated key semantics, repository persistence input
fields, storage provider metadata, and warning behavior were preserved.

## Validation

- `cargo test -p underlay-media --all-features`
- `effigy rust:check`

`cargo test -p underlay-media --all-features` passed with 56 unit tests passed,
5 doc-tests passed, and 6 doc-tests ignored.

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 36 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

The media renditions service no longer appears in the god-file report. The next
largest Rust warning is
`rust/crates/underlay-migration-core/src/tests/pipeline_basic_tests.rs`.

## Next Target Evidence

Queue `g06.105` as a migration-core pipeline basic tests modularity audit.
This is a shared migration behavior test file, so the next batch should
classify fixture setup, repository flows, pipeline assertions, failure cases,
and helper extraction before moving test code.
