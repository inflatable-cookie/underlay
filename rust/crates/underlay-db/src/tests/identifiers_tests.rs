use super::*;

#[test]
fn validates_plain_identifiers() {
    assert!(validate_sql_identifier("media_versions").is_ok());
    assert!(validate_sql_identifier("_private").is_ok());
    assert!(validate_sql_identifier("table1").is_ok());
}

#[test]
fn rejects_unsafe_identifiers() {
    assert!(validate_sql_identifier("").is_err());
    assert!(validate_sql_identifier("Media").is_err());
    assert!(validate_sql_identifier("1table").is_err());
    assert!(validate_sql_identifier("table-name").is_err());
    assert!(validate_sql_identifier("table;drop").is_err());
}

#[test]
fn validates_qualified_table_names() {
    assert!(validate_qualified_table_name("platform.audit_log").is_ok());
    assert!(validate_qualified_table_name("audit_log").is_ok());
    assert!(validate_qualified_table_name("platform..audit_log").is_err());
    assert!(validate_qualified_table_name("a.b.c").is_err());
    assert!(validate_qualified_table_name("platform.audit-log").is_err());
}

#[test]
fn formats_quoted_identifiers() {
    assert_eq!(quote_sql_identifier("audit_log").unwrap(), "\"audit_log\"");
    assert_eq!(
        format_qualified_table_name("platform.audit_log").unwrap(),
        "\"platform\".\"audit_log\""
    );
    assert_eq!(
        format_schema_table("media", "media_versions").unwrap(),
        "\"media\".\"media_versions\""
    );
}

#[test]
fn typed_identifiers_quote_without_revalidating_at_call_site() {
    let identifier = SqlIdentifier::parse("audit_log").unwrap();
    assert_eq!(identifier.as_str(), "audit_log");
    assert_eq!(identifier.quoted(), "\"audit_log\"");

    let table = QualifiedTableName::parse("platform.audit_log").unwrap();
    assert_eq!(table.schema().unwrap().as_str(), "platform");
    assert_eq!(table.table().as_str(), "audit_log");
    assert_eq!(table.to_string(), "platform.audit_log");
    assert_eq!(table.quoted(), "\"platform\".\"audit_log\"");
}

#[test]
fn typed_identifiers_reject_unsafe_values() {
    assert!(SqlIdentifier::parse("AuditLog").is_err());
    assert!(QualifiedTableName::parse("platform.audit-log").is_err());
    assert!(QualifiedTableName::from_schema_table("platform", "audit;drop").is_err());
}
