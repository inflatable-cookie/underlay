# g06.079 - Devtools Migration-Bundle Tests Modularity Audit

## Why

After `g06.078`, the largest remaining Rust high-error god-file is
`underlay-devtools/src/tests/migration_bundle_tests.rs`.

Migration-bundle tests cover devtools bundle generation, validation, and
consumer-facing migration evidence. They should be split from behavior evidence
rather than file size alone.

## Goal

Classify the devtools migration-bundle test surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-devtools/src/tests/migration_bundle_tests.rs` by behavior
  family
- identify shared fixtures, bundle setup helpers, validation assertions, and
  edge-case groups
- decide whether the next batch should split test modules, extract helper
  fixtures, or defer behind a broader devtools migration-bundle checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing devtools public APIs
- changing migration-bundle semantics
- changing generated bundle formats
- changing consumer apps

## Acceptance Criteria

- migration-bundle tests are grouped by stable behavior family
- helper/fixture extraction opportunities are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a test-structure audit. Expected impact is none.

## Current State

`g06.079` is next.

## Next Task

Execute `g06.079`: devtools migration-bundle tests modularity audit.
