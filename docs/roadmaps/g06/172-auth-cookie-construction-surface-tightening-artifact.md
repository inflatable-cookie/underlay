# g06.172 Artifact - Auth Cookie Construction Surface Tightening

## Result

`AuthCookieConfig` no longer exposes raw public fields.

The config keeps its existing builder API, but external consumers can no longer
construct arbitrary struct literals or mutate raw cookie domain, prefix, path,
secure, lifetime, or SameSite fields directly. Read access now goes through
explicit accessors.

## Change

- Changed `AuthCookieConfig` fields from public to crate-private.
- Added accessors for domain, secure flag, refresh token max age, SameSite,
  cookie prefix, and refresh token path.
- Updated cookie config doctest examples to use checked builders.
- Updated Underlay cookie tests to exercise the accessor surface.

## Consumer Rollout

Classification: breaking public API tightening with completed current-family
rollout for known direct-field users.

- `underlay-reference` commit `9369ebb`: migrated custom CSRF cookie helpers to
  `AuthCookieConfig` accessors.
- `underlay-reference` commit `a023e45`: migrated cookie domain and prefix setup
  to checked builders.
- `acowtancy/farmyard` commit `202daf1`: migrated cookie domain setup to checked
  builder with startup failure on invalid config.
- `acowtancy` commit `d7d0d7e`: updated the Farmyard pointer.

No unchecked `with_domain`, `with_cookie_prefix`, or `with_refresh_token_path`
calls remain in the current consumer family.

## Validation

- `cargo test -p underlay-http`: passed.
- `underlay-reference` `effigy acme-api/validate`: passed.
- `acowtancy/farmyard` `cargo check -p farmyard-api`: passed.
- `acowtancy` `effigy farmyard/validate`: blocked by existing unrelated clippy
  `too_many_arguments` findings after the build phase.
- Fleet-wide unchecked cookie setter scan: clean.

## Next Lane

Move to `g06.173`: Rust security hardening closeout audit.
