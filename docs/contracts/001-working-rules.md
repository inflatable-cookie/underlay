# 001 - Working Rules

Status: active
Owner: repo maintainers
Depends on: `docs/architecture/product-guardrails.md`

## Contract

- Treat `docs/roadmaps/`, `docs/specs/`, and `docs/logs/` as the execution
  authority chain for active Underlay work.
- Every generation work item must exist as one numbered Northstar task file
  directly under `docs/roadmaps/gNN/` (`NNN-<slug>.md`, referenced as
  `gNN.NNN`). The generation README owns the roadmap and approved frontier.
- Use the strict spec lane when a consumer-facing shared-surface wave needs a
  tighter boundary than the task alone.
- No milestone wrapper or nested `batch-cards/` hierarchy is supported; a
  former milestone and its cards collapse into top-level tasks with later IDs
  assigned in dependency order.
- In the strict lane, a bare `continue` should resolve through the previous
  closeout's `Next Task`, normally into the current ready roadmap.
- Completed roadmaps must not remain advertised as ready.
- If there is no ready roadmap, re-enter planning instead of widening the package
  migration by implication.

## Generation Rule

Treat roadmap generations as substantial sequencing eras, not tiny buckets. In
a long-running repo, expect roughly 20 to 40 roadmap files in one generation
before rollover is even worth discussing.

In sequential mode:

- every roadmap in the old generation must be explicitly closed, paused,
  superseded, or moved to triage as a non-authoritative candidate
- the roadmap front doors must reflect that closed state before the next
  generation opens
- stale specs from the closing generation must be archived or removed from
  `docs/specs/`

In parallel mode:

- multiple active generations may coexist when the work streams are genuinely
  independent
- each generation operates as its own queue
- roadmap files stay directly under `docs/roadmaps/gNN/` as the sole
  executable planning units
- each generation README owns that generation's roadmap and approved frontier

## Current Posture

Underlay runs parallel active generations:

- `g01`–`g10` are compacted archival roll-ups under `docs/roadmaps/archive/`
- `g11` is active: `g11.001` complete (`v0.9.7`), `g11.002` ready
  (five-consumer adoption), `g11.003` blocked (fleet closeout)
- `g12` is active: `g12.001` owns the Nightfire extraction and rollout
- the completed monorepo rollout spec remains archived
- open triage notes are retained evidence, not execution authority
- a later generation requires explicit planning and numbered tasks before
  dispatch

## Next Task

Run `g11.002` (Underlay Reference first) and the `g12.001` Card 278 lane.
