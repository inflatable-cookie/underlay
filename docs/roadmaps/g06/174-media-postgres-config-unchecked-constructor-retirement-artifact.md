# g06.174 Artifact - Media Postgres Config Unchecked Constructor Retirement

## Result

`underlay-media-postgres::PostgresMediaConfig::with_schema` has been retired.

The crate now exposes only the fallible `try_with_schema` constructor for custom
schema names. `Default` still uses the known-valid built-in `media` schema.

## Change

- Removed the public panic-on-invalid `with_schema` constructor.
- Removed the panic expectation test for that constructor.
- Kept `try_with_schema` and `try_with_tables` as the validated construction
  surface.

## Consumer Impact

Classification: breaking public API tightening with no current-family caller
impact.

The current consumer family does not call `PostgresMediaConfig::with_schema`.
Callers that need a custom schema should use `try_with_schema`.

## Validation

- `cargo test -p underlay-media-postgres`: passed.
- Current-family and Underlay source scan found no `PostgresMediaConfig::with_schema`
  callers.

## Next Lane

Move to `g06.175`: post-hardening Rust quality checkpoint.
