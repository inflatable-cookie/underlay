# 021 - Jobs And Operator Assessment

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.020` repaired the media authority drift enough for the next assessment
wave to proceed honestly.

The next system family in the contract order is jobs and operator systems,
anchored by `060`.

## Goals

- assess the live jobs/events/operator implementation against `060`
- separate true contract failures from older layering or packaging residue
- identify the smallest honest repair set for the shared operator boundary
- leave explicit findings and a bounded next lane instead of broad infra churn

## Non-Goals

- executing large queue/scheduler refactors in the same batch
- skipping ahead to Nightfire/migration before operator findings are explicit
- application-specific jobs or dashboard work

## Inputs

- [docs/contracts/060-jobs-events-and-operator-systems.md](/Users/tom/Dev/projects/underlay/docs/contracts/060-jobs-events-and-operator-systems.md)
- `rust/crates/underlay-jobs/**`
- `rust/crates/underlay-events/**`
- `rust/crates/underlay-audit/**`
- `rust/crates/underlay-security-alerts/**`
- `rust/crates/underlay-ratelimit/**`
- `rust/crates/underlay-email/**`

## Exit Criteria

- the live jobs/operator implementation is reviewed against `060`
- the real findings are documented in severity order
- the next repair step is expressed as one bounded roadmap lane or a small
  repair set
- the Nightfire/migration assessment can start without ambiguity about the
  shared operator boundary

## Findings

### 1. Domain events are split across two shared surfaces with different authority levels

Severity: medium

`underlay-events` defines the durable event row shape and write seam, but
`underlay-jobs::outbox` owns the actual reliable processing model, notification
channel, and polling/listener behavior. The contract already suspected this,
and the code confirms it.

Evidence:

- [rust/crates/underlay-events/src/lib.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-events/src/lib.rs:1)
- [rust/crates/underlay-jobs/src/outbox.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-jobs/src/outbox.rs:1)

Impact:

- there is no single obvious “domain events system” owner in the repo
- maintainers can misread `underlay-events` as the full eventing stack when it
  is really only the append/schema seam

### 2. `underlay-audit` still relies on dynamic table names rather than a stronger shared schema contract

Severity: medium

The crate is honest in code, but the boundary is weaker than a fixed shared
schema contract. Query and writer functions accept app-supplied fully qualified
table names, validated only by a character whitelist.

Evidence:

- [rust/crates/underlay-audit/src/lib.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-audit/src/lib.rs:1)
- [rust/crates/underlay-audit/src/query.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-audit/src/query.rs:1)
- [rust/crates/underlay-audit/src/writer.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-audit/src/writer.rs:1)

Impact:

- the shared audit surface is a row/query contract over app-owned tables, not a
  stronger shared schema/module
- docs and contract wording need to keep that distinction explicit

### 3. Shared email templating is optional and useful, but its retained ownership is still debatable

Severity: low

The template engine is cleanly isolated behind a feature flag and does not
distort the rest of the email adapter seam. The open question is not whether it
works, but whether it still deserves long-term retained Underlay ownership.

Evidence:

- [rust/crates/underlay-email/src/templates.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-email/src/templates.rs:1)
- [rust/crates/underlay-email/src/lib.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-email/src/lib.rs:1)

Impact:

- this is an ownership-challenge question, not a current contract break

## Assessment Result

The operator layer does not need a broad implementation fix next. It needs a
boundary repair:

- make the domain-event/outbox ownership split explicit
- keep audit honest as an app-table contract rather than over-claiming shared
  schema ownership
- clarify the status of shared email templating without forcing a redesign in
  the same batch

## Next Task

Execute `g04.022`: repair the operator boundary and audit authority drift.
