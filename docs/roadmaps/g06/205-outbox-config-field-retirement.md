# g06.205 - Outbox Config Field Retirement

## Status

Complete.

## Scope

Close the `underlay_jobs_postgres::outbox::OutboxConfig` public-field
compatibility boundary.

## Change

- Made `OutboxConfig` fields private.
- Added read-only accessors for batch size and fallback interval.
- Kept existing builders as the supported mutation path.
- Updated outbox processor internals and crate tests to use accessors.

## Compatibility

Impact: coordinated breaking change.

No known consumer in the proof family directly read the Underlay
`OutboxConfig` fields. New apps must use builders and accessors instead of
direct field reads or struct literals.

## Validation

- `cargo test -p underlay-jobs-postgres`
- `effigy rust:check`
