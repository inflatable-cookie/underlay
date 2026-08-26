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

`g09.045` completed the bootstrap/runtime/access assessment
([assessment](./2026-08/26-225903-g09-045-bootstrap-runtime-access-assessment.md)).
`g09.046` is in review
([execution log](./2026-08/26-232742-g09-046-bootstrap-runtime-access-authority.md)).
Keep `g09.047` planned until that merge lands.
