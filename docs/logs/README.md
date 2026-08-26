# Logs

Logs capture meaningful documentation and delivery batches for Underlay.

## Current Evidence Window

The active window is the current generation `g10` under `docs/logs/2026-08/`.
The `g09` config-convergence and dependency-upgrade closeout lives in the same
month shard. Earlier month shards are frozen lineage.

## Cadence

Decided (g08.030): **keep the per-batch log cadence.** One log per meaningful
card or batch, under month shards. It served the g08 generation well as a live
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

`g10.002` is complete and merged
([execution log](./2026-08/25-190914-g10-002-active-guide-normalization.md)).
`g10.003` is complete and merged
([execution log](./2026-08/25-203725-g10-003-workspace-shape-conformance.md)).
`g10.004` is complete and merged
([execution log](./2026-08/25-232150-g10-004-acowtancy-evidence-repair.md)).
`g10.005` is complete and merged
([execution log](./2026-08/26-110905-g10-005-underlay-reference-normalization.md)).
`g10.006`–`g10.010` are complete
([fleet closeout](./2026-08/26-151525-g10-006-010-fleet-closeout.md)).
`g10.011` is complete
([assessment](./2026-08/26-153051-g10-011-foundation-transport-assessment.md)).
`g10.012`–`g10.014` are complete. `g10.015`–`g10.016` are compiled
([planning log](./2026-08/26-161417-g10-015-016-migration-testing-assessment-compiled.md)).
Execute `g10.015`; keep the active evidence window aligned to `g10` without
reconstructing closed history.
