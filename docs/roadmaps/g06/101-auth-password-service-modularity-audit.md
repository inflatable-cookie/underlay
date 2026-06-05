# g06.101 - Auth Password Service Modularity Audit

## Why

After `g06.100`, the largest remaining Rust warning-level production file in
the god-file report is `underlay-auth-password/src/service.rs`.

Password auth is security-sensitive. It should be split from responsibility and
public-surface evidence, not file size alone.

## Goal

Classify the auth password service surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-auth-password/src/service.rs` by responsibility family
- identify public service methods, password hashing, policy validation,
  login/lockout behavior, password change behavior, reset behavior, and helper
  boundaries
- identify which exports and service method contracts must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader password-auth checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing auth password public APIs
- changing password hashing semantics
- changing login, lockout, or reset behavior
- changing consumer apps

## Acceptance Criteria

- auth password service responsibilities are grouped by stable behavior family
- public exports and service-visible helpers are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a production-code structure audit. Expected impact is none unless the
audit finds public exports that must move; if so, stop and re-enter planning.

## Current State

`g06.101` is ready.

## Next Task

Execute `g06.101`: auth password service modularity audit.
