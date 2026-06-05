# g06.119 - Jobs Runner Tests Modularity Audit

## Why

After `g06.118`, the largest remaining Rust warning-level file in the god-file
report is `underlay-jobs/src/tests/runner_tests.rs`.

The file validates shared job runner behavior. It should be split from test
responsibility evidence, not line count alone.

## Goal

Classify the jobs runner test surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-jobs/src/tests/runner_tests.rs` by test responsibility
  family
- identify runner setup, handler behavior, repository assertions, retry
  behavior, timeout/cancellation behavior, and fixture helpers
- identify shared helpers that can move without weakening test clarity
- decide whether the next batch should split test modules, extract helper
  fixtures, or defer behind a broader jobs runner checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing jobs public APIs
- changing runner, retry, timeout, or cancellation behavior
- changing repository semantics
- changing consumer apps

## Acceptance Criteria

- test responsibilities are grouped by stable behavior family
- fixture/helper boundaries are recorded
- behavior coverage that must remain stable is recorded
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a test-structure audit. If the audit finds behavior that requires a
public jobs change, stop and re-enter planning.

## Current State

`g06.119` is ready.

## Next Task

Execute `g06.119`: jobs runner tests modularity audit.
