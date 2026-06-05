# g06.115 Artifact - Migration-Core Pipeline Decision Tests Modularity Audit

## Summary

`underlay-migration-core/src/tests/pipeline_decision_tests.rs` is the largest
remaining Rust warning-level file after `g06.114`. It validates migration
pipeline decision behavior: cached-decision reuse, plugin invalidation, human
override precedence, governance failures for invalid cached records, and
low-confidence unresolved queue behavior.

The current test surface groups:

- `run_reuses_cached_decisions_and_skips_new_journal_entries()`
- `run_records_invalidation_reason_when_plugin_dependency_changes()`
- `run_prefers_human_override_in_provenance_chain()`
- `run_surfaces_governance_issue_for_invalid_cached_record()`
- `run_queues_low_confidence_ai_decisions_as_unresolved()`

## Boundary Evidence

The file depends on shared test support from `src/tests/support.rs`:

- `decision_fingerprint_for()`
- `InMemoryRunStore`
- `MockAssetResolver`
- `MockDecisionResolver`
- `MockPlugin`
- `MockSource`

The parent test module already supports directory modules:

- `src/tests/lib_tests.rs` declares `mod pipeline_decision_tests;`
- `pipeline_basic_tests` has already moved to a directory module with focused
  child modules

The split can preserve the parent path by replacing `pipeline_decision_tests.rs`
with `pipeline_decision_tests/mod.rs` and child modules.

## Behavior Evidence

Baseline validation:

- `cargo test -p underlay-migration-core --all-features`
- 43 unit tests passed
- 0 doc-tests

The split should preserve decision reuse counts, resolver call expectations,
decision journal lengths, invalidation reason reporting, human-vs-AI
precedence, verify-stage governance failure behavior, unresolved queue counts,
threshold values, and store persistence assertions.

## Decision

Queue `g06.116` as a migration-core pipeline decision tests internal split.

Suggested module shape:

- `pipeline_decision_tests/mod.rs`: shared imports, small fixture helpers, and
  seeded decision helpers
- `pipeline_decision_tests/reuse.rs`: cached decision reuse and human override
  tests
- `pipeline_decision_tests/invalidation.rs`: plugin dependency invalidation
  tests
- `pipeline_decision_tests/governance.rs`: invalid cached record governance
  failure tests
- `pipeline_decision_tests/unresolved.rs`: low-confidence unresolved queue
  tests

## Public API Impact

Expected impact: none.

This is a test-only split. If preserving coverage requires changing
migration-core APIs or decision behavior, stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
