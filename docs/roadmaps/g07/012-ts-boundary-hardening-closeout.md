# g07.012 - TS Boundary Hardening Upgrade-Note And Closeout Checkpoint

Status: ready
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.001` through `g07.011` classified, reinforced, and consumer-checked the
retained TypeScript runtime, client, pattern, template, testing, and tools
surface.

The final checkpoint should summarize what changed, what remained compatible,
and what future compatibility-export retirements are now safe to consider.

## Goals

- [ ] write the g07 upgrade-note and closeout artifact
- [ ] summarize retained preferred import paths
- [ ] record consumer edits and validation
- [ ] decide whether any compatibility re-export retirement should be queued
  after g07
- [ ] close or extend g07 deliberately

## Non-Goals

- broad source refactors
- new consumer migrations
- removing exports without a specific retirement card
- opening a new generation

## Execution Plan

- [ ] review g07 artifacts from `001` through `011`
- [ ] write the closeout artifact and upgrade note
- [ ] update front doors and contracts to the next honest task
- [ ] run final docs and TS support validation

## Acceptance Criteria

- [ ] consuming apps have clear upgrade guidance
- [ ] retained TS public paths are summarized
- [ ] compatibility-only exports have an explicit retirement/defer decision
- [ ] g07 queue state is accurate

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy validate`
- targeted consumer status checks

## Consumer Upgrade Impact

Documentation and upgrade guidance only unless the closeout discovers a missed
consumer update.

## Next Task

Execute this TS boundary hardening closeout checkpoint.
