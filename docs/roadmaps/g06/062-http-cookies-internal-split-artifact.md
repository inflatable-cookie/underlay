# g06.062 Artifact - HTTP Cookies Internal Split

## Summary

`underlay-http/src/cookies.rs` is now a small public front door over focused
private modules.

New private module layout:

- `cookies/policy.rs`: `SameSite`
- `cookies/typed_values.rs`: `CookieName`, `CookiePath`, `CookieDomain`
- `cookies/config.rs`: `AuthCookieConfig`
- `cookies/builders.rs`: individual Set-Cookie string builders
- `cookies/extractors.rs`: cookie header extractors
- `cookies/headers.rs`: response header append helpers
- `cookies/validation.rs`: validation helpers and `AuthCookieError`

## Compatibility

The split preserves:

- `underlay_http::cookies::*`
- crate-root exports from `underlay-http/src/lib.rs`
- public `AuthCookieConfig` fields
- raw setters: `with_domain`, `with_cookie_prefix`,
  `with_refresh_token_path`
- typed setters and fallible typed setters
- cookie header strings, names, paths, domains, SameSite, Secure, HttpOnly, and
  Max-Age behavior
- extractor behavior for malformed/missing/empty values
- `AuthCookieError` path and conversion behavior

## Public API Impact

Expected impact: none.

This was a private module split only. Raw config setters and public config
fields remain in place because current consumers still use them.

## Validation

- `cargo test -p underlay-http --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` failed on known structural backlog:
  `scan.attention-markers`, `scan.comment-ratio`, and `scan.god-files`

Structural movement:

- `underlay-http/src/cookies.rs`: 569 lines to 30 lines
- `scan.god-files`: 57 findings to 56 findings

Next batch validation:

- `cargo test -p underlay-http --all-features`
- `effigy rust:check`
- consumer checks only if public import paths move
- `effigy qa:docs`
- `effigy qa:northstar`
