# g06.055 Artifact - Jobs Public Model Modularity Audit

## Summary

`underlay-jobs/src/types.rs` is broad app-facing contract surface, but it has a
safe internal split shape if the public front doors stay stable.

The file currently groups:

- identifiers: `JobId`, `DeadLetterId`
- status and config: `JobStatus`, `JobConfig`, backoff constants
- retry timing: `BackoffStrategy`, `BackoffJitter`, deterministic jitter helper
- progress and persisted rows: `JobProgress`, `Job`
- dead-letter models: `DeadLetter`, `DeadLetterFilters`, `JobErrorRecord`,
  `JobFailureOutcome`
- scheduled task models: `ScheduledTask`, `ScheduledTaskDefinition`
- filters: `JobFilters`
- handler result/error/trait: `JobResult`, `JobHandlerError`, `JobHandler`

## Consumer Evidence

The six-consumer scan found crate-root imports such as:

- `underlay_jobs::{Job, JobConfig, JobHandler, JobHandlerError}`
- `underlay_jobs::{Job, JobFilters, JobStatus}`
- `underlay_jobs::JobConfig`

No current consumer source import requires a new public submodule path.

## Decision

Queue `g06.056` as a jobs types internal split.

The split should preserve:

- `underlay_jobs` root exports
- `underlay_jobs::types::*` compatibility
- `JobHandler`, `JobStore`, and `DeadLetterStore` signatures
- serialized shapes for `Job`, `DeadLetter`, `ScheduledTask`, `JobProgress`,
  and `JobErrorRecord`
- retry/backoff behavior, including deterministic jitter

## Public API Impact

Expected impact: none.

This should be a private module split only. If a trait signature, serialized
shape, or public import path must change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-jobs --all-features`
- `cargo test -p underlay-jobs-postgres --all-features`
- `effigy qa:docs`
- `effigy qa:northstar`

Next code batch validation:

- `cargo test -p underlay-jobs --all-features`
- `cargo test -p underlay-jobs-postgres --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
