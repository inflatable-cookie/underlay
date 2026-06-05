# g06.066 Artifact - Migration-Core Orchestrator Internal Split

## Summary

`underlay-migration-core/src/pipeline/orchestrator.rs` is now a smaller public
orchestrator front door over focused private helper modules.

New private module layout:

- `pipeline/orchestrator.rs`: `MigrationOrchestrator`, `new`, `stage_order`,
  and `run`
- `pipeline/orchestrator/stages.rs`: extract, normalize, transform,
  materialize, assets, and verify stage helpers
- `pipeline/orchestrator/decide.rs`: decision fingerprinting, reuse,
  invalidation, journaling, unresolved queue, and governance issue handling

## Compatibility

The split preserves:

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

This was a private helper split only. No public imports, method signatures,
trait bounds, stage output shapes, or persisted snapshot shapes changed.

## Validation

- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` failed on known structural backlog:
  `scan.attention-markers`, `scan.comment-ratio`, and `scan.god-files`

Structural movement:

- `pipeline/orchestrator.rs`: 487 lines to 149 lines
- `scan.god-files`: 15 high-error files to 14 high-error files
- `scan.god-files`: total findings stayed at 55 because the new helper files
  are warning-sized

Next batch validation:

- targeted devtools tests from `effigy test --plan` or a focused Cargo command
- `effigy rust:check`
- consumer checks only if public devtools imports move
- `effigy qa:docs`
- `effigy qa:northstar`
