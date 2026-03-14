# 035 - Background Job Reliability and Observability

Status: Complete
Owner: Platform
Created: 2026-03-11
Depends on: 031

## Overview

Improve `underlay-jobs` with safer retry behavior, dead-letter handling, and job lifecycle events while keeping the core system generic and PostgreSQL-friendly.

## Research Basis

- `docs/research/implementation-decision-records/idr-background-job-improvements.md`
- `docs/research/translation-memos/background-job-improvements.md`
- `docs/research/value-tracks/background-job-patterns.md`
- `docs/research/specimen-dossiers/sidekiq.md`
- `docs/research/specimen-dossiers/bullmq.md`
- `docs/research/specimen-dossiers/temporal.md`
- `docs/guides/055-background-jobs.md`

## Likely Implementation Surface

- `rust/crates/underlay-jobs/src/`
- `rust/crates/underlay-jobs/migrations/`
- `docs/guides/055-background-jobs.md`

## Phase 35.1 - Retry Safety

- [x] Add opt-in jitter support to exponential backoff without silently changing existing retry timing.
- [x] Add targeted tests that verify retry spread and bounded delay behavior.
- [x] Document the default policy for new jobs versus existing configured jobs.

## Phase 35.2 - Dead Letters and Events

- [x] Add dead-letter persistence and repository traits for failed job inspection and retry.
- [x] Add lifecycle events for enqueue, claim, start, complete, fail, and dead-letter flows.
- [x] Keep observability hooks generic so apps can attach metrics, tracing, or dashboards without framework lock-in.

## Phase 35.3 - Consumer Rollout and Documentation

- [x] Add an upgrade note entry in `docs/guides/190-upgrade-compatibility.md`.
- [x] Update `docs/guides/055-background-jobs.md` with migration steps for new SQL artifacts and rollout guidance.
- [x] Document retention, requeue, and operational ownership expectations for dead letters.

## Deferred

- Shared admin UI dashboards for job monitoring.
- Workflow orchestration or DAG semantics.
- Rate limiting unless a later batch proves it belongs in the core crate.

## Consumer Upgrade Impact

- Expected impact class: `additive`, with explicit migration steps if new SQL tables are introduced.
- If dead-letter support requires consumers to copy a new SQL migration, the upgrade note must call that out first and provide the exact sequence.
- Retry jitter should not become an unannounced default change for existing deployments.
- Event hooks must be documented as optional integration points rather than mandatory observability infrastructure.

## Validation

```bash
cargo check -p underlay-jobs --all-features
cargo test -p underlay-jobs --all-features
effigy validate
```

## Next Task

Roadmap complete on 2026-03-11. Next broad batch: take `g01.036` for declarative migration verification rules, or close the remaining `In progress` roadmaps if you want the current wave fully settled first.
