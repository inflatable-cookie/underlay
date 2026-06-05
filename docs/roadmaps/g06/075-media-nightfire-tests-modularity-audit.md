# g06.075 - Media Nightfire Tests Modularity Audit

## Why

After `g06.074`, the largest remaining Rust high-error god-file is
`underlay-media/src/tests/nightfire_tests.rs`.

Nightfire tests cover media analysis behavior. They should be split from
evidence rather than file size alone.

## Goal

Classify the media Nightfire test surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-media/src/tests/nightfire_tests.rs` by behavior family
- identify shared fixtures, analysis inputs, output assertions, and edge-case
  groups
- decide whether the next batch should split test modules, extract helper
  fixtures, or defer behind a broader media Nightfire checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing media public APIs
- changing Nightfire analysis semantics
- changing blob/media storage behavior
- changing consumer apps

## Acceptance Criteria

- Nightfire tests are grouped by stable behavior family
- helper/fixture extraction opportunities are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a test-structure audit. Expected impact is none.

## Current State

`g06.075` is next.

## Next Task

Execute `g06.075`: media Nightfire tests modularity audit.
