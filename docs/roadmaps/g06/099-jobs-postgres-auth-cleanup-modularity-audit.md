# g06.099 - Jobs Postgres Auth Cleanup Modularity Audit

## Why

After `g06.098`, the largest remaining Rust warning-level production file in
the god-file report is
`underlay-jobs-postgres/src/tasks/auth_cleanup.rs`.

Auth cleanup jobs touch security-adjacent retention and deletion behavior. They
should be split from responsibility evidence, not file size alone.

## Goal

Classify the jobs Postgres auth cleanup surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-jobs-postgres/src/tasks/auth_cleanup.rs` by responsibility
  family
- identify public task config, repository/query helpers, cleanup behavior,
  reporting behavior, and tests
- identify which exports and task contracts must remain stable
- decide whether the next batch should split internal modules, extract model
  files, or defer behind a broader jobs Postgres checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing auth cleanup public APIs
- changing retention or deletion semantics
- changing scheduled job behavior
- changing consumer apps

## Acceptance Criteria

- auth cleanup responsibilities are grouped by stable behavior family
- public exports and task-visible helpers are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a production-code structure audit. Expected impact is none unless the
audit finds public exports that must move; if so, stop and re-enter planning.

## Current State

`g06.099` is complete.

Artifact:

- [099 artifact](./099-jobs-postgres-auth-cleanup-modularity-audit-artifact.md)

## Next Task

Execute `g06.100`: jobs Postgres auth cleanup internal split.
