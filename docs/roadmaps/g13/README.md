# g13 - Poodle Producer Releases

Owner: repo maintainers
Started: 2026-09-14

## Current Generation

`g13` is the operator-directed producer lane for truthful published Poodle
pins. `g13.001` shipped Underlay `v0.9.9` with Poodle `0.4.1`. Poodle `0.4.2`
then replaced that web package after repairing its published Svelte Tabs
cross-window bridge regression. `g13.002` moved Underlay to that corrected
package and released `v0.9.10` without consumer edits.

This README owns the `g13` roadmap and approved frontier. The
`g13.NNN` files below are the sole executable planning units; there is
no milestone wrapper or nested `batch-cards/` hierarchy.

## Task Sequence

1. [x] [`g13.001`](001-poodle-0-4-1-producer-release.md) — Poodle `0.4.1`
   producer release, complete at Underlay `v0.9.9`.
2. [x] [`g13.002`](002-poodle-0-4-2-producer-release.md) — pin
   `poodle-svelte` exact `0.4.2`, regenerate `bun.lock`, prove the current
   caller surface, and release Underlay `v0.9.10` (`complete`).

## Approved Frontier

No ready task remains in g13. `g13.002` produced the corrected Underlay tag
required by Desktop `g02.109`.

## Dependencies And Parallelism

`g13` remains independent of `g11` media rollout and `g12` Nightfire
extraction. `g13.002` touches the root web dependency, lock, synchronous
release metadata, and its own evidence only. Desktop `g02.109` is serial
behind the produced `v0.9.10` tag. Desktop, Market, Longhorn, Poodle, and the
consumer fleet remain outside this repository lane.

## Next Task

No further g13 task is ready. Downstream adoption belongs to Desktop g02.109
and the owning consumer lanes.
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
