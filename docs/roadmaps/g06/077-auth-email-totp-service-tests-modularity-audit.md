# g06.077 - Auth Email TOTP Service Tests Modularity Audit

## Why

After `g06.076`, the largest remaining Rust high-error god-file is
`underlay-auth-email-totp/src/tests/service_tests.rs`.

Email TOTP tests cover authentication and recovery behavior. They should be
split from behavior evidence rather than file size alone.

## Goal

Classify the auth email TOTP service test surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-auth-email-totp/src/tests/service_tests.rs` by behavior
  family
- identify shared fixtures, setup helpers, token/code issuance behavior, and
  verification/recovery edge-case groups
- decide whether the next batch should split test modules, extract helper
  fixtures, or defer behind a broader auth email TOTP checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing auth public APIs
- changing email TOTP token/code semantics
- changing recovery or verification behavior
- changing consumer apps

## Acceptance Criteria

- auth email TOTP tests are grouped by stable behavior family
- helper/fixture extraction opportunities are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a test-structure audit. Expected impact is none.

## Current State

`g06.077` is complete.

Artifact:

- [077 artifact](./077-auth-email-totp-service-tests-modularity-audit-artifact.md)

## Next Task

Execute `g06.078`: auth email TOTP service tests internal split.
