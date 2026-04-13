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

## Current Posture

Underlay is in a strict-ready posture around the active `g02.007`
package-consolidation lane.

## Next Task

Execute the active strict lane in `docs/specs/001-g02-package-consolidation-strict-lane.md`.
