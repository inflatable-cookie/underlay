### Fixed

- `underlay-migration-core`: excluded `resolver_version` and `prompt_version` from `decision_fingerprint` and length-prefixed string fields in `decision_fingerprint` and `record_fingerprint` so `DecisionReusePolicy::Compatible` works across compatible version bumps without failing early on `FingerprintMismatch` or colliding on newline delimiters.
- `underlay-migration-core`: `is_version_compatible` now treats semver `0.x.y` minor bumps as incompatible.
- `underlay-migration-core`: `build_unresolved` now stamps RFC 9562 UUIDv8 (`0x80`) version bits onto its SHA-256 digest instead of UUIDv7 (`0x70`).
- `underlay-db`: `describe_db_error`, `map_db_error`, and `map_db_error_ref` log detailed Postgres/SQLx diagnostics via `tracing` and omit raw table/column/constraint names, `DETAIL`, `HINT`, and raw driver error strings from API-facing `AppError.message`.
- `underlay-db`: `run_sql_dir_with_options` now uses `tokio::fs` instead of blocking `std::fs`.
- `underlay-db`: `reorder_scoped` qualifies scope columns with the target table name to avoid collisions with `u(id, ord)` and sorts `added_ids`/`removed_ids` deterministically.
- `underlay-db`: `TypedExistsCheck::check` returns `Err(sqlx::Error::Configuration(_))` instead of `Ok(false)` when no conditions are configured.
- `underlay-db`: `validate_sql_identifier` / `SqlIdentifier::parse` / `QualifiedTableName::parse` enforce PostgreSQL's 63-byte `NAMEDATALEN - 1` limit (`IdentifierError::TooLong`) and reject leading/trailing whitespace instead of trimming.
- `underlay-validation`: `ValidatedJsonRejection::into_response` preserves `FieldError::code` via `error.fieldErrorCodes` and maps `JsonRejection::MissingJsonContentType` to `415 Unsupported Media Type` with static safe error messages.
- `underlay-validation`: `ValidationError::Display` sorts field keys deterministically.
- `underlay-validation-derive`: `#[derive(Validate)]` supports `Option<T>` fields (skipping validation when `None`, validating inner values when `Some`).
- `underlay-config`: added `#[source]` on `ConfigError::Decode` and replaced `path.exists()` in `ConfigStack::merge_optional_file` with direct `std::io::ErrorKind::NotFound` handling.
- `underlay-query`: replaced `type Err = ()` in `FilterOperator::from_str` and `SortDirection::from_str` with typed `ParseFilterOperatorError` and `ParseSortDirectionError` types.
