# g06.030 Artifact - Raw Operator Wrapper Removal Readiness

## Summary

`g06.030` removed the raw-string audit and security-alert operator wrappers
after the six known consumers moved to typed table config.

## Removal Decision

Decision: remove now.

Reason:

- `g06.029` proved zero current consumer usage across the six known apps.
- The wrappers added no behavior beyond parsing raw strings into typed tables.
- Keeping them would keep teaching the weaker API shape.

## Removed Audit APIs

- `append_audit_log()`
- `append_audit_log_async()`
- `list_audit_logs()`
- `get_audit_log_by_id()`
- `count_audit_logs()`

Retained audit APIs:

- `AuditTable`
- `append_audit_log_to_table()`
- `append_audit_log_to_table_async()`
- `list_audit_logs_from_table()`
- `get_audit_log_by_id_from_table()`
- `count_audit_logs_from_table()`

## Removed Security-Alert APIs

- `load_ip_signal_counts()`
- `has_recent_alert()`
- `insert_alert_event()`

Retained security-alert APIs:

- `LoginAttemptsTable`
- `SecurityAlertEventsTable`
- `SecurityAlertTables`
- `load_ip_signal_counts_from_table()`
- `has_recent_alert_in_table()`
- `insert_alert_event_into_table()`

## Remaining Dynamic Identifier Inventory

Retained, already validated:

- `underlay-media-postgres::PostgresMediaConfig`
  - `try_with_schema()` parses `SqlIdentifier`
  - `try_with_tables()` parses each table name as `SqlIdentifier`
  - query code uses quoted fully qualified table names

Retained compatibility, next candidate:

- `underlay-db::ExistsCheck`
  - public builder still accepts raw schema/table/column strings
  - execution validates and quotes identifiers before SQL construction
  - next improvement is a typed builder over `QualifiedTableName` and
    `SqlIdentifier`

Retained test helper:

- `underlay-testing::TestDb`
  - schema name is generated internally from UUIDv7 characters
  - not app supplied
  - acceptable as test-only helper, but can still be cleaned up later with
    `SqlIdentifier` for consistency

## Consumer Impact

Impact: breaking at the Underlay API level, but current-consumer blast radius is
zero after `g06.029`.

Consumers outside the known six must move to typed table APIs.

## Next Move

Use `g06.031` to plan the remaining DB helper migration, especially
`ExistsCheck`.
