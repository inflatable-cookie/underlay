# g13 - Poodle Producer Releases

Owner: repo maintainers
Started: 2026-09-14

## Current Generation

`g13` is the operator-directed producer lane for truthful published Poodle
pins. `g13.001` shipped Underlay `v0.9.9` with Poodle `0.4.1`. Poodle `0.4.2`
then replaced that web package after repairing its published Svelte Tabs
cross-window bridge regression. `g13.002` moves Underlay to that corrected
package and prepares `v0.9.10` without consumer edits.

This README owns the `g13` roadmap and approved frontier. The
`g13.NNN` files below are the sole executable planning units; there is
no milestone wrapper or nested `batch-cards/` hierarchy.

## Task Sequence

1. [x] [`g13.001`](001-poodle-0-4-1-producer-release.md) — Poodle `0.4.1`
   producer release, complete at Underlay `v0.9.9`.
2. [ ] [`g13.002`](002-poodle-0-4-2-producer-release.md) — pin
   `poodle-svelte` exact `0.4.2`, regenerate `bun.lock`, prove the current
   caller surface, and prepare Underlay `v0.9.10` (`ready`).

## Approved Frontier

Ready: `g13.002`. It follows completed `g13.001` and produces the corrected
Underlay tag required by Desktop `g02.109`.

## Dependencies And Parallelism

`g13` remains independent of `g11` media rollout and `g12` Nightfire
extraction. `g13.002` touches the root web dependency, lock, synchronous
release metadata, and its own evidence only. Desktop `g02.109` is serial
behind the produced `v0.9.10` tag. Desktop, Market, Longhorn, Poodle, and the
consumer fleet remain outside this repository lane.

## Next Task

Run `g13.002`.
<!-- northstar:lifecycle:begin schema=northstar.lifecycle.projection.v2 digest=sha256:def280657ebe3b43d4acc3c8b7c3c9e36b2845dc90b4d09bd157f2049fa6f031 -->
| Generation | Disposition | Runway state |
| --- | --- | --- |
| g11 | open | planning_required |
| g12 | open | planning_required |
| g13 | open | planning_required |
| Task | Status | Stage | Revision | Record digest |
| --- | --- | --- | --- | --- |
| g12.002 | complete | none | 8 | sha256:77d3fad9d4fddc202e121350453db1fd50b2da313f4e23ba365f3c4b84ba6a9c |
| g12.003 | complete | none | 8 | sha256:05cbbdf87f96ef842b4e70b8b9a53a96423e20bb5fe64493fc848e35050a5cc5 |
| g13.001 | complete | none | 8 | sha256:e3c2120126b98eab06603171a71aa6eb247d7b5083e3b337ac300c3660b2a8b3 |
<!-- northstar:lifecycle:end -->
