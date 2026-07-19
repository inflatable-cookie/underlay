# 2026-07-18 - Consumer audit: underlay-reference (reference app)

First of the six-consumer g08-adoption audits. Method: fresh mechanical gates
(fmt/clippy/build/test on acme-api; svelte-check on the TS packages) + two
independent code-level agents (Rust posture, TS posture) + manual verification
of every flagged finding.

## Mechanical baseline (before fixes)

- **acme-api tests did not compile** — `crates/infra/src/tests/config_tests.rs`
  missing `use std::path::Path`. Their `effigy validate` only runs `build`, so
  the broken test never failed CI.
- 5 clippy errors (`-D warnings`); fmt drift across the api crate.

## Fixed (committed, pushed)

**Security**
- **Open-redirect (6 sites)** — admin save-and-close used
  `formReturnTo.startsWith("/")`, which accepts protocol-relative `//evil.com`.
  Replaced with underlay `resolveRedirectTo`. (Login pages themselves were
  already safe.)
- **Stored XSS** — `acme-ui` `TaskNotesRenderer.svelte` piped user markdown
  through raw `marked` → `{@html}`. Now `sanitizeHtml(marked.parse(...))`.
- **Login user-enumeration timing** — hand-rolled login returned early on
  unknown-email and missing-credential with **no KDF pass** (account-existence
  oracle). Added `dummy_verify` (one Argon2 pass) on both miss paths.
- **Blob fail-closed** — production silently used `NoopAdapter` (uploads
  accepted then discarded). Now panics unless `ACME_ALLOW_NOOP_BLOB=1`.

**Quality**
- infra test-compile fix; clippy fixes (`TrustedProxyConfig` +
  `RateLimitBackendType` derive `Default`, needless-bool simplification,
  `#[allow(too_many_arguments)]` on 3 db helpers matching existing house style).
- Removed `noImplicitAny: false` from `acme-admin`/`acme-front` tsconfig (the
  g08.024 anti-pattern). svelte-check stays green.

Validation: `cargo build`+`test` green (86 passed); svelte-check 0 errors across
acme-admin/acme-front/acme-ui.

## Deferred — flagged for decision (architectural, not one-line fixes)

The reference app **hand-rolls auth** (`acme-api/crates/auth/src/local/`)
instead of adopting underlay's hardened services. That reimplementation misses
several g08 protections:

1. **Refresh-replay: no session-family revocation.** `local/session.rs` returns
   `TokenInvalid` on a replayed refresh token but does not revoke the family —
   the reuse-detection auto-revocation from underlay's `SessionManager` is
   absent. (Fingerprint mismatch on refresh only warns, doesn't reject.)
2. **2FA not throttled.** `local/totp.rs` calls raw
   `verify_totp_with_replay_protection`, not `verify_second_factor_throttled`;
   only the outer login rate-limit applies to 2FA guessing.
3. **Duplicate spoofable XFF helpers.** `routes/shared/auth/mod.rs` has the
   correct trusted-proxy path *and* two helpers that split `X-Forwarded-For`
   with no trust validation (used by register/login).

Recommendation: adopt underlay's `PasswordAuthService` / `SessionManager` /
`verify_second_factor_throttled` in the reference app so it *demonstrates* the
hardened posture rather than diverging from it. Sizeable; needs a decision.

## Noted (lower priority)

- Rate limiting uses a custom Redis backend, not the new `PostgresBackend`
  (acceptable — it is a real distributed path).
- Media upload reimplements magic-byte sniffing instead of adopting
  `initiate_upload_validated`/`finalise_upload_verified` (it does sniff).
- **63 `as never` + 5 `as any`** casts in acme-admin erode the g08 TS hardening.
- 13 pre-existing acme-api clippy lints (large `Err` variants, too-many-args);
  acme-api's `effigy validate` runs `build` only — recommend adding
  `test` + `clippy` to the gate.
- TS packages have empty `package.json` script blocks (all validation via
  effigy).
- Audit logging is fire-and-forget (`let _ =`).

## Next

acowtancy audit (largest/most-developed after this). The other four consumers
are far smaller and expected to echo a subset of these at lesser scale.
