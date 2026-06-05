# g06.073 - Auth JWT Service Tests Modularity Audit

## Why

After `g06.072`, the remaining Rust high-error god-files are test-heavy or
outside the Rust production adapter line. The largest Rust item is
`underlay-auth-jwt/src/tests/service_tests.rs`.

Test god-files still matter for reference-grade maintainability because they
hide behavioral contracts and make future auth/session work harder to reason
about.

## Goal

Classify the auth JWT service test surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-auth-jwt/src/tests/service_tests.rs` by behavior family
- identify test helpers, fixture setup, session rotation, token, and store
  behavior groups
- decide whether the next batch should split test modules, extract helper
  fixtures, or defer behind a broader auth-JWT checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing auth/session public APIs
- changing token semantics
- changing refresh rotation behavior
- changing consumer apps

## Acceptance Criteria

- auth JWT service tests are grouped by stable behavior family
- helper/fixture extraction opportunities are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a test-structure audit. Expected impact is none.

## Current State

`g06.073` is complete.

Artifact:

- [073 artifact](./073-auth-jwt-service-tests-modularity-audit-artifact.md)

## Next Task

Execute `g06.074`: auth JWT service tests internal split.
