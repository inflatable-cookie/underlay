# g06.052 - Rust Structural Backlog Checkpoint

## Why

`g06.049` and `g06.051` tightened two high-value structural areas:
devtools migration-bundle boundaries and migration-core pipeline internals.

Effigy doctor still reports structural backlog across attention markers,
comment ratio, and god-files. Before starting another code split, Underlay
needs a fresh checkpoint that uses current doctor output and file-size evidence
to choose the next meaningful batch.

## Goal

Re-run the Rust structural backlog audit from the current codebase and choose
the next reference-grade cleanup batch.

## Scope

In scope:

- inspect current Effigy doctor reports
- separate known backlog from newly introduced findings
- rank remaining Rust god-file pressure by shared-library impact
- classify whether the next batch should target migration-core residuals,
  media/domain breadth, HTTP helpers, devtools reports, or test-only files
- update roadmap/contracts if the next batch changes public Rust direction

Out of scope:

- release execution or publishing
- broad consumer app changes
- changing public APIs without a new rollout card
- reverting unrelated local worktree changes

## Acceptance Criteria

- current Rust structural backlog is documented
- known versus new doctor findings are separated
- next code batch is chosen with rationale
- validation surface for the next batch is explicit

## Consumer Upgrade Impact

Expected impact: planning/audit only.

Any code batch selected from this checkpoint must classify consumer impact
before landing.

## Current State

`g06.052` is next after `g06.051`.

## Next Task

Execute `g06.052`: Rust structural backlog checkpoint.
