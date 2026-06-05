# g06.095 - Devtools Migration Report Modularity Audit

## Why

After `g06.094`, the largest remaining Rust warning-level production file in
the god-file report is `underlay-devtools/src/migration_report.rs`.

Migration reports are shared devtools output. They should be easy to inspect
and extend without mixing data models, aggregation, and rendering in one file.

## Goal

Classify the devtools migration report surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-devtools/src/migration_report.rs` by responsibility family
- identify public report models, aggregation helpers, rendering behavior, and
  tests
- identify which exports must remain stable for devtools callers
- decide whether the next batch should split internal modules, extract model
  files, or defer behind a broader devtools report checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing migration report public APIs
- changing report output semantics
- changing migration execution behavior
- changing consumer apps

## Acceptance Criteria

- migration report responsibilities are grouped by stable behavior family
- public exports and caller-visible helpers are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a production-code structure audit. Expected impact is none unless the
audit finds public exports that must move; if so, stop and re-enter planning.

## Current State

`g06.095` is ready.

## Next Task

Execute `g06.095`: devtools migration report modularity audit.
