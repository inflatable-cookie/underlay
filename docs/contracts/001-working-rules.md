# 001 - Working Rules

Status: active
Owner: repo maintainers
Depends on: `docs/architecture/product-guardrails.md`

## Contract

- Treat `docs/roadmaps/`, `docs/specs/`, and `docs/logs/` as the execution
  authority chain for active Underlay work.
- Use the strict spec lane when a consumer-facing shared-surface wave needs a
  tighter boundary than the roadmap alone.
- In the strict lane, a bare `continue` should resolve through the previous
  closeout's `Next Task`, normally into the current ready card.
- Completed cards must not remain advertised as ready.
- If there is no ready card, re-enter planning instead of widening the package
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
- batch cards stay with their generation under `docs/roadmaps/gNN/batch-cards/`
- each generation README remains the authoritative front door for its thread

## Current Posture

Underlay is in parallel mode:

- `g06` owns the Rust platform-contract transition thread
- `g03`, `g04`, and `g05` are closed historical generations
- there is no active strict master spec at the repo level right now

## Next Task

Execute `g06.015` and open a new spec only if the Rust platform-contract lane
needs provisional planning beyond its roadmap and contracts.
