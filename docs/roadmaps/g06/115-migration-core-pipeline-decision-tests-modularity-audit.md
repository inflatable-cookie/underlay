# g06.115 - Migration-Core Pipeline Decision Tests Modularity Audit

## Why

After `g06.114`, the largest remaining Rust warning-level file in the god-file
report is
`underlay-migration-core/src/tests/pipeline_decision_tests.rs`.

The file validates shared migration decision behavior. It should be split from
test responsibility evidence, not line count alone.

## Goal

Classify the migration-core pipeline decision test surface and decide the
safest next structural batch.

## Scope

In scope:

- inspect `underlay-migration-core/src/tests/pipeline_decision_tests.rs` by
  test responsibility family
- identify cached-decision reuse, human override precedence, invalidation,
  unresolved queue behavior, governance issue reporting, and helper fixture
  setup
- identify shared helpers that can move without weakening test clarity
- decide whether the next batch should split test modules, extract helper
  fixtures, or defer behind a broader migration-core decision checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing migration-core public APIs
- changing decision reuse, invalidation, or governance behavior
- changing pipeline execution semantics
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

`g06.115` is ready.

## Next Task

Execute `g06.115`: migration-core pipeline decision tests modularity audit.
