# g06.028 - Typed Operator Table Config

## Why

`g06.027` found no immediate SQL injection blocker, but audit and
security-alert helpers still accept raw `&str` table names at public call
boundaries.

Those paths validate and quote identifiers today. Reference-grade Underlay
should make the safe path structural instead: callers should pass typed table
config or `QualifiedTableName` values, not raw strings that each function must
remember to validate.

## Goal

Move operator-facing audit and security-alert SQL helpers to typed table
configuration.

## Scope

In scope:

- introduce typed table config for `underlay-audit`
- introduce typed table config for `underlay-security-alerts`
- prefer `underlay-db` identifier/table types over raw string validation
- keep existing behavior where possible while changing unsafe public shape
- update consumers if they call the affected APIs directly
- update contracts and guides

Out of scope:

- jobs/media adapter movement
- broad migration-core or devtools file splitting
- TypeScript/Svelte work
- release execution or publishing

## Contract References

- `001`: working rules
- `021`: database migration and schema workflow
- `033`: error codes and operator audit
- `060`: jobs, events, and operator systems
- `122`: Rust public API inventory

## Acceptance Criteria

- audit query helpers no longer require raw table-name strings on new public
  APIs
- security-alert helpers no longer require raw table-name strings on new public
  APIs
- identifier validation happens at config construction, not at every query
  call site
- consumer impact is classified as additive, deprecation, or breaking
- targeted Rust and consumer checks pass or failures are classified

## Consumer Upgrade Impact

Impact: additive.

The raw-string audit and security-alert functions remain source-compatible
wrappers. New public typed APIs are available for consumer migration:

- `underlay_audit::AuditTable`
- `append_audit_log_to_table()`
- `append_audit_log_to_table_async()`
- `list_audit_logs_from_table()`
- `get_audit_log_by_id_from_table()`
- `count_audit_logs_from_table()`
- `underlay_security_alerts::LoginAttemptsTable`
- `underlay_security_alerts::SecurityAlertEventsTable`
- `underlay_security_alerts::SecurityAlertTables`
- `load_ip_signal_counts_from_table()`
- `has_recent_alert_in_table()`
- `insert_alert_event_into_table()`

## Current State

`g06.028` is complete.

See
[`028-typed-operator-table-config-artifact.md`](028-typed-operator-table-config-artifact.md).

## Next Task

Execute `g06.029`: consumer typed operator table adoption and raw-wrapper
deprecation decision.
