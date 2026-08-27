# Logs

Logs capture meaningful documentation and delivery batches for Underlay.

## Current Evidence Window

The active window is the current generation `g09` under `docs/logs/2026-08/`.
The `g09` config-convergence and dependency-upgrade closeout lives in the same
month shard. Earlier month shards are frozen lineage.

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
Execute ready read-only assessment `g09.057` for contracts `027`–`029`.
