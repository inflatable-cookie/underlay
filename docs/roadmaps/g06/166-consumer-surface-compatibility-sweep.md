# g06.166 - Consumer Surface Compatibility Sweep

## Why

The TypeScript source structure is now clean enough to test against the real
extension boundary: the known consumer apps.

Before more reference-grade architecture work lands, the six consumer roots
should be checked against the current Underlay public facades.

## Goal

Prove current Underlay compatibility across the known consumer family and record
any import, build, or template-surface drift.

## Scope

In scope:

- inspect the six known consumer roots listed in `AGENTS.md`
- check Underlay imports against documented public surfaces
- run targeted consumer validation where each workspace exposes a reasonable
  local task
- classify any required consumer updates as bounded follow-up work
- record whether current g06 source splits were breaking in practice

Out of scope:

- broad consumer feature work
- production deployment
- new Underlay public APIs unless a compatibility blocker proves one is needed
- unrelated app cleanup

## Acceptance Criteria

- every known consumer root is inspected
- any stale Underlay import path is listed
- validation commands and failures are recorded
- required consumer updates are classified by app
- Underlay-side follow-up is separated from app-side rollout work

## Consumer Upgrade Impact

Expected impact: audit only.

If drift is found, follow-up cards may include consumer updates.

## Current State

`g06.166` is ready.

## Next Task

Execute `g06.166`: consumer surface compatibility sweep.
