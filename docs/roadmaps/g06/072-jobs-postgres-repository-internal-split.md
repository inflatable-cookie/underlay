# g06.072 - Jobs-Postgres Repository Internal Split

## Why

`g06.071` found that `underlay-jobs-postgres/src/postgres.rs` exposes a narrow
crate-root adapter surface but mixes public repository model, direct repository
methods, SQL helpers, retry/dead-letter coordination, and `JobStore`
implementation in one high-error file.

## Goal

Split jobs-postgres repository internals into focused private modules while
preserving public root exports, repository methods, `RepoError`, SQL semantics,
dead-letter behavior, and `JobStore` behavior.

## Scope

In scope:

- split `RepoError` and result alias into a focused private/public model module
  if root exports stay intact
- split direct repository operations by helper family, such as create/claim,
  status transitions, failure/dead-letter handling, query/list/count, and
  maintenance
- split `JobStore` impl into a focused module if it keeps the same trait
  behavior
- preserve `JobRepository` construction and event-sink behavior
- preserve crate-root exports
- update tests only where private module paths need explicit imports

Out of scope:

- changing `RepoError` variants
- changing repository method signatures
- changing `JobStore` trait behavior
- changing SQL semantics
- changing retry or dead-letter behavior
- changing scheduled-task, notifier, scheduler, outbox, or task APIs
- consumer rollout unless public root exports move

## Acceptance Criteria

- `postgres.rs` becomes a smaller public adapter front door or thin
  coordinator
- root exports remain source-compatible
- jobs-postgres tests pass
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public exports, methods, error variants, SQL
semantics, or trait behavior must change, stop and re-enter planning.

## Current State

`g06.072` is complete.

Artifact:

- [072 artifact](./072-jobs-postgres-repository-internal-split-artifact.md)

## Next Task

Execute `g06.073`: auth JWT service tests modularity audit.
