# g06.028 Artifact - Typed Operator Table Config

## Summary

`g06.028` moved audit and security-alert dynamic SQL helpers to typed table
configuration while preserving source compatibility.

## Code Changes

- Added `underlay_audit::AuditTable`.
- Added typed audit helpers:
  - `append_audit_log_to_table()`
  - `append_audit_log_to_table_async()`
  - `list_audit_logs_from_table()`
  - `get_audit_log_by_id_from_table()`
  - `count_audit_logs_from_table()`
- Kept existing raw-string audit helpers as compatibility wrappers.
- Added `underlay_security_alerts::LoginAttemptsTable`.
- Added `underlay_security_alerts::SecurityAlertEventsTable`.
- Added `underlay_security_alerts::SecurityAlertTables`.
- Added typed security-alert helpers:
  - `load_ip_signal_counts_from_table()`
  - `has_recent_alert_in_table()`
  - `insert_alert_event_into_table()`
- Kept existing raw-string security-alert helpers as compatibility wrappers.

## Security Posture

Runtime SQL values remain bound parameters.

Dynamic SQL identifiers now have a preferred typed construction path backed by
`underlay_db::QualifiedTableName`. The retained raw-string wrappers parse into
the same table types before SQL construction.

No new SQL interpolation path accepts an unchecked identifier.

## Consumer Impact

Impact: additive.

No consumer update is required immediately. Consumers should migrate app config
to construct typed table locations once and pass those types through operator
state.

## Validation

- `cargo test -p underlay-audit -p underlay-security-alerts --all-features`
  passed.

## Next Move

Use `g06.029` to migrate the six known consumers onto the typed table APIs,
then decide whether the raw-string wrappers should become explicitly deprecated.
