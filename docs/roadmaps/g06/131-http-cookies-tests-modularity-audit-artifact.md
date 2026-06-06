# g06.131 Artifact - HTTP Cookies Tests Modularity Audit

## Summary

`underlay-http/src/tests/cookies_tests.rs` is the next Rust warning-level file
after `g06.130`. It validates auth and CSRF cookie generation, extraction,
configuration, typed wrappers, validation failures, and header append/clear
behavior.

The current file groups:

- refresh-token cookie generation
- logged-in cookie generation
- CSRF cookie generation
- refresh-token extraction
- CSRF-token extraction
- cookie prefix behavior
- SameSite and path customization
- invalid prefix/path/domain/value rejection
- chained config builders
- typed cookie domain/path/name wrappers
- try-builder validation
- `Set-Cookie` append/clear behavior

## Boundary Evidence

The parent test module is declared from `cookies.rs`:

- `#[path = "tests/cookies_tests.rs"] mod tests;`

The split can preserve the parent module by replacing the flat file with a
`tests/cookies_tests/` module directory and updating the path to
`tests/cookies_tests/mod.rs`.

The tests currently use `super::*`, so the new module front door can keep the
same imports and expose only local test modules.

## Behavior Evidence

Existing tests cover security-sensitive cookie behavior:

- refresh token cookie is HttpOnly, Secure by default, SameSite=Lax, scoped to
  `/v1/auth`, and has Max-Age
- local dev refresh token cookie omits Secure
- logged-in and CSRF cookies are not HttpOnly
- CSRF cookie remains Secure and SameSite=Lax by default
- extraction handles present, missing, no-header, and prefixed cookie names
- SameSite=None is rejected when Secure is false
- invalid cookie prefix, path, domain, domain label, value, and empty value are
  rejected
- typed cookie wrappers validate at construction
- try-builders validate early
- CSRF set/clear appends two `Set-Cookie` headers and clears with Max-Age=0

Baseline validation:

- `cargo test -p underlay-http cookies --all-features`
- 25 tests passed

## Decision

Queue `g06.132` as an HTTP cookies tests internal split.

Suggested module shape:

- `tests/cookies_tests/mod.rs`: shared imports and child module declarations
- `tests/cookies_tests/builders.rs`: refresh, logged-in, CSRF, SameSite, path,
  and value validation tests
- `tests/cookies_tests/extractors.rs`: refresh and CSRF extraction tests
- `tests/cookies_tests/config.rs`: prefix, builder, typed wrapper, domain/path,
  and try-builder validation tests
- `tests/cookies_tests/headers.rs`: set/clear header append behavior

This keeps behavior coverage intact while reducing the flat test file.

## Public API Impact

Expected impact: none.

This is test-only. If preserving tests requires changing cookie APIs or cookie
behavior, stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-http cookies --all-features`
- `cargo test -p underlay-http --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
