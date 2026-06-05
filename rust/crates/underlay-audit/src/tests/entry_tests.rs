use super::*;

#[test]
fn audit_action_round_trip() {
    let actions = vec![
        AuditAction::Create,
        AuditAction::Update,
        AuditAction::Delete,
        AuditAction::Publish,
        AuditAction::Custom("my_action".to_string()),
    ];

    for action in actions {
        let s = action.as_str();
        let parsed = AuditAction::parse_lossy(s);
        assert_eq!(action, parsed);
    }
}

#[test]
fn audit_entry_builder() {
    let entry = AuditEntry::new(
        Some(Uuid::nil()),
        AuditAction::Create,
        "pathway",
        Uuid::nil(),
    )
    .with_details(serde_json::json!({"title": "Test"}))
    .with_correlation_id("req-123")
    .with_ip_address("192.168.1.1");

    assert_eq!(entry.action, AuditAction::Create);
    assert_eq!(entry.resource_type, "pathway");
    assert_eq!(entry.correlation_id, Some("req-123".to_string()));
}
