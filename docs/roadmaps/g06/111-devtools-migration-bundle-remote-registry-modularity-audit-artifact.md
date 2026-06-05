# g06.111 Artifact - Devtools Migration-Bundle Remote Registry Modularity Audit

## Summary

`underlay-devtools/src/migration_bundle/remote_registry.rs` is the largest
remaining Rust warning-level production file after `g06.110`. It combines
remote-reference detection and parsing, remote publish, remote pull, registry
ping, blob upload, OCI manifest construction, OCI manifest fetch, package-layer
fetch, digest verification, and error mapping in one file.

The crate-private surface used by `migration_bundle/run.rs` is:

- `is_remote_ref()`
- `remote_publish()`
- `remote_pull()`

## Boundary Evidence

The remote registry code depends on migration-bundle private helpers:

- `decode_package()`
- `sha256_digest()`
- `validate_bundle_package()`
- `write_pulled_outputs()`
- `local_store::validate_sha256_digest()`
- `BundlePublishOptions`
- `BundlePublishReport`
- `BundlePullOptions`
- `BundlePullReport`
- `MigrationBundleError`

Existing validation includes one Docker-backed ignored test:

- `migration_bundle_remote_registry_round_trip`

That test covers build, remote publish to a local Docker registry, digest-based
remote pull, and pulled output creation when explicitly enabled.

## Behavior Evidence

Baseline validation:

- `cargo test -p underlay-devtools --all-features`
- 23 unit tests passed
- 1 Docker/local-registry test ignored
- 0 doc-tests

The split should preserve behavior conservatively:

- remote refs are only `http://` or `https://`
- remote publish requires a tag reference, not digest
- digest refs must pass SHA-256 validation
- tag refs must be `<repo>:<tag>` after the registry URL path
- registry ping checks `/v2/`
- blob upload uses the registry upload start and finish flow
- relative upload `Location` headers are resolved against the registry
- publish writes an OCI manifest with config and one package layer
- pull accepts OCI/Docker manifest media types
- pull reads the first manifest layer as the package layer
- pulled blob digest must match the layer digest
- publish/pull status strings remain `published-remote` and `pulled-remote`

## Decision

Queue `g06.112` as a migration-bundle remote registry internal split.

Suggested module shape:

- `remote_registry.rs`: small module front door and crate-private re-exports
- `remote_registry/reference.rs`: `RemoteRegistryRef`, `is_remote_ref()`, and
  remote reference parsing
- `remote_registry/client.rs`: registry ping and blob upload helpers
- `remote_registry/publish.rs`: `remote_publish()`, config/package blob upload,
  and manifest publish
- `remote_registry/pull.rs`: `remote_pull()`, manifest fetch, layer selection,
  blob fetch, digest verification, package validation, and output writing

## Public API Impact

Expected impact: none.

This is tooling-internal. If preserving crate-private entry points, remote
reference semantics, OCI manifest shape, or publish/pull behavior forces an API
or behavior change, stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-devtools --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
