# g06.086 - Migration-Core Decision-Memory Internal Split

## Why

`g06.085` found that `underlay-migration-core/src/decision_memory.rs` mixes
public models, fingerprinting, parsing/indexing, validation, reuse evaluation,
and provenance selection in one production module.

## Goal

Split decision-memory internals into focused modules while preserving the
crate-root public API and all serialization/behavior contracts.

## Scope

In scope:

- move public decision-memory models into a focused model module
- split fingerprinting and canonical JSON helpers into a focused module
- split index build/merge/parse behavior into a focused module
- split journal/unresolved/index validation helpers into a focused module
- split reuse evaluation and version compatibility helpers into a focused
  module
- split effective decision and provenance chain helpers into a focused module
- preserve root exports from `underlay-migration-core`
- adjust internal imports only as needed

Out of scope:

- changing migration-core public APIs
- changing decision-memory serialization semantics
- changing migration verification or drift behavior
- changing consumer apps

## Acceptance Criteria

- `decision_memory.rs` becomes a small front door
- behavior groups live in focused internal modules
- crate-root decision-memory re-exports remain stable
- focused decision-memory tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public paths, signatures, or serialization shapes
must change, stop and re-enter planning.

## Current State

`g06.086` is complete.

Artifact:

- [086 artifact](./086-migration-core-decision-memory-internal-split-artifact.md)

## Next Task

Execute `g06.087`: AI runtime tests modularity audit.
