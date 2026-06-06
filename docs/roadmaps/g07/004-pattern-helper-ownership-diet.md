# g07.004 - Pattern Helper Ownership Diet

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.003` cleaned active import guidance so consumers are taught the retained
runtime, client, utils, and pattern-root paths.

The `patterns` package still exports a mixed surface: retained workflow shells,
contextual action helpers, and many lower-level helpers now taught through
`runtime/*`. Before changing exports or consumers, this card classifies the
pattern root by ownership and caller risk.

## Goals

- [x] classify every `@decodelabs/underlay/patterns` root export as retained,
  runtime-owned, utils-owned, template-owned, or candidate-retire
- [x] identify exports that are kept only for compatibility
- [x] identify consumer proof needed before any retirement or relocation
- [x] keep retained workflow shells obvious and small
- [x] avoid package export changes unless caller proof is complete inside this
  card

## Non-Goals

- broad consumer import migration without a classified affected surface
- template-system rewrite
- moving Poodle primitives into Underlay
- adding new compatibility aliases
- changing Rust crate boundaries

## Execution Plan

- [x] inspect `ts/src/patterns/index.ts` and the implementation files it exports
- [x] compare the pattern root against contracts `090` and `100`
- [x] scan active docs and source examples for pattern-root dependencies
- [x] scan the six-consumer family for exact pattern-root imports
- [x] write an ownership table and only queue bounded follow-on changes

## Acceptance Criteria

- [x] every pattern-root export has an ownership classification
- [x] retained workflow-shell exports are separated from lower helper exports
- [x] compatibility-only exports have caller evidence and a proposed disposition
- [x] no consumer-visible export change is made without same-card proof

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- targeted `rg` scans over Underlay and the six-consumer family

## Consumer Upgrade Impact

Breaking for unknown callers that imported selection/reorder session helpers
from `@decodelabs/underlay/patterns`.

No known consumer app update is required. The six-consumer scan found no active
code imports of those helpers from the pattern root. The helpers remain
available from `@decodelabs/underlay/runtime/data`.

## Next Task

Move to `g07.005`: duplicated auth-aware fetch orchestration decision.
