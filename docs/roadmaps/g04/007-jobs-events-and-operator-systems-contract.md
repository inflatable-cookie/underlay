# 007 - Jobs Events And Operator Systems Contract

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.006` settles the storage/blob/media layer. The next dependency is the
shared operator and async infrastructure surface: jobs, scheduled tasks,
events, audit, security alerts, rate limiting, and email.

## Goals

- define the shared async and operator-facing infrastructure contract
- separate durable control-plane semantics from app-local admin workflows
- prepare later system-page and runtime assessment work on top of a clear
  backend operator boundary

## Non-Goals

- implementation repair beyond light authority alignment needed to write the
  contract
- app-specific operator UX or admin route design
- Nightfire/content migration work

## Inputs

- `rust/crates/underlay-jobs/**`
- `rust/crates/underlay-events/**`
- `rust/crates/underlay-email/**`
- `rust/crates/underlay-audit/**`
- `rust/crates/underlay-security-alerts/**`
- `rust/crates/underlay-ratelimit/**`

## Outputs

- [`docs/contracts/060-jobs-events-and-operator-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/060-jobs-events-and-operator-systems.md)
- refreshed contract and roadmap front doors so `g04` now points at the
  Nightfire/migration lane

## Outcome

The operator-systems contract now exists.

It settles:

- the shared job queue, handler, runner, scheduler, notifier, and dead-letter
  boundary
- the outbox/domain-event processing seam
- reusable audit entry/query/writer surfaces
- pluggable email, rate-limit, and security-alert seams
- the line between shared control-plane mechanics and app-local admin workflow

It also records current drift to assess later, especially the split event/outbox
story across `underlay-events` and `underlay-jobs`, the dynamic audit-table
surface, and the still-open question of whether shared email templating remains
the right ownership choice.

## Next Task

Execute `g04.008`: write `070-nightfire-and-migration-systems.md`.
