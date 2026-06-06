# g06.144 - HTTP Client Tests Modularity Audit

## Why

`g06.143` selected `ts/tests/client/http-refactored.test.ts` as the safest next
TypeScript structural target. It is the largest high-severity god-file and is
test-only.

The test file protects the public `client/http` surface. It should be split
from evidence about behavior families and helper boundaries, not from file size
alone.

## Goal

Classify the HTTP client test surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `ts/tests/client/http-refactored.test.ts` by test responsibility
  family
- identify public HTTP client behaviors covered by each group
- identify shared helpers and fixture boundaries
- decide whether the next batch should split test modules or defer behind a
  broader HTTP client checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing public HTTP client APIs
- changing HTTP client behavior
- changing consumer apps

## Acceptance Criteria

- HTTP client test responsibilities are grouped by stable behavior family
- public HTTP behavior boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a test-structure audit. If the audit finds HTTP client behavior that
must change, stop and re-enter planning.

## Current State

`g06.144` is complete.

Artifact:

- [144 artifact](./144-http-client-tests-modularity-audit-artifact.md)

## Next Task

Execute `g06.145`: HTTP client tests internal split.
