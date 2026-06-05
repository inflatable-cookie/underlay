# g06.023 - Jobs Postgres Adapter Extraction Plan

## Why

`g06.022` extracted the small auth-state Postgres adapter. The largest
remaining mixed contract/adapter runtime surface is `underlay-jobs`.

Jobs has a broader Postgres shape than media or auth-state: store, dead
letters, scheduled tasks, outbox, runner wiring, and operational task helpers.
It needs an explicit extraction plan before moving code.

## Goal

Inventory and plan the `underlay-jobs` Postgres adapter extraction.

## Scope

In scope:

- inventory `underlay-jobs` Postgres modules, feature flags, root exports, and
  consumer imports
- decide whether extraction should be one adapter crate or multiple adapter
  crates
- identify the smallest executable first extraction batch
- name affected consumers and expected Cargo/source changes
- update contracts and roadmap controls

Out of scope:

- moving jobs code in this planning card
- extracting audit or security-alert adapters
- release execution or publishing
- TypeScript package boundary work

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `060`: jobs, events, and operator systems
- `122`: Rust public API inventory
- `020-reference-grade-underlay-architecture`: target architecture

## Acceptance Criteria

- jobs Postgres surfaces are inventoried by module and exported symbol
- consumer usage of `underlay-jobs/postgres`, `scheduler`, `outbox`, and
  `full` is recorded
- extraction shape is selected with rationale
- next executable batch is opened
- validation passes or failures are classified

## Consumer Upgrade Impact

Impact: planning only.

The follow-up execution batch is likely breaking.

## Current State

`g06.023` is complete.

The selected shape is one new `underlay-jobs-postgres` adapter crate. The core
`underlay-jobs` crate keeps job contracts, runner, registry, event hub, store
traits, dead-letter traits, and pure scheduler configuration.

See
[`023-jobs-postgres-adapter-extraction-plan-artifact.md`](023-jobs-postgres-adapter-extraction-plan-artifact.md).

## Next Task

Execute `g06.024`: Jobs Postgres adapter extraction execution.
