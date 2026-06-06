# g06.152 - Optimistic Tests Modularity Audit

## Why

After `g06.151`, `effigy doctor` reports
`ts/tests/patterns/optimistic.test.ts` as the only remaining high-severity
god-file.

The file is test code, but it still affects maintainability. Before splitting
it, audit test responsibilities and coverage boundaries so the next batch can
stay mechanical.

## Goal

Audit `ts/tests/patterns/optimistic.test.ts` and produce a focused split plan
that preserves coverage and test intent.

## Scope

In scope:

- inventory the behavior covered by `optimistic.test.ts`
- classify test groups and fixtures
- identify shared setup helpers that can move into support files
- propose a module shape for the next internal test split
- record validation commands

Out of scope:

- changing optimistic runtime behavior
- weakening test coverage
- changing consumer apps
- performing the split

## Acceptance Criteria

- artifact records test behavior groups
- artifact records helper/setup boundaries
- artifact records validation evidence
- next split card is queued only if the split can be mechanical

## Consumer Upgrade Impact

Expected impact: none.

This is test-only structural work.

## Current State

`g06.152` is complete.

Artifact:

- [`152-optimistic-tests-modularity-audit-artifact.md`](152-optimistic-tests-modularity-audit-artifact.md)

## Next Task

Execute `g06.153`: optimistic tests internal split.
