# g06.170 Artifact - Config Environment Filename Boundary Hardening

## Result

`underlay-config::ConfigStack` now validates environment and local overlay names
before resolving optional overlay files.

The public builder API is unchanged. Valid consumers can keep using
`with_environment()`, `with_environment_from_env()`, and
`with_optional_local_overlay()` as before.

## Change

- Added `ConfigError::InvalidOverlayName`.
- Rejected empty overlay names at the resolution boundary.
- Rejected leading or trailing whitespace.
- Rejected `.` and `..`.
- Rejected `/` and `\` path separators.
- Rejected control characters.
- Added focused tests for path-like environment names and dot local overlays.

## Consumer Impact

Classification: additive hardening with narrow breaking behavior for invalid
configuration names.

The known consumer family uses `with_environment_from_env()` plus `"local"` and
does not require source changes for normal `dev`, `uat`, `production`, or
similar environment names.

## Validation

- `cargo test -p underlay-config`: passed.

## Next Lane

Move to `g06.171`: runtime mutex poison availability hardening.
