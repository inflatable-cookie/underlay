# g06.118 Artifact - Blob Local Adapter Internal Split

## Summary

`underlay-blob/src/adapters/local.rs` is now a small module front door with
stable public re-exports. The previous mixed local adapter implementation was
split into focused internal modules.

New module shape:

- `adapters/local.rs`: front door, public re-exports, and test module
- `adapters/local/config.rs`: `LocalConfig`
- `adapters/local/adapter.rs`: `LocalAdapter`, construction, debug, helper
  methods, and `BlobAdapter` implementation
- `adapters/local/path.rs`: local object-key validation, path containment, and
  empty-parent cleanup
- `adapters/local/mime.rs`: content-type guessing

## Public API Impact

None expected.

The feature-gated `underlay_blob::{LocalAdapter, LocalConfig}` exports,
object-key validation behavior, path-safety behavior, development-only
read/write helpers, cleanup behavior, upload plan fields, URL formatting,
metadata behavior, health-check behavior, and content-type guessing were
preserved.

## Validation

- `cargo test -p underlay-blob --all-features`
- `effigy rust:check`

`cargo test -p underlay-blob --all-features` passed with 20 unit tests passed,
3 doc-tests passed, and 1 doc-test ignored.

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 29 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 10 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

`underlay-blob/src/adapters/local.rs` no longer appears in the god-file report.
The next largest Rust warning is
`rust/crates/underlay-jobs/src/tests/runner_tests.rs`.

## Next Target Evidence

Queue `g06.119` as a jobs runner tests modularity audit before splitting
`underlay-jobs/src/tests/runner_tests.rs`. This is shared job execution
coverage, so the next batch should classify runner setup, retry behavior,
timeout/cancellation behavior, repository assertions, and fixture helpers
before moving test code.
