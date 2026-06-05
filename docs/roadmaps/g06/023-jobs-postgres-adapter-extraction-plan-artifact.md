# g06.023 Artifact - Jobs Postgres Adapter Extraction Plan

## Summary

`underlay-jobs` still mixes stable job contracts with concrete Postgres runtime
adapters. The next extraction should create one `underlay-jobs-postgres`
adapter crate and leave `underlay-jobs` as the contract crate.

This is a breaking consumer update, but the current six consumers are
non-production and already follow the same local wrapper pattern.

## Current Underlay Surface

`rust/crates/underlay-jobs/Cargo.toml` exposes these adapter features:

- `postgres = ["sqlx", "thiserror", "uuid"]`
- `scheduler = ["cron", "postgres"]`
- `outbox = ["postgres"]`
- `full = ["postgres", "scheduler", "outbox"]`

`rust/crates/underlay-jobs/src/lib.rs` currently exports:

- core modules: `types`, `store`, `runner`, `registry`, `events`,
  `dead_letters`
- Postgres modules: `postgres`, `postgres_dead_letters`, `postgres_rows`,
  `postgres_scheduled`
- Postgres-adjacent modules: `scheduler`, `outbox`, `tasks`
- root adapter exports: `JobRepository`, `RepoError`,
  `PgDeadLetterRepository`, `ScheduledTaskRepository`, `PgJobNotifier`,
  `JOB_NOTIFY_CHANNEL`
- SQL constants: `JOB_TABLES_SQL`, `JOB_NOTIFY_SQL`,
  `JOB_DEAD_LETTERS_SQL`, `DOMAIN_EVENT_NOTIFY_SQL`

## Module Inventory

| Module | Current role | Target |
| --- | --- | --- |
| `types` | stable job domain model | stay in `underlay-jobs` |
| `store` | `JobStore` trait and store-facing contracts | stay in `underlay-jobs` |
| `dead_letters` | `DeadLetterStore` trait and dead-letter model | stay in `underlay-jobs` |
| `registry` | handler registry | stay in `underlay-jobs` |
| `runner` | generic runner plus Postgres notifier helper | keep generic runner in `underlay-jobs`; move notifier helper to adapter extension |
| `events` | job lifecycle event hub | stay in `underlay-jobs` |
| `postgres` | SQLx `JobRepository` and `RepoError` | move to `underlay-jobs-postgres` |
| `postgres_dead_letters` | SQLx dead-letter repository | move to `underlay-jobs-postgres` |
| `postgres_scheduled` | SQLx scheduled task repo and Pg LISTEN/NOTIFY | move to `underlay-jobs-postgres` |
| `postgres_rows` | private SQLx row mappings | move to `underlay-jobs-postgres` |
| `scheduler` | pure config plus Postgres-bound runtime scheduler | keep pure config in `underlay-jobs`; move runtime scheduler to adapter |
| `outbox` | SQLx durable outbox processor and notifier | move to `underlay-jobs-postgres` in this generation |
| `tasks` | SQLx operational task helpers | move to `underlay-jobs-postgres` |

## Selected Shape

Create one concrete adapter crate:

- `underlay-jobs`: stable contract crate
- `underlay-jobs-postgres`: concrete Postgres adapter crate

`underlay-jobs-postgres` should own:

- `JobRepository`
- `RepoError`
- `PgDeadLetterRepository`
- `ScheduledTaskRepository`
- `PgJobNotifier`
- `Scheduler`
- `outbox::*`
- `tasks::*`
- Postgres row mappings
- Postgres SQL constants
- the runner notification extension currently implemented as
  `JobRunner::run_with_notifier`

This avoids leaving SQLx, PgListener, and Postgres migration artifacts inside
the contract crate.

## Why One Adapter Crate

Outbox, scheduled tasks, job repository operations, and LISTEN/NOTIFY currently
share Postgres infrastructure and migration shape. Splitting them into multiple
adapter crates before the first extraction would increase package count without
removing a real dependency cycle.

The later architectural direction can still consolidate contract crates. This
step is about making the adapter boundary enforceable first.

## Consumer Inventory

Current dependency features:

| Consumer | Current dependency |
| --- | --- |
| `underlay-reference/acme-api` | `underlay-jobs` with `full` |
| `contact-patch/cp-api` | `underlay-jobs` with `full` |
| `compli-me/api` | `underlay-jobs` with `full` |
| `acowtancy/farmyard/crates/jobs` | `underlay-jobs` with `full` |
| `songsprout/nursery` | `underlay-jobs` with `full` |
| `loophole/composer/composer-api` | `underlay-jobs` with `postgres` |

Current source usage:

- API state/main files import or construct root `underlay_jobs::JobRepository`
  in all six consumers.
- App-local jobs crates re-export broad `underlay_jobs::{...}` contract types.
- `underlay_jobs::outbox::*` is re-exported by `underlay-reference`,
  `contact-patch`, `compli-me`, and `acowtancy`.
- `underlay_jobs::tasks::*` is re-exported by `underlay-reference`,
  `contact-patch`, `compli-me`, `acowtancy`, and `songsprout`.
- No consumer uses `underlay_jobs::postgres::*` directly in the scanned source.

## Execution Plan

Open `g06.024` as the code movement batch:

1. Add `rust/crates/underlay-jobs-postgres`.
2. Move Postgres repository, scheduled task, dead-letter, row, outbox, task,
   SQL constant, and notifier code into the adapter crate.
3. Keep `underlay-jobs` focused on traits, domain types, runner, registry,
   events, and pure scheduler config.
4. Move Postgres notification runner support into an adapter extension trait or
   adapter helper.
5. Update six consumers to depend on `underlay-jobs-postgres` for concrete
   Postgres usage.
6. Remove `postgres`, `outbox`, `scheduler`, and `full` as mixed adapter
   features from the core crate once consumers are migrated.

## Expected Consumer Changes

Consumer code should move:

- `underlay_jobs::JobRepository` to `underlay_jobs_postgres::JobRepository`
- `underlay_jobs::outbox::*` to `underlay_jobs_postgres::outbox::*`
- `underlay_jobs::tasks::*` to `underlay_jobs_postgres::tasks::*`
- Postgres runner notifier imports to `underlay_jobs_postgres::*`

Consumer Cargo files should keep `underlay-jobs` for core contracts and add
`underlay-jobs-postgres` for concrete storage.

## Validation Plan

Underlay:

- `effigy rust:check`
- targeted `cargo check` for `underlay-jobs` and `underlay-jobs-postgres` if
  Effigy output needs narrower proof

Consumers:

- check each local jobs wrapper crate and API crate touched by the import
  movement
- classify unrelated pre-existing failures separately

## Decision

Proceed with `g06.024`: Jobs Postgres adapter extraction execution.
