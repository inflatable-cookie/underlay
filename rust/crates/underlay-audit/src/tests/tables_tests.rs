use crate::AuditTable;

#[test]
fn audit_table_accepts_valid_names() {
    assert_eq!(
        AuditTable::parse("platform.audit_log").unwrap().quoted(),
        "\"platform\".\"audit_log\""
    );
    assert_eq!(
        AuditTable::parse("audit_log").unwrap().quoted(),
        "\"audit_log\""
    );
}

#[test]
fn audit_table_rejects_invalid_names() {
    assert!(AuditTable::parse("audit; DROP TABLE users").is_err());
    assert!(AuditTable::parse("audit-log").is_err());
    assert!(AuditTable::parse("platform.audit.log").is_err());
}
