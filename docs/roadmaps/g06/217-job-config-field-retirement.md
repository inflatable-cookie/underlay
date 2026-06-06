# g06.217 - Job Config Field Retirement

## Status

Complete.

## Scope

Close public-field compatibility boundaries for:

- `underlay_jobs::JobConfig`

## Change

- Made job config fields private.
- Added read-only accessors and row-driven builder helpers.
- Updated Underlay job and Postgres job internals.
- Migrated known consumer job config literals and direct config field tests.

## Compatibility

Impact: coordinated breaking change.

Known consumers were migrated to presets, builders, and accessors. New apps
must use those APIs instead of direct field reads or struct literals.

## Validation

- `cargo test -p underlay-jobs`
- `cargo check -p underlay-jobs-postgres`
- `cargo check -p acme-jobs`
- `cargo check -p acme-api`
- `cargo check -p cp-jobs`
- `cargo check -p cp-api`
- `cargo check -p compli-me-jobs`
- `cargo check -p compli-me-api`
- `cargo check -p nursery-api`
- `cargo check -p composer-api`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
