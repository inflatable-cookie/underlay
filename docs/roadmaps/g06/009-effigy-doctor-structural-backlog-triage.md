# g06.009 - Effigy Doctor Structural Backlog Triage

## Why

`g06.008` closed the consumer compatibility proof for the Rust platform
contract changes.

The remaining repo-owned signal is structural. `effigy doctor` still reports
configuration and scan backlog that should be classified before this generation
continues into more Rust refactors.

## Goal

Turn the current `effigy doctor` failures into a bounded backlog and identify
which findings, if any, belong in the next Rust platform-contract repair batch.

## Scope

In scope:

- inspect the current `effigy doctor` output
- decide whether the unsupported `isolation` key is a quick config fix or a
  deferred Effigy compatibility item
- inventory attention-marker, comment-ratio, and god-file findings
- classify each finding as current-lane, deferred backlog, or unrelated
  historical debt
- prepare the next g06 card or closeout decision from that classification

Out of scope:

- broad file splits without a scoped acceptance target
- rewriting historical docs just to satisfy scanners
- release execution or publishing
- consumer-repo cleanup unrelated to the g06 Rust platform-contract lane

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `120`: tooling, testing, and contract artifacts
- `122`: Rust public API inventory

## Acceptance Criteria

- current `effigy doctor` findings are recorded
- unsupported Effigy config key handling is decided
- Rust god-file findings are triaged into actionable next work or deferred
  backlog
- stale-marker and comment-ratio findings are classified without hiding them
- the next g06 move is explicit

## Findings

`effigy doctor` was rerun after removing the stale unsupported `[isolation]`
key from `effigy.toml`.

Current remaining failures:

- `scan.attention-markers`: 11 findings, including two error-level findings
- `scan.comment-ratio`: 12 findings, including three error-level findings
- `scan.god-files`: 60 findings, including 22 error-level findings

The manifest issue is closed. The current Effigy schema does not support
`[isolation]`, so the old `node_modules` / `target` entry was removed instead
of rehomed.

## Triage

| Finding family | Classification | Decision |
| --- | --- | --- |
| `scan.god-files` Rust critical files | current-lane | Start with the largest Rust platform files: `underlay-devtools/src/bin/underlay-devtools.rs`, `underlay-migration-core/src/pipeline.rs`, and the related migration-core test mass. These map directly to `120` and `122`. |
| `scan.god-files` Rust high files | deferred backlog | Auth, blob, jobs, media, and HTTP high files are real structural debt, but they should follow the first split batch rather than widen it. |
| `scan.god-files` TS files | deferred non-g06 backlog | The active generation is Rust platform-contract work. TS/template files should move under a separate TS/UI structural lane. |
| `scan.attention-markers` Rust findings | mixed | The `// Security` enum grouping is a scanner false-positive. Test-environment and cleanup notes are low-risk commentary cleanup. |
| `scan.attention-markers` TS deprecation finding | intentional | The `@deprecated` navigation helper marker is public API guidance, not stale release debt. Keep it until the TS runtime lane retires the surface. |
| `scan.comment-ratio` Rust findings | deferred cleanup | `underlay-ratelimit`, `underlay-email`, `underlay-http`, `underlay-blob`, and `underlay-jobs` comment-heavy files should be trimmed only when their modules are reopened. |

## Validation

- `effigy doctor` now has no manifest-schema failure.
- Scanner failures remain and are classified above.

## Current State

`g06.009` is complete.

## Next Task

Execute `g06.010`: first Rust god-file split repair batch.
