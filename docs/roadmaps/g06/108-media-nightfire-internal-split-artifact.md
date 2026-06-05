# g06.108 Artifact - Media Nightfire Internal Split

## Summary

`underlay-media/src/nightfire.rs` is now a small module front door with stable
public re-exports. The previous mixed front file was split into focused
internal modules.

New module shape:

- `nightfire.rs`: front door, public re-exports, and test module
- `nightfire/context.rs`: `NightfireMediaVisitContext`
- `nightfire/resolver.rs`: `resolve_nightfire_media_usage()`
- `nightfire/matcher.rs`: media reference match, matcher trait, field rules,
  and common field-name matcher
- `nightfire/extractor.rs`: field-name and block-registry extractor types,
  sync methods, owner-field validation, and walker trait impls
- `nightfire/registry.rs`: existing block handler registry surface
- `nightfire/walk.rs`: existing recursive walking internals

## Public API Impact

None expected.

The public `underlay_media::nightfire::*` exports, Nightfire locator behavior,
owner-field mismatch errors, persisted-owner validation, common field names,
handler-map registration behavior, recursive walk semantics, and sync report
behavior were preserved.

## Validation

- `cargo test -p underlay-media --all-features`
- `effigy rust:check`

`cargo test -p underlay-media --all-features` passed with 56 unit tests passed,
5 doc-tests passed, and 6 doc-tests ignored.

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 34 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

`underlay-media/src/nightfire.rs` no longer appears in the god-file report. The
next largest Rust warning is `rust/crates/underlay-devtools/src/seed_bundle.rs`.

## Next Target Evidence

Queue `g06.109` as a devtools seed-bundle modularity audit before splitting
`underlay-devtools/src/seed_bundle.rs`. Seed bundles are shared development and
bootstrap tooling, so the next batch should classify bundle models, filesystem
I/O, validation, serialization, and tests before moving code.
