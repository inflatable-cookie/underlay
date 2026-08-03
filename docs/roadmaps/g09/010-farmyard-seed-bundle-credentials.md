# g09.010 - Farmyard Seed-Bundle Shared Dev Credentials

Status: complete
Completed: 2026-08-03
Owner: repo maintainers

## Purpose

farmyard's dev users come from the legacy-dump seed-bundle harness (hook
indexes, contract versions), not a `migrations_dev` seed file. Its dev
admin credentials therefore do not match the shared fleet set
(`admin@example.com` / `UnderlayDev2026!` / shared TOTP). Aligning needs a
designed hook, not a drive-by edit.

## Evidence

- `farmyard/crates/migration/src/seed_bundle_post_sql.rs`,
  `legacy_user.rs`, `fixtures/seed-bundle-replay/`
- Audit item 10

## Planned Changes

- [x] Identify the post-seed hook point (post-sql hook index) where a
  dev-only credential override can set the shared password hash + TOTP on
  the seeded superadmin account.
- [x] Implement the override as an effigy-env-only step (env
  `Local|Effigy|Test` + local DB host guard, matching farmyard's seed
  gate).
- [x] Verify: fresh seed replay → login with the shared credentials works;
  existing dev DBs unaffected unless re-seeded.

## Consumer Upgrade Impact

Impact class: `additive` (dev-only). No production surface.

## Validation

- [x] Seed replay in the acowtancy dev stack + login with shared creds
- [x] `cargo check --workspace --all-features --all-targets` (farmyard)

## Stop Conditions

If the hook index does not admit a clean override point, document the
accepted variant (farmyard keeps legacy-dump creds) and close.

## Completion Notes

Completed 2026-08-03 (farmyard 1a2b19e0 + d6167e84). Code-level hook on the legacy-user baseline-import replay arm (NOT the contract-versioned published hook index — evidence in the implementation): applies the shared fleet password hash + TOTP for the seeded superadmin when `env.is_local_dev()` AND the target DB URL is local. state/dev-seeds SQL aligned so the dev_overlay phase can't clobber it. farmyard-migration 333/333 tests green (override gate test included). Runtime replay + login verification: pending. Also fixed in flight: fleet TOTP secret corrected to valid base32 (`UNDERLAYDEVTOTPSECRET234567ABCDE` — the original contained `8`, outside A-Z2-7, and was 33 chars) across all consumer seeds + guide 192; migration config.rs deprecated loader call site (g09.004 follow-up). Note: part of this card's work landed early in d6167e84 via an accidental `git add -A` during g09.007 (HEAD was left self-contradictory for ~1h); this commit completes it. Open question parked: dan@decode.co.uk is a seeded superadmin with no dev credentials — scope was tom@inflatablecookie.com only.

## Next Task

`g09.011` shell-tab schema env.
