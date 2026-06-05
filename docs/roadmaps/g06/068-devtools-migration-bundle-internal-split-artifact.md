# g06.068 Artifact - Devtools Migration-Bundle Internal Split

## Summary

`underlay-devtools/src/migration_bundle.rs` is now a small tooling front door.
The public crate-root API is unchanged.

New private modules:

- `migration_bundle/model.rs`: public option/report/error/ref types
- `migration_bundle/package.rs`: package model, digest, payload decode, layout
  validation, and layer descriptor helpers
- `migration_bundle/build.rs`: bundle build orchestration
- `migration_bundle/output.rs`: pulled layout and media-shard output writing
- `migration_bundle/run.rs`: publish, pull, and run routing

Existing private modules remain in place:

- `migration_bundle/local_store.rs`
- `migration_bundle/remote_registry.rs`
- `migration_bundle/media_shards.rs`

## Preserved Behavior

The split preserved:

- crate-root `underlay_devtools::*` migration-bundle exports
- public option/report/error/ref type names and fields
- `BundleRunOptions::from_bundle_ref` and `BundleRunOptions::bundle_ref`
- `MigrationBundleRef::parse_digest_pinned`, `as_str`, `digest`, `Display`,
  and `FromStr`
- build, publish, pull, and run function names and behavior
- digest mismatch rejection for publish, local pull, and remote pull
- local-store fallback order: explicit option, `UNDERLAY_LOCAL_OCI_DIR`,
  `.underlay-local-oci`
- remote registry tag publish and digest pull semantics
- pulled-output paths, including sanitized media-shard filenames
- package JSON shape, OCI layout validation, layer descriptors, sidecars, and
  media-shard mapping validation
- crate-private helper access needed by `seed_bundle.rs` and tests

## Structural Result

`migration_bundle.rs` moved from a high-error god-file into a front door:

- `migration_bundle.rs`: 27 lines
- `migration_bundle/build.rs`: 165 lines
- `migration_bundle/model.rs`: 164 lines
- `migration_bundle/package.rs`: 101 lines
- `migration_bundle/output.rs`: 37 lines
- `migration_bundle/run.rs`: 86 lines

`effigy doctor` now reports:

- `scan.god-files`: 54 findings, 13 errors, 41 warnings
- `scan.attention-markers`: 11 findings, 2 errors, 9 warnings
- `scan.comment-ratio`: 12 findings, 3 errors, 9 warnings

The doctor failure remains the known structural backlog.

## Public API Impact

Impact: none.

This was a private module split. No consumer app update is required.

## Validation

- `cargo test -p underlay-devtools --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` expected failure on known structural scans
