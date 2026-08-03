# Roadmaps

Roadmaps are executable delivery plans for Underlay library work.

## Rules

- Keep one active queue per generation and use backlog for deferred scope.
- In sequential mode, maintain one active generation.
- In parallel mode, keep every active generation front door accurate for its
  thread.
- Keep durable inventories, CSVs, and machine-readable reference artifacts in
  [../contracts/](../contracts/).
- If active work changes consumer-visible behavior, APIs, configuration,
  migrations, or integration patterns, include a `Consumer Upgrade Impact`
  section in the roadmap.

## Generation model

- Use generation folders such as `docs/roadmaps/g01/`.
- Roadmap filenames use `NNN-<slug>.md`.
- Roadmap references use generation-qualified IDs such as `g01.021`.
- Generation rollover is manual only.
- Treat generations as substantial sequencing eras, not one-or-two-file
  buckets. A healthy default is roughly 20 to 40 roadmap files in one
  generation before rollover is even worth discussing.
- In sequential mode, close or rehome every roadmap in the current generation
  and purge stale specs from `docs/specs/` before opening the next generation.
- In parallel mode, multiple active generations may coexist when the work
  streams are genuinely independent. Each generation then operates as its own
  queue.

## Layout

- `gNN/` generation milestones
- `gNN/batch-cards/` generation-local execution cards when that generation uses
  strict posture
- `generation-index.md` active generation mode and history
- `backlog/` deferred items with promotion criteria

## Evidence Boundary

Roadmap bodies are execution records as well as plans. They may retain
sibling-repo file references, local path evidence, and concrete caller
inventories when that is necessary to preserve delivery history. Do not treat
that historical evidence style as the model for active library-facing guides or
README surfaces.

## Mode

- `parallel`

## Active generation

- `g09` - Config Convergence Follow-Through ([front door](g09/README.md))

## Current Queue

`g08` (complete) acted on the July 2026 deep audit. `g09` carries the
follow-through from the 2026-08-03 config-convergence self-audit. Card
detail lives in [`g09/README.md`](g09/README.md).

- `g09.001`-`g09.003` — small real gaps (silent prod CORS, legacy env-var
  signal, operator `local.toml` note)
- `g09.004`-`g09.007` — dead code and remaining duplication
- `g09.008` — config model front-door guide
- `g09.009`-`g09.012` — variants to converge or deliberately park

## Archived generations

Closed generations `g01`-`g07` (639 roadmap files; `g06` alone is 367) are
**frozen archival record**, not active navigation. Enter them only via
[`generation-index.md`](generation-index.md), which is the authoritative history.
Do not open new work in a closed generation.

They remain in place under `docs/roadmaps/g0N/` rather than being physically
relocated: ~15 active contracts/guides link into them and the closed generations
cross-link each other with relative paths, so a bulk move would break links that
the front-door link check (nine fixed files) cannot detect. Physical relocation
to an archive surface is deferred to a human-reviewed pass; the frozen-record
designation and the generation-index give the same navigational benefit without
that risk.

## Next Task

**`g09.001`** — prod-empty-origins boot warning. `g08` is fully complete
(all 32 cards done across all five lanes); the config convergence (2026-08)
landed and is recorded in `docs/logs/2026-08/03-104132-config-convergence.md`.
`g09` scopes the self-audit follow-through; see its
[front door](g09/README.md) for the queue.
