# 022 - Operator Boundary And Audit Authority Repair

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.021` assessed the live jobs/events/operator implementation against `060`.

The core async and operator mechanics are broadly real: jobs have a durable
queue, the scheduler is layered on top of jobs, dead letters are first-class,
and rate limiting and security alerts have clear shared seams.

The main drift is boundary clarity:

- domain events are split between `underlay-events` as a write/schema seam and
  `underlay-jobs::outbox` as the actual durable processing model
- `underlay-audit` still depends on app-supplied dynamic table names instead of
  a stronger shared schema contract
- shared email templating may be retained usefully, but its current status as a
  durable Underlay-owned surface should be challenged explicitly

## Goals

- make the shared domain-event/outbox authority chain explicit and honest
- repair the audit authority docs to match the real dynamic-table contract
- decide whether shared email templating remains retained Underlay ownership or
  should be demoted to optional convenience status in docs/contracts

## Non-Goals

- broad queue or scheduler refactors
- application-specific jobs/admin UX work
- moving on to Nightfire before the operator boundary is clear

## Inputs

- [docs/roadmaps/g04/021-jobs-and-operator-assessment.md](/Users/tom/Dev/projects/underlay/docs/roadmaps/g04/021-jobs-and-operator-assessment.md)
- [docs/contracts/060-jobs-events-and-operator-systems.md](/Users/tom/Dev/projects/underlay/docs/contracts/060-jobs-events-and-operator-systems.md)
- `rust/crates/underlay-jobs/src/outbox.rs`
- `rust/crates/underlay-events/src/lib.rs`
- `rust/crates/underlay-audit/**`
- `rust/crates/underlay-email/src/templates.rs`

## Exit Criteria

- the shared operator boundary is truthful in both docs and code
- the outbox/domain-event split is explicitly documented without ambiguity
- audit authority matches the live table-location model
- the next higher assessment lane can treat operator systems as a stable lower
  dependency

## Changes

- updated `060` so the contract now states plainly that `underlay-events` owns
  the append/schema seam while `underlay-jobs::outbox` owns durable processing
  and notification behavior
- repaired the audit section so it describes the real app-owned table model
  instead of implying stronger shared schema ownership
- downgraded shared email templating from implicit core surface to explicit
  optional convenience status
- aligned the crate-level docs in `underlay-events`, `underlay-audit`, and
  `underlay-email` to the same authority split

## Result

The operator boundary is now honest without forcing a larger refactor:

- domain events have one append/schema contract and one processing contract
- audit is clearly documented as a shared row/query model over app-owned tables
- shared email templating remains retained for now, but only as optional
  convenience rather than a required pillar of the operator layer

## Next Task

Execute `g04.023`: assess the Nightfire and migration systems against `070`.
