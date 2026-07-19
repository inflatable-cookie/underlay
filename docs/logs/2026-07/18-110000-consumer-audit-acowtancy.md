# 2026-07-18 - Consumer audit: acowtancy

Second of the six-consumer g08-adoption audits (after underlay-reference).
Method: fresh mechanical gates (fmt/clippy/build on farmyard; svelte-check on the
TS apps) + two independent code-level agents (farmyard Rust posture, TS posture)
+ manual verification. acowtancy is the largest/most-developed consumer.

## Mechanical baseline

- farmyard **builds clean** and is **clippy-clean** (better than acme-api), but
  `cargo test --workspace` **does not compile** (stale inline test module in
  `main.rs` + `test_support.rs`; drifted `ErrorLogListQuery`/`TransformOperation*`
  types and handler arities). Its QA gate runs build + clippy only; the managed
  test suite is separate and never gated, so the rot passed CI.

## Fixed (committed, pushed)

**Security**
- **Open-redirect — 62 dairy files / 68 sites.** Every admin save-and-close used
  `formReturnTo.startsWith("/")` (accepts `//evil.com`), while the login page one
  directory away already used `resolveRedirectTo` and *commented* that the bare
  check is unsafe. All 68 sites now route through `resolveRedirectTo`
  (58 by a scripted transform, 4 nested-ternary stragglers by hand). Confirmed
  exploitable: `returnTo` flows from a URL param via hidden form inputs into
  `goto()`. svelte-check green (0 errors, 7740 files).
- **Login user-enumeration timing (farmyard)** — `verify_user_credentials`
  returned early on unknown-email / missing-credential with no KDF pass. Added
  `dummy_verify` on both miss paths.
- **Blob fail-closed (farmyard)** — an S3 init failure fell back to `NoopAdapter`
  in **all** environments (uploads accepted then discarded = data loss; worse
  than acme-api, which at least gated by env). Now refuses to boot outside
  local/dev/test unless `ACOWTANCY_ALLOW_NOOP_BLOB=1`.

**Quality**
- Removed `noImplicitAny: false` from dairy + cream tsconfig. svelte-check stays
  green for both.

Validation: farmyard `cargo build` + `clippy -D warnings` clean; dairy (7740) and
cream (2745) svelte-check 0 errors.

## Deferred — laid out as roadmap cards `g03.019`-`g03.022` in acowtancy/ledger

- **019 auth service adoption (farmyard, highest priority):** refresh-replay has
  no token-family revocation; fingerprint mismatch on refresh not enforced; 2FA
  unthrottled on the single-shot `/v1/auth/login`; four duplicate spoofable
  `X-Forwarded-For` helpers with `ConnectInfo` never wired; refresh token echoed
  in the JSON body. Same hand-rolled-auth divergence as acme-api, at larger scale.
- **020 media/blob production:** raw adapter instead of
  `initiate_upload_validated`/`finalise_upload_verified`; no byte sniffing;
  client `sha256` stored-not-verified; no prod S3; two TODOs (restricted-media
  entitlement, password-reset email).
- **021 TS type-safety + Nightfire hygiene:** 22 `as never` + 17 `as any`; froyo
  renders markdown via a homegrown escape-first `renderMarkdown` copy-pasted 8×,
  off the sanctioned underlay sanitized path (not a live XSS — escapes first —
  but divergent and duplicated); cream/cattle-grid/froyo missing npm check
  scripts.
- **022 gate hardening + test-compile repair:** fix the non-compiling farmyard
  tests and add the managed suite to the QA gate.

## Differences from underlay-reference

- **Better:** SSR-safe (dairy SPA; cream re-enables SSR on auth pages but runs
  nothing unsafe there); CORS env-guarded with prod hard-exits; security-alerts +
  audit wired with logged (non-swallowed) errors; no deprecated underlay APIs;
  no token leakage on session GET; dairy's 2FA `{@html}` correctly uses
  `sanitizeSvgHtml`; cattle-grid TS is clean (0 casts).
- **Worse:** open-redirect at 10× the scale (62 vs 6 files); blob NoopAdapter
  fallback ungated across all envs; tests do not compile at all.

## Next

Remaining four consumers (compli-me, contact-patch, songsprout, loophole) are
smaller and expected to echo a subset of these — primarily the open-redirect and
`noImplicitAny` patterns, plus their own hand-rolled-auth stories.
