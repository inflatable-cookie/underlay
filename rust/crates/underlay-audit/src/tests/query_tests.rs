use super::*;

#[test]
fn filters_builder() {
    let filters = AuditLogFilters::new()
        .with_action("create")
        .with_resource_type("pathway")
        .with_pagination(100, 50);

    assert_eq!(filters.action, Some("create".to_string()));
    assert_eq!(filters.resource_type, Some("pathway".to_string()));
    assert_eq!(filters.limit, 100);
    assert_eq!(filters.offset, 50);
}

#[test]
fn with_pagination_clamps_negative_and_unbounded_values() {
    let negative = AuditLogFilters::new().with_pagination(-10, -5);
    assert_eq!(negative.limit, 1);
    assert_eq!(negative.offset, 0);

    let zero_limit = AuditLogFilters::new().with_pagination(0, 0);
    assert_eq!(zero_limit.limit, 1);
    assert_eq!(zero_limit.offset, 0);

    let unbounded = AuditLogFilters::new().with_pagination(i64::MAX, 42);
    assert_eq!(unbounded.limit, MAX_AUDIT_LOG_LIMIT);
    assert_eq!(unbounded.offset, 42);
}
