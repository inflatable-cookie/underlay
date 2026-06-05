# g06.033 Artifact - Raw Existence Helper Deprecation Decision

## Decision

Remove the raw existence helper surface instead of retaining or deprecating it.

Removed public `underlay-db` APIs:

- `ExistsCheck`
- `value_exists`
- `value_exists_excluding`
- `value_exists_in_scope`
- `value_exists_in_scope_excluding`
- `number_exists_in_scope`
- `number_exists_in_scope_excluding`

Retained public `underlay-db` APIs:

- `TypedExistsCheck`
- `value_exists_typed`
- `value_exists_excluding_typed`

## Why

`g06.032` migrated the known consumer usage to typed helpers. A fresh consumer
scan found no remaining raw existence helper usage across the current six-app
family.

The raw helpers validated and quoted identifiers, so they were not a live SQL
injection issue. They were still the weaker public shape because schema, table,
and column names entered the API as ordinary strings and validation happened
late. Removing them makes the reference-grade path unambiguous.

## Consumer Usage Proof

Current consumer scan target family:

- `underlay-reference`
- `contact-patch`
- `compli-me`
- `acowtancy`
- `songsprout`
- `loophole/composer`

Result:

- no raw `ExistsCheck` usage remains
- no raw `value_exists*` scoped helper usage remains
- Farmyard retains app-local wrappers over `TypedExistsCheck` for repeated
  content schema slug/key checks

## Docs And Contract Changes

Updated active docs to teach only typed existence checks:

- `docs/guides/050-database.md`
- `docs/guides/200-project-sync.md`
- `docs/patterns/000-index.md`
- `docs/patterns/live-validation-endpoint.md`
- `docs/contracts/021-database-migration-and-schema-workflow.md`
- `docs/contracts/040-storage-blob-and-media-systems.md`
- `docs/contracts/122-rust-public-api-inventory.md`

Historical roadmap artifacts still mention the removed APIs as evidence of the
path that led to this batch.

## Compatibility Classification

Impact: breaking public API cleanup.

Consumer impact: no code changes required in the current six-app family because
the migration landed in `g06.032`.

## Validation

Validation passed:

- `cargo test -p underlay-db existence --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- raw helper symbol scan across the six consumer roots
- `cargo check --workspace` in `underlay-reference/acme-api`
- `cargo check --workspace` in `contact-patch/cp-api`
- `cargo check --workspace` in `compli-me/api`
- `cargo check --workspace` in `acowtancy/farmyard`
- `cargo check --workspace` in `songsprout/nursery`
- `cargo check --workspace` in `loophole/composer/composer-api`

Known non-blocking output:

- Farmyard emitted an existing dead-code warning in `farmyard-migration`.
