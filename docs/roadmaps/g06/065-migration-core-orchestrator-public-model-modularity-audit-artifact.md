# g06.065 Artifact - Migration-Core Orchestrator Public Model Modularity Audit

## Summary

`underlay-migration-core/src/pipeline/orchestrator.rs` is stable
library-facing pipeline orchestration surface with a safe internal split shape
if `MigrationOrchestrator` and crate-root exports stay intact.

The file currently groups:

- public orchestrator type: `MigrationOrchestrator<S, P, D, A>`
- public constructor: `MigrationOrchestrator::new`
- public stage list: `MigrationOrchestrator::stage_order`
- public run entry point: `MigrationOrchestrator::run`
- stage execution flow: extract, normalize, transform, decide, materialize,
  assets, verify
- resume and checkpoint coordination
- decision fingerprinting, reuse, invalidation, journaling, unresolved queue,
  and governance issue collection
- integrity gate enforcement before materialization
- verification input construction and verification failure handling
- final `PipelineRunReport` assembly

## Consumer Evidence

Public usage is crate-root oriented:

- `underlay_migration_core::MigrationOrchestrator` is re-exported from
  `src/lib.rs`.
- `underlay_migration_core::pipeline::MigrationOrchestrator` remains available
  through `src/pipeline.rs`.
- No consumer scan hit direct `underlay_migration_core::pipeline::...`
  imports.
- Farmyard imports `MigrationOrchestrator` from the crate root and constructs
  it with `MigrationOrchestrator::new(source, plugin, resolver, assets)`.
- Underlay migration-core tests exercise `stage_order`, full successful runs,
  transform failure mapping, resume compatibility, decision reuse, low
  confidence unresolved queues, decision invalidation, governance issues, and
  integrity gate failures.

## Decision

Queue `g06.066` as a migration-core orchestrator internal split.

The split should preserve:

- crate-root `MigrationOrchestrator` export
- `underlay_migration_core::pipeline::MigrationOrchestrator`
- public fields on `MigrationOrchestrator<S, P, D, A>`
- `new`, `stage_order`, and `run`
- stage order
- stage snapshot and checkpoint persistence behavior
- decision journal and unresolved queue record behavior
- integrity gate timing before materialization
- verification input and failure behavior
- `PipelineRunReport` fields and serialized stage/report model shapes

## Public API Impact

Expected impact: none.

This should be a private module/function split only. If the split requires
changing public method signatures, trait bounds, stage output shapes, persisted
snapshot shapes, or root exports, stop and re-enter planning.

## Validation

- `effigy test --plan`
- `cargo test -p underlay-migration-core --all-features`

Next code batch validation:

- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`
- Farmyard `cargo check --workspace` only if public root imports move
- `effigy qa:docs`
- `effigy qa:northstar`
