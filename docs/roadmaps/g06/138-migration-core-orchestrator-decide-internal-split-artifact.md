# g06.138 Artifact - Migration-Core Orchestrator Decide Internal Split

## Summary

The migration-core decide stage now lives under a focused `decide/` module
directory instead of one large `decide.rs` file.

Changed files:

- `underlay-migration-core/src/pipeline/orchestrator/decide.rs` removed
- `underlay-migration-core/src/pipeline/orchestrator/decide/mod.rs`
- `underlay-migration-core/src/pipeline/orchestrator/decide/input.rs`
- `underlay-migration-core/src/pipeline/orchestrator/decide/prior.rs`
- `underlay-migration-core/src/pipeline/orchestrator/decide/write.rs`

## Module Shape

- `mod.rs`: decide-stage loop, resume handling, resolver execution,
  fingerprint mismatch guard, final output assembly, and stage persistence
- `input.rs`: decision type, confidence threshold, and
  `DecisionFingerprintInput` construction
- `prior.rs`: prior journal loading, journal validation, reuse evaluation,
  cached decision reconstruction, and invalidation event construction
- `write.rs`: decision journal record construction/persistence and unresolved
  queue record construction/persistence

The public `MigrationOrchestrator::run(...)` path remains stable.

## Behavior Preserved

The split keeps existing decide behavior:

- resume loads persisted decide output before new work
- decision fingerprints include canonical record, decision type, resolver
  version, prompt version, and target schema version
- plugin invalidation participates in reuse evaluation
- invalid cached journal records become governance issues and are excluded from
  reuse
- reusable prior decisions skip resolver calls
- low-confidence AI decisions are appended to the unresolved queue and get
  `Value::Null` outcomes
- invalidation events are counted when reuse is rejected with a reason
- resolver fingerprint mismatch fails the decide stage
- valid fresh decisions are appended to the decision journal
- output counts remain derived from the final decision vector and counters

## Validation

Passed:

- `cargo test -p underlay-migration-core pipeline_decision --all-features`
  - 5 focused decision-pipeline tests passed
- `cargo test -p underlay-migration-core --all-features`
  - 43 unit tests passed
  - 0 doc-tests
- `effigy rust:check`

Known backlog:

- `effigy doctor` still fails on the existing structural scan backlog:
  attention markers, comment ratio, and god-files.
- God-file findings dropped from 20 to 19 after this split.
- No Rust production files remain in the current god-file report.
- The next Rust warning-level target is
  `underlay-validation/tests/derive_tests.rs`.

## Public API Impact

None.

This was an internal module split. No public migration API, decision
fingerprint content, reuse behavior, invalidation behavior, unresolved queue
behavior, journal write behavior, governance issue code, output count, or
consumer import path changed.
