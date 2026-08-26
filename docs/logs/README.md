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

`g09.021` is complete and merged
([execution log](./2026-08/25-181500-g09-021-monorepo-contract-authority.md)).
`g09.022` is complete and merged
([execution log](./2026-08/25-190914-g09-022-active-guide-normalization.md)).
`g09.023` is complete and merged
([execution log](./2026-08/25-203725-g09-023-workspace-shape-conformance.md)).
`g09.024` is complete and merged
([execution log](./2026-08/25-232150-g09-024-acowtancy-evidence-repair.md)).
`g09.025` is complete and merged
([execution log](./2026-08/26-110905-g09-025-underlay-reference-normalization.md)).
`g09.026`–`g09.030` are complete
([fleet closeout](./2026-08/26-151525-g09-026-030-fleet-closeout.md)).
`g09.031` is complete
([assessment](./2026-08/26-153051-g09-031-foundation-transport-assessment.md)).
`g09.032`–`g09.034` are complete. `g09.035`–`g09.036` are compiled
([planning log](./2026-08/26-161417-g09-035-036-migration-testing-assessment-compiled.md)).
`g09.035` is complete
([assessment](./2026-08/26-162845-g09-035-database-migration-assessment.md)).
`g09.036` is complete
([assessment](./2026-08/26-164407-g09-036-testing-posture-assessment.md)).
`g09.037`–`g09.044` are compiled
([planning log](./2026-08/26-165722-g09-037-044-migration-testing-repairs-compiled.md));
the invalid generation/card hierarchy was then repaired
([recovery log](./2026-08/26-171952-g09-roadmap-authority-recovery.md)). Execute
`g09.037` without reconstructing closed history.
