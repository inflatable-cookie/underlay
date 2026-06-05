# g06.056 Artifact - Jobs Types Internal Split

## Summary

`underlay-jobs/src/types.rs` is now a small public front door over private jobs
type modules.

New internal modules:

- `types/ids.rs`
- `types/status.rs`
- `types/backoff.rs`
- `types/config.rs`
- `types/records.rs`
- `types/dead_letters.rs`
- `types/scheduled.rs`
- `types/filters.rs`
- `types/handlers.rs`

The split preserves existing `underlay_jobs` root exports and
`underlay_jobs::types::*` compatibility.

## Public API Impact

Impact: none expected.

No handler, store, runner, scheduler, or Postgres adapter behavior changed.
Serialized job, dead-letter, scheduled-task, progress, and error-record shapes
remain unchanged.

The only test update made `Duration` and `Utc` imports explicit in
`types_tests.rs`; those names were previously inherited from the monolithic
parent module.

## Structural Impact

`types.rs` moved from a 659-line monolith to a 25-line front door.

Largest new type modules:

- `types/config.rs`: 215 lines
- `types/handlers.rs`: 97 lines
- `types/dead_letters.rs`: 84 lines
- `types/backoff.rs`: 78 lines
- `types/records.rs`: 76 lines

`effigy doctor` still fails on the known structural backlog, but the god-file
scan moved from 60 findings and 16 errors after `g06.054` to 59 findings and
15 errors.

## Validation

- `cargo test -p underlay-jobs --all-features`
- `cargo test -p underlay-jobs-postgres --all-features`
- `effigy rust:check`
- `effigy doctor` - expected structural backlog failure, with one fewer
  god-file error
- `effigy qa:docs`
- `effigy qa:northstar`
