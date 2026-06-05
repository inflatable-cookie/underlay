# g06.113 - Devtools Lib Tests Modularity Audit

## Why

After `g06.112`, the largest remaining Rust warning-level file in the god-file
report is `underlay-devtools/src/tests/lib_tests.rs`.

The file validates shared devtools crate behavior. It should be split from test
responsibility evidence, not line count alone.

## Goal

Classify the devtools lib test surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-devtools/src/tests/lib_tests.rs` by test responsibility
  family
- identify environment helper tests, governance report formatting, decision
  invalidation formatting, policy loading, pipeline report loading, and shared
  fixtures
- identify helper extraction that can reduce size without hiding assertions
- decide whether the next batch should split test modules, extract fixtures, or
  defer behind a broader devtools test checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing devtools public APIs
- changing report formatting behavior
- changing migration report loading semantics
- changing consumer apps

## Acceptance Criteria

- test responsibilities are grouped by stable behavior family
- fixture/helper boundaries are recorded
- behavior coverage that must remain stable is recorded
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a test-structure audit. If the audit finds behavior that requires a
public devtools change, stop and re-enter planning.

## Current State

`g06.113` is complete.

Artifact:

- [113 artifact](./113-devtools-lib-tests-modularity-audit-artifact.md)

## Next Task

Execute `g06.114`: devtools lib tests internal split.
