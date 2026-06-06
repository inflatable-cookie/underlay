# g07.004 - Pattern Helper Ownership Diet

Status: ready
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

- [ ] classify every `@decodelabs/underlay/patterns` root export as retained,
  runtime-owned, utils-owned, template-owned, or candidate-retire
- [ ] identify exports that are kept only for compatibility
- [ ] identify consumer proof needed before any retirement or relocation
- [ ] keep retained workflow shells obvious and small
- [ ] avoid package export changes unless caller proof is complete inside this
  card

## Non-Goals

- broad consumer import migration without a classified affected surface
- template-system rewrite
- moving Poodle primitives into Underlay
- adding new compatibility aliases
- changing Rust crate boundaries

## Execution Plan

- [ ] inspect `ts/src/patterns/index.ts` and the implementation files it exports
- [ ] compare the pattern root against contracts `090` and `100`
- [ ] scan active docs and source examples for pattern-root dependencies
- [ ] scan the six-consumer family for exact pattern-root imports
- [ ] write an ownership table and only queue bounded follow-on changes

## Acceptance Criteria

- [ ] every pattern-root export has an ownership classification
- [ ] retained workflow-shell exports are separated from lower helper exports
- [ ] compatibility-only exports have caller evidence and a proposed disposition
- [ ] no consumer-visible export change is made without same-card proof

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- targeted `rg` scans over Underlay and the six-consumer family

## Consumer Upgrade Impact

None for the audit posture.

If this card makes an export or import-path change, update this section with
the affected consumers and validation evidence before closing it.

## Next Task

Execute this pattern helper ownership diet.
