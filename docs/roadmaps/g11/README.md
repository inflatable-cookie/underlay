# g11 - Immutable Verified Media Publication

Owner: repo maintainers
Started: 2026-09-02

## Current Generation

`g11` owns the shared repair and five-consumer rollout for media
finalisation. Contact Patch Bughunt Card 015 proved that the current
split `get_bytes` and `finalise_upload`/`head` surface cannot bind
inspected bytes to a later ready/current database transition. The
operator confirmed the invariant applies to Underlay Reference, Contact
Patch, Compli Me, Acowtancy, and Songsprout.

This README owns the `g11` roadmap and approved frontier. The
`g11.NNN` files below are the sole executable planning units; there is
no milestone wrapper or nested `batch-cards/` hierarchy.

## Task Sequence

1. [x] [`g11.001`](001-immutable-verified-media-publication.md) —
   shared primitive, owned recovery, `v0.9.6`/`v0.9.7` releases
   (`complete`)
2. [ ] [`g11.002`](002-five-consumer-verified-media-adoption.md) —
   five-consumer adoption from `v0.9.7` (`ready`)
3. [ ] [`g11.003`](003-verified-media-fleet-closeout.md) — fleet
   proof and closeout (`blocked` on `g11.002`)

Old-to-new map: the former `g11.001` milestone wrapper and its
completed batch cards 001–004 collapsed into the completed `g11.001`;
remaining consumer adoption became `g11.002`; fleet closeout became
`g11.003`. Full map:
`docs/logs/2026-09/09-160000-flattened-task-switchover-manifest.md`.

## Approved Frontier

Ready: `g11.002` (Underlay Reference first, then the remaining
consumer lanes independently). Blocked: `g11.003` until all applicable
consumer lanes merge.

## Dependencies And Parallelism

The shared implementation and release are done and serial behind this
generation. Consumer adoption starts only from a validated released
tag, which exists (`v0.9.7` at `8a7ce84b`). Independent consumer
repositories may run in parallel, subject to their own shared-authority
and same-repo merge ordering. Contact Patch Card 015 resumes on its
retained agent/workspace rather than creating a duplicate lane. `g11`
is independent of `g12`.

## Next Task

Run `g11.002`, Underlay Reference first.
<!-- northstar:lifecycle:begin schema=northstar.lifecycle.projection.v2 digest=sha256:fddca8ff83f90d187146e5dd01b686737e9c00b378f24d20c1450bb1721a9664 -->
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
| g13.001 | complete | none | 8 | sha256:e3c2120126b98eab06603171a71aa6eb247d7b5083e3b337ac300c3660b2a8b3 |
| g13.002 | complete | none | 8 | sha256:27ebe07858d9d9e19b4774e67cdb633762cf2744a672ef74c99cdae746ff499a |
<!-- northstar:lifecycle:end -->
