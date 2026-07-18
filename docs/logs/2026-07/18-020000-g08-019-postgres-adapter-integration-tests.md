# 2026-07-18 - g08.019 postgres adapter integration tests (unblocked)

## Context

The five `-postgres` adapter crates were full of hand-written SQL (66 runtime
`sqlx::query(...)` calls, no compile-checked `query!` macros) with no integration
coverage. The card was blocked: `underlay-testing::TestDb` was testcontainers-only
and the environment had no Docker/Postgres.

## Unblock

Two moves, on the maintainer's steer that the consumer apps run Postgres via
effigy's containerd container system:

1. **`TestDb` accepts an external database.** `TestDb::new()` now honours
   `UNDERLAY_TEST_DATABASE_URL` and connects to an already-provisioned Postgres,
   falling back to a testcontainer only when the env var is unset. `_container`
   became `Option`. Per-test schema isolation is unchanged. This is the durable
   CI story `g08.025` flagged as missing — the suite no longer hard-requires a
   Docker API.
2. **Provisioned Postgres 16 on the effigy Colima/containerd profile**
   (`colima nerdctl --profile effigy -- run -d -p 55432:5432 postgres:16-alpine`)
   and pointed the tests at `postgres://postgres:postgres@127.0.0.1:55432/postgres`.

## Coverage added (17 tests, all `#[ignore]`d)

- **auth-state-postgres** (5): create/load/consume round-trip, user+state_type
  scoping, public update + typed round-trip, expiry, delete.
- **security-alerts** (4): IP/account/global signal-count aggregation, IP +
  scoped alert insert/cooldown dedupe.
- **audit** (3): append + get-by-id, list/count filters (user/action/resource),
  ordered pagination.
- **media-postgres** (4): media CRUD, version lifecycle
  (create/finalize/set-current/find-by-hash/list), soft-delete/trash/restore/
  hard-delete, usage track/count/list/remove.
- **jobs-postgres** (1 lifecycle): create/get/claim(`fetch_next`)/succeed,
  claim/fail terminal, cancel, list filters. The adapter hardcodes the `platform`
  schema, so this rebuilds `platform` from the crate's own migrations (0001,
  0002, 0004 — 0003 targets an app-owned `domain_events` table) via `raw_sql`
  (simple protocol, for the `$$` trigger bodies) in a single test to avoid
  cross-test contention.

Each fixture builds the adapter's tables from its **actual** column usage rather
than a consumer migration, so the tests double as a schema-contract check.

Following up on the apparent drift: the four **adopted** adapters
(`jobs-postgres` 13 consumer refs, `audit` 11, `security-alerts` 10,
`auth-state-postgres` 8) match their consumers — verified e.g. farmyard's
`auth.auth_state` has exactly the adapter's columns, which is why the fixtures
and production agree. The only divergence is **`underlay-media-postgres`, which
no consumer uses** (0 Cargo refs): consumers use the `underlay-media` domain
crate but roll their own persistence, so its schema (with `alt_text`, a simple
`field` usage table) differs from theirs by design. That is not a consumer bug
and needs no migration change — documented in the crate as unadopted, with the
integration-test fixture as the canonical shape for any future adopter.

## Validation

- `cargo test -p underlay-auth-state-postgres -p underlay-jobs-postgres -p
  underlay-media-postgres -p underlay-audit -p underlay-security-alerts --lib --
  --ignored` with `UNDERLAY_TEST_DATABASE_URL`: **17 passed**.
- Normal `cargo test` (no database): integration tests ignored, existing unit
  tests pass — CI without Postgres stays green.
- `cargo check --workspace`: clean.

## Consumer Upgrade Notes

Impact class **none**. Test-only additions plus a backward-compatible `TestDb`
enhancement (external-URL opt-in; testcontainers still the default).

## Next

`g08` fully closed — all 32 cards done. CI is now wired
(`.github/workflows/rust.yml` runs these tests against a `postgres:16` service
via `UNDERLAY_TEST_DATABASE_URL`). No consumer reconciliation is needed: the
adopted adapters match their consumers, and the only divergent adapter
(`media-postgres`) is unadopted.
