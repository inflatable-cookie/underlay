### Changed

- `underlay-validation`: `validators::range` now rejects `NaN` with code
  `range.nan` instead of failing open. The numeric bounds are now expressed by
  the new `validators::RangeNumeric` trait, implemented for the primitive
  integer and floating-point types, so unordered floats cannot pass a range
  check.
- `underlay-validation`: `validators::pattern` (and therefore
  `#[validate(pattern = "...")]`) caches compiled regexes in a bounded
  process-wide cache, so repeated validation no longer recompiles the same
  literal pattern. Invalid patterns are cached as failures too.
- `underlay-validation`: `validation_to_app_error` omits `field_errors`
  (`fieldErrors` on the wire) when there are no field-scoped errors, matching
  contract 010 `g06.181`.
- `underlay-config`: `discover_config_dir` only probes `./config` and
  `../config` when the candidate still resolves inside the checkout root, so a
  checkout run from its own root no longer picks up configuration from the
  parent directory.
- `underlay-db`: `run_sql_dir_with_options` executes unsplit scripts through
  `sqlx::raw_sql`, preserving procedural blocks that contain semicolons,
  returns the real `sqlx::Error` instead of wrapping SQL failures in
  `sqlx::Error::Io`, and propagates `read_dir` entry errors instead of
  silently skipping the file.
- `underlay-db`: `PaginationBuilder::keyset_condition` and
  `keyset_order_by` now return `Result<String, IdentifierError>` and quote the
  sort column through `SqlIdentifier::quoted()`; an invalid column is a typed
  error instead of a panic. `CursorPaginationParams::limit` documentation now
  states the actual default of 30.
- `underlay-soft-delete`: `SoftDeletable::soft_delete_in_tx` returns
  `SoftDeleteResult` instead of `bool`, so `soft_delete` and
  `soft_delete_with_batch_id` can report `SoftDeleteResult::AlreadyDeleted`
  as well as `NotFound` and `Deleted`. `impl_soft_deletable!` checks row
  existence when its update matches nothing, and `AlreadyDeleted` rows are not
  counted by the batch helpers. Public ids use `underlay_core::Uuid`
  throughout, and the generated macros now use hygienic `$crate::` paths so
  consumers do not need `sqlx`, `async-trait`, or `uuid` in scope.

### Consumer actions (breaking; target `0.11.0`)

- Update manual `SoftDeletable` implementations to return
  `SoftDeleteResult`, and pass `underlay_core::Uuid` values (the soft-delete
  macros and helpers no longer take raw `uuid::Uuid`).
- Update `keyset_condition` / `keyset_order_by` call sites for the `Result`
  return and the quoted column in the generated SQL.
- Custom value types passed to `validators::range` must implement
  `underlay_validation::validators::RangeNumeric`.
- Validation: `effigy rust:clippy`, `effigy fmt:rust:check`, and the touched
  crate tests.
- Rollback: pin the previous tag; no migration or data change is involved.
