# Refresh-token reuse detection + family revocation — fleet rollout

Date: 2026-07-19
Scope: compli-me, contact-patch, songsprout/nursery, loophole/composer
Reference: `underlay-auth-jwt` `SessionManager` (`session.rs`)
Governing refs: consumer-audit cards compli-me `g01.013`, contact-patch `g01.004`,
songsprout `g02.002`, loophole/composer `g02.002`

## Problem

Every consumer's local auth rotates the refresh token on use (bumping
`refresh_token_id` / `refresh_token_version`) and rejects a stale token, but on
detecting reuse it only rejected the one request — it did not revoke the session.
A stolen refresh token that the attacker rotated first stays valid while the
victim is merely bounced. The fix was deferred from the g08 audit because a naive
"revoke on any version mismatch" breaks the legitimate concurrent double-refresh
(one client, same token, two in-flight requests): the loser's version no longer
matches, and revoking there would kill a valid session.

## The model (RFC 6819 / OAuth 2.0 Security BCP)

The underlay `SessionManager` already implements the correct shape; each app now
mirrors it:

1. **Reuse detection → revoke the family.** A refresh token that verifies and
   targets an *active* session but is not its current token — stale fingerprint,
   or a superseded `refresh_token_id`/`version` — is a replayed/stolen token.
   Revoke the whole session (`is_active = FALSE`, reason
   `refresh_reuse_detected`). Best-effort: a revoke-write failure is logged, not
   allowed to mask the auth error.
2. **Rotation is an atomic compare-and-swap.** The rotation `UPDATE` gained
   `AND is_active AND refresh_token_id = <read id> AND refresh_token_version =
   <read version>` and reports whether a row changed. Exactly one of two
   concurrent refreshes wins.
3. **CAS-lost → reject without revoking.** Losing the swap means another refresh
   rotated the same token first — the legitimate double-submit race, *not* reuse
   (superseded tokens are caught in step 1 before the CAS). The loser is rejected
   and retries with the freshly issued token; the family is not touched.

The key ordering insight: two legitimate racers both read the same current
id/version, so both pass the step-1 checks; only the CAS separates them. A
genuinely superseded token is read *after* a rotation, so its id/version no
longer match and step 1 revokes. Revoking in step 1 therefore cannot kill a valid
in-flight refresh.

## Per-app changes

All four apps share one auth template, so the transform is uniform: add a
`rotate_session_if_current` CAS (returning `bool`), route the reuse-detection
branches through a `revoke_family_on_reuse` helper, and reject the CAS-loss
without revoking. Removed the now-dead unconditional `update_session` in the
three method-based apps.

- **compli-me** — `crates/auth/src/local/{mod.rs,sessions.rs}`, `auth.sessions`.
- **contact-patch** — `crates/auth/src/local/session.rs`, `auth.sessions`.
- **nursery** — `crates/auth/src/underlay_local.rs`, `accounts.session`. IP/UA
  session-fingerprint drift stays warn+migrate (matches `SessionManager`, which
  does not reject roaming clients); only refresh-token reuse revokes.
- **composer** — `composer-api/.../routes/auth_local.rs` (free functions);
  `update_session_for_refresh` became the CAS.

`cargo check --workspace --all-features --all-targets` green in all four.

## Test status

The logic mirrors the underlay `SessionManager`, which has DB-backed tests. A
per-app concurrent-refresh + reuse-detection integration test still needs a live
Postgres (the standing per-app DB-harness gap tracked in every card, same infra
story as the foundation's `UNDERLAY_TEST_DATABASE_URL`). Not added here.

## Remaining deferred (unchanged)

songsprout + composer prod S3; composer real admin auth + 2FA. These are product
/ credential-design work, not hardening tweaks.
