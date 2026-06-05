# g06.051 Artifact - Migration-Core Pipeline Internal Split

## Summary

`underlay-migration-core/src/pipeline.rs` is now a small module front door.

The public crate-root exports and serialized stage/report model shapes are
unchanged. The split moved private pipeline support code into focused internal
modules.

## Code Changes

- Kept `pipeline/types.rs` as the public stage/report model.
- Moved `MigrationOrchestrator` into `pipeline/orchestrator.rs`.
- Moved resume compatibility and resume-stage checks into
  `pipeline/resume.rs`.
- Moved stage snapshot/checkpoint persistence and load helpers into
  `pipeline/checkpoints.rs`.
- Moved provenance, low-confidence, and unresolved decision helpers into
  `pipeline/decision_support.rs`.
- Moved stage failure classification into `pipeline/errors.rs`.
- Kept `pipeline.rs` as the module front door:
  - internal module declarations
  - `pub use orchestrator::MigrationOrchestrator`
  - `pub use types::*`

## Size Result

Pipeline module file sizes after the split:

- `pipeline/orchestrator.rs`: 487 lines
- `pipeline/types.rs`: 182 lines
- `pipeline/checkpoints.rs`: 77 lines
- `pipeline/decision_support.rs`: 74 lines
- `pipeline/resume.rs`: 65 lines
- `pipeline/errors.rs`: 47 lines
- `pipeline.rs`: 9 lines

The orchestrator is still substantial, but it now owns only the stage flow. The
private helper families are no longer buried at the bottom of one file.

## Public API Impact

Expected impact: none.

Root exports remain stable through `underlay_migration_core`:

- `MigrationOrchestrator`
- stage output/report types
- `StageName`
- `ResumeDiagnostics`

Persisted JSON shapes are unchanged because `pipeline/types.rs` was not
rewritten.

## Consumer Proof

Farmyard is the active consumer of this surface. It imports
`MigrationOrchestrator`, `MigrationPlugin`, `PipelineRunReport`, `RunMetadata`,
`RunStore`, and related model types from the crate root.

Validation showed Farmyard remains source-compatible.

## Validation

- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`
- Farmyard `cargo check --workspace`
- `effigy qa:docs`
- `effigy qa:northstar`

Farmyard still reports its pre-existing dead-code warning in
`farmyard-migration`.
