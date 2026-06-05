# g06.083 - Auth Password Service Tests Modularity Audit

## Why

After `g06.082`, the only remaining Rust high-error god-file is
`underlay-auth-password/src/tests/service_tests.rs`.

Password auth tests cover security-sensitive registration, login, reset,
verification, and credential behavior. They should be split from behavior
evidence rather than file size alone.

## Goal

Classify the auth password service test surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-auth-password/src/tests/service_tests.rs` by behavior
  family
- identify shared fixtures, password hashing/setup helpers, registration/login
  behavior, reset behavior, and error/security edge-case groups
- decide whether the next batch should split test modules, extract helper
  fixtures, or defer behind a broader auth password checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing auth password public APIs
- changing password hashing or verification semantics
- changing reset/session security behavior
- changing consumer apps

## Acceptance Criteria

- auth password service tests are grouped by stable behavior family
- helper/fixture extraction opportunities are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a test-structure audit. Expected impact is none.

## Current State

`g06.083` is next.

## Next Task

Execute `g06.083`: auth password service tests modularity audit.
