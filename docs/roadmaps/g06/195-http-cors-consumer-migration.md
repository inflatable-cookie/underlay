# g06.195 - HTTP CORS Consumer Migration

## Status

Complete.

## Scope

Move the six known consumers onto the additive CORS API introduced in
`g06.194`.

## Change

- Replaced direct `underlay_http::CorsConfig` field writes in:
  - `underlay-reference`
  - `contact-patch`
  - `compli-me`
  - `acowtancy/farmyard`
  - `songsprout/nursery`
  - `loophole/composer`
- Used `CorsConfig::with_origin_values` where apps already parse origin header
  values.
- Used `CorsConfig::try_with_origins` where app config still carries external
  origin strings.
- Left app-local `CorsConfig` structs unchanged; those are consumer config
  models, not Underlay public-field usage.

## Compatibility

Impact: additive consumer migration.

No Underlay API was removed in this artifact. The known consumers no longer need
direct `underlay_http::CorsConfig` field construction for their CORS layer
conversion paths.

## Validation

- `cargo check -p acme-api`
- `cargo check -p cp-api`
- `cargo check -p compli-me-api`
- `cargo check -p farmyard-api`
- `cargo check -p nursery-api`
- `cargo check -p composer-api`
