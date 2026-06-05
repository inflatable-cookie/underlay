# g06.071 Artifact - Jobs-Postgres Repository Public Model Modularity Audit

## Summary

`underlay-jobs-postgres/src/postgres.rs` is the remaining Rust production
high-error god-file. It owns the public `JobRepository` adapter and `RepoError`
type plus the `JobStore` implementation for the shared `underlay-jobs`
contract.

The file currently groups:

- public error model: `RepoError`
- crate-local result alias: `Result<T>`
- public repository type: `JobRepository`
- constructor and event-sink configuration
- direct job repository methods: create, claim, running/progress/heartbeat,
  success/failure, cancel, stalled reset, get/list/count, archive, purge
- permanent-failure dead-letter insertion via `PgDeadLetterRepository`
- `JobStore` trait implementation for runner use
- raw UUID conversion helper
- job table SQL fragments embedded in method bodies

Adjacent adapter families are already separate:

- `postgres_dead_letters.rs`: public `PgDeadLetterRepository`
- `postgres_scheduled.rs`: public `ScheduledTaskRepository`,
  `PgJobNotifier`, and `JOB_NOTIFY_CHANNEL`
- `scheduler.rs`: public `Scheduler`
- `runner_ext.rs`: public `PostgresJobRunnerExt`
- `outbox.rs`: public outbox helpers
- `tasks/`: public maintenance tasks

## Consumer Evidence

Public usage is crate-root oriented:

- `src/lib.rs` re-exports `JobRepository` and `RepoError` from the private
  `postgres` module.
- Current consumers import `underlay_jobs_postgres::JobRepository`,
  `RepoError`, `PgDeadLetterRepository`, `PgJobNotifier`,
  `PostgresJobRunnerExt`, `ScheduledTaskRepository`, `Scheduler`, SQL
  constants, `outbox`, and `tasks` through crate root or app-local re-exports.
- Current consumers do not import `underlay_jobs_postgres::postgres::...`.
- Some consumers match `RepoError::Database(sqlx::Error::Database(...))`, so
  error variants are consumer-visible.
- Consumers construct `JobRepository::new(pool)` and use
  `with_event_sink(...)` in app-local job task handlers.
- API state structs store `Arc<JobRepository>` in multiple consumers.

## Decision

Queue `g06.072` as a jobs-postgres repository internal split.

The split should preserve:

- crate-root `underlay_jobs_postgres::JobRepository`
- crate-root `underlay_jobs_postgres::RepoError`
- `RepoError` variants and display behavior
- `JobRepository::new` and `JobRepository::with_event_sink`
- direct repository method names and signatures
- `JobStore` trait implementation behavior
- retry/dead-letter behavior in `mark_failed`
- SQL semantics, including claim ordering, status transitions, retry backoff,
  scheduled-task completion update, stalled-job handling, list/count filters,
  archive, and purge helpers
- existing root exports for scheduled, dead-letter, notifier, scheduler,
  runner extension, outbox, tasks, and SQL constants

## Public API Impact

Expected impact: none.

This should be a private module/function split only. If the split requires
changing root exports, public methods, error variants, trait implementation
behavior, SQL semantics, or consumer import paths, stop and re-enter planning.

## Validation

- `cargo test -p underlay-jobs-postgres --all-features`

Next code batch validation:

- `cargo test -p underlay-jobs-postgres --all-features`
- `effigy rust:check`
- targeted consumer `cargo check` only if root exports move
- `effigy qa:docs`
- `effigy qa:northstar`
