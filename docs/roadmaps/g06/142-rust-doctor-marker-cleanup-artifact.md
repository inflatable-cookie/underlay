# g06.142 Artifact - Rust Doctor Marker Cleanup

## Summary

Rust no longer contributes attention-marker, comment-ratio, or god-file
findings to `effigy doctor`.

Changed Rust files:

- `underlay-auth/src/types.rs`
- `underlay-db/src/tests/existence_tests.rs`
- `underlay-testing/src/fixtures.rs`
- `underlay-testing/src/test_db.rs`
- `underlay-testing/src/tests/test_db_tests.rs`
- `underlay-ratelimit/src/backend.rs`
- `underlay-email/src/lib.rs`
- `underlay-http/src/path.rs`
- `underlay-http/src/query.rs`
- `underlay-blob/src/config.rs`

## Cleanup Shape

Attention-marker cleanup:

- changed the auth event category comment from `// Security` to
  `// Threat signals`, removing a false-positive critical marker
- rewrote Rust test notes that were being classified as deferred work
- rewrote the fixture placeholder comment into a precise synthetic-hash note

Comment-ratio cleanup:

- removed oversized ignored examples from small Rust API files
- trimmed repeated crate-level prose where exported names already carry the
  contract
- preserved useful public API documentation and runnable doc-tests

## Validation

Passed:

- `cargo test -p underlay-blob --doc --all-features`
- `cargo test -p underlay-http --doc --all-features`
- `cargo test -p underlay-ratelimit --doc --all-features`
- `cargo test -p underlay-email --doc --all-features`
- `effigy rust:check`

Doctor status:

- `scan.attention-markers`: 10 findings to 5 findings
- `scan.comment-ratio`: 12 findings to 7 findings
- `scan.god-files`: 18 findings, unchanged
- remaining attention-marker findings are TypeScript-only
- remaining comment-ratio findings are TypeScript-only
- remaining god-file findings are TypeScript-only

## Public API Impact

None.

This was comment/docs cleanup only. No Rust API, behavior, validation rule,
storage key, migration behavior, auth behavior, rate-limit behavior, or
consumer import path changed.
