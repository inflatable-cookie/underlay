# g06.089 - Auth WebAuthn Service Modularity Audit

## Why

After `g06.088`, the largest remaining Rust warning-level production file is
`underlay-auth-webauthn/src/service.rs`.

WebAuthn is security-sensitive production auth code. It should be split only
from responsibility and public-surface evidence, not from file size alone.

## Goal

Classify the auth WebAuthn service surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-auth-webauthn/src/service.rs` by responsibility family
- identify public service types, challenge/session behavior, registration,
  authentication, verification, and repository boundaries
- decide whether the next batch should split internal modules, extract stable
  model files, or defer behind a broader WebAuthn checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing auth WebAuthn public APIs
- changing WebAuthn challenge or verification semantics
- changing credential security behavior
- changing consumer apps

## Acceptance Criteria

- WebAuthn service responsibilities are grouped by stable behavior family
- helper/model extraction opportunities are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a production-code structure audit. Expected impact is none unless the
audit finds public exports that must move; if so, stop and re-enter planning.

## Current State

`g06.089` is next.

## Next Task

Execute `g06.089`: auth WebAuthn service modularity audit.
