# g06.085 - Migration-Core Decision-Memory Modularity Audit

## Why

After `g06.084`, the Rust high-error god-file line is cleared. The largest
remaining Rust warning file is
`underlay-migration-core/src/decision_memory.rs`.

Decision memory is production migration infrastructure. It should be split only
from behavior and public-surface evidence, not from file size alone.

## Goal

Classify the migration-core decision-memory surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-migration-core/src/decision_memory.rs` by responsibility
  family
- identify public models, storage/serialization behavior, validation helpers,
  and internal implementation groups
- decide whether the next batch should split internal modules, extract stable
  model files, or defer behind a broader migration-core checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing migration-core public APIs
- changing decision-memory serialization semantics
- changing migration verification or drift behavior
- changing consumer apps

## Acceptance Criteria

- decision-memory responsibilities are grouped by stable behavior family
- helper/model extraction opportunities are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a production-code structure audit. Expected impact is none unless the
audit finds public exports that must move; if so, stop and re-enter planning.

## Current State

`g06.085` is next.

## Next Task

Execute `g06.085`: migration-core decision-memory modularity audit.
