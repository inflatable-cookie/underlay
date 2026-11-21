### Fixed

- `underlay-migration-core`: `IntegrityEvidence::default()` and `IntegrityGateResult::default()` now fail closed (`digest_verified: false`, `sidecar_checksums_verified: false`, `passed: false`).
- `underlay-migration-core`: `AiThresholdPolicy` validates thresholds against `NaN` and out-of-range (`!(0.0..=1.0)`) values on `try_new` / `try_with_*` / `Deserialize` / `validate` and fails closed to `1.0` in `threshold_for`.
- `underlay-query`: `parse_sort_string` trims whitespace around the direction token so `"created_at: desc"` parses as `SortDirection::Desc`.
- `underlay-validation-derive`: `#[validate(required)]` on `Option<T>` rejects `None` with `"required"` instead of skipping inside `if let Some(...)`.
- `underlay-validation`: `ValidationError::add_field` (and `merge` / `add_nested`) appends distinct failure messages cleanly instead of overwriting earlier errors on the same field.
- `underlay-validation`: `validators::slug` delegates to `slugify::is_valid_slug_format` so both enforce the 2-100 character slug length rule consistently.
- `underlay-validation`: `cached_pattern` releases the global pattern cache `Mutex` before compiling `Regex::new`.
- `underlay-db`: `is_local_database_url` rejects libpq `?host=` / `?hostaddr=` query parameter overrides pointing to non-local hosts and bare `".test"` domains.
- `underlay-db`: `Cursor` stores values in `BTreeMap` for deterministic base64 encoding across field insertion orders.
- `underlay-db`: clarified `ReorderConflict` and backward `PaginatedResponse.has_more` documentation and semantics.
- `underlay-metrics`: `metrics_handler` logs `prometheus::Error` via `tracing::error!` instead of discarding it.

### Added

- `underlay-query`: derived `Debug` and `Clone` on `WhereBuilder`.
- `underlay-core`: added `From<RawUuid>`, `From<Uuid> for RawUuid`, `AsRef<RawUuid>`, `FromStr`, `PartialOrd`, and `Ord` on `Uuid`, plus `PartialEq`/`Eq` on core DTOs and `AppError`.
