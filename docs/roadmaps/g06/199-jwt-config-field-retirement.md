# g06.199 - JWT Config Field Retirement

## Status

Complete.

## Scope

Close the `underlay_auth_jwt::JwtConfig` public-field compatibility boundary.

## Change

- Made `JwtConfig` fields private, including the base64 key material fields.
- Added read-only accessors for key material and token behavior values.
- Added builder-style behavior setters for lifetime, issuer, audience, and
  validation leeway.
- Kept `from_env`, `from_env_with_defaults`, `from_values`, and `generate` as
  the supported construction paths.
- Kept `JwtBehaviorDefaults` as the non-secret typed app-config literal shape.
- Updated JWT service internals, crate tests, and known consumer key-generation
  helpers to use accessors.

## Compatibility

Impact: coordinated breaking change.

Known consumers that read generated JWT key fields were migrated to accessors.
New apps must use constructors, builders, and accessors instead of direct
`JwtConfig` field reads or struct literals.

## Validation

- `cargo test -p underlay-auth-jwt`
