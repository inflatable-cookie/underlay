# g06.116 Artifact - Migration-Core Pipeline Decision Tests Internal Split

## Summary

`underlay-migration-core/src/tests/pipeline_decision_tests.rs` was replaced by
a focused `pipeline_decision_tests/` module directory while preserving the
parent `mod pipeline_decision_tests;` path.

New module shape:

- `pipeline_decision_tests/mod.rs`: shared imports, fixture helpers, decision
  fingerprint helper, and seeded journal helper
- `pipeline_decision_tests/reuse.rs`: cached decision reuse and human override
  precedence tests
- `pipeline_decision_tests/invalidation.rs`: plugin dependency invalidation
  tests
- `pipeline_decision_tests/governance.rs`: invalid cached-record governance
  failure tests
- `pipeline_decision_tests/unresolved.rs`: low-confidence unresolved queue
  tests

## Public API Impact

None.

This was a test-only split. Migration-core APIs, decision reuse behavior,
invalidation behavior, governance failure behavior, unresolved queue behavior,
and pipeline execution semantics were not changed.

## Validation

- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`

`cargo test -p underlay-migration-core --all-features` passed with 43 unit
tests passed and 0 doc-tests.

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 30 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

`pipeline_decision_tests.rs` no longer appears in the god-file report. The next
largest Rust production warning is `rust/crates/underlay-blob/src/adapters/local.rs`.

## Next Target Evidence

Queue `g06.117` as a blob local adapter modularity audit before splitting
`underlay-blob/src/adapters/local.rs`. The local adapter is shared storage
infrastructure, so the next batch should classify path resolution, object-key
safety, stream/file I/O, metadata handling, delete/list behavior, and tests
before moving production code.
