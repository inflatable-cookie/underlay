# g06.133 Artifact - Migration-Core Drift Modularity Audit

## Summary

`underlay-migration-core/src/drift.rs` is the next Rust production
warning-level file after `g06.132`. It owns public drift-report models, run
threshold checks, decision-lineage checks, and category summaries.

The current file groups:

- public drift severity, issue, report, category summary, threshold, and
  lineage input models
- default drift thresholds
- `detect_drift_from_run(...)`
- `detect_drift_with_lineage(...)`
- unresolved decision threshold check
- governance issue threshold check
- verify-stage pass requirement check
- decision index validation
- expected bundle digest check
- decision index entry bundle digest checks
- index-to-journal fingerprint and decision ID checks
- journal-to-index missing fingerprint warnings
- lineage mismatch threshold check
- category summary aggregation
- crate-local drift tests

## Boundary Evidence

The public surface is re-exported from `src/lib.rs`:

- `detect_drift_from_run`
- `detect_drift_with_lineage`
- `DecisionLineageInput`
- `DriftCategorySummary`
- `DriftDetectionReport`
- `DriftIssue`
- `DriftSeverity`
- `DriftThresholds`

Current in-repo callers are tests and root exports. The split must preserve
`crate::drift::{...}` and root `underlay_migration_core::{...}` imports.

## Behavior Evidence

Existing focused tests cover:

- unresolved decision, governance, and failed verify-stage drift issues
- blocking issue counts
- category summary counts
- decision index lineage mismatch detection
- lineage mismatch threshold issue

Baseline validation:

- `cargo test -p underlay-migration-core drift --all-features`
- 2 focused drift tests passed

## Decision

Queue `g06.134` as a migration-core drift internal split.

Suggested module shape:

- `drift/mod.rs`: public drift front door, re-exports, and test module
  declaration
- `drift/model.rs`: public drift and lineage model types
- `drift/run.rs`: run-report threshold checks and report assembly
- `drift/lineage.rs`: decision index/journal lineage checks
- `drift/summary.rs`: category summary aggregation

This keeps public names stable while separating run-state checks from lineage
validation.

## Public API Impact

Expected impact: none.

If preserving the split requires changing exported names, serialized field
names, issue codes, severity assignment, thresholds, or drift semantics, stop
and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-migration-core drift --all-features`
- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
