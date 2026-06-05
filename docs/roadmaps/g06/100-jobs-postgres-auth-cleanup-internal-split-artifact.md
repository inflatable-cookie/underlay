# g06.100 Artifact - Jobs Postgres Auth Cleanup Internal Split

## Summary

`underlay-jobs-postgres/src/tasks/auth_cleanup.rs` is now a small module front
door with stable public re-exports. The former mixed auth cleanup module was
split into focused modules.

New module shape:

- `auth_cleanup.rs`: front door and public re-exports
- `auth_cleanup/purge.rs`: purge handlers for expired sessions, auth states,
  login attempts, rate-limit entries, email TOTP codes, and verification
  sessions
- `auth_cleanup/inactive_accounts.rs`: inactive-account suspension handler and
  policy builder methods

## Public API Impact

None expected.

The `underlay_jobs_postgres::tasks` exports, job type strings, maintenance job
config, SQL statements, retention defaults, inactive-account defaults, role
filtering, clamp behavior, status strings, reason strings, and logging fields
were preserved.

## Validation

- `cargo test -p underlay-jobs-postgres --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`

`cargo test -p underlay-jobs-postgres --all-features` passed with 4 unit tests
passed and 4 ignored doc-tests.

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 38 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

The original auth cleanup file no longer appears in the god-file report. The
next largest Rust production warning is
`rust/crates/underlay-auth-password/src/service.rs`.

## Next Target Evidence

Queue `g06.101` as an auth password service modularity audit before splitting
`underlay-auth-password/src/service.rs`. Password auth is security-sensitive,
so the next batch should classify hashing, policy validation, login/lockout,
password changes, reset flows, and public service methods before moving code.
