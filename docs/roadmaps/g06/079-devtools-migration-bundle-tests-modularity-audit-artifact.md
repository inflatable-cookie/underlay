# g06.079 Artifact - Devtools Migration-Bundle Tests Modularity Audit

## Summary

`underlay-devtools/src/tests/migration_bundle_tests.rs` is the largest
remaining Rust high-error god-file after the auth email TOTP split. It is
test-only, but it covers migration-bundle build, publish, pull, run,
digest-ref parsing, media shard safety, and remote registry behavior in one
file.

The file currently groups:

- shared temporary-directory helper
- bundle build report/layout assertions
- local-store publish/pull round-trip tests
- digest-mismatch and digest-pinned run guard tests
- typed `MigrationBundleRef` parser tests
- typed `BundleRunOptions` constructor test
- pulled media shard filename sanitization test
- local-store digest-pinned replay test
- deterministic media shard build/mapping test
- ignored Docker registry round-trip test
- Docker registry guard, port picker, and wait helper

## Behavior Evidence

The test file covers these stable contracts:

- bundle build writes layout JSON with real sha256 digests
- publish requires an existing bundle file
- local-store publish/pull preserves digest and writes pulled output
- publish rejects digest-pinned refs when the digest mismatches the bundle
- migration run requires a digest-pinned bundle ref
- `MigrationBundleRef` accepts valid sha256 digest refs and rejects tag-only
  or malformed digest refs
- `BundleRunOptions::from_bundle_ref` preserves the typed ref and output/store
  paths
- pulled media shard output names sanitize hostile shard ids
- migration run can replay a digest-pinned bundle from the local store
- media assets split into deterministic shards with mapping metadata
- the ignored remote registry round trip exercises Docker-backed publish/pull
  when explicitly enabled

## Decision

Queue `g06.080` as a devtools migration-bundle tests internal split.

The split should preserve:

- all test names or comparably searchable behavior names
- temp-dir and Docker registry helper behavior
- local-store build/publish/pull/run coverage
- digest-ref parser and run-option coverage
- media shard safety and deterministic build coverage
- ignored remote registry coverage
- existing production code and public APIs

Suggested test module shape:

- `migration_bundle_tests.rs`: test module front door
- `migration_bundle_tests/support.rs`: `temp_dir`, Docker registry guard,
  port picker, and registry wait helper
- `migration_bundle_tests/local_store.rs`
- `migration_bundle_tests/refs.rs`
- `migration_bundle_tests/media_shards.rs`
- `migration_bundle_tests/remote_registry.rs`

## Public API Impact

Expected impact: none.

This should be a test-only split. If production devtools APIs or bundle
semantics must change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-devtools --all-features migration_bundle`

Next code batch validation:

- `cargo test -p underlay-devtools --all-features migration_bundle`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
