# g06.030 - Raw Operator Wrapper Removal Readiness

## Why

`g06.028` introduced typed audit and security-alert table APIs.
`g06.029` migrated the six known consumers and deprecated the raw wrappers.

Before removing compatibility wrappers, Underlay should decide whether those
helpers still provide useful bootstrap ergonomics or whether they now only hide
unsafe habits.

## Goal

Assess removal readiness for deprecated raw operator table wrappers and scan
the remaining dynamic identifier helper surface.

## Scope

In scope:

- inspect deprecated raw wrappers in `underlay-audit`
- inspect deprecated raw wrappers in `underlay-security-alerts`
- scan remaining Rust dynamic identifier helpers outside audit/security-alerts
- classify each remaining raw helper as retained compatibility, deprecation, or
  removal candidate
- update contracts with the final posture

Out of scope:

- consumer migration already completed in `g06.029`
- unrelated adapter extraction
- TypeScript/Svelte work
- release execution or publishing

## Contract References

- `021`: database migration and schema workflow
- `023`: release and compatibility rollout
- `060`: jobs, events, and operator systems
- `122`: Rust public API inventory

## Acceptance Criteria

- deprecated raw operator wrappers have a removal/retention decision
- remaining dynamic identifier helper surface is inventoried
- no broad shared API is removed without explicit compatibility classification
- targeted Rust checks pass or failures are classified

## Consumer Upgrade Impact

Impact: deprecation unless raw wrappers are removed in this batch. Removal would
be breaking but should have low current blast radius after `g06.029`.

## Current State

`g06.030` is complete.

See
[`030-raw-operator-wrapper-removal-readiness-artifact.md`](030-raw-operator-wrapper-removal-readiness-artifact.md).

## Next Task

Execute `g06.031`: remaining typed DB helper migration plan.
