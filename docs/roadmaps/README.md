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

- `g08` - Audit Remediation And Edge Hardening ([front door](g08/README.md))

## Current Queue

`g08` runs five lanes; Lane A (security) leads. Full card detail lives in
[`g08/README.md`](g08/README.md).

- Lane A - Security and edge hardening: `g08.001`-`g08.010` (complete)
- Lane B - Correctness bugs and test gate: `g08.011`-`g08.014` (complete)
- Lane C - Rust structural seams: `g08.015`-`g08.020` (complete)
- Lane D - TypeScript surface and SSR safety: `g08.021`-`g08.024` (complete)
- Lane E - Docs, versioning, and i18n posture: `g08.025`-`g08.030` (complete)

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

**`g08` is fully complete — all 32 cards done across all five lanes.** Lanes A
(security) and B (correctness/test gate) closed with the six-consumer rollout;
Lane C (Rust seams) closed including `g08.019`, unblocked by making `TestDb` run
against an external `UNDERLAY_TEST_DATABASE_URL` (17 adapter integration tests
green on Postgres 16 via effigy containerd); Lane D (TS surface) complete with
the `g08.022`/`g08.023` collapses deferred under their stop conditions; Lane E
(docs/versioning/i18n) complete. underlay is `0.8.0`, `v0.8.0` tagged at the
six-consumer proof point. Next: `g09` scoping (maintainer direction). Ops
follow-up (not a g08 card): wire `UNDERLAY_TEST_DATABASE_URL` into real CI, and
reconcile the consumer-migration drift the adapter fixtures surfaced.
