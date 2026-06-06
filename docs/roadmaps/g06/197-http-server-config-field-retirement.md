# g06.197 - HTTP Server Config Field Retirement

## Status

Complete.

## Scope

Close the `underlay_http::HttpServerConfig` struct-field compatibility boundary
after `g06.194` introduced checked constructors and accessors.

## Change

- Made `HttpServerConfig` fields private.
- Kept the public constructor/accessor surface:
  - `new`
  - `try_new`
  - `from_env`
  - `try_from_env`
  - `socket_addr`
  - `base_url`
  - `http_base_url`
  - `https_base_url`
  - read-only accessors for bind address, port, and public host
- Updated docs and tests to use methods instead of fields.

## Compatibility

Impact: coordinated breaking change.

The six known consumers do not use `underlay_http::HttpServerConfig` directly.
New apps must use methods instead of public field reads.

## Validation

- `cargo test -p underlay-http --all-features`
- `effigy rust:check`
- exact consumer scan for `underlay_http::HttpServerConfig` and
  `HttpServerConfig` construction
