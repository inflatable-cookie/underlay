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

- Use generation folders such as `docs/roadmaps/g11/`.
- Task filenames use `NNN-<slug>.md` and are referenced as `gNN.NNN`.
- The generation README owns the roadmap and approved frontier;
  `docs/roadmaps/gNN/NNN-<slug>.md` is the sole executable planning unit.
- No milestone wrapper or nested `batch-cards/` hierarchy is supported.
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

- `gNN/README.md` generation front door: roadmap and approved frontier
- `gNN/NNN-<slug>.md` executable Northstar task files, one per outcome
- `archive/gNN.md` non-procedural roll-ups for compacted closed generations
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

- [`g11`](g11/README.md) — immutable verified media publication and
  five-consumer rollout; `v0.9.7` is released and consumer lanes are
  unblocked.
- [`g12`](g12/README.md) — standalone Rust and TypeScript/Svelte Nightfire
  extraction with direct consumer adoption.

## Current Queue

`g11.001` is complete and `v0.9.7` is released at `8a7ce84b`. Ready:
`g11.002` five-consumer adoption, Underlay Reference first, as independent
repository lanes. Blocked: `g11.003` fleet closeout until those lanes merge.

`g12.001` is operator-confirmed. Card 272 closed incomplete after excluding
Rust; Acowtancy Market Card 278 is the ready corrective lane. The release and
consumer lanes remain serial behind its accepted dual-language proof.

## Archived generations

Closed generations `g01`-`g10` are compacted non-procedural roll-ups under
[`archive/`](archive/README.md), not active navigation. Enter them only via
[`generation-index.md`](generation-index.md), which is the authoritative
history. Do not open new work in a closed generation. Current contract
evidence links resolve to the matching roll-up; git history remains the
full-fidelity archive.

## Next Task

Run `g11.002` (Underlay Reference first) while Market Card 278 runs under
`g12.001`.
