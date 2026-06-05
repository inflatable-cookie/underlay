# g06.067 Artifact - Devtools Migration-Bundle Public Model Modularity Audit

## Summary

`underlay-devtools/src/migration_bundle.rs` is a tooling-only migration-bundle
front door with a narrow public crate-root API and a broad private helper body.

The file currently groups:

- public option, report, error, and typed reference types
- public build, publish, pull, and run entry points
- digest-pinned `MigrationBundleRef` parsing and validation
- package encode/decode, payload decode, digest, and OCI layer helper logic
- pulled-output writing for layout and media shards
- local-store and remote-registry routing
- media-shard collection and validation through existing private modules

## Consumer Evidence

Public usage is crate-root oriented:

- `underlay-devtools/src/lib.rs` re-exports the migration-bundle option/report
  types, `MigrationBundleRef`, `MigrationBundleError`, and entry functions.
- The `underlay-devtools` CLI calls the crate-root build, publish, pull, and
  run functions.
- `seed_bundle.rs` uses private migration-bundle internals inside the same
  crate for local-store and publish/pull delegation.
- Migration-bundle tests exercise local publish/pull, digest mismatch
  rejection, digest-pinned run requirements, typed `MigrationBundleRef`, pulled
  media-shard path sanitization, deterministic media shards, and ignored remote
  registry round-trip behavior.
- The current consumer scan found no named consumer importing devtools
  migration-bundle symbols directly.

## Decision

Queue `g06.068` as a devtools migration-bundle internal split.

The split should preserve:

- crate-root exports from `underlay_devtools`
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

## Public API Impact

Expected impact: none.

This should be a private module/function split only. If the split requires
changing crate-root exports, public struct fields, function signatures,
digest-pinned reference behavior, package JSON shape, or local/remote
publish/pull semantics, stop and re-enter planning.

## Validation

- `cargo test -p underlay-devtools --all-features`

Next code batch validation:

- `cargo test -p underlay-devtools --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
