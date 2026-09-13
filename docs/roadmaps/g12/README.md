# g12 - Standalone Nightfire Package

Status: active
Owner: repo maintainers and Acowtancy cross-repo Coordinator
Started: 2026-09-04

## Current Generation

`g12` moves the generic Rust and TypeScript/Svelte Nightfire system
into one standalone repository and removes its accidental coupling to
Underlay. The operator selected crate name `nightfire` and immutable
Git-tag distribution.

This README owns the `g12` roadmap and approved frontier. The
`g12.NNN` file below is the sole executable planning unit; there is no
milestone wrapper or nested `batch-cards/` hierarchy.

## Task Sequence

1. [ ] [`g12.001`](001-standalone-nightfire-extraction-and-consumer-rollout.md)
   — complete both language extractions, release, retain Underlay
   compatibility facades, move Froyo and Farmyard, then remove
   Underlay from Bovine Desktop's frozen graph (`active`)
2. [ ] [`g12.002`](002-adopt-effigy-hosted-lifecycle-hook.md) — adopt
   configuration-only lifecycle hooks with truthful parallel-generation
   projections (`queued` behind Northstar task
   `73d569cd-82c1-427d-b636-d75117bbe350`)

`g12.001` was already flat and is unchanged by the switchover except
for this frontier wording.

## Approved Frontier

Ready: Acowtancy Market Card 278 lane under `g12.001` only. The
release and consumer lanes remain serial behind its accepted
dual-language proof. `g12.002` is a separate maintenance task queued behind
Northstar's plural lifecycle contract.

## Dependencies And Parallelism

`g12` is independent of `g11` media consumer rollout. The Nightfire
release is serial behind Card 278. Underlay compatibility, Froyo
adoption, and Farmyard Rust adoption touch separate implementation
lanes and may run in parallel after that release. Desktop is serial
behind Froyo.

## Next Task

Dispatch Acowtancy Market Card 278. Queue may dispatch `g12.002` after its
Northstar prerequisite closes; it does not change the product frontier.
<!-- northstar:lifecycle:begin schema=northstar.lifecycle.projection.v2 digest=sha256:339f0f952c3613bbd95f9de92f05357ff50815df2e995b535e2a888fdc4c20eb -->
| Generation | Disposition | Runway state |
| --- | --- | --- |
| g11 | open | planning_required |
| g12 | open | planning_required |
| Task | Status | Stage | Revision | Record digest |
| --- | --- | --- | --- | --- |
| g12.002 | complete | none | 8 | sha256:77d3fad9d4fddc202e121350453db1fd50b2da313f4e23ba365f3c4b84ba6a9c |
<!-- northstar:lifecycle:end -->
