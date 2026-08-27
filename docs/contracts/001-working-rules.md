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

- `g07` and `g08` are closed
- `g09` is the active generation
- `g03`, `g04`, and `g05` are closed historical generations
- no strict spec is active; the completed monorepo rollout spec is archived
- `g09.021` is complete
- `g09.022` is complete
- `g09.023` is complete
- `g09.024` is complete
- `g09.025` is complete
- `g09.026`–`g09.030` are complete
- the monorepo consumer-workspace strict lane is archived
- `g09.031` is complete
- `g09.032` and `g09.033` are complete
- `g09.034` is complete
- `g09.035` is complete
- `g09.036` is complete
- `g09.037` is complete
- `g09.038` is complete
- `g09.039` is complete
- `g09.040`–`g09.043` are complete
- `g09.044` is complete
- `g09.045` is complete with a `drifting` verdict
- `g09.046` is complete
- `g09.047`–`g09.052` are complete
- `g09.053` is complete in Underlay Reference PR6, merge commit `f89e3616`
- `g09.054` is complete with an exact-head six-root conforming verdict
- `g09.055` is complete in Acowtancy PR63, merge commit `ad74d23e`
- `g09.056` is complete in Acowtancy PR65, merge commit `22219f59`
- `g09.057` is complete with a `drifting` verdict
- `g09.058` and `g09.059` are complete across all five affected targets
- papercuts wave 3 is complete in Underlay PR12, merge commit `9e26ba9a`
- `g09.060` is ready and dispatched for the bounded Contract `023`
  released-dependency normalization

## Next Task

Launch the published `g09.060` worker handoff. Review its PR at exact head and
merge only with explicit operator authorisation.
