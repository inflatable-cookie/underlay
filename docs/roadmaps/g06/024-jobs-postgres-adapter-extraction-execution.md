# g06.024 - Jobs Postgres Adapter Extraction Execution

## Why

`g06.023` selected a one-crate Postgres adapter extraction for
`underlay-jobs`. The current crate still exposes SQLx repositories, outbox,
scheduled-task runtime, operational task helpers, and Postgres SQL constants
from the core job contract package.

That keeps consuming apps pointed at a mixed contract/adapter crate and makes
the reference-grade architecture harder to reason about.

## Goal

Extract the concrete Postgres job runtime into `underlay-jobs-postgres` and
migrate the six known consumers.

## Scope

In scope:

- add `rust/crates/underlay-jobs-postgres`
- move Postgres repositories, row mappings, scheduled task runtime, notifier,
  dead-letter repository, outbox processor, operational task helpers, and SQL
  constants into the adapter crate
- leave `underlay-jobs` as the core job contract crate
- move Postgres-specific runner notification support into the adapter crate
- update Underlay docs and public API inventory
- update all six consumer apps that currently depend on `underlay-jobs` with
  `postgres`, `scheduler`, `outbox`, or `full`
- validate Underlay and touched consumers

Out of scope:

- audit or security-alert adapter extraction
- splitting outbox into an independent contract crate
- changing app-specific job handlers or business logic
- publishing or release execution

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `060`: jobs, events, and operator systems
- `122`: Rust public API inventory
- `g06.023`: jobs extraction plan artifact

## Acceptance Criteria

- `underlay-jobs` no longer depends on SQLx for Postgres storage, LISTEN/NOTIFY,
  outbox processing, or operational task helpers
- `underlay-jobs-postgres` owns all concrete Postgres job runtime exports
- consumers import concrete repositories, outbox helpers, and task helpers from
  `underlay-jobs-postgres`
- consumer Cargo files keep `underlay-jobs` for contracts and add
  `underlay-jobs-postgres` for concrete storage
- Underlay and consumer validation passes or failures are classified
- docs/contracts reflect the new package boundary

## Consumer Upgrade Impact

Impact: breaking.

Current consumers must update imports and dependencies:

- `underlay_jobs::JobRepository` becomes
  `underlay_jobs_postgres::JobRepository`
- `underlay_jobs::outbox::*` becomes `underlay_jobs_postgres::outbox::*`
- `underlay_jobs::tasks::*` becomes `underlay_jobs_postgres::tasks::*`
- concrete Postgres scheduler/notifier imports move to
  `underlay_jobs_postgres`

The breaking impact is acceptable for this generation because the six known
consumers are not in production and the user explicitly allowed consumer
updates during the reference-grade reset.

## Current State

`g06.024` is complete.

Underlay now has:

- `underlay-jobs` as the core job contract crate
- `underlay-jobs-postgres` as the concrete Postgres adapter crate
- Postgres repositories, scheduled task runtime, outbox processing, maintenance
  task helpers, SQL constants, and LISTEN/NOTIFY runner support in the adapter
  crate

The six known consumers were migrated to keep `underlay-jobs` for contracts and
add `underlay-jobs-postgres` for concrete storage/runtime usage.

Validation passed:

- Underlay: `effigy rust:check`
- `underlay-reference/acme-api`: `cargo check -p acme-jobs -p acme-api`
- `contact-patch/cp-api`: `cargo check -p cp-jobs -p cp-api`
- `compli-me/api`: `cargo check -p compli-me-jobs -p compli-me-api`
- `songsprout/nursery`: `cargo check -p nursery-jobs -p nursery-api`
- `acowtancy/farmyard`: `cargo check -p farmyard-jobs -p farmyard-api`
- `loophole/composer/composer-api`: `cargo check -p composer-api`

## Next Task

Execute `g06.025`: six-consumer rollout and compatibility retirement proof.
