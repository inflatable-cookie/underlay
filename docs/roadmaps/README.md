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

- `g09` - Config Convergence And Contract Fidelity ([front door](g09/README.md))

## Current Queue

`g09.001`–`g09.020` completed config convergence, dependency upgrades, and
elective majors. `g09.021`–`g09.030` completed the strict monorepo
consumer-workspace rollout
across the six-consumer family. `g09.031` completed the foundation and transport
assessment. `g09.032` normalized request-context rejection envelopes, and
`g09.033` synchronized the page-list contract artifacts. `g09.034` completed the
bounded HTTP-client fallback repair. `g09.035` and `g09.036` completed the
migration/testing assessments with `drifting` verdicts. `g09.037` completed the
shared repair and `g09.038` completed the Underlay Reference proof. `g09.039`
completed the Contact Patch rollout and `g09.040`–`g09.043` completed the
remaining consumer repairs. `g09.044` closed the migration/testing proof.
`g09.045` completed the bootstrap/runtime/access assessment with a `drifting`
verdict. `g09.046` is complete. `g09.047` completed the Underlay Reference
runtime/access proof in PR5, merge commit `6af27837`. Its merge clears the
shared reference dependency for `g09.048`–`g09.052`. Exact-main evidence and
the remaining product/security policy were settled on 2026-08-27, so all five
independent consumer lanes were dispatched through target-owned handoffs.
Contact Patch PR5, Compli Me PR7, Songsprout PR5, Composer PR5, and Acowtancy
PR62 have merged, completing `g09.048`–`g09.052`. Underlay Reference PR6 merged
as `f89e3616`, completing `g09.053`. The first `g09.054` exact-root pass proved
workspace/env authority but found an Acowtancy FAQ JSON-LD script-breakout
risk. `g09.055` repaired it in Acowtancy PR63. The resumed pass then found the
merged SSR regression was not portable; `g09.056` repaired that in PR65.
`g09.054` is complete and `g09.057` is the ready next assessment.
See
[`g09/README.md`](g09/README.md).

## Archived generations

Closed generations `g01`-`g08` are **frozen archival record**, not active
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

Execute ready read-only assessment `g09.057` for contracts `027`–`029`.
