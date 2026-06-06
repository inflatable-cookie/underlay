# g06.207 - Job Runner Config Field Retirement

## Status

Complete.

## Scope

Close the `underlay_jobs::JobRunnerConfig` public-field compatibility boundary.

## Change

- Made `JobRunnerConfig` fields private.
- Added read-only accessors for poll interval and batch size.
- Added builder-style setters for poll interval and batch size.
- Updated Underlay runner internals, Postgres notifier integration, tests, and
  guide examples.
- Migrated known consumer job-worker configuration in:
  - `underlay-reference`
  - `contact-patch`
  - `compli-me`
  - `acowtancy`
  - `songsprout`

## Compatibility

Impact: coordinated breaking change.

Known consumer struct literals were migrated. New apps must use
`JobRunnerConfig::default()` with builders and accessors instead of direct
field reads or struct literals.

## Validation

- `cargo test -p underlay-jobs`
- `cargo check -p underlay-jobs-postgres`
- `cargo check -p acme-jobs`
- `cargo check -p cp-jobs`
- `cargo check -p compli-me-jobs`
- `cargo check -p farmyard-jobs`
- `cargo check -p nursery-jobs`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
