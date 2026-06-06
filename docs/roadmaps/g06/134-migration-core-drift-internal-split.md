# g06.134 - Migration-Core Drift Internal Split

## Why

`g06.133` found that `underlay-migration-core/src/drift.rs` mixes public drift
models, run-report threshold checks, decision-lineage checks, mismatch
thresholding, and category summary aggregation in one module.

The next split should make the drift safety boundary easier to reason about
without changing public drift APIs or serialized report shape.

## Goal

Split migration-core drift into focused internal modules while preserving
public drift APIs and behavior.

## Scope

In scope:

- replace `drift.rs` with a `drift/` module directory
- keep `drift/mod.rs` as the public module front door
- move public drift models into `model.rs`
- move run-report threshold checks into `run.rs`
- move decision-lineage checks into `lineage.rs`
- move category summary aggregation into `summary.rs`
- preserve existing drift tests

Out of scope:

- changing migration public APIs
- changing drift issue codes or severity
- changing serialized drift report shape
- changing verification or integrity behavior
- changing consumer apps

## Acceptance Criteria

- public root exports remain stable
- drift model serialization shape remains stable
- focused and full migration-core tests pass
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal module split. If exported drift APIs or drift semantics
must change, stop and re-enter planning.

## Current State

`g06.134` is ready.

## Next Task

Execute `g06.134`: migration-core drift internal split.
