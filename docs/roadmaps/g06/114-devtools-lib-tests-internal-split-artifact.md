# g06.114 Artifact - Devtools Lib Tests Internal Split

## Summary

`underlay-devtools/src/tests/lib_tests.rs` was replaced by a focused
`tests/lib_tests/` module directory. The crate test module path in `src/lib.rs`
now points at `tests/lib_tests/mod.rs`.

New module shape:

- `tests/lib_tests/mod.rs`: shared imports and module declarations
- `tests/lib_tests/support.rs`: temp-dir fixture helper
- `tests/lib_tests/env.rs`: environment helper tests
- `tests/lib_tests/decision_reports.rs`: decision invalidation and governance
  issue formatting tests
- `tests/lib_tests/pipeline_reports.rs`: decide/pipeline loading, drift,
  recovery, verification, integrity, and audit artifact tests
- `tests/lib_tests/policy_reports.rs`: governance policy loading and summary
  formatting tests

## Public API Impact

None.

This was a test-only split. Devtools APIs, report formatting, report loading,
verification/audit artifact writing, and policy summary behavior were not
changed.

## Validation

- `cargo test -p underlay-devtools --all-features`
- `effigy rust:check`

`cargo test -p underlay-devtools --all-features` passed with 23 unit tests
passed, 1 Docker/local-registry test ignored, and 0 doc-tests.

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 31 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

`underlay-devtools/src/tests/lib_tests.rs` no longer appears in the god-file
report. The next largest Rust warning is
`rust/crates/underlay-migration-core/src/tests/pipeline_decision_tests.rs`.

## Next Target Evidence

Queue `g06.115` as a migration-core pipeline decision tests modularity audit
before splitting `pipeline_decision_tests.rs`. This is shared migration decision
behavior coverage, so the next batch should classify cached-decision reuse,
human override precedence, invalidation, unresolved queue behavior, governance
issue reporting, and helper fixture setup before moving test code.
