# g08.019 - Postgres Adapter Integration Tests

Status: blocked
Owner: repo maintainers
Started: 2026-07-17
Completed:

## Blocker (2026-07-17)

Stopped per the stop condition. The `TestDb` harness is testcontainers-backed
and needs **Docker**; the local environment has no Docker, no standalone
Postgres, and no `DATABASE_URL`. Integration tests written here could not be
run or validated, and the card's validation ("adapter passes contract tests
against Postgres") cannot be met without a Postgres service. Writing
unrunnable tests would ship unverified SQL assumptions, so this card is parked
pending the CI/Postgres provisioning decision in `g08.025`. Unblock: a CI job
(or local Docker/Postgres) that runs migrations + these adapter tests against
real Postgres.

## Purpose

Cover the untested DB adapter crates. Test counts: `underlay-auth-state-postgres` (now has a table-name validation test; renamed in g08.018),
`underlay-jobs-postgres` 4, `underlay-security-alerts` 5, `underlay-audit` 7,
`underlay-media-postgres` 7 - exactly the crates full of hand-written SQL (66
runtime `sqlx::query(...)` calls, zero compile-checked `query!` macros
workspace-wide, so nothing validates SQL against the schema at build time). The
`underlay-testing::TestDb` schema-per-test harness already exists and is good.

## Evidence

- `underlay-testing::TestDb` (existing harness)
- adapter crates listed above; hardcoded `platform.` schema in
  `underlay-jobs-postgres` (see `g08.020` follow-up)

## Governing References

- [022 Testing posture and shared harnesses](../../contracts/022-testing-posture-and-shared-harnesses.md)
- [021 Database migration and schema workflow](../../contracts/021-database-migration-and-schema-workflow.md)

## Planned Changes

- [ ] Add testcontainers-backed integration tests for each `-postgres` adapter
  using `TestDb`.
- [ ] Consider `sqlx::query!` offline mode (or a CI job running migrations +
  adapter tests against real Postgres) so SQL is schema-checked.
- [ ] Regenerate or delete the stale root `tarpaulin-report.html` (covered in
  `g08.026`).

## Consumer Upgrade Impact

Impact class: `none`.

## Validation

- [ ] each `-postgres` adapter has integration coverage of its main queries
- [ ] `cargo test -p underlay-auth-state-postgres -p underlay-jobs-postgres -p underlay-media-postgres -p underlay-audit -p underlay-security-alerts`
- [ ] `effigy validate`

## Stop Conditions

Stop if CI lacks a Postgres service; that is a CI-provisioning decision to raise
(see `g08.025` on the missing CI story).

## Next Task

`g08.020` workspace dependency and lint hygiene.
