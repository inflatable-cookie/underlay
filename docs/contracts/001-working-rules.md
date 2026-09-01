# 001 - Working Rules

Status: active
Owner: repo maintainers
Depends on: `docs/architecture/product-guardrails.md`

## Contract

- Treat `docs/roadmaps/`, `docs/specs/`, and `docs/logs/` as the execution
  authority chain for active Underlay work.
- Every generation work item must exist as a numbered roadmap file directly
  under `docs/roadmaps/gNN/`.
- Use the strict spec lane when a consumer-facing shared-surface wave needs a
  tighter boundary than the roadmap alone.
- Strict batch cards may decompose one roadmap; they must not substitute for the
  roadmap queue or advance generation numbering independently.
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
  superseded, or moved to backlog
- the roadmap front doors must reflect that closed state before the next
  generation opens
- stale specs from the closing generation must be archived or removed from
  `docs/specs/`

In parallel mode:

- multiple active generations may coexist when the work streams are genuinely
  independent
- each generation operates as its own queue
- roadmap files stay directly under `docs/roadmaps/gNN/`; optional batch cards
  stay under `docs/roadmaps/gNN/batch-cards/`
- each generation README remains the authoritative front door for its thread

## Current Posture

Underlay is in sequential mode:

- `g01`–`g09` are closed historical generations
- `g10` is a bounded maintenance generation for the explicit Northstar
  instruction and language-quality audit
- `g10.001` and its strict card 001 are the sole ready lane
- the completed monorepo rollout spec remains archived
- open triage and backlog files are retained evidence, not execution authority
- a later generation requires explicit planning and numbered roadmaps before
  dispatch

## Next Task

Execute `g10.001` from card 001 and stop at its PR for orchestrator review. Do
not reopen `g09` or widen the audit into consumer work.
