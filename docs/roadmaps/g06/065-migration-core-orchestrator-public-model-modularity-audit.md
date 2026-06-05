# g06.065 - Migration-Core Orchestrator Public Model Modularity Audit

## Why

After `g06.064`, the largest remaining Rust production god-file is
`underlay-migration-core/src/pipeline/orchestrator.rs`.

Migration-core is library-facing and already has a carefully preserved
crate-root export contract, so the next move should classify the orchestrator
surface before changing file shape.

## Goal

Classify the migration-core orchestrator public and internal model surface and
decide the safest next structural batch.

## Scope

In scope:

- inspect `underlay-migration-core/src/pipeline/orchestrator.rs` by helper
  family
- classify exported orchestration types, traits, errors, and helper flows
- scan internal and consumer usage for direct imports or crate-root reliance
- decide whether the next batch should split orchestrator internals, target
  another migration-core file, or defer behind a broader migration-core
  checkpoint
- update the Rust public API inventory if the orchestrator boundary needs
  tighter wording

Out of scope:

- changing migration execution behavior
- changing resume, checkpoint, or decision behavior
- changing run-store or plugin trait contracts
- changing crate-root exports
- changing devtools bundle behavior

## Acceptance Criteria

- orchestrator surface is grouped by stable contract family
- import paths and internal call sites are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is an audit and planning checkpoint. Any breaking migration-core contract
change must be promoted into an explicit follow-up card before execution.

## Current State

`g06.065` is next after `g06.064`.

## Next Task

Execute `g06.065`: migration-core orchestrator public model modularity audit.
