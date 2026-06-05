# g06.004 - HTTP Safe-Builder Consolidation

## Why

The Rust audit hardened auth and CSRF cookie construction by centralizing
validation in `underlay-http`.

That is the right shape, but consumers can still drift if they hand-build auth
or CSRF cookies instead of using the shared helpers.

## Goal

Make the shared HTTP cookie builders the primary consumer path and record the
remaining raw cookie construction surface before any deprecation.

## Scope

In scope:

- scan the six-consumer family for hand-built auth, refresh, and CSRF cookies
- migrate straightforward duplicated CSRF helpers to `underlay-http`
- keep `AuthCookieConfig`, `CookieName`, `CookiePath`, and `CookieDomain` as the
  central construction boundary
- document any consumer-specific cookie behavior that should remain app-owned
- classify impact under `023`

Out of scope:

- changing app route structure
- changing session persistence behavior
- retiring raw compatibility fields before all consumers have moved
- weakening SameSite/Secure validation for local convenience

## Contract References

- `020`: HTTP transport and server boundary
- `023`: release and compatibility rollout
- `030`: auth and session systems
- `122`: Rust public API inventory

## Consumer Upgrade Impact

Impact classification: `additive` for current shared builder APIs.

Any later raw-field retirement is `deprecation` and needs consumer proof before
landing.

## Acceptance Criteria

- consumer scan records remaining hand-built auth, refresh, and CSRF cookie
  construction
- direct duplicate CSRF helpers are migrated where the shared helper already
  matches behavior
- app-local cookie behavior that should remain outside Underlay is named
- targeted consumer checks run for every touched API workspace
- Underlay docs point new auth-cookie code at the safe builder path

## Consumer Proof

Scan scope:

- `underlay-reference/acme-api`
- `contact-patch/cp-api`
- `compli-me/api`
- `acowtancy/farmyard`
- `songsprout/nursery`
- `loophole/composer/composer-api`

Findings:

- no scanned API workspace has remaining route-local `Set-Cookie` string
  assembly for auth, refresh, logged-in, or CSRF cookies
- `underlay-reference` and `contact-patch` expose CSRF token routes and now use
  `underlay_http::set_csrf_cookie` / `extract_csrf_token`
- `compli-me`, `acowtancy/farmyard`, and `songsprout/nursery` use the shared
  auth-cookie helpers for auth session paths
- `loophole/composer/composer-api` did not show auth-cookie builder call sites
  in this scan
- remaining consumer startup config moved from compatibility setters
  `with_domain` / `with_cookie_prefix` to fallible typed setters where present

## Code Changes

- `underlay-reference/acme-api`: cookie domain and prefix config now use
  `try_with_domain` and `try_with_cookie_prefix`.
- `contact-patch/cp-api`: cookie domain and prefix config now use
  `try_with_domain` and `try_with_cookie_prefix`.
- `compli-me/api`: cookie domain and prefix config now use `try_with_domain`
  and `try_with_cookie_prefix`.
- `acowtancy/farmyard`: cookie domain config now uses `try_with_domain` and
  exits during startup on invalid configuration.
- `songsprout/nursery`: cookie config bootstrap now returns
  `AuthCookieError` and exits during startup on invalid configuration.
- `underlay-soft-delete`: legacy restore/purge macros now emit
  `sqlx::PgPool`, fixing a cross-crate macro expansion break found during
  Farmyard validation.

## Validation

- `cargo check -p acme-api`
- `cargo check -p cp-api`
- `cargo check -p compli-me-api`
- `cargo check -p farmyard-api`
- `cargo check -p nursery-api`
- `cargo test -p underlay-soft-delete --all-features`
- `git diff --check`

## Current State

`g06.004` is complete.

## Next Task

Execute `g06.005`: DB identifier and schema boundary normalization.
