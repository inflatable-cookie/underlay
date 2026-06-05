# g06.050 Artifact - Migration-Core Public Model Modularity Audit

## Summary

`underlay-migration-core` should stay a broad model crate for now.

The public API is intentionally library-facing: apps implement traits, pass root
model types around, and consume root re-exports. The next improvement should be
an internal file split, not a crate split or public API diet.

## Evidence

- `underlay-migration-core/src` is 6,113 Rust lines.
- Largest files:
  - `pipeline.rs`: 722 lines
  - `verification_rules.rs`: 492 lines
  - `decision_memory.rs`: 444 lines
  - `verification.rs`: 305 lines
  - `drift.rs`: 295 lines
- `lib.rs` re-exports the public model from the crate root.
- Effigy doctor still fails only on known structural scans:
  `scan.attention-markers`, `scan.comment-ratio`, and `scan.god-files`.
- Consumer scan found direct runtime use in Farmyard’s migration crate:
  `MigrationOrchestrator`, `MigrationPlugin`, `DecisionResolver`,
  `RunStore`, `PipelineRunReport`, and related root-level model types.

## Public Model Classification

Keep broad:

- `plugin.rs` owns the core app/plugin traits and stage batch/result shapes.
- `run_store.rs` owns checkpoint, snapshot, decision journal, unresolved queue,
  and run-summary storage contracts.
- `context.rs`, `manifest.rs`, `oci.rs`, `policy.rs`, `integrity.rs`,
  `audit.rs`, `drift.rs`, and `recovery.rs` are small or coherent enough to
  leave alone.
- `decision_memory.rs` is large but cohesive: fingerprinting, decision index,
  journal parsing, reuse evaluation, provenance chain, and validation all
  operate on the same model.
- `verification_rules.rs` is large but cohesive: declarative rule types,
  standard rule constructors, rule evaluation, and benchmarking are one public
  family.

Split next:

- `pipeline.rs` is the only high-value mechanical split target. It contains
  the public orchestrator plus private resume planning, stage persistence,
  snapshot loading, failure classification, provenance helpers, and unresolved
  decision construction.

## Decision

Queue `g06.051` as a mechanical internal split of `pipeline.rs`.

The split should preserve:

- root re-exports in `underlay_migration_core`
- `pub use crate::pipeline::{...}` compatibility
- consumer source imports, especially Farmyard
- persisted run-store snapshot shapes

The likely module shape:

- `pipeline/types.rs` stays as the public stage/report model
- `pipeline/orchestrator.rs` owns `MigrationOrchestrator`
- `pipeline/resume.rs` owns resume compatibility and `ResumePlan`
- `pipeline/checkpoints.rs` owns persist/load stage helpers
- `pipeline/decision_support.rs` owns provenance and unresolved-decision
  helpers
- `pipeline/errors.rs` owns stage failure classification

This is a code organization batch, not a public API redesign.

## Consumer Impact

Expected impact: none if root exports remain stable.

Stop and replan if the split requires moving or renaming public root exports,
trait methods, or persisted JSON shapes.

## Validation

- `effigy tasks`
- `effigy doctor` - expected failure on known structural scan backlog
- `cargo test -p underlay-migration-core --all-features`
- `effigy qa:docs`
- `effigy qa:northstar`

Next code batch validation:

- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`
- Farmyard `cargo check --workspace` if public exports move or root imports
  change
- `effigy qa:docs`
- `effigy qa:northstar`
