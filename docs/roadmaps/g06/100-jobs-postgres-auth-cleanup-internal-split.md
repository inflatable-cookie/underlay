# g06.100 - Jobs Postgres Auth Cleanup Internal Split

## Why

`g06.099` found that `underlay-jobs-postgres/src/tasks/auth_cleanup.rs` mixes
six simple purge job handlers with the more complex inactive-account suspension
handler in one security-adjacent production file.

The next split should reduce reasoning load without changing task exports, job
type strings, SQL, or retention behavior.

## Goal

Split auth cleanup jobs into focused internal modules while preserving all
public task names and behavior.

## Scope

In scope:

- keep `auth_cleanup.rs` as the small module front door
- move purge job handlers into a focused purge module
- move inactive-account suspension into a focused inactive-account module
- preserve `underlay_jobs_postgres::tasks` exports
- preserve job type strings and `JobConfig::maintenance()` behavior
- preserve SQL strings, status values, reason strings, and logging fields
- preserve builder defaults and clamps

Out of scope:

- changing auth cleanup public APIs
- changing retention or deletion semantics
- changing scheduled job behavior
- adding new database integration tests
- changing consumer apps

## Acceptance Criteria

- `auth_cleanup.rs` becomes a small module front door
- purge jobs and inactive-account suspension live in focused modules
- public task exports remain stable
- jobs Postgres tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public task names, job type strings, SQL behavior,
or builder semantics must change, stop and re-enter planning.

## Current State

`g06.100` is complete.

Artifact:

- [100 artifact](./100-jobs-postgres-auth-cleanup-internal-split-artifact.md)

## Next Task

Execute `g06.101`: auth password service modularity audit.
