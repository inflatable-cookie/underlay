# Logs

Logs capture meaningful documentation and delivery batches for Underlay.

## Current Evidence Window

- `2026-03/23-153502-poodle-field-cluster-review-handoff.md`
- `2026-03/24-084409-poodle-list-container-review-handoff.md`
- `2026-04/09-221225-poodle-era-overhaul-recovery-and-g01-098-open.md`
- `2026-04/09-221400-roll-to-g02-consumer-normalization-era.md`

The March Poodle handoffs remain part of the lineage, but the new April log is
the active control chain for the current shared-surface normalization queue and
its rollover into `g02`.

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

Keep the active evidence window aligned to `g02.001`, adding only the logs
needed to reach the live next task without reconstructing the queue from the
older contraction-era history.
