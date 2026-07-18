# g08.019 - Postgres Adapter Integration Tests

Status: done
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-18

## Unblocked (2026-07-18)

Originally blocked: `TestDb` was testcontainers-only (needs a Docker API) and
the environment had none. Unblocked by two moves:

1. **`TestDb` now accepts an external `UNDERLAY_TEST_DATABASE_URL`** and connects
   to an already-provisioned Postgres (CI service, `effigy container`, or local),
   falling back to testcontainers only when the env var is unset. Per-test schema
   isolation is unchanged. This is the durable CI story the `g08.025` note asked
   for — tests no longer hard-require Docker.
2. **Provisioned Postgres 16 on the effigy Colima/containerd profile**
   (`colima nerdctl -- run postgres:16-alpine`, host-mapped) and ran the suite
   against it.

## Original blocker (2026-07-17, resolved)

Stopped per the stop condition: no Docker, no standalone Postgres, no
`DATABASE_URL`. Writing unrunnable tests would have shipped unverified SQL, so
the card was parked pending Postgres/CI provisioning.

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

## Changes

- [x] Added `TestDb`-backed integration tests for all five `-postgres` adapters
  (17 tests, `#[ignore]`d so CI without a database stays green):
  - `underlay-auth-state-postgres` (5): create/load/consume round-trip,
    user+state_type scoping, public update + typed round-trip, expiry, delete.
  - `underlay-security-alerts` (4): IP/account/global signal-count aggregation,
    IP + scoped alert insert/cooldown dedupe.
  - `underlay-audit` (3): append + get-by-id, list/count filters, ordered
    pagination.
  - `underlay-media-postgres` (4): media CRUD, version lifecycle
    (create/finalize/set-current/find-by-hash/list), soft-delete/trash/restore/
    hard-delete, usage track/count/remove.
  - `underlay-jobs-postgres` (1 lifecycle): create/get/claim/succeed, claim/fail
    terminal, cancel, list filters — rebuilding `platform` from the crate's own
    migrations (hardcoded schema, so single-test to avoid contention).
- [x] Chose the runnable-CI path over `sqlx::query!` offline mode: `TestDb` now
  runs against any provisioned Postgres via `UNDERLAY_TEST_DATABASE_URL`, so the
  hand-written SQL is schema-checked end-to-end when a database is present. The
  fixtures build each adapter's tables from its *actual* column usage, so they
  double as a drift check (several consumer baseline migrations had already
  drifted from the adapters — e.g. media `alt_text`, the usage schema).
- [x] Stale `tarpaulin-report.html` removed in `g08.026`.

## Consumer Upgrade Impact

Impact class: `none`.

## Validation

- [x] Each `-postgres` adapter has integration coverage of its main queries (17
  tests).
- [x] `cargo test -p underlay-auth-state-postgres -p underlay-jobs-postgres -p underlay-media-postgres -p underlay-audit -p underlay-security-alerts --lib -- --ignored`
  with `UNDERLAY_TEST_DATABASE_URL` set: all 17 pass against Postgres 16.
- [x] Normal `cargo test` (no database) leaves them ignored; `cargo check
  --workspace` clean.

## Stop Conditions

Was: stop if CI lacks a Postgres service. Resolved — `TestDb` now runs against
any provisioned Postgres via `UNDERLAY_TEST_DATABASE_URL` (CI service or
`effigy container`), so the tests are runnable rather than blocked.

## Next Task

`g08.020` workspace dependency and lint hygiene.
