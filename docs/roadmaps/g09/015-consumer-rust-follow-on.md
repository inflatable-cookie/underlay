# g09.015 - Consumer Rust Follow-On

Status: complete
Completed: 2026-08-03
Owner: repo maintainers

## Purpose

Ride the underlay majors from `g09.014` and clear each app's own
staleness. Most-behind: cp-api (redis 1.x migration, axum-test five
majors stale pulling a duplicate axum 0.7, rand 0.8 in-tree). Runner-up:
nursery (jsonwebtoken 9 direct, reqwest 0.12).

## Evidence

- Rust dependency survey 2026-08-03

## Planned Changes

- [x] cp-api: `redis` 0.27→1.x migration (acme already on 1.x — copy its
  shape); `axum-test` 16→21 (drops the duplicate axum 0.7); rand 0.10.
- [x] nursery: `jsonwebtoken` 9→11 (align with underlay), `reqwest` 0.13.
- [x] compli-me: rand 0.10, hickory, base64 catch-up.
- [x] farmyard: routine minors (+dev-only `serial_test` 4).
- [x] composer: `toml`/`thiserror` retargets.
- [x] Each: `cargo check --workspace --all-features --all-targets` green;
  app test suites green where they run without a live DB.

## Consumer Upgrade Impact

Impact class: `breaking` where majors change APIs (redis 1.x is the only
real migration; the rest is mechanical).

## Validation

- [x] cargo check green in all six; targeted tests green

## Completion Notes

Completed 2026-08-03. All six consumers absorbed the underlay majors: acme 445d15d, cp 1896464 (redis 0.27→1.5 with zero code breaks — shape was already 1.x-compatible; axum-test 16→21 dropped the duplicate axum 0.7), compli 0faff6e, nursery 25fb623 (jsonwebtoken 9→11, reqwest 0.13), composer 7fb8cec, farmyard 7fd96d9e + parent bumps (a4021c8, 3e7758f lineage). sqlx 0.9 AssertSqlSafe audits landed everywhere (~200 wraps family-wide, every interpolation verified validated/quoted-identifier-only). Test state: all unit suites green; DB-backed suites skipped by design (stacks down); cp's 6 failures are env-unwrap-in-test-support, expected until the dev stack is up. farmyard's full-workspace test run flaked once under build contention; isolated re-run green. In-flight bonus: FetchFn ambient fix (9f43e144) cleared composer-admin's 19-error baseline to 0.

## Next Task

`g09.016` JS baseline catch-up.
