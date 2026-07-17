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
- Lane C - Rust structural seams: `g08.015`-`g08.020` (all done except `g08.019` postgres integration tests)
- Lane D - TypeScript surface and SSR safety: `g08.021`-`g08.024` (complete)
- Lane E - Docs, versioning, and i18n posture: `g08.025`-`g08.030`
  (`g08.029` paused on an i18n contract decision)

Closed generations `g01`-`g07` are archival; see each generation README and
[`generation-index.md`](generation-index.md).

## Next Task

Lane A (security, `g08.001`-`g08.010`) and Lane B (correctness/test gate) are
complete; the Lane A checkpoint is closed (no new contract; six consumer apps
scanned and fixed). Lane C done except `g08.019` (blocked on Docker/Postgres).
Lane D complete (`g08.021`-`g08.024`; `g08.022` broad subpath collapse and
`g08.023` presentation file-split both deferred under their stop conditions).
Lane E in progress: `g08.025` front-door doc repair and `g08.026`
committed-artifact cleanup done. Next: `g08.027` contract-sync decision.
