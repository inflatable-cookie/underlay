# g06.142 - Rust Doctor Marker Cleanup

## Why

`g06.141` confirmed that Rust god-file findings are gone, but Rust still
contributes attention-marker and comment-ratio findings to `effigy doctor`.

The next Rust batch should remove false-positive or stale doctor findings
without changing public APIs or behavior.

## Goal

Clean up Rust doctor marker findings where the fix improves scan signal and
does not weaken useful API documentation.

## Scope

In scope:

- inspect Rust entries in `scan.attention-markers`
- inspect Rust entries in `scan.comment-ratio`
- remove or rewrite stale/false-positive comments
- preserve useful public API documentation
- rerun `effigy doctor` and Rust validation

Out of scope:

- TypeScript doctor findings
- behavior changes
- public API changes
- consumer app updates

## Acceptance Criteria

- Rust attention-marker findings are reduced or explicitly justified
- Rust comment-ratio findings are reduced or explicitly justified
- no public Rust API behavior changes
- `effigy rust:check` passes
- targeted Rust tests pass where touched
- roadmap artifact records the remaining doctor state

## Consumer Upgrade Impact

Expected impact: none.

This should be comment/docs cleanup only. If behavior needs to change, stop and
re-enter planning.

## Current State

`g06.142` is ready.

## Next Task

Execute `g06.142`: Rust doctor marker cleanup.
