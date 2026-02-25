use super::*;

#[test]
fn test_exists_check_simple_query() {
    let check = ExistsCheck::new("content", "summary_item").value("slug", "test-slug");

    assert_eq!(check.schema, "content");
    assert_eq!(check.table, "summary_item");
    assert_eq!(check.conditions.len(), 1);
    assert!(check.exclude_id.is_none());
}

#[test]
fn test_exists_check_composite_query() {
    let id = Uuid::nil();
    let check = ExistsCheck::new("learning", "pathway")
        .value("slug", "test")
        .nullable_value("year", Some(2024))
        .excluding(id);

    assert_eq!(check.conditions.len(), 2);
    assert!(check.exclude_id.is_some());
}

#[test]
fn test_exists_check_multi_scope() {
    let pathway_id = Uuid::nil();
    let check = ExistsCheck::new("learning", "module")
        .value("slug", "mod-1")
        .scope("pathway_id", pathway_id)
        .value_i32("start_year", 2024);

    assert_eq!(check.conditions.len(), 3);
}

#[test]
fn test_exists_check_nullable_none() {
    let check = ExistsCheck::new("learning", "pathway")
        .value("slug", "test")
        .nullable_value("year", None);

    assert_eq!(check.conditions.len(), 2);
    // The None value should still be added as a condition
    match &check.conditions[1] {
        Condition::NullableIntEquals { value, .. } => assert!(value.is_none()),
        _ => panic!("Expected NullableIntEquals"),
    }
}

// Note: Integration tests with actual database connections should be
// in the tests/ directory using testcontainers.
