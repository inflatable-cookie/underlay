# g06.003 — Auth Session Contract Reset

## Why

The Rust audit hardened refresh-token rotation by making `SessionStore`
compare the active refresh-token fingerprint, token ID, and version before
accepting a rotation.

That behavior is correct, but it is a public trait contract. It needs an
explicit rollout proof so future auth work does not change session storage
semantics by accident.

## Goal

Freeze the auth/session public contract around atomic refresh rotation and
prove the current consumer family does not carry unplanned `SessionStore`
implementations.

## Scope

In scope:

- document `SessionStore::rotate_session_if_current` semantics
- scan named consumers for direct `SessionStore` implementations
- decide whether the session-store trait belongs in `underlay-auth-jwt` for now
  or should move behind `underlay-auth` in a later milestone
- keep refresh replay tests in `underlay-auth-jwt`
- record consumer impact under `023`

Out of scope:

- another trait break without consumer implementation proof
- adding compatibility shims for stale refresh rotation
- moving provider-specific JWT behavior into app-local code

## Contract References

- `023`: release and compatibility rollout
- `030`: auth and session systems
- `122`: Rust public API inventory

## Consumer Upgrade Impact

Impact classification: `no direct code change` for current named consumers.

The trait change is `breaking` for any future or unknown direct `SessionStore`
implementation because `rotate_session_if_current` now requires previous
refresh-token fingerprint, token ID, and version checks.

## Acceptance Criteria

- consumer scan proves whether `SessionStore` has external implementations
- `030` or a follow-up contract note records atomic refresh rotation semantics
- `underlay-auth-jwt` focused tests still cover stale refresh rejection
- targeted consumer checks run for any app touched by auth/session rollout work

## Consumer Proof

Scan scope:

- `underlay-reference`
- `contact-patch`
- `compli-me`
- `acowtancy`
- `songsprout`
- `loophole/composer`

Findings:

- no named consumer directly references `SessionStore`
- no named consumer implements `SessionStore`
- no named consumer references `rotate_session_if_current`
- named consumers do carry app-local refresh-token fingerprint, token ID, and
  version fields in their own auth/session storage
- app-local refresh paths that bypass `SessionManager` must preserve the same
  compare-and-swap semantics

## Contract Decision

`SessionStore` stays in `underlay-auth-jwt` for now.

Moving it into `underlay-auth` is plausible, but not part of this card. That
move would be another public trait break and should happen only with a focused
proposal and fresh consumer implementation proof.

## Evidence

- `docs/contracts/030-auth-and-session-systems.md` now records atomic refresh
  rotation semantics.
- `rust/crates/underlay-auth-jwt/src/session.rs` keeps
  `rotate_session_if_current` as the refresh rotation path.
- `rust/crates/underlay-auth-jwt/src/tests/service_tests.rs` covers stale
  refresh rejection through `rotate_session_if_current_rejects_stale_refresh_state`.

## Current State

`g06.003` is complete.

## Next Task

Execute `g06.004`: HTTP safe-builder consolidation and consumer cookie cleanup.
