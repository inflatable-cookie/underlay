# g06.022 - Postgres Runtime Adapter Isolation Batch

## Why

`g06.021` proved the first explicit adapter crate by extracting media Postgres
code out of the media contract crate.

The same architecture pressure remains in other runtime crates that still mix
app-facing contracts with concrete Postgres implementation modules. The next
batch should inventory those seams and pick the next narrow extraction target.

## Goal

Select and execute the next bounded Postgres adapter isolation step.

## Scope

In scope:

- inventory Postgres feature/module usage in runtime crates
- compare `underlay-auth`, `underlay-jobs`, `underlay-audit`, and
  `underlay-security-alerts` as candidate targets
- choose one target with the smallest useful consumer-proof surface
- update `underlay-reference` first if consumer changes are needed
- update the remaining affected consumers in the same batch
- classify release impact under `023`

Out of scope:

- extracting every Postgres-backed crate in one batch
- release execution or publishing
- TypeScript package boundary work
- devtools migration-bundle isolation

## Contract References

- `001`: working rules
- `020`: HTTP transport and server boundary
- `023`: release and compatibility rollout
- `030`: auth and session systems
- `120`: background jobs, audit, and ops contracts
- `122`: Rust public API inventory
- `020-reference-grade-underlay-architecture`: target architecture

## Acceptance Criteria

- candidate runtime Postgres adapter seams are inventoried
- one extraction target is selected with rationale
- affected Underlay code is moved or narrowed
- affected consumers are updated or explicitly unaffected
- targeted Underlay and consumer validation passes or failures are classified
- no root compatibility export is added unless this card records why

## Consumer Upgrade Impact

Impact: likely breaking.

The current generation allows controlled breaking changes because the six known
consumers are not production deployments.

## Current State

`g06.022` is complete.

Artifact:

- `022-postgres-runtime-adapter-isolation-batch-artifact.md`

## Next Task

Execute `g06.023`: Jobs Postgres adapter extraction plan.
