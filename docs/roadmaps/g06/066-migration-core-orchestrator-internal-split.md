# g06.066 - Migration-Core Orchestrator Internal Split

## Why

`g06.065` found that `underlay-migration-core/src/pipeline/orchestrator.rs`
has a narrow public API but a large internal `run` flow.

Consumers depend on `MigrationOrchestrator` through crate-root exports, not on
private helper layout.

## Goal

Split orchestrator internals into focused private modules or helper functions
while preserving public orchestrator methods, stage execution behavior,
persisted snapshot shapes, and crate-root exports.

## Scope

In scope:

- split extract, normalize, transform, materialize, assets, and verify stage
  helpers if it reduces the `run` method materially
- split decision-stage internals into a focused helper family
- preserve `MigrationOrchestrator::new`, `stage_order`, and `run`
- preserve `MigrationOrchestrator` public fields
- preserve all existing root and `pipeline` exports
- update tests only where internal module parent imports need to become
  explicit

Out of scope:

- changing stage order
- changing plugin, resolver, asset resolver, or run-store trait contracts
- changing resume, checkpoint, decision journal, unresolved queue, or
  verification behavior
- changing stage output/report serialized shapes
- changing devtools bundle behavior
- consumer rollout unless public imports move

## Acceptance Criteria

- `pipeline/orchestrator.rs` becomes a smaller public orchestrator front door
  or thin coordinator
- public exports remain source-compatible
- `underlay-migration-core` tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public imports, method signatures, stage/report
shapes, or persisted snapshot behavior must move, stop and re-enter planning.

## Current State

`g06.066` is next after `g06.065`.

## Next Task

Execute `g06.066`: migration-core orchestrator internal split.
