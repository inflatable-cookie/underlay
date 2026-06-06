# g06.152 Artifact - Optimistic Tests Modularity Audit

## Summary

`ts/tests/patterns/optimistic.test.ts` is the only remaining high-severity
god-file after `g06.151`.

The file is test-only and mirrors the already-split runtime modules under
`ts/src/patterns/optimistic/`.

## Test Groups

Current coverage groups:

- `createOptimisticList`
  - initialization
  - add
  - remove
  - update
  - set
  - pending IDs
- `createOptimisticToggle`
  - initialization
  - toggle
  - set
- `createOptimisticValue`
  - initialization
  - set
  - custom equality
- `createOptimisticCounter`
  - initialization
  - increment
  - decrement
  - set

Related focused tests already exist:

- `ts/tests/patterns/optimistic-barrel.test.ts`
- `ts/tests/patterns/optimistic-helpers.test.ts`

## Helper Boundaries

The main repeated fixture is the list item shape:

- `type OptimisticTestItem = { id: string; name: string }`

The split can either duplicate that small local type in the list test or move it
to a tiny support module. Prefer local duplication unless more shared fixtures
appear during implementation.

No mocking or environment setup is required. Tests use `svelte/store` `get()`
and direct public optimistic builders.

## Split Plan

Suggested module shape:

- `ts/tests/patterns/optimistic/list.test.ts`
- `ts/tests/patterns/optimistic/toggle.test.ts`
- `ts/tests/patterns/optimistic/value.test.ts`
- `ts/tests/patterns/optimistic/counter.test.ts`

Then remove or replace `ts/tests/patterns/optimistic.test.ts` so the high
god-file finding disappears.

Keep the existing related tests unchanged:

- `ts/tests/patterns/optimistic-barrel.test.ts`
- `ts/tests/patterns/optimistic-helpers.test.ts`

## Validation Evidence

Passed:

- `bun x vitest run ts/tests/patterns/optimistic.test.ts ts/tests/patterns/optimistic-barrel.test.ts ts/tests/patterns/optimistic-helpers.test.ts`
  - 63 tests passed

Doctor:

- `effigy doctor` still fails on standing structural scans
- god-file findings are `15` total, `1` high
- `ts/tests/patterns/optimistic.test.ts` is the remaining high-severity
  god-file

## Public API Impact

None.

This is test-only structural work.

## Decision

Queue `g06.153` as a mechanical optimistic tests internal split.
