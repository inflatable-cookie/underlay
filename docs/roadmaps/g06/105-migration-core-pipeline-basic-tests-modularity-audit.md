# g06.105 - Migration-Core Pipeline Basic Tests Modularity Audit

## Why

After `g06.104`, the largest remaining Rust warning-level file in the god-file
report is
`underlay-migration-core/src/tests/pipeline_basic_tests.rs`.

The file validates shared migration pipeline behavior. It should be split from
test responsibility evidence, not line count alone.

## Goal

Classify the migration-core pipeline basic test surface and decide the safest
next structural batch.

## Scope

In scope:

- inspect `underlay-migration-core/src/tests/pipeline_basic_tests.rs` by test
  responsibility family
- identify fixture setup, repository setup, migration definitions, pipeline
  execution assertions, and failure-mode coverage
- identify shared helpers that can move without weakening test clarity
- decide whether the next batch should split test modules, extract helper
  fixtures, or defer behind a broader migration-core test checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing migration-core public APIs
- changing pipeline behavior
- changing migration ordering, verification, or drift semantics
- changing consumer apps

## Acceptance Criteria

- test responsibilities are grouped by stable behavior family
- fixture/helper boundaries are recorded
- behavior coverage that must remain stable is recorded
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a test-structure audit. If the audit finds behavior that requires a
public migration-core change, stop and re-enter planning.

## Current State

`g06.105` is ready.

## Next Task

Execute `g06.105`: migration-core pipeline basic tests modularity audit.
