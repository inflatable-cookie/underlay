# g06.070 Artifact - Migration-Core Verification-Rules Internal Split

## Summary

`underlay-migration-core/src/verification_rules.rs` is now a small public
front door. The crate-root API and serialized rule/result shapes are unchanged.

New private modules:

- `verification_rules/model.rs`: rule, metric, expectation, and result types
- `verification_rules/evaluate.rs`: evaluator coordinator
- `verification_rules/row_count.rs`: row-count metric evaluation
- `verification_rules/field_rules.rs`: not-null, unique, and referential
  integrity evaluation
- `verification_rules/value_path.rs`: JSON path, value signature, index
  preview, and rule-name sanitization helpers
- `verification_rules/standard_verification_rules.rs`: standard rule
  constructors
- `verification_rules/benchmark.rs`: declarative vs plugin benchmark helper

## Preserved Behavior

The split preserved:

- crate-root `underlay_migration_core::*` exports
- rule/result type names and public fields
- serde `snake_case` rule/result shapes
- `VerificationRuleKind` variants and field names
- `VerificationMetric` variants
- `CountExpectation` variants
- `evaluate_verification_rules` behavior and readable failure messages
- `benchmark_verification_paths` signature and plugin verification behavior
- `standard_verification_rules::{unique, not_null, row_count_exact,
  row_count_min, referential_integrity}`
- `PipelinePolicy.verification_rules` and `VerificationInput.rules`

## Structural Result

`verification_rules.rs` moved from a high-error god-file into a front door:

- `verification_rules.rs`: 14 lines
- `verification_rules/benchmark.rs`: 43 lines
- `verification_rules/evaluate.rs`: 59 lines
- `verification_rules/field_rules.rs`: 141 lines
- `verification_rules/model.rs`: 77 lines
- `verification_rules/row_count.rs`: 68 lines
- `verification_rules/standard_verification_rules.rs`: 95 lines
- `verification_rules/value_path.rs`: 26 lines

`effigy doctor` now reports:

- `scan.god-files`: 53 findings, 12 errors, 41 warnings
- `scan.attention-markers`: 11 findings, 2 errors, 9 warnings
- `scan.comment-ratio`: 12 findings, 3 errors, 9 warnings

The doctor failure remains the known structural backlog.

## Public API Impact

Impact: none.

This was a private module split. No consumer app update is required.

## Validation

- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` expected failure on known structural scans
