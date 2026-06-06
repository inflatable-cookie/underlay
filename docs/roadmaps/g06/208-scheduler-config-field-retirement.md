# g06.208 - Scheduler Config Field Retirement

## Status

Complete.

## Scope

Close the `underlay_jobs::SchedulerConfig` public-field compatibility boundary.

## Change

- Made `SchedulerConfig` fields private.
- Kept the existing `with_tick_interval_secs` builder.
- Kept `tick_interval()` for `Duration` reads.
- Added `tick_interval_secs()` for raw seconds reads.

## Compatibility

Impact: breaking for unknown direct field users.

No known consumer code used `SchedulerConfig` struct literals or direct field
reads. New apps must use `SchedulerConfig::default()` or `SchedulerConfig::new()`
with builders and accessors.

## Validation

- `cargo test -p underlay-jobs`
- `cargo check -p underlay-jobs-postgres`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
