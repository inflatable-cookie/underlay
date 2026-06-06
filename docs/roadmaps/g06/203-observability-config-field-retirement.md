# g06.203 - Observability Config Field Retirement

## Status

Complete.

## Scope

Close the `underlay_observability::ObservabilityConfig` public-field
compatibility boundary.

## Change

- Made `ObservabilityConfig` fields private.
- Added read-only accessors for fallback level, format, and environment.
- Kept existing builders as the supported mutation path.
- Updated tracing initialization internals and crate tests to use accessors.

## Compatibility

Impact: coordinated breaking change.

No known consumer in the proof family directly read the Underlay
`ObservabilityConfig` fields. New apps must use builders and accessors instead
of direct field reads or struct literals.

## Validation

- `cargo test -p underlay-observability`
- `effigy rust:check`
