# g06.153 - Optimistic Tests Internal Split

## Why

`g06.152` found that `ts/tests/patterns/optimistic.test.ts` is the only
remaining high-severity god-file and already splits cleanly by optimistic
runtime primitive.

## Goal

Split the optimistic tests into focused files without changing runtime behavior
or weakening coverage.

## Scope

In scope:

- extract list tests to `ts/tests/patterns/optimistic/list.test.ts`
- extract toggle tests to `ts/tests/patterns/optimistic/toggle.test.ts`
- extract value tests to `ts/tests/patterns/optimistic/value.test.ts`
- extract counter tests to `ts/tests/patterns/optimistic/counter.test.ts`
- remove the monolithic `ts/tests/patterns/optimistic.test.ts`
- keep optimistic barrel and helper tests unchanged

Out of scope:

- changing optimistic runtime behavior
- changing public optimistic exports
- changing consumer apps

## Acceptance Criteria

- focused optimistic tests pass
- optimistic barrel and helper tests pass
- `effigy doctor` no longer reports high-severity god-files, or any remaining
  high finding is recorded with cause
- `effigy qa:docs` passes
- roadmap artifact records final test module shape and coverage impact

## Consumer Upgrade Impact

Expected impact: none.

This is test-only structural work.

## Current State

`g06.153` is ready.

## Next Task

Execute `g06.153`: optimistic tests internal split.
