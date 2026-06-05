# g06.080 Artifact - Devtools Migration-Bundle Tests Internal Split

## Summary

`underlay-devtools/src/tests/migration_bundle_tests.rs` is now a small test
front door. The previous test monolith was split into focused modules under
`underlay-devtools/src/tests/migration_bundle_tests/`.

The split is test-only. It does not change devtools production code,
migration-bundle semantics, generated bundle formats, public APIs, or consumer
apps.

## Module Shape

- `migration_bundle_tests.rs`: explicit test module front door
- `migration_bundle_tests/support.rs`: temporary directory helper, Docker
  registry guard, free-port picker, and registry wait helper
- `migration_bundle_tests/local_store.rs`: local build, publish, pull, and run
  tests
- `migration_bundle_tests/refs.rs`: digest-pinned ref parsing and typed run
  option tests
- `migration_bundle_tests/media_shards.rs`: media shard output sanitization and
  deterministic shard/mapping tests
- `migration_bundle_tests/remote_registry.rs`: ignored Docker-backed remote
  registry round-trip test

## Behavior Preserved

- focused migration-bundle tests pass with 12 passed and 1 Docker integration
  test ignored
- bundle build layout and digest assertions remain covered
- local-store publish/pull/run behavior remains covered
- digest mismatch and digest-pinned run guards remain covered
- typed bundle-ref parsing behavior remains covered
- media shard sanitization and deterministic mapping behavior remain covered
- ignored remote registry behavior remains available for explicit Docker runs

## Public API Impact

None.

This was a Rust test-structure split only.

## Validation

- `cargo test -p underlay-devtools --all-features migration_bundle`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` remains expected-fail on the known backlog; `scan.god-files`
  improved from 49 findings / 8 errors to 48 findings / 7 errors.
