# g06.119 Artifact - Jobs Runner Tests Modularity Audit

## Summary

`underlay-jobs/src/tests/runner_tests.rs` is the largest remaining Rust
warning-level file after `g06.118`. It validates shared job runner behavior:
registered handler dispatch, no-work behavior, unknown job filtering, failure
recording, handler config propagation, permanent failure handling, runner
config defaults, and batch limiting.

The current test surface groups:

- in-memory `JobStore` fixture and event sink
- `make_test_job()` and `TestHandler`
- `runner_dispatches_jobs_to_registered_handler()`
- `runner_returns_false_when_no_jobs_available()`
- `runner_ignores_unknown_job_types()`
- `runner_records_failures()`
- `runner_passes_handler_config_to_failure_path()`
- `runner_flags_permanent_failures_for_store()`
- `job_runner_config_default_values()`
- `run_batch_processes_limited_jobs()`

## Boundary Evidence

The parent test module is declared in `src/runner.rs`:

- `#[path = "tests/runner_tests.rs"] mod tests;`

The split can preserve the runner test module by updating that path to
`tests/runner_tests/mod.rs` and replacing the flat file with focused child
modules.

The file defines its own `MemStore`, `FailureCall`, `RecordingEventSink`, and
handlers, so no production helper extraction is needed.

## Behavior Evidence

Baseline validation:

- `cargo test -p underlay-jobs --all-features`
- 20 unit tests passed
- 2 doc-tests passed
- 3 doc-tests ignored

The split should preserve dispatch event ordering, success/failure/dead-letter
recording, no-work return behavior, unknown-type filtering, handler config
propagation, permanent failure flags, default runner config, and batch limit
behavior.

## Decision

Queue `g06.120` as a jobs runner tests internal split.

Suggested module shape:

- `tests/runner_tests/mod.rs`: shared imports, fixture store, event sink,
  `make_test_job()`, and `TestHandler`
- `tests/runner_tests/dispatch.rs`: dispatch, no-work, and unknown job tests
- `tests/runner_tests/failures.rs`: failure recording, handler config, and
  permanent failure tests
- `tests/runner_tests/batch.rs`: runner config defaults and batch limiting
  tests

## Public API Impact

Expected impact: none.

This is a test-only split. If preserving coverage requires changing jobs APIs
or runner behavior, stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-jobs --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
