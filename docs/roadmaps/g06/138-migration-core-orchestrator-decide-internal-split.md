# g06.138 - Migration-Core Orchestrator Decide Internal Split

## Why

`g06.137` found that `pipeline/orchestrator/decide.rs` mixes decide-stage
orchestration with fingerprint input construction, prior decision reuse,
journal writes, unresolved queue writes, invalidation tracking, and report
assembly.

The decide stage is crate-internal but safety-sensitive. The next step is a
mechanical internal split that keeps decision behavior stable.

## Goal

Split the migration-core decide stage into focused internal modules without
changing public APIs, decision fingerprints, reuse semantics, journal writes,
unresolved queue behavior, or output counts.

## Scope

In scope:

- replace `pipeline/orchestrator/decide.rs` with a `decide/` module directory
- keep `decide_stage(...)` in `decide/mod.rs`
- extract fingerprint input construction into `decide/input.rs`
- extract prior decision validation/reuse handling into `decide/prior.rs`
- extract decision journal and unresolved queue write helpers into
  `decide/write.rs`
- preserve current focused decision-pipeline test behavior

Out of scope:

- changing migration public APIs
- changing decision fingerprint contents
- changing reuse, invalidation, or unresolved semantics
- changing journal or decision-index persistence behavior
- changing consumer apps

## Acceptance Criteria

- `MigrationOrchestrator::run(...)` continues to compile unchanged
- focused decision-pipeline tests pass with all features
- full `underlay-migration-core` tests pass with all features
- `effigy rust:check` passes
- roadmap artifact records the final module shape and public API impact

## Consumer Upgrade Impact

Expected impact: none.

This should be an internal module split. If decision behavior or public APIs
need to change, stop and re-enter planning.

## Current State

`g06.138` is complete.

Artifact:

- [138 artifact](./138-migration-core-orchestrator-decide-internal-split-artifact.md)

## Next Task

Execute `g06.139`: validation derive tests modularity audit.
