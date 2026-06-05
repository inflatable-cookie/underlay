# g06.099 Artifact - Jobs Postgres Auth Cleanup Modularity Audit

## Summary

`underlay-jobs-postgres/src/tasks/auth_cleanup.rs` is the largest remaining
Rust warning-level production file. It is a single public task module that
combines standard auth cleanup job handlers and inactive-account suspension
logic.

The current surface groups:

- `PurgeExpiredSessionsJob`
- `PurgeAuthStatesJob`
- `PurgeLoginAttemptsJob`
- `PurgeRateLimitEntriesJob`
- `PurgeEmailTotpCodesJob`
- `PurgeVerificationSessionsJob`
- `SuspendInactiveAccountsJob`

The first six handlers are simple purge jobs with stable job type strings and
single SQL statements. `SuspendInactiveAccountsJob` carries custom policy
configuration, transaction handling, account status changes, and optional
active-session revocation.

## Public Surface Evidence

The public exports come through `underlay_jobs_postgres::tasks`:

- `PurgeExpiredSessionsJob`
- `PurgeAuthStatesJob`
- `PurgeLoginAttemptsJob`
- `PurgeRateLimitEntriesJob`
- `PurgeEmailTotpCodesJob`
- `PurgeVerificationSessionsJob`
- `SuspendInactiveAccountsJob`

Docs reference these names and job types:

- `docs/guides/055-background-jobs.md`
- `docs/guides/081-auth-security-alerting.md`

Stable job type strings:

- `purge_expired_sessions`
- `purge_auth_states`
- `purge_login_attempts`
- `purge_rate_limit_entries`
- `purge_email_totp_codes`
- `purge_verification_sessions`
- `suspend_inactive_accounts`

## Behavior Evidence

The crate test suite currently does not exercise `auth_cleanup.rs` directly.
Validation coverage is limited to compilation and the broader
`underlay-jobs-postgres` test surface:

- `cargo test -p underlay-jobs-postgres --all-features`
- 4 unit tests passed
- 4 doc-tests ignored

The split should therefore be conservative: preserve SQL strings, builder
defaults, job type strings, logging fields, and transaction structure.

## Decision

Queue `g06.100` as a jobs Postgres auth cleanup internal split.

The split should preserve:

- all `underlay_jobs_postgres::tasks` public exports
- all job type strings
- `JobConfig::maintenance()` behavior
- default login-attempt retention of 30 days
- inactive-account defaults: 1095 days, `student`/`tester`, batch limit 500,
  session revocation enabled
- inactive-account role trimming/filtering behavior
- inactive-account min clamps for days and batch limit
- all SQL statements and status/reason strings
- existing docs references

Suggested module shape:

- `auth_cleanup.rs`: module front door and public re-exports
- `auth_cleanup/purge.rs`: purge job handlers for sessions, auth states,
  login attempts, rate limits, email TOTP codes, and verification sessions
- `auth_cleanup/inactive_accounts.rs`: inactive-account suspension handler and
  policy builder methods

## Public API Impact

Expected impact: none.

This should be an internal split. If preserving task exports, job type strings,
or SQL behavior forces a public API change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-jobs-postgres --all-features`

Next code batch validation:

- `cargo test -p underlay-jobs-postgres --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
