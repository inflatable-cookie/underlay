# g06.061 Artifact - HTTP Cookies Public Model Modularity Audit

## Summary

`underlay-http/src/cookies.rs` is stable app-facing auth and CSRF cookie
surface with a safe internal split shape if both public front doors stay
intact.

The file currently groups:

- cookie policy: `SameSite`
- typed values: `CookieName`, `CookiePath`, `CookieDomain`
- config and builders: `AuthCookieConfig`
- individual cookie header builders:
  `refresh_token_cookie`, `clear_refresh_token_cookie`, `logged_in_cookie`,
  `clear_logged_in_cookie`, `csrf_token_cookie`, `clear_csrf_token_cookie`
- extractors: `extract_refresh_token`, `extract_csrf_token`,
  `extract_refresh_token_default`
- response header append helpers: `set_auth_cookies`, `set_csrf_cookie`,
  `clear_auth_cookies`, `clear_csrf_cookie`
- validation export: `AuthCookieError`

## Consumer Evidence

Consumer usage is broad and uses both public paths:

- `underlay_http::AuthCookieConfig`
- `underlay_http::{clear_auth_cookies, extract_refresh_token, set_auth_cookies}`
- `underlay_http::{set_csrf_cookie, extract_csrf_token}`
- `underlay_http::cookies::SameSite`

Observed consumer behavior:

- Underlay Reference and Contact Patch use SameSite through
  `underlay_http::cookies::SameSite`.
- Contact Patch, Compli-me, Underlay Reference, and Acowtancy store
  `AuthCookieConfig` in app state and use crate-root auth-cookie helpers.
- Contact Patch and Compli-me use fallible typed setters for domain and cookie
  prefix.
- Underlay Reference and Acowtancy still use raw setters for domain and/or
  prefix in at least one startup path.
- Songsprout stores `AuthCookieConfig` and uses `local_dev`, `default`,
  `with_refresh_token_max_age`, and `try_with_domain`.
- No direct Loophole/Composer cookie helper usage was found in this scan.

## Decision

Queue `g06.062` as an HTTP cookies internal split.

The split should preserve:

- `underlay_http::cookies::*` compatibility
- crate-root cookie exports from `underlay-http/src/lib.rs`
- public `AuthCookieConfig` fields and builder names
- raw builder compatibility for `with_domain`, `with_cookie_prefix`, and
  `with_refresh_token_path`
- fallible typed builder behavior
- cookie header strings, names, paths, domains, SameSite, Secure, HttpOnly, and
  Max-Age behavior
- extractor behavior for malformed/missing/empty values
- `AuthCookieError` path and conversions

## Public API Impact

Expected impact for the split: none.

Any move to retire raw config setters or hide public config fields would be a
separate breaking consumer rollout, not part of the internal split.

## Validation

- `cargo test -p underlay-http --all-features`

Next code batch validation:

- `cargo test -p underlay-http --all-features`
- `effigy rust:check`
- consumer checks only if public import paths or raw setters move
- `effigy qa:docs`
- `effigy qa:northstar`
