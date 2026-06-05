# g06.109 Artifact - Devtools Seed Bundle Modularity Audit

## Summary

`underlay-devtools/src/seed_bundle.rs` is the largest remaining Rust
warning-level production file after `g06.108`. It combines public option/report
types, private package and manifest models, seed-bundle build, migration-bundle
publish delegation, seed-bundle pull/extraction, digest helpers, layer
descriptor construction, output directory creation, and payload decoding.

The current public surface is crate-root re-exported from `underlay-devtools`:

- `SeedBundleBuildOptions`
- `SeedBundleBuildReport`
- `SeedBundlePullOptions`
- `SeedBundlePullReport`
- `seed_bundle_build()`
- `seed_bundle_publish()`
- `seed_bundle_pull()`

## Boundary Evidence

Seed bundles reuse migration-bundle internals:

- `BundlePublishOptions`
- `BundlePublishReport`
- `BundlePullOptions`
- `MigrationBundleError`
- `migration_bundle_pull()`
- `migration_bundle_publish()`
- `migration_bundle::local_store`

The CLI depends on the crate-root exports in
`src/bin/underlay-devtools/seed.rs`. Contract inventory
`docs/contracts/122-rust-public-api-inventory.md` treats seed bundles as
tooling-only public API and records that public option structs still accept raw
CLI values at the edge.

## Behavior Evidence

Baseline validation:

- `cargo test -p underlay-devtools --all-features`
- 23 unit tests passed
- 1 Docker/local-registry test ignored
- 0 doc-tests

There are no seed-bundle-specific unit tests in the current crate. The split
must therefore preserve behavior conservatively:

- build rejects missing source directories
- build requires `manifest.json`
- build collects `.sql` files in filename order
- SQL layers use `OciLayerKind::DataChunk`
- SQL layers retain `underlay.seed.file_name` and
  `underlay.seed.apply_order`
- bundle packages retain the shared `BundlePackage` envelope shape
- publish delegates to migration-bundle publish
- pull delegates to migration-bundle pull, then reconstructs `manifest.json`
  and SQL files from local-store payloads
- payload decode errors remain validation errors

## Decision

Queue `g06.110` as a devtools seed-bundle internal split.

Suggested module shape:

- `seed_bundle.rs`: small module front door and public re-exports
- `seed_bundle/model.rs`: public option/report types and private
  `SeedManifest`
- `seed_bundle/package.rs`: private `BundlePackage`, digest helpers, layer
  descriptor helpers, parent-dir creation, and payload decoding
- `seed_bundle/build.rs`: `seed_bundle_build()`
- `seed_bundle/pull.rs`: `seed_bundle_pull()` and manifest/SQL extraction
- `seed_bundle/publish.rs`: `seed_bundle_publish()` delegation

## Public API Impact

Expected impact: none.

This should be an internal split. If preserving crate-root exports, package
shape, or seed-bundle build/publish/pull behavior forces a public API change,
stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-devtools --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
