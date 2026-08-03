# g09.010 - Farmyard Seed-Bundle Shared Dev Credentials

Status: ready
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

- [ ] Identify the post-seed hook point (post-sql hook index) where a
  dev-only credential override can set the shared password hash + TOTP on
  the seeded superadmin account.
- [ ] Implement the override as an effigy-env-only step (env
  `Local|Effigy|Test` + local DB host guard, matching farmyard's seed
  gate).
- [ ] Verify: fresh seed replay → login with the shared credentials works;
  existing dev DBs unaffected unless re-seeded.

## Consumer Upgrade Impact

Impact class: `additive` (dev-only). No production surface.

## Validation

- [ ] Seed replay in the acowtancy dev stack + login with shared creds
- [ ] `cargo check --workspace --all-features --all-targets` (farmyard)

## Stop Conditions

If the hook index does not admit a clean override point, document the
accepted variant (farmyard keeps legacy-dump creds) and close.

## Next Task

`g09.011` shell-tab schema env.
