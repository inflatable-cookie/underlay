# g06.137 Artifact - Migration-Core Orchestrator Decide Modularity Audit

## Summary

`underlay-migration-core/src/pipeline/orchestrator/decide.rs` is the next Rust
production warning-level file after `g06.136`. It owns the decide stage loop,
fingerprint input construction, prior decision validation/reuse, resolver
execution, decision journal writes, unresolved queue writes, invalidation
tracking, governance issue collection, and report assembly.

The current file groups:

- resume checkpoint short-circuit for the decide stage
- per-record decision type and confidence threshold selection
- `DecisionFingerprintInput` construction
- resolver fingerprint calculation
- plugin invalidation check
- prior decision journal chain loading and validation
- effective prior decision selection and reuse evaluation
- cached decision provenance parsing
- low-confidence unresolved queue handling for reused decisions
- invalidation event recording
- fresh resolver execution
- resolver fingerprint mismatch protection
- decision journal record construction and validation
- decision journal persistence
- low-confidence unresolved queue handling for fresh decisions
- decide-stage output count and report construction
- stage output persistence

## Boundary Evidence

The decide stage is crate-internal:

- `orchestrator.rs` declares `mod decide`
- `decide_stage(...)` is `pub(super)` on `MigrationOrchestrator`
- the public API remains `MigrationOrchestrator::run(...)`

Current in-repo coverage lives under `pipeline_decision_tests` and exercises
the public run path rather than direct decide helpers.

Behavior boundaries to preserve:

- resume loads a persisted decide output before doing new work
- fingerprint input includes canonical record, decision type, resolver version,
  prompt version, and target schema version
- plugin invalidation participates in reuse evaluation
- invalid prior journal records become governance issues and are excluded from
  reuse
- effective prior decisions can be reused
- low-confidence AI decisions are queued unresolved and have `Value::Null`
  outcomes in decide output
- invalidation events are recorded when reuse is rejected for a reason
- resolver fingerprint mismatch fails the decide stage
- valid fresh decisions are appended to the decision journal
- unresolved records are appended and then validated for governance evidence
- output counts remain based on the final decision vector

## Behavior Evidence

Existing focused tests cover:

- cached decision reuse skips new resolver journal entries
- human override takes precedence in the prior decision provenance chain
- invalid cached records surface governance issues
- plugin dependency invalidation records an invalidation reason
- low-confidence AI decisions enter the unresolved queue

Baseline validation:

- `cargo test -p underlay-migration-core pipeline_decision --all-features`
- 5 focused decision-pipeline tests passed

## Decision

Queue `g06.138` as a migration-core orchestrator decide internal split.

Suggested module shape:

- `orchestrator/decide/mod.rs`: `decide_stage(...)`, stage loop, final output
  assembly, and public-to-super boundary
- `orchestrator/decide/input.rs`: decision type, threshold, and
  `DecisionFingerprintInput` construction
- `orchestrator/decide/prior.rs`: prior journal loading, validation,
  effective prior selection, reuse evaluation, and invalidation event handling
- `orchestrator/decide/write.rs`: decision journal record construction,
  unresolved record construction, validation, and persistence helpers

This keeps `MigrationOrchestrator::run(...)` and the crate public surface
stable while separating decision safety concerns from report assembly.

## Public API Impact

Expected impact: none.

If preserving the split requires changing decision fingerprints, reuse
semantics, invalidation behavior, unresolved queue behavior, journal writes,
governance issue codes, output counts, or public imports, stop and re-enter
planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-migration-core pipeline_decision --all-features`
- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
