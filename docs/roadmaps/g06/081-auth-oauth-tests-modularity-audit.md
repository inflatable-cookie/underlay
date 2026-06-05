# g06.081 - Auth OAuth Tests Modularity Audit

## Why

After `g06.080`, the largest remaining Rust high-error god-file is
`underlay-auth-oauth/src/tests/lib_tests.rs`.

OAuth tests cover auth-provider configuration, redirect/callback behavior, and
security-sensitive token/session handling. They should be split from behavior
evidence rather than file size alone.

## Goal

Classify the auth OAuth test surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-auth-oauth/src/tests/lib_tests.rs` by behavior family
- identify shared fixtures, provider setup, callback/session behavior, and
  error/security edge-case groups
- decide whether the next batch should split test modules, extract helper
  fixtures, or defer behind a broader auth OAuth checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing auth OAuth public APIs
- changing redirect/callback semantics
- changing token/session security behavior
- changing consumer apps

## Acceptance Criteria

- auth OAuth tests are grouped by stable behavior family
- helper/fixture extraction opportunities are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a test-structure audit. Expected impact is none.

## Current State

`g06.081` is next.

## Next Task

Execute `g06.081`: auth OAuth tests modularity audit.
