# g06.120 Artifact - Jobs Runner Tests Internal Split

## Summary

`underlay-jobs/src/tests/runner_tests.rs` was replaced by a focused
`tests/runner_tests/` module directory. The runner test module path in
`src/runner.rs` now points at `tests/runner_tests/mod.rs`.

New module shape:

- `tests/runner_tests/mod.rs`: shared imports, in-memory store fixture,
  failure-call capture, event sink, `make_test_job()`, and `TestHandler`
- `tests/runner_tests/dispatch.rs`: dispatch, no-work, and unknown-type tests
- `tests/runner_tests/failures.rs`: failure recording, handler config, and
  permanent failure tests
- `tests/runner_tests/batch.rs`: runner config defaults and batch limiting
  tests

## Public API Impact

None.

This was a test-only split. Jobs APIs, runner dispatch behavior, failure
recording, dead-letter behavior, handler config propagation, runner defaults,
and batch limiting behavior were not changed.

## Validation

- `cargo test -p underlay-jobs --all-features`
- `effigy rust:check`

`cargo test -p underlay-jobs --all-features` passed with 20 unit tests passed,
2 doc-tests passed, and 3 doc-tests ignored.

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 28 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 10 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

`underlay-jobs/src/tests/runner_tests.rs` no longer appears in the god-file
report. The next largest Rust production warning is
`rust/crates/underlay-validation-derive/src/lib.rs`.

## Next Target Evidence

Queue `g06.121` as a validation derive crate modularity audit before splitting
`underlay-validation-derive/src/lib.rs`. Derive macros are public compile-time
API, so the next batch should classify parsing, attribute handling, generated
validation code, error reporting, and tests before moving production code.
