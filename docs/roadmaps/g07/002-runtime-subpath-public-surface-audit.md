# g07.002 - Runtime Subpath Public Surface Audit

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.001` found that `runtime/*` is the main consumer-facing front door for
many retained workflow helpers, while most runtime modules are barrels over
`patterns/*` implementations.

That posture can be correct, but each runtime subpath needs an explicit
classification before any import-path cleanup or consumer rollout.

## Goals

- [x] classify each runtime subpath as stable-domain, compatibility-barrel,
  candidate-split, or candidate-retire
- [x] record the implementation owner behind each runtime subpath
- [x] identify active docs that teach stale runtime or pattern import paths
- [x] decide which runtime import paths are preferred for consumers
- [x] queue only bounded follow-on implementation cards

## Non-Goals

- changing exports
- changing consumer imports
- retiring the root `runtime` convenience barrel
- refactoring workflow controller internals

## Execution Plan

- [x] inspect `runtime/ai`, `runtime/auth`, `runtime/browser`, `runtime/data`,
  `runtime/feedback`, `runtime/forms`, `runtime/media`,
  `runtime/navigation`, and `runtime/relations`
- [x] compare each subpath to contracts `090` and `100`
- [x] scan active guides and source JSDoc examples for stale import advice
- [x] write a runtime-subpath classification artifact

## Acceptance Criteria

- [x] every runtime subpath has a classification and retained/candidate status
- [x] docs drift is separated from source/API drift
- [x] consumer-affecting changes are deferred to rollout-proof cards
- [x] no public export changes are made in this audit card

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- targeted `rg` scans for runtime and pattern import guidance

## Consumer Upgrade Impact

None for this audit card.

Follow-on cards may require consumer import updates if they retire or move a
public subpath.

## Next Task

Move to `g07.003`: runtime import guidance cleanup.
