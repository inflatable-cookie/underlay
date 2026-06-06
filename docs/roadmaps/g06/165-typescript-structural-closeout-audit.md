# g06.165 - TypeScript Structural Closeout Audit

## Why

`g06.160` through `g06.163` cleared the TypeScript source god-file warnings.
`g06.164` accepted the remaining test-file warnings as intentional backlog.

The lane now needs a final closeout pass that records the current structural
state and confirms no source modularity blocker remains.

## Goal

Close the TypeScript structural cleanup lane with current evidence, remaining
warnings, and a clear next-lane recommendation.

## Scope

In scope:

- rerun doctor and docs validation
- confirm remaining `scan.god-files` findings are test-only
- summarize the production source modules split in this lane
- record any residual TypeScript structural risks
- recommend the next reference-grade lane

Out of scope:

- new public API changes
- additional source file splitting unless validation reveals a regression
- consumer-app rollout
- Rust cleanup

## Acceptance Criteria

- final doctor state is recorded
- remaining TypeScript warnings are classified
- consumer impact is stated
- the next lane is named

## Consumer Upgrade Impact

Expected impact: none.

This is an audit and closeout batch.

## Current State

`g06.165` is complete.

Final state:

- production TypeScript source god-file warnings are cleared
- remaining `scan.god-files` findings are 9 warning-only test files
- `g06.164` classifies those warnings as intentional backlog
- public facades stayed stable through the source splits
- no consumer-app upgrade is expected from this closeout batch

Source modules split during this lane:

- `RelationSelector.svelte` internals were split into focused helper modules
- HTTP client types, token-store, and header helpers were split from
  `client/http`
- storage types, availability checks, envelopes, and wrappers were split from
  `patterns/storage`
- server and client pagination controllers were split from
  `patterns/pagination`

Residual risks:

- large test files remain a navigation cost, but not a production source
  boundary risk
- consumer workspaces still need a direct compatibility sweep against the
  current Underlay surface

Recommendation: move next to consumer surface compatibility proof. The source
shape is now clean enough that the useful question is whether the six known
consumers still bind to the intended public facades.

## Next Task

Execute `g06.166`: consumer surface compatibility sweep.
