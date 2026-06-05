# g06.029 Artifact - Consumer Typed Operator Table Adoption

## Summary

`g06.029` migrated the six known consumers from raw-string audit and
security-alert helper calls to typed operator table config.

## Consumer Commits

- `underlay-reference/acme-api`: `7707917` - `Use typed operator table config`
- `contact-patch/cp-api`: `8a2cbe6` - `Use typed operator table config`
- `compli-me/api`: `af0fde4` - `Use typed operator table config`
- `acowtancy/farmyard`: `188eda4` - `Use typed operator table config`
- `acowtancy`: `714dba0` - `Update farmyard typed table config pointer`
- `songsprout/nursery`: `fd69e14` - `Use typed operator table config`
- `songsprout`: `37f6313` - `Update nursery typed table config pointer`
- `loophole/composer/composer-api`: `3411627` - `Use typed operator table config`
- `loophole`: `f506fa3` - `Update composer typed table config pointer`

## Underlay Posture

At `g06.029` close, raw-string audit and security-alert helpers became
deprecated compatibility wrappers. `g06.030` removed them after the readiness
check.

Preferred APIs:

- `AuditTable`
- `append_audit_log_to_table()`
- `append_audit_log_to_table_async()`
- `list_audit_logs_from_table()`
- `get_audit_log_by_id_from_table()`
- `count_audit_logs_from_table()`
- `LoginAttemptsTable`
- `SecurityAlertEventsTable`
- `SecurityAlertTables`
- `load_ip_signal_counts_from_table()`
- `has_recent_alert_in_table()`
- `insert_alert_event_into_table()`

## Scan Evidence

The six-consumer scan found no remaining calls to:

- `append_audit_log_async(`
- `append_audit_log(`
- `list_audit_logs(`
- `get_audit_log_by_id(`
- `count_audit_logs(`
- `load_ip_signal_counts(`
- `has_recent_alert(`
- `insert_alert_event(`

Fixed app-local SQL over known local tables remains intentionally unchanged.

## Validation

Passed targeted checks:

- `cargo check -p acme-auth -p acme-db -p acme-api`
- `cargo check -p cp-auth -p cp-db -p cp-api`
- `cargo check -p compli-me-auth -p compli-me-db -p compli-me-api`
- `cargo check -p farmyard-auth -p farmyard-db -p farmyard-api`
- `cargo check -p nursery-auth -p nursery-api`
- `cargo check -p composer-api`

## Next Move

Use `g06.030` to decide whether the deprecated raw wrappers are still needed
for bootstrap ergonomics, and to scan remaining dynamic identifier helpers
outside audit/security-alerts.
