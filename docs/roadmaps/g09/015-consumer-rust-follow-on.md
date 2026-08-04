# g09.015 - Consumer Rust Follow-On

Status: ready
Owner: repo maintainers

## Purpose

Ride the underlay majors from `g09.014` and clear each app's own
staleness. Most-behind: cp-api (redis 1.x migration, axum-test five
majors stale pulling a duplicate axum 0.7, rand 0.8 in-tree). Runner-up:
nursery (jsonwebtoken 9 direct, reqwest 0.12).

## Evidence

- Rust dependency survey 2026-08-03

## Planned Changes

- [ ] cp-api: `redis` 0.27→1.x migration (acme already on 1.x — copy its
  shape); `axum-test` 16→21 (drops the duplicate axum 0.7); rand 0.10.
- [ ] nursery: `jsonwebtoken` 9→11 (align with underlay), `reqwest` 0.13.
- [ ] compli-me: rand 0.10, hickory, base64 catch-up.
- [ ] farmyard: routine minors (+dev-only `serial_test` 4).
- [ ] composer: `toml`/`thiserror` retargets.
- [ ] Each: `cargo check --workspace --all-features --all-targets` green;
  app test suites green where they run without a live DB.

## Consumer Upgrade Impact

Impact class: `breaking` where majors change APIs (redis 1.x is the only
real migration; the rest is mechanical).

## Validation

- [ ] cargo check green in all six; targeted tests green

## Next Task

`g09.016` JS baseline catch-up.
