# g07.010 - Consumer Import Compatibility Sweep

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.001` through `g07.009` classified and reinforced the retained TS public
surface. The next step is to compare the six-consumer family against the
preferred import paths before any compatibility export retirement is proposed.

## Goals

- [x] scan the six-consumer family for Underlay TS imports
- [x] classify imports as preferred, compatibility-only, app-local, or
  candidate-migration
- [x] identify any consumer updates needed before compatibility exports can be
  retired
- [x] keep consumer changes bounded and only make them if the affected surface
  is proven

## Non-Goals

- broad consumer UI rewrites
- changing app-local DTOs or route names
- retiring Underlay exports without same-card consumer proof
- adding new compatibility aliases

## Execution Plan

- [x] scan `underlay-reference`, `contact-patch`, `compli-me`, `acowtancy`,
  `songsprout`, and `loophole/composer`
- [x] compare consumer imports with g07 preferred paths for `runtime/*`,
  `client/*`, `templates`, `testing`, and `tools`
- [x] write a compatibility matrix and classify any required consumer edits
- [x] update consumers only if the edits are narrow and clearly tied to retained
  public paths

## Acceptance Criteria

- [x] all six consumer roots have import compatibility evidence
- [x] compatibility-only Underlay imports are identified
- [x] any proposed export retirement has consumer proof or is deferred
- [x] no breaking change lands without same-card consumer updates

## Validation

- targeted consumer import scans
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- `effigy check:types`

Add consumer-local validation only if this card changes consumer code.

## Consumer Upgrade Impact

Additive consumer cleanup.

`contact-patch/cp-client` now re-exports suggestion request helpers from
`@inflatable-cookie/underlay/client/suggestions` instead of the compatibility-only
`runtime/data` path. No Underlay public API changed.

## Next Task

Execute `g07.011`: stale components config cleanup.
