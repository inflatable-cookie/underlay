# 2026-07-17 - g08 Lane A checkpoint + six-consumer rollout

## Context

Post-Batch-3 planning checkpoint (per the g08 generation runway). Two
questions: (1) is the security posture consumer-communicable and does any
finding warrant a new contract; (2) apply the resulting fixes across the six
consumer apps.

## Checkpoint decision: no new contract

All ten Lane A findings landed in existing governing contracts, updated this
generation:

- `020` - trusted-proxy IP resolution, error-header boundary, http-client
  timeouts/SSRF profiles, CORS gating
- `030` - token-exposure boundary, refresh-replay family revocation,
  login-miss timing, throttled second factor
- `031` - safe production CORS posture
- `040` - upload content-type/size/magic-byte enforcement

No finding is orphaned. The single real gap was **communication**: the rules
span four contracts, so consumers had no one place to see the required
actions. This note plus the guide `190` matrix row close that gap. No new
contract is compiled.

## Six-consumer rollout (contract 023)

Scanned all six consumer roots for breakage from the g08 behavioral/config
changes. Consumers depend on Underlay via `path = "../../underlay/..."` (Rust)
and `file:` (TS), so changes are seen live. Fixes applied and each affected
Rust api crate re-checked with `cargo check` (all clean).

### Applied fixes

- **CORS startup panic - all six apps.** Every app built a mirror-origin +
  credentials CORS config for local/dev and passed it to `cors_layer`, which
  now panics on that combination. Switched each to `cors_layer_for_env(config,
  env)`:
  - `underlay-reference` `acme-api/crates/api/src/routes/mod.rs`
  - `contact-patch` `cp-api/crates/api/src/routes/mod.rs`
  - `compli-me` `api/crates/api/src/routes/mod.rs`
  - `acowtancy` `farmyard/crates/api/src/main.rs`
  - `songsprout` `nursery/crates/api/src/bootstrap.rs`
  - `loophole/composer` `composer-api/crates/api/src/main.rs`
  (Apps whose env enum re-exports `underlay_observability::Environment` pass
  `config.env` directly; the rest map a string env to the enum.)
- **Client-IP regression - underlay-reference.** `ctx.ip_address()` in
  `acme-api` returned `None` because the server ran without `ConnectInfo` and
  never installed a `TrustedProxyConfig` extension. Added
  `into_make_service_with_connect_info::<SocketAddr>()` and an extension
  mapped from the app's own trusted-proxy config.
- **Error-header tests - acowtancy.** Two `farmyard-api` tests asserted the
  removed `x-error-message`/`x-error-context` headers; updated to assert the
  headers are absent and read the message/context from the `ErrorDetail`
  response extension instead.
- **Open redirect - acowtancy.** `dairy` login `resolveReturnTo` guarded only
  with `startsWith("/")`, which still allows `//evil.com`. Routed through
  underlay `resolveRedirectTo()`.

### Advisory (no code change; verify at deploy)

- **Upload SVG allowlists** (`underlay-reference`, `contact-patch`,
  `loophole/composer`): app-owned server MIME validators still accept
  `image/svg+xml`. Their own validators keep working, but Underlay's client
  pipeline no longer offers SVG and its `finalise_upload_verified` would
  reject it. Removing SVG is a product decision - left to each app. If SVG is
  genuinely needed, serve it as an attachment / sandboxed origin.
- **In-memory rate limiter** (all apps): `InMemoryBackend` is now
  single-instance/non-prod. Apps with a Redis/distributed option
  (`contact-patch`) must set it; others must move to `PostgresBackend` before
  running more than one replica.
- **`plain:` OAuth secrets** (apps using `OAuthTokenCipher`): a stored
  `plain:`-prefixed secret is now rejected. Verify the deployed
  `AUTH_OAUTH_SECRET_KEY`-encrypted values, or add
  `.with_plain_migration(true)` for a bounded re-encryption window.

### Cleared (no action)

Session-token reads (all apps read tokens off login/refresh, not session-GET,
which still carry tokens); post-login redirect elsewhere (hardcoded paths);
error-header reads (none outside the two acowtancy tests); client-IP in apps
that roll their own header parsing.

## Consumer Upgrade Note (fleet)

Impact class: **breaking** (CORS build call) + **configuration** (rate-limit
backend, trusted proxy, oauth secret).

Required actions when bumping to g08 Underlay:

1. Replace `underlay_http::cors_layer(cfg)` with
   `cors_layer_for_env(cfg, env)` anywhere a mirror-origin + credentials CORS
   config can be built (all six apps needed this).
2. If you read `RequestContext::ip_address()`, install a `TrustedProxyConfig`
   request extension and serve with
   `into_make_service_with_connect_info::<SocketAddr>()`; otherwise the IP is
   `None`.
3. Multi-instance deployments: configure `PostgresBackend` (feature
   `postgres`) or another shared rate-limit backend.
4. If any stored OAuth secret is `plain:`-prefixed, add
   `.with_plain_migration(true)` while you re-encrypt, then remove it.
5. Post-login redirects from a query param must go through
   `resolveRedirectTo()`.
6. Uploads: SVG/HTML/JS are rejected by the shared pipeline by default;
   opt in server-side only with attachment/sandboxed serving.

Validation: `effigy validate` (Underlay); per app `cargo check` on the api
crate and the app's own test suite.

Links: contracts `020`/`030`/`031`/`040`, guide `068`, roadmap `g08`
Batches 1-3 logs.

## Current State

Underlay `effigy validate` green. All six consumer api crates `cargo check`
clean against the g08 tree with the fixes applied. Lane A + Lane B complete;
g08 acceptance criteria 1-4 ticked.

## Next

Lane C `g08.015` Rust error taxonomy (structural debt).
