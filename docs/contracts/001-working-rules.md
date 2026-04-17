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

## Generation Rollover Rule

Treat roadmap generations as substantial sequencing eras, not tiny buckets. In
a long-running repo, expect roughly 20 to 40 roadmap files in one generation
before rollover is even worth discussing.

Treat rollover as full closeout:

- every roadmap in the old generation must be explicitly closed, paused,
  superseded, or moved to backlog
- the roadmap front doors must reflect that closed state before the next
  generation opens
- stale specs and batch cards from the closing generation must be archived or
  removed from `docs/specs/`

If those closeout conditions are not satisfied, repair the current generation
instead of opening a new one.

## Current Posture

Underlay is in a strict-ready posture around the active `g02.007`
package-consolidation lane.

## Next Task

Execute the active strict lane in `docs/specs/001-g02-package-consolidation-strict-lane.md`.
