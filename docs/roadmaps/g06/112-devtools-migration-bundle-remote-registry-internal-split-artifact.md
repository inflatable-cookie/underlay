# g06.112 Artifact - Devtools Migration-Bundle Remote Registry Internal Split

## Summary

`underlay-devtools/src/migration_bundle/remote_registry.rs` is now a small
module front door with crate-private re-exports. The previous mixed remote
registry implementation was split into focused internal modules.

New module shape:

- `remote_registry.rs`: front door, crate-private re-exports, and shared media
  type constants
- `remote_registry/reference.rs`: remote-reference detection and parsing
- `remote_registry/client.rs`: registry client construction, ping, and blob
  upload helpers
- `remote_registry/publish.rs`: remote publish, config/package blob upload,
  and OCI manifest publish
- `remote_registry/pull.rs`: remote pull, manifest fetch, package-layer fetch,
  digest verification, package validation, and pulled-output writing

## Public API Impact

None expected.

The crate-private `is_remote_ref()`, `remote_publish()`, and `remote_pull()`
entry points used by `migration_bundle/run.rs` were preserved. Remote reference
semantics, registry ping behavior, blob upload flow, OCI manifest shape,
digest verification, output writing, and status strings were preserved.

## Validation

- `cargo test -p underlay-devtools --all-features`
- `effigy rust:check`

`cargo test -p underlay-devtools --all-features` passed with 23 unit tests
passed, 1 Docker/local-registry test ignored, and 0 doc-tests.

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 32 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

`migration_bundle/remote_registry.rs` no longer appears in the god-file report.
The next largest Rust warning is
`rust/crates/underlay-devtools/src/tests/lib_tests.rs`.

## Next Target Evidence

Queue `g06.113` as a devtools lib tests modularity audit before splitting
`underlay-devtools/src/tests/lib_tests.rs`. This is shared tooling test
coverage, so the next batch should classify environment helpers, governance
report formatting, decision invalidation formatting, policy loading, and
pipeline report loading before moving test code.
