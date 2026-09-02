# Logs

Logs capture meaningful documentation and delivery batches for Underlay.

## Current Evidence Window

The latest evidence window is `g11` under `docs/logs/2026-09/`. Card 003's
owned-promotion recovery merged as PR #25, with closeout in
`2026-09/02-230000-g11-003-owned-promotion-closeout.md`. Card 002 published
`v0.9.6`, with release evidence in
`2026-09/02-194057-g11-002-underlay-v0-9-6-release.md`. Card 004 is the ready,
explicitly authorized `v0.9.7` release. Closed `g10`, `g09`, and earlier
generation shards are frozen lineage.

## Cadence

Decided (g08.030): **keep the per-batch log cadence.** One log per meaningful
roadmap or batch, under month shards. It served the g08 generation well as a live
control chain and evidence trail. Gap months (e.g. 2026-05 empty, 2026-06 one
file) reflect genuinely low-activity periods, not a reason to drop the ritual —
an empty month is honest signal, not overhead.

## Rules

- Store logs under month shards such as `docs/logs/2026-03/`.
- Use filenames in `DD-HHMMSS-<slug>.md` format.
- Record one log per meaningful update cycle or batch.
- Do not create a separate log for every tiny task.
- When work is driven by a roadmap, include the roadmap ID in the log body.
- If the batch changes consumer-visible behavior, APIs, configuration, migrations, or integration patterns, include a `Consumer Upgrade Notes` block using the compatibility template or link the fuller upgrade note.

## Historical Evidence Boundary

Logs are archival records. They may preserve:

- raw local filesystem paths
- sibling-repo file references
- exact shell commands captured during the batch

Keep that evidence intact when it is part of the historical record. Do not use
log formatting as the model for active guide, architecture, or README content,
where repo-local links and normalized prose references are required instead.

## Next Task

`g09.047` completed the Underlay Reference proof
([closeout](./2026-08/27-122210-g09-047-reference-proof-closeout.md)).
`g09.052` completed in Acowtancy PR62
([closeout](./2026-08/27-143321-g09-052-acowtancy-closeout.md)); the other four
consumer lanes then merged
([closeout](./2026-08/27-143842-g09-048-051-consumer-closeout.md)). The planning
repair promoted the Reference CSRF fix as `g09.053` and renumbered fleet
closeout to `g09.054`
([log](./2026-08/27-145100-g09-053-054-planning-repair.md)). The target handoff
produced Underlay Reference PR6, which merged as `f89e3616`
([closeout](./2026-08/27-155724-g09-053-reference-csrf-closeout.md)). The first
fleet-closeout pass then found a real Acowtancy FAQ JSON-LD script-breakout
risk and paused `g09.054`
([log](./2026-08/27-160408-g09-054-partial-fleet-proof.md)). Acowtancy PR63 and
PR65 closed the product and regression defects. The exact-head fleet proof is
now complete
([closeout](./2026-08/27-174415-g09-054-bootstrap-runtime-access-fleet-closeout.md)).
`g09.057` then assessed contracts `027`–`029`
([assessment](./2026-08/27-175930-g09-057-canonical-path-runtime-workflow-assessment.md)).
Runtime maturity conforms. The operator authorised closed-world retirement with
no compatibility windows and canonical `:batch-delete`
([promotion](./2026-08/27-181454-g09-058-059-route-retirement-promotion.md));
all five target-owned workers were
[dispatched](./2026-08/27-182502-g09-058-059-route-retirement-dispatch.md).
The first four merges were recorded in the
[partial closeout](./2026-08/27-201331-g09-058-partial-059-closeout.md).
Acowtancy PR67 then merged, completing both roadmaps and the route-retirement
phase
([fleet closeout](./2026-08/27-203800-g09-058-059-route-retirement-closeout.md)).
The operator then chose to continue `g09`, promoting the Contract `023`
released-dependency drift as ready `g09.060`
([promotion](./2026-08/27-205758-g09-060-released-dependency-promotion.md)).
The concurrent papercuts wave 3 handoff overlaps Contract `023`, so `g09.060`
was serialized behind that closeout before dispatch
([collision gate](./2026-08/27-210231-g09-060-contract-link-collision-gate.md)).
Underlay PR12 merged its reviewed head as `9e26ba9a`, closing papercuts wave 3
and promoting `g09.060` to ready
([closeout](./2026-08/27-214114-papercuts-wave3-closeout-and-g09-060-promotion.md)).
The serial `g09.060` worker was then published from exact planning base
`ec67dfbf`
([dispatch](./2026-08/27-214255-g09-060-dispatch.md)).
The worker rewrote Contract `023` onto released Git-tag pins and set the
roadmap in review
([execution](./2026-08/27-215648-g09-060-released-dependency-contract-normalization.md)).
Underlay PR13 merged at its reviewed head, completing `g09.060` and leaving
`g09` paused with no ready roadmap
([closeout](./2026-08/27-222835-g09-060-released-dependency-contract-closeout.md)).
The operator then chose a green-doctor finish line and promoted the two error
families into parallel `g09.061` and `g09.062` roadmaps
([promotion](./2026-08/27-223823-g09-061-062-doctor-error-promotion.md)).
Both independent worker handoffs were then published from exact planning base
`049fae4d`
([dispatch](./2026-08/27-224244-g09-061-062-doctor-error-dispatch.md)).
The workers completed the attention-marker policy and workspace-shape split in
Underlay PR14 and PR15. Both reviewed heads merged, exact `main` doctor is green,
and `g09.061`/`g09.062` are complete
([closeout](./2026-08/27-232400-g09-061-062-doctor-error-closeout.md)).
The closure audit then confirmed all 62 numbered roadmaps complete, no active
strict spec, and no batch-card queue. `g09` is now frozen with no successor
generation opened
([generation closeout](./2026-08/27-233954-g09-generation-closeout.md)).
Papercuts wave 4 then landed merge closeout and retired-path inventory tooling
([log](./2026-08/28-085000-papercuts-wave4-merge-closeout.md)).

## Next Task

Card 003 merged as PR #25. Execute the explicitly approved Card 004 release for
`v0.9.7` against the exact pushed candidate.
