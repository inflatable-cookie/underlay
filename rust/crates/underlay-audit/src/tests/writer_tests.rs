use crate::AuditTable;

#[test]
fn valid_table_names() {
    assert!(AuditTable::parse("platform.audit_log").is_ok());
    assert!(AuditTable::parse("audit_log").is_ok());
}

#[test]
fn invalid_table_names() {
    assert!(AuditTable::parse("audit; DROP TABLE users").is_err());
    assert!(AuditTable::parse("audit-log").is_err());
}
