# 2026-07-17 - g08 Batch 2: trust boundary and edge posture

## Context

Lane A continuation after Batch 1 closed the XSS -> takeover chain. Batch 2
(`g08.004`-`g08.007`) repairs the client-input trust boundary: spoofable IP,
leaked error headers, permissive CORS, and untrusted upload content types.

## Changes

### g08.004 - Upload content-type, SVG, and size enforcement (underlay-blob)

- `BlobUploadConfig` gains a server-side MIME allowlist
  (`DEFAULT_ALLOWED_CONTENT_TYPES`: jpeg/png/gif/webp/avif/pdf; no SVG/HTML/JS)
  plus `validate_upload_request` (size cap + allowlist).
- New `BlobAdapterUploadExt`: `initiate_upload_validated` enforces config
  before signing; `finalise_upload_verified` re-checks size/allowlist and
  magic-byte-sniffs stored bytes (`sniff` module) so an HTML payload cannot
  land under an image label.
- S3 download disposition filename now RFC 6266-escaped via
  `content_disposition_attachment` (header-injection fix).
- Local adapter mime map no longer maps html/js (served as octet-stream).
- TS `ALLOWED_IMAGE_TYPES` drops SVG, adds avif; documented as UX hint only.
- Contract `040` records the enforcement boundary; SVG is server-side opt-in
  and must be attachment-served or sandboxed.
- Card evidence paths predated the media/blob split; enforcement lives in
  `underlay-blob`.

### g08.005 - Trusted-proxy IP resolution

- `TrustedProxyConfig` enum (default `None` = trust no forwarding headers,
  socket peer only; `CloudflareHeader`; `RealIpHeader`;
  `ForwardedFor { trusted_hops }` = rightmost-untrusted XFF hop). Installed as
  a request extension; `RequestContext` resolves through it with
  `ConnectInfo` fallback. Leftmost-XFF trust is gone.
- Email-only login rate limit (`rate_limit_email_max_attempts`, default
  30/window) checked before the `email:ip` key - IP rotation no longer yields
  unbounded per-account attempts.
- Security alerts: per-account (`LoginFailuresForAccount`,
  `DistributedFailuresForAccount`) and global (`GlobalLoginFailureSurge`)
  signals, scoped store helpers + cooldown, migration
  `0002__security_alert_scopes.sql`.
- Contract `020` trusted-proxy rule; guide `068` rewritten off the spoofable
  extract-first-XFF pattern.

### g08.006 - Internal error-header leak

- `ApiError::into_response` carries message/context in response extensions
  (`ErrorDetail`), not headers. Logging middleware reads extensions and
  strips legacy `x-error-message`/`x-error-context` from every response.
  `x-error-code` (stable code) remains. Sanitized envelope is the only
  client-facing error surface.

### g08.007 - CORS mirror-origin gating

- `CorsConfig::default()` now allows no cross-origin access; wildcard is
  explicit opt-in.
- `try_cors_layer_for_env`/`cors_layer_for_env` refuse mirror+credentials
  outside `Environment::Local`/`Test`; env-less `cors_layer` panics on the
  combination. Wildcard-with-credentials cannot activate in prod.
- Contract `031` safe-prod-CORS rule; guides `066`/`070` corrected (both
  showed stale struct-literal config; `066` showed the dangerous
  mirror-fallback as the deploy pattern).

## Validation

- `cargo test --workspace --all-features`: green (73 suites, 0 failures).
- `cargo check --workspace --all-features`: clean.
- New tests: spoofed-XFF invariance, email-only limit under IP rotation,
  account/global alert evaluators, mirror+credentials env rejection (+ panic
  path), no-internal-detail-headers assertions, magic-byte sniff matrix,
  validated initiate/finalise (HTML-as-PNG rejected).
- `effigy validate` remains blocked by pre-existing red navigation unit tests
  (owned by `g08.014`).

## Consumer Upgrade Notes

- `configuration` impact (`g08.005`, `g08.007`): consumers must declare
  `TrustedProxyConfig` to keep header-derived client IPs (default now trusts
  none), and must declare explicit CORS origins (default now allows no cross
  origin; mirror+credentials is Local/Test-only).
- `behavioral` impact (`g08.004`, `g08.006`): uploads now enforce a server
  size cap, MIME allowlist (no SVG by default), and magic-byte verification;
  `x-error-message`/`x-error-context` headers no longer ship - read logs or
  the error envelope.
- All require six-consumer proof per `023` before release.

## Next

`g08` Batch 3 (Lane A): `g08.008` distributed rate-limit backend, `g08.009`
http-client SSRF/timeout defaults, `g08.010` auth hardening batch. Lane B
(`g08.011`-`g08.014`) still ready in parallel.
