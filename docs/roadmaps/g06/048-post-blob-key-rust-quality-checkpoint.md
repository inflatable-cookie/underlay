# g06.048 - Post-Blob-Key Rust Quality Checkpoint

## Why

`g06.038` through `g06.047` moved object-key construction, adapter convenience
methods, shared media rows, app-local media rows, rendition generation, and the
one live non-media consumer path onto validated `BlobObjectKey` boundaries.

Before starting another structural Rust lane, Underlay needs a checkpoint that
re-audits the current Rust architecture with this work included.

## Goal

Re-run the Rust quality audit from the current codebase state and choose the
next reference-grade architecture batch.

## Scope

In scope:

- audit current Underlay Rust crates for modularity, public API shape, security
  boundaries, and extensibility
- include the impact of the object-key lane in the assessment
- inspect Effigy doctor structural backlog without treating known backlog as a
  blocker by default
- identify the next meaningful batch, not a micro-fix
- update roadmap/contracts if the next batch changes public Rust direction

Out of scope:

- release execution or publishing
- broad TypeScript/Svelte work
- reverting unrelated local worktree changes
- changing consumer apps unless the checkpoint finds a consumer-visible Rust
  boundary that must move immediately

## Acceptance Criteria

- current Rust quality state is documented
- next architecture batch is chosen with rationale
- known structural backlog is separated from newly introduced risks
- validation surface for the next batch is explicit

## Consumer Upgrade Impact

Expected impact: planning/audit only unless the checkpoint promotes a
consumer-visible Rust change.

## Current State

`g06.048` is next after `g06.047`.

## Next Task

Execute `g06.048`: post-blob-key Rust quality checkpoint.
