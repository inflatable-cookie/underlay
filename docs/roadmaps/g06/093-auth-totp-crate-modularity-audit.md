# g06.093 - Auth TOTP Crate Modularity Audit

## Why

After `g06.092`, the largest remaining Rust warning-level production file in
the god-file report is `underlay-auth-totp/src/lib.rs`.

TOTP is security-sensitive auth code. It should be split from responsibility
evidence, not file size alone.

## Goal

Classify the auth TOTP crate surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-auth-totp/src/lib.rs` by responsibility family
- identify public types, secret generation, provisioning URI construction,
  code verification, backup-code behavior, and tests
- identify which crate-root exports and helper methods must remain stable
- decide whether the next batch should split internal modules, extract model
  files, or defer behind a broader auth TOTP checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing auth TOTP public APIs
- changing secret generation semantics
- changing code verification behavior
- changing backup-code behavior
- changing consumer apps

## Acceptance Criteria

- auth TOTP responsibilities are grouped by stable behavior family
- public exports and security-sensitive helpers are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a production-code structure audit. Expected impact is none unless the
audit finds public exports that must move; if so, stop and re-enter planning.

## Current State

`g06.093` is complete.

Artifact:

- [093 artifact](./093-auth-totp-crate-modularity-audit-artifact.md)

## Next Task

Execute `g06.094`: auth TOTP crate internal split.
