# g06.133 - Migration-Core Drift Modularity Audit

## Why

After `g06.132`, the next Rust production warning-level file in the god-file
report is `underlay-migration-core/src/drift.rs`.

Migration drift detection is part of the promotion safety boundary. It should
be split from evidence about thresholds, issue detection, lineage checks, and
public model impact, not from file size alone.

## Goal

Classify the migration-core drift surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-migration-core/src/drift.rs` by responsibility family
- identify public models, threshold handling, run-report checks, lineage checks,
  and helper boundaries
- identify public API or migration behavior that must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader migration drift checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing migration public APIs
- changing drift detection semantics
- changing verification or integrity behavior
- changing consumer apps

## Acceptance Criteria

- drift responsibilities are grouped by stable behavior family
- public API and migration behavior boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds drift behavior
that must change, stop and re-enter planning.

## Current State

`g06.133` is ready.

## Next Task

Execute `g06.133`: migration-core drift modularity audit.
