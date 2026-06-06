# g06.198 - Rate Limit Config Field Retirement

## Status

Complete.

## Scope

Close the `underlay_ratelimit::RateLimitConfig` public-field compatibility
boundary.

## Change

- Made `RateLimitConfig` fields private.
- Added `max_requests()` and `window()` accessors.
- Kept `window_seconds()` and constructor helpers.
- Updated Underlay's in-memory backend and default backend trait helper to use
  accessors.
- Updated the known custom consumer backends and middleware field reads in:
  - `underlay-reference`
  - `contact-patch`
  - `loophole/composer`

## Compatibility

Impact: coordinated breaking change.

Known consumers that read `RateLimitConfig` fields were migrated first. New
apps must use constructors and accessors instead of direct field reads.

## Validation

- `cargo test -p underlay-ratelimit`
- `effigy rust:check`
- `cargo check -p acme-auth`
- `cargo check -p cp-auth`
- `cargo check -p compli-me-auth`
- `cargo check -p farmyard-auth`
- `cargo check -p nursery-auth`
- `cargo check -p composer-api`
