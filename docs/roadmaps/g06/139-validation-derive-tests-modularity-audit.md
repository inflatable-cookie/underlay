# g06.139 - Validation Derive Tests Modularity Audit

## Why

After `g06.138`, no Rust production files remain in the god-file report. The
next Rust warning-level file is `underlay-validation/tests/derive_tests.rs`.

The validation derive tests cover macro behavior that protects consuming app
input validation. They should be split from evidence about test responsibility
families and macro behavior coverage, not from file size alone.

## Goal

Classify the validation derive test surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-validation/tests/derive_tests.rs` by test responsibility
  family
- identify derive macro behavior covered by each group
- identify helper fixtures or test data that can be extracted safely
- decide whether the next batch should split test modules or defer behind a
  broader validation test checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing validation public APIs
- changing derive macro behavior
- changing validation semantics
- changing consumer apps

## Acceptance Criteria

- derive test responsibilities are grouped by stable behavior family
- macro behavior boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a test-structure audit. If the audit finds validation behavior that
must change, stop and re-enter planning.

## Current State

`g06.139` is ready.

## Next Task

Execute `g06.139`: validation derive tests modularity audit.
