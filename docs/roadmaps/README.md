# Roadmaps

Roadmaps are executable delivery plans for Underlay library work.

## Rules

- Keep one active queue per generation and use backlog for deferred scope.
- In sequential mode, maintain at most one active generation. A deliberate
  between-generation pause may have none.
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

- `gNN/README.md` generation front door
- `gNN/NNN-<slug>.md` executable roadmap files
- `gNN/batch-cards/` optional strict-spec decomposition; batch cards may refine
  a roadmap but never replace the generation roadmap queue
- `generation-index.md` active generation mode and history
- `backlog/` deferred items with promotion criteria

## Evidence Boundary

Roadmap bodies are execution records as well as plans. They may retain
sibling-repo file references, local path evidence, and concrete caller
inventories when that is necessary to preserve delivery history. Do not treat
that historical evidence style as the model for active library-facing guides or
README surfaces.

## Mode

- `sequential`

## Active generation

- [`g11`](g11/README.md) — immutable verified media publication and
  five-consumer rollout; `v0.9.7` is released and consumer lanes are
  unblocked.

## Current Queue

`g11.001` Cards 001–004 are complete and `v0.9.7` is released at `8a7ce84b`.
Affected consumer lanes are unblocked and run as independent repository lanes.

## Archived generations

Closed generations `g01`-`g10` are **frozen archival record**, not active
navigation. Enter them only via
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

Resume Underlay Reference on `v0.9.7`, then route the other consumer ownership
upgrades as independent repository lanes.
