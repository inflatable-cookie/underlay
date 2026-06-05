# g06.069 Artifact - Migration-Core Verification-Rules Public Model Modularity Audit

## Summary

`underlay-migration-core/src/verification_rules.rs` is stable library-facing
verification-rule model plus a private evaluator/helper body.

The file currently groups:

- public rule model: `VerificationRule`, `VerificationRuleKind`,
  `VerificationMetric`, `CountExpectation`
- public result model: `RuleEngineResult`, `VerificationBenchmarkResult`
- public evaluator: `evaluate_verification_rules`
- public benchmark helper: `benchmark_verification_paths`
- public `standard_verification_rules` constructor module
- private row-count, not-null, unique, referential-integrity, metric, path,
  value-signature, index-preview, and rule-name sanitization helpers

## Usage Evidence

Public usage is crate-root oriented:

- `src/lib.rs` re-exports the rule model, result model, evaluator, benchmark
  helper, and `standard_verification_rules`.
- `context.rs` stores `PipelinePolicy.verification_rules:
  Vec<VerificationRule>`.
- `verification.rs` embeds `Vec<VerificationRule>` in `VerificationInput` and
  calls `evaluate_verification_rules` during `verify_stage`.
- `pipeline/orchestrator/stages.rs` builds `VerificationInput` from
  `ctx.policy.verification_rules`.
- Migration-core verification tests import the surface from the crate root.
- The current consumer scan found no named consumer importing
  `underlay_migration_core::verification_rules::...` directly. Consumer hits
  for similarly named types are app-local and unrelated.

## Decision

Queue `g06.070` as a migration-core verification-rules internal split.

The split should preserve:

- crate-root exports from `underlay_migration_core`
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

## Public API Impact

Expected impact: none.

This should be a private module/function split only. If the split requires
changing root exports, public fields, enum variants, serde shapes, evaluator
messages, benchmark behavior, or policy/input rule fields, stop and re-enter
planning.

## Validation

- `cargo test -p underlay-migration-core --all-features verification`

Next code batch validation:

- `cargo test -p underlay-migration-core --all-features verification`
- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
