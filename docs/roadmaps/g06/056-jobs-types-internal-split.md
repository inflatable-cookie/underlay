# g06.056 - Jobs Types Internal Split

## Why

`g06.055` found that `underlay-jobs/src/types.rs` is the largest remaining
production/shared Rust god-file and has a safe internal split shape.

The jobs model is app-facing contract surface, so the split must preserve all
existing public front doors.

## Goal

Split `underlay-jobs/src/types.rs` into focused private modules while
preserving root exports, `types::*` compatibility, and serialized model shapes.

## Scope

In scope:

- split job identifiers/status/config/backoff into focused internal modules
- split progress and persisted job row models
- split dead-letter models and filters
- split scheduled task models
- split handler result/error/trait models
- preserve `underlay_jobs` root exports
- preserve `underlay_jobs::types::*` compatibility
- update tests only where module parent imports need to become explicit

Out of scope:

- changing job store, dead-letter store, or handler trait signatures
- changing runner, scheduler, registry, or Postgres adapter behavior
- changing serialized job/dead-letter/scheduled-task shapes
- consumer rollout unless public imports move

## Acceptance Criteria

- `types.rs` becomes a small module front door
- public exports remain source-compatible
- jobs tests pass with `--all-features`
- jobs Postgres tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal module split. If public imports, trait signatures, or
serialized shapes must move, stop and re-enter planning.

## Current State

`g06.056` is complete.

Artifact:

- [056 artifact](./056-jobs-types-internal-split-artifact.md)

## Next Task

Execute `g06.057`: DB pagination public model modularity audit.
