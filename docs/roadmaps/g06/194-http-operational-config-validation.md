# g06.194 - HTTP Operational Config Validation

## Status

Complete.

## Scope

Add checked HTTP operational config paths without breaking current consumers.

## Change

- Added `CorsConfigError`.
- Added `CorsConfig::try_with_origins` so external origin strings fail fast
  instead of being silently dropped.
- Added `CorsConfig::with_origin_values` for apps that already parse origin
  header values at their config edge.
- Added read-only `CorsConfig` accessors as the migration path away from public
  field reads.
- Added `HttpServerConfigError`.
- Added `HttpServerConfig::try_new` and `HttpServerConfig::try_from_env` for
  checked bind address, port, and public host handling.
- Added read-only `HttpServerConfig` accessors as the migration path away from
  public field reads.

## Compatibility

Impact: additive.

Existing `CorsConfig` and `HttpServerConfig` public fields remain available for
the current consumer family. This artifact creates the replacement API but does
not yet remove struct literals or direct field reads.

Consumer apps should migrate CORS construction to:

- `CorsConfig::try_with_origins` for external string origins.
- `CorsConfig::with_origin_values` for already parsed `HeaderValue` origins.
- Accessors instead of direct field reads.

Apps using `HttpServerConfig` should migrate env-facing paths to
`HttpServerConfig::try_from_env` and direct constructors to
`HttpServerConfig::try_new`.

## Security

The checked paths reject invalid operational inputs before they can leak into
runtime behavior:

- malformed CORS origins no longer disappear silently on strict construction;
- bind addresses must parse as IP addresses;
- invalid ports fail instead of falling back to the default on strict env load;
- public hosts cannot include schemes, path/query/fragment content, userinfo, or
  whitespace.

## Validation

- `cargo test -p underlay-http --all-features`
