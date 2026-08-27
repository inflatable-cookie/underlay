# Underlay Reference CSRF Token Reads Rotate Live Proof

Status: open
Captured: 2026-08-27

## Observation

Underlay Reference `GET /v1/auth/csrf-token` mints and stores a new token on
every read. A second browser tab fetching a token therefore invalidates the
first tab's cached header while its cookie remains live.

Acowtancy PR62 exposed the same failure mode during exact-head review and added
a two-tab regression proving that token reads reuse a non-empty live cookie.
The Reference implementation remains in
`apps/acme-api/crates/api/src/routes/shared/auth/mod.rs` at merge commit
`6af2783768e04c8def9b6bb1de5c90cbb69a7892`.

## Impact

Underlay Reference does not yet prove the fleet's cross-tab cookie-mutation
posture. A normal second-tab token fetch can cause a valid first-tab mutation
to fail CSRF validation.

## Disposition

Keep open. Repair and prove stable token issuance in the Underlay Reference
owning lane before `g09.053` fleet closeout. Do not mix the implementation into
consumer PRs or declare fleet CSRF conformance from the merged `g09.047` proof.
