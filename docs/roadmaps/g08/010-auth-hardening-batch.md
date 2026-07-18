# g08.010 - Auth Hardening Batch

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Close the remaining medium-severity auth-edge gaps. The crypto core is sound;
these are behavior gaps around it: refresh-token replay is detected but the
session family is not revoked; app-TOTP and password-reset have no attempt
throttle; the login miss path skips Argon2 and leaks account existence by
timing; and the OAuth token cipher passes `plain:`-prefixed values through
unencrypted.

## Evidence

- replay-no-revoke `rust/crates/underlay-auth-jwt/src/session.rs:132-138,165-167`
- second-factor throttle `rust/crates/underlay-auth-totp/src/service.rs` (`verify_second_factor`),
  reset `rust/crates/underlay-auth-password/src/service/passwords.rs`
- login timing `rust/crates/underlay-auth-password/src/service/login.rs:52-59`
- cipher passthrough `rust/crates/underlay-auth-oauth/src/token_cipher.rs:67`

## Governing References

- [030 Auth and session systems](../../contracts/030-auth-and-session-systems.md)

## Planned Changes

- [x] Revoke the whole session family on refresh-replay detection (RFC 6819 /
  OAuth BCP).
- [x] Add per-user attempt caps to TOTP and backup-code verify.
- [ ] ~~Rate-limit password-reset initiation~~ — **deferred, not shipped** (see
  Completion Notes: `reset_password` is admin-internal with no rate key and
  there is no in-tree reset-initiation endpoint to wire; the throttle belongs
  to the consumer flow that owns initiation). *(Audit note: this was previously
  bundled into a checked box, overstating scope.)*
- [x] Verify against a static dummy Argon2 hash on the unknown-email path to
  remove the timing oracle.
- [x] Remove or migration-gate the `plain:` decrypt passthrough.

## Consumer Upgrade Impact

Impact class: `behavioral`. Replay now forces re-auth; second-factor guessing is
throttled. Requires six-consumer proof per `023`.

## Validation

- [x] tests: replay revokes family; capped second-factor attempts; constant-time
  login miss (`unknown_email_miss_costs_a_kdf_pass` in
  `tests/service_tests/login.rs` — asserts the miss path costs ~one KDF pass);
  no `plain:` passthrough. *(Audit note: the timing test landed at generation
  close; before that `dummy_verify` was wired but untested.)*
- [x] `cargo test -p underlay-auth-jwt -p underlay-auth-totp -p underlay-auth-password -p underlay-auth-oauth`
- [x] `effigy validate`

## Stop Conditions

Stop if session-family revocation would break a legitimate concurrent-refresh
pattern; the atomic `rotate_session_if_current` path should already handle the
race, so confirm before widening revocation.

## Completion Notes

Completed 2026-07-17.
- **Replay revoke**: reuse of a superseded refresh token (stale fingerprint
  or mismatched id/version) now deletes the whole session family in
  `SessionManager::refresh_session`. The lost-CAS path is left as the
  legitimate concurrent-refresh race (stop condition honored) - it returns
  the error without revoking. Test proves the family (incl. the current
  token) dies after a replay.
- **Second-factor throttle**: `verify_second_factor_throttled` caps per-user
  attempts against a `RateLimitBackend`, incrementing only on failure and
  resetting on success. `underlay-auth-totp` gains an `underlay-ratelimit`
  dep.
- **Login timing**: unknown-email and no-credential paths run one KDF pass
  (`dummy_verify`) so the miss path costs the same as a real verify.
- **`plain:` passthrough**: `OAuthTokenCipher` now rejects `plain:` secrets
  by default; readable only via explicit `with_plain_migration(true)` for a
  bounded re-encryption window.
- **Deferred**: password-reset-initiation throttle - `reset_password` is
  admin-internal with no rate key and there is no in-tree reset-initiation
  endpoint to wire; left for the consumer flow that owns initiation. The `L`
  rate-limit bound remains available on the password service.
Contract `030` updated (family revocation, login-miss timing, throttled
2FA). `cargo test` across the four auth crates green; workspace green;
`effigy validate` green.

## Next Task

Lane A complete. Planning checkpoint, then `g08.011` (Lane B) if not already
underway.
