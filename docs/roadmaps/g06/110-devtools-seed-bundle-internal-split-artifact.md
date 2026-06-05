# g06.110 Artifact - Devtools Seed Bundle Internal Split

## Summary

`underlay-devtools/src/seed_bundle.rs` was replaced by a focused
`seed_bundle/` module directory while preserving the crate-root seed-bundle
exports.

New module shape:

- `seed_bundle/mod.rs`: front door and public re-exports
- `seed_bundle/model.rs`: public option/report types and private
  `SeedManifest`
- `seed_bundle/package.rs`: private `BundlePackage`, digest helpers, layer
  descriptor helpers, parent-dir creation, and payload encoding/decoding
- `seed_bundle/build.rs`: `seed_bundle_build()`
- `seed_bundle/publish.rs`: `seed_bundle_publish()` delegation
- `seed_bundle/pull.rs`: `seed_bundle_pull()` and manifest/SQL extraction

## Public API Impact

None expected.

The crate-root seed-bundle exports, package JSON shape, SQL file ordering,
layer annotations, publish delegation, pull local-store reuse, manifest
reconstruction, and payload validation behavior were preserved.

## Validation

- `cargo test -p underlay-devtools --all-features`
- `effigy rust:check`

`cargo test -p underlay-devtools --all-features` passed with 23 unit tests
passed, 1 Docker/local-registry test ignored, and 0 doc-tests.

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 33 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

`underlay-devtools/src/seed_bundle.rs` no longer appears in the god-file report.
The next largest Rust warning is
`rust/crates/underlay-devtools/src/migration_bundle/remote_registry.rs`.

## Next Target Evidence

Queue `g06.111` as a devtools migration-bundle remote registry modularity
audit before splitting `migration_bundle/remote_registry.rs`. Remote registry
behavior is shared tooling infrastructure, so the next batch should classify
reference parsing, HTTP request/response behavior, OCI manifest/blob handling,
auth/header behavior, Docker-registry test coverage, and error mapping before
moving code.
