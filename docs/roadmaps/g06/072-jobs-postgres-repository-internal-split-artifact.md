# g06.072 Artifact - Jobs-Postgres Repository Internal Split

## Summary

`underlay-jobs-postgres/src/postgres.rs` is now a small public adapter front
door. The crate-root API and SQL behavior are unchanged.

New private modules:

- `postgres/create.rs`: create and claim-batch repository methods
- `postgres/status.rs`: running, progress, heartbeat, success, and cancel
  methods
- `postgres/failure.rs`: failure, retry, and dead-letter coordination
- `postgres/query.rs`: get, list, and count methods
- `postgres/maintenance.rs`: stalled-job, archive, and purge methods
- `postgres/job_store.rs`: `JobStore` implementation

`postgres.rs` now owns:

- `RepoError`
- `Result<T>`
- `JobRepository`
- `JobRepository::new`
- `JobRepository::with_event_sink`
- raw UUID conversion helper
- private module wiring

## Preserved Behavior

The split preserved:

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

## Structural Result

`postgres.rs` moved from a high-error god-file into a front door:

- `postgres.rs`: 53 lines
- `postgres/create.rs`: 91 lines
- `postgres/status.rs`: 127 lines
- `postgres/failure.rs`: 115 lines
- `postgres/query.rs`: 78 lines
- `postgres/maintenance.rs`: 80 lines
- `postgres/job_store.rs`: 69 lines

`effigy doctor` now reports:

- `scan.god-files`: 52 findings, 11 errors, 41 warnings
- `scan.attention-markers`: 11 findings, 2 errors, 9 warnings
- `scan.comment-ratio`: 12 findings, 3 errors, 9 warnings

The doctor failure remains the known structural backlog.

## Public API Impact

Impact: none.

This was a private module split. No consumer app update is required.

## Validation

- `cargo test -p underlay-jobs-postgres --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` expected failure on known structural scans
