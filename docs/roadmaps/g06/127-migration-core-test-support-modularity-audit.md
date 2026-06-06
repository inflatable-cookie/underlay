# g06.127 - Migration-Core Test Support Modularity Audit

## Why

After `g06.126`, the next Rust warning-level file in the god-file report is
`underlay-migration-core/src/tests/support.rs`.

Shared test support affects many migration-core behavior tests. It should be
split from evidence about fixture families and test helper contracts, not from
file size alone.

## Goal

Classify the migration-core test support surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-migration-core/src/tests/support.rs` by fixture and helper
  family
- identify plugin fixture, pipeline fixture, checkpoint, decision, verification,
  and assertion helper boundaries
- identify helper behavior that existing tests rely on
- decide whether the next batch should split internal test modules, extract
  helper files, or defer behind a broader migration-core test cleanup checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing migration public APIs
- changing production migration behavior
- changing test assertions unrelated to support structure
- changing consumer apps

## Acceptance Criteria

- test support responsibilities are grouped by stable fixture/helper family
- test behavior dependencies are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a test-support structure audit. If preserving tests requires changing
production migration behavior, stop and re-enter planning.

## Current State

`g06.127` is complete.

Artifact:

- [127 artifact](./127-migration-core-test-support-modularity-audit-artifact.md)

## Next Task

Execute `g06.128`: migration-core test support internal split.
