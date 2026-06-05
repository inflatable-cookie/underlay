# g06.105 Artifact - Migration-Core Pipeline Basic Tests Modularity Audit

## Summary

`underlay-migration-core/src/tests/pipeline_basic_tests.rs` is the largest
remaining Rust warning-level file after `g06.104`. It is a test file covering
the core migration pipeline happy path, transform failure, resume behavior,
resume compatibility rejection, and verify-stage semantic failure.

The current test surface groups:

- `stage_order_is_stable()`
- `reuse_policy_has_strict_default_choice_available()`
- `run_executes_all_stages_and_returns_report()`
- `run_maps_transform_failure_to_stage_error()`
- `run_resumes_from_completed_normalize_stage()`
- `run_rejects_incompatible_resume_checkpoint()`
- `run_fails_verify_on_plugin_semantic_issue()`

## Boundary Evidence

The file depends on shared test support from `src/tests/support.rs`:

- `InMemoryRunStore`
- `MockAssetResolver`
- `MockDecisionResolver`
- `MockPlugin`
- `MockSource`

The existing module root is `src/tests/lib_tests.rs`, which declares
`mod pipeline_basic_tests;`. Rust module resolution can preserve that public
test path by replacing `pipeline_basic_tests.rs` with
`pipeline_basic_tests/mod.rs` and focused child modules.

## Behavior Evidence

The baseline validation covers all migration-core tests:

- `cargo test -p underlay-migration-core --all-features`
- 43 unit tests passed
- 0 doc-tests

The split should preserve test names or keep them close enough that failure
output remains recognizable. It should not change migration pipeline behavior,
stage ordering, checkpoint compatibility rules, decision resolution behavior,
verification failure mapping, or shared support mocks.

## Decision

Queue `g06.106` as a migration-core pipeline basic tests internal split.

Suggested module shape:

- `pipeline_basic_tests/mod.rs`: shared imports, small constructors/helpers,
  stage-order invariant, and reuse-policy invariant
- `pipeline_basic_tests/full_run.rs`: full successful run assertions
- `pipeline_basic_tests/failures.rs`: transform and verify failure mapping
- `pipeline_basic_tests/resume.rs`: successful resume and incompatible
  checkpoint rejection

The split should favor local helper constructors only where they reduce repeated
`Arc<Mutex<u64>>` setup. It should avoid hiding the pipeline assertions behind
large generic fixtures.

## Public API Impact

Expected impact: none.

This is a test-only split. If preserving coverage requires changing
migration-core APIs or pipeline semantics, stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
