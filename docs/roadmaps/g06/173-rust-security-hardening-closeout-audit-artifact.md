# g06.173 Artifact - Rust Security Hardening Closeout Audit

## Result

The Rust security-hardening batch is complete for the issues found in
`g06.169`.

The main runtime construction and availability gaps were repaired:

- config overlay names are bounded before optional file resolution
- runtime helper mutex poison no longer panics in cache, circuit-breaker, or dev
  email capture paths
- auth cookie config raw fields are no longer public, and current consumers use
  accessors or checked builders

## Evidence

- Production panic scan no longer finds the repaired mutex poison sites.
- Current consumer scan has no unchecked `with_domain`, `with_cookie_prefix`, or
  `with_refresh_token_path` calls.
- Current consumer scan has no direct `AuthCookieConfig` field reads except the
  intentional accessor calls in `underlay-reference`.
- Consumer rollout commits are recorded in `g06.172`.

## Remaining Findings

`underlay-media-postgres::PostgresMediaConfig::with_schema` remains a public
panic-on-invalid constructor beside `try_with_schema`.

No current consumer uses it, so this is a clean next card: remove or deprecate
the unchecked constructor, keep `Default` and `try_with_schema`, and update the
crate tests.

Other remaining production scan hits are invariant-only or test/doc residue:

- fixed-size SHA digest slice conversion
- static regex/HMAC construction invariants
- single-flight local loader invariant
- inline tests in implementation files

## Validation

Validation from the hardening batch:

- `cargo test -p underlay-http`: passed
- `cargo test -p underlay-config`: passed
- `cargo test -p underlay-http -p underlay-ai-runtime -p underlay-email`:
  passed
- `effigy rust:check`: passed
- `effigy qa:docs`: passed
- `effigy qa:northstar`: passed
- `effigy doctor`: passed with the known 9 warning-only test-size findings

Consumer validation:

- `underlay-reference` `effigy acme-api/validate`: passed
- `acowtancy/farmyard` `cargo check -p farmyard-api`: passed
- `acowtancy` `effigy farmyard/validate`: blocked by existing unrelated clippy
  `too_many_arguments` findings after the build phase

## Next Lane

Move to `g06.174`: media Postgres config unchecked constructor retirement.
