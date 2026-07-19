# Trusted-proxy `X-Forwarded-For` resolution — foundation + fleet rollout

Date: 2026-07-19
Scope: underlay-http (foundation), compli-me, songsprout/nursery (consumers)
Governing refs: consumer-audit cards compli-me `g01.013`, songsprout `g02.002`

## Problem

The g08 consumer audit found two apps keying rate-limit / lockout / audit / 2FA
state on a client IP read straight from the first `X-Forwarded-For` hop with no
trusted-proxy validation. An attacker rotates the header to evade per-IP
throttles and to forge the 2FA session fingerprint. Both apps had neither
`ConnectInfo` wiring nor a trusted-proxy config, so the fix was deferred from the
in-place audit pass as real request-pipeline work.

## Foundation (underlay-http)

`underlay_http::TrustedProxyConfig` (enum: `None` | `CloudflareHeader` |
`RealIpHeader` | `ForwardedFor { trusted_hops }`) and the `RequestContext`
extractor that resolves the IP from `ConnectInfo<SocketAddr>` + the installed
config already existed. Two additions made it consumable outside the extractor:

- **`TrustedProxyConfig::from_env()`** — env-driven constructor. `TRUSTED_PROXY`
  selects the mode (`none` default, `cloudflare`, `real-ip`, `forwarded-for`),
  `TRUSTED_PROXY_HOPS` sets the hop depth for `forwarded-for` (default 1). An
  unrecognised mode fail-closes to `None` (a typo cannot start trusting
  client-supplied headers) and warns when the `tracing` feature is on. A pure
  `parse_env` helper carries the logic and is unit-tested without touching
  process env.
- **`pub fn resolve_client_ip(headers, &config, socket_ip)`** — the resolver was
  `pub(in crate::context)`, usable only via the extractor. Exposed publicly so
  tower middleware and custom extractors holding the raw `Request`/`Parts` can
  resolve identically by pulling `ConnectInfo` + `TrustedProxyConfig` from the
  extensions.

Tests: `context_tests.rs` gained `test_trusted_proxy_from_env_parsing` (all mode
aliases, hop parsing, fail-closed on typo) and
`test_public_resolve_client_ip_matches_extractor_resolution`. `cargo test -p
underlay-http` green (12 context tests); clippy `--all-features --all-targets`
clean.

## Consumer wiring

Both apps: serve with `into_make_service_with_connect_info::<SocketAddr>()` and
install `Extension(TrustedProxyConfig::from_env())`.

- **compli-me** — the three helpers in `routes/shared/auth/helpers.rs` source
  IP+UA from the `RequestContext` extractor; six handlers (login/session/
  passkeys) take `RequestContext` instead of `HeaderMap`.
- **songsprout/nursery** — `middleware.rs::extract_client_ip` and the auth
  crate's `get_client_ip`/`get_client_ip_from_parts` resolve from request/parts
  extensions (the `crates/auth` crate gained an `underlay-http` dep);
  `handlers/auth.rs` `audit_ip`/`audit_ua`/`extract_session_fingerprint` source
  from `RequestContext` (7 handlers switched off `HeaderMap`).

`cargo check --workspace --all-features --all-targets` green in both apps. A
repo-wide grep confirms no raw `x-forwarded-for` / `x-real-ip` header reads
remain in either app's non-test code.

## Deployment note

Default (`TRUSTED_PROXY` unset) trusts no forwarding headers and uses the socket
peer. Deployments fronted by a proxy must set `TRUSTED_PROXY` to match the
topology or every client will resolve to the proxy's IP. The remaining deferred
items on both cards (refresh-family revocation; prod S3 on songsprout) are
unaffected.
