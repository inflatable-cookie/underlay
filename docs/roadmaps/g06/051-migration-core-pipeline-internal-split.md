# g06.051 - Migration-Core Pipeline Internal Split

## Why

`g06.050` found that `underlay-migration-core` should remain a broad public
model crate, but `pipeline.rs` is an oversized mixed file. It holds the public
orchestrator plus private resume, checkpoint, decision-support, and
failure-classification helpers.

This can be split without changing the crate-root public API.

## Goal

Split `underlay-migration-core/src/pipeline.rs` into smaller internal modules
while preserving all public root exports and persisted model shapes.

## Scope

In scope:

- keep `pipeline/types.rs` as the public stage/report model
- move `MigrationOrchestrator` into an internal orchestrator module
- move resume compatibility helpers into an internal resume module
- move stage snapshot/checkpoint load and persist helpers into an internal
  checkpoint module
- move decision provenance/unresolved decision helpers into an internal
  decision-support module
- move stage failure classification into an internal errors module
- preserve `pub use crate::pipeline::{...}` and crate-root exports
- update tests only where module paths require it

Out of scope:

- changing `MigrationPlugin`, `DecisionResolver`, `AssetResolver`, or
  `RunStore` trait signatures
- changing `PipelineRunReport`, stage output, checkpoint, or decision journal
  serialized shapes
- changing consumer app migration behavior
- broad migration engine redesign
- release execution or publishing

## Acceptance Criteria

- `pipeline.rs` becomes a small module front door
- public exports remain source-compatible
- persisted JSON model shapes do not change
- `underlay-migration-core` tests pass
- Farmyard remains source-compatible if root exports are touched

## Consumer Upgrade Impact

Expected impact: none.

This is an internal module split. If public imports or trait signatures must
move, stop and re-enter planning.

## Current State

`g06.051` is complete.

Artifact:

- [051 artifact](./051-migration-core-pipeline-internal-split-artifact.md)

## Next Task

Execute `g06.052`: Rust structural backlog checkpoint.
