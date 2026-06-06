# g06.137 - Migration-Core Orchestrator Decide Modularity Audit

## Why

After `g06.136`, the next Rust production warning-level file in the god-file
report is `underlay-migration-core/src/pipeline/orchestrator/decide.rs`.

The decide orchestrator is part of the migration decision safety boundary. It
should be split from evidence about stage inputs, candidate classification,
decision matching, journal/index writes, and report construction, not from file
size alone.

## Goal

Classify the orchestrator decide surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-migration-core/src/pipeline/orchestrator/decide.rs` by
  responsibility family
- identify public or crate-visible types and helper boundaries
- identify migration behavior that must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader migration orchestrator checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing migration public APIs
- changing decision semantics
- changing journal or decision-index persistence behavior
- changing consumer apps

## Acceptance Criteria

- decide responsibilities are grouped by stable behavior family
- public and crate-visible behavior boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds migration decide
behavior that must change, stop and re-enter planning.

## Current State

`g06.137` is ready.

## Next Task

Execute `g06.137`: migration-core orchestrator decide modularity audit.
