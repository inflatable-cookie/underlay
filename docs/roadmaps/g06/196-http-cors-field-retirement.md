# g06.196 - HTTP CORS Field Retirement

## Status

Complete.

## Scope

Close the `underlay_http::CorsConfig` struct-literal compatibility boundary
after the six known consumers migrated to builders and accessors in `g06.195`.

## Change

- Made `CorsConfig` fields private.
- Kept the public builder/accessor surface:
  - `with_any_origin`
  - `with_mirror_origin`
  - `with_origins`
  - `try_with_origins`
  - `with_origin_values`
  - `with_header`
  - `with_headers`
  - `with_credentials`
  - `with_max_age`
  - read-only accessors for each config value
- Updated Underlay tests to assert through the public accessors.

## Compatibility

Impact: coordinated breaking change.

The six known consumers were migrated first and compile against the private-field
shape. New consuming apps must use the builder/accessor API instead of
constructing `CorsConfig` with public fields.

## Validation

- `cargo test -p underlay-http --all-features`
- `effigy rust:check`
- `cargo check -p acme-api`
- `cargo check -p cp-api`
- `cargo check -p compli-me-api`
- `cargo check -p farmyard-api`
- `cargo check -p nursery-api`
- `cargo check -p composer-api`
