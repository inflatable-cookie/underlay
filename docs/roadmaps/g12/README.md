# g12 - Standalone Nightfire Package

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
3. [ ] [`g12.004`](004-prospective-merge-protocol-migration.md) — migrate the
   Queue control manifest to v4 prospective-merge evaluation (`ready`
   configuration maintenance; independent of product sequencing)
4. [ ] [`g12.005`](005-consume-nightfire-v0-2-0.md) — consume nightfire
   `v0.2.0` internally, keep historical facades, keep Underlay-owned
   `./nightfire/media` (`ready`; Card 274 slice)

`g12.001` was already flat and is unchanged by the switchover except
for this frontier wording.

## Approved Frontier

Ready: Acowtancy Market Card 278 lane under `g12.001` only. The
release and consumer lanes remain serial behind its accepted
dual-language proof. `g12.002` is a separate maintenance task queued behind
Northstar's plural lifecycle contract.
`g12.004` is separately authorized portfolio maintenance and changes no product
frontier.

## Dependencies And Parallelism

`g12` is independent of `g11` media consumer rollout. The Nightfire
release is serial behind Card 278. Underlay compatibility, Froyo
adoption, and Farmyard Rust adoption touch separate implementation
lanes and may run in parallel after that release. Desktop is serial
behind Froyo.

## Next Task

Dispatch `g12.005` (consume nightfire v0.2.0; Card 274). Do not publish
without the operator. Farmyard bridge-drop is Acowtancy g05.155 after
this repository tags.
<!-- northstar:lifecycle:begin schema=northstar.lifecycle.projection.v2 digest=sha256:c2fcd46900f7500a2f200fe83068e5eb942ce28e198d285c5db6339d712d683e -->
| Generation | Disposition | Runway state |
| --- | --- | --- |
| g11 | open | planning_required |
| g12 | open | planning_required |
| g13 | open | planning_required |
| Task | Status | Stage | Revision | Record digest |
| --- | --- | --- | --- | --- |
| g12.002 | complete | none | 8 | sha256:77d3fad9d4fddc202e121350453db1fd50b2da313f4e23ba365f3c4b84ba6a9c |
| g12.003 | complete | none | 8 | sha256:05cbbdf87f96ef842b4e70b8b9a53a96423e20bb5fe64493fc848e35050a5cc5 |
| g12.004 | complete | none | 8 | sha256:a9c141f6c2c3a6cc36fa2b8ad1d2004b0c8f1ccf532d3aa06693d6ec93fe724e |
| g12.005 | complete | none | 8 | sha256:d326252feb63f5d07fe602e2a583cbaf38da80b35bb44e4b7792b770c56b4ade |
| g13.001 | complete | none | 8 | sha256:e3c2120126b98eab06603171a71aa6eb247d7b5083e3b337ac300c3660b2a8b3 |
| g13.002 | complete | none | 8 | sha256:27ebe07858d9d9e19b4774e67cdb633762cf2744a672ef74c99cdae746ff499a |
<!-- northstar:lifecycle:end -->
