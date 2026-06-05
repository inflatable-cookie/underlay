# g06.055 - Jobs Public Model Modularity Audit

## Why

`underlay-jobs/src/types.rs` is now the largest production/shared Rust
god-file after the media domain and rendition splits. It is also broad
app-facing contract surface, so it should be audited before any mechanical
split.

## Goal

Classify the `underlay-jobs` public model surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-jobs/src/types.rs` by domain family
- classify which types are stable app-facing contracts
- identify private/internal helpers that can move without source breakage
- decide whether the next batch should split `types.rs`, split runtime
  orchestration instead, or defer jobs in favor of another structural target
- preserve crate-root exports unless a breaking change is explicitly approved

Out of scope:

- changing job store, handler, runner, scheduler, or notification traits
- changing serialized job payload/status shapes
- moving Postgres adapter behavior
- consumer rollout unless the audit finds a public import risk

## Acceptance Criteria

- produce an audit artifact with a recommended next code batch
- classify expected consumer impact
- update the Rust public API inventory if the jobs boundary needs tighter notes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an audit and planning checkpoint. Any breaking jobs contract change
must be promoted into an explicit follow-up card before execution.

## Current State

`g06.055` is complete.

Artifact:

- [055 artifact](./055-jobs-public-model-modularity-audit-artifact.md)

## Next Task

Execute `g06.056`: jobs types internal split.
