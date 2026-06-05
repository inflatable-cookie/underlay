# g06.113 Artifact - Devtools Lib Tests Modularity Audit

## Summary

`underlay-devtools/src/tests/lib_tests.rs` is the largest remaining Rust
warning-level test file after `g06.112`. It combines environment helper tests,
decision invalidation report formatting, governance issue report formatting,
pipeline report loading and derived artifact generation, governance policy
loading/formatting, and a temp-dir fixture helper.

The current test surface groups:

- `require_env_returns_value_when_present()`
- `require_env_returns_missing_error_when_absent()`
- `format_decision_invalidation_report_summarizes_by_reason()`
- `governance_report_formats_and_lists_top_issues()`
- `load_decide_stage_output_supports_decide_and_pipeline_shapes()`
- `governance_policy_report_loads_and_formats_summary()`
- `temp_dir()`

## Boundary Evidence

The parent module path is declared in `src/lib.rs`:

- `#[path = "tests/lib_tests.rs"] mod tests;`

The split can preserve that path by replacing `lib_tests.rs` with a
`lib_tests/` directory and updating `src/lib.rs` to
`#[path = "tests/lib_tests/mod.rs"] mod tests;`.

The tests cover crate-root behavior and imports from:

- env helpers and `DevtoolError`
- migration report builders, formatters, loaders, and artifact writers
- `underlay_migration_core` report and policy models

## Behavior Evidence

Baseline validation:

- `cargo test -p underlay-devtools --all-features`
- 23 unit tests passed
- 1 Docker/local-registry test ignored
- 0 doc-tests

The split should preserve test coverage and keep assertions readable. It should
not change devtools public APIs, report formatting, report loading,
verification/audit artifact writing, policy summary formatting, or migration
core model construction.

## Decision

Queue `g06.114` as a devtools lib tests internal split.

Suggested module shape:

- `tests/lib_tests/mod.rs`: shared imports and module declarations
- `tests/lib_tests/support.rs`: temp-dir fixture helper
- `tests/lib_tests/env.rs`: `require_env()` tests
- `tests/lib_tests/decision_reports.rs`: decision invalidation and governance
  issue formatting tests
- `tests/lib_tests/pipeline_reports.rs`: decide/pipeline loading, drift,
  recovery, verification, integrity, and audit artifact tests
- `tests/lib_tests/policy_reports.rs`: governance policy loading and summary
  formatting test

## Public API Impact

Expected impact: none.

This is a test-only split. If preserving coverage requires changing devtools
APIs or report behavior, stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-devtools --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
