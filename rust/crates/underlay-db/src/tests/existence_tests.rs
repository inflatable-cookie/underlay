use super::*;

#[test]
fn typed_value_exists_query_quotes_identifiers() {
    let table = QualifiedTableName::from_schema_table("content", "summary_item").unwrap();
    let column = SqlIdentifier::parse("slug").unwrap();

    assert_eq!(
        value_exists_query(&table, &column, false),
        "SELECT EXISTS(SELECT 1 FROM \"content\".\"summary_item\" WHERE \"slug\" = $1 AND deleted_at IS NULL)"
    );
}

#[test]
fn typed_value_exists_query_excludes_current_id() {
    let table = QualifiedTableName::from_schema_table("learning", "pathway").unwrap();
    let column = SqlIdentifier::parse("slug").unwrap();

    assert_eq!(
        value_exists_query(&table, &column, true),
        "SELECT EXISTS(SELECT 1 FROM \"learning\".\"pathway\" WHERE \"slug\" = $1 AND id <> $2 AND deleted_at IS NULL)"
    );
}

#[test]
fn typed_exists_check_builds_composite_query() {
    let pathway_id = Uuid::nil();
    let check = TypedExistsCheck::from_schema_table("learning", "module")
        .unwrap()
        .value("slug", "mod-1")
        .unwrap()
        .scope("pathway_id", pathway_id)
        .unwrap()
        .value_i32("start_year", 2024)
        .unwrap()
        .excluding(Uuid::nil());

    assert_eq!(
        check.query(),
        "SELECT EXISTS(SELECT 1 FROM \"learning\".\"module\" WHERE \"slug\" = $1 AND \"pathway_id\" = $2 AND \"start_year\" = $3 AND id <> $4 AND deleted_at IS NULL)"
    );
}

#[test]
fn typed_exists_check_builds_nullable_query() {
    let check = TypedExistsCheck::from_schema_table("learning", "pathway")
        .unwrap()
        .value("slug", "pathway")
        .unwrap()
        .nullable_value("year", None)
        .unwrap();

    assert_eq!(
        check.query(),
        "SELECT EXISTS(SELECT 1 FROM \"learning\".\"pathway\" WHERE \"slug\" = $1 AND \"year\" IS NOT DISTINCT FROM $2 AND deleted_at IS NULL)"
    );
}

#[test]
fn typed_exists_check_can_include_deleted() {
    let check = TypedExistsCheck::parse_table("content.summary_item")
        .unwrap()
        .value("slug", "summary")
        .unwrap()
        .include_deleted();

    assert_eq!(
        check.query(),
        "SELECT EXISTS(SELECT 1 FROM \"content\".\"summary_item\" WHERE \"slug\" = $1)"
    );
}

#[test]
fn typed_exists_check_rejects_invalid_identifiers() {
    assert!(TypedExistsCheck::from_schema_table("content", "summary-item").is_err());

    let check = TypedExistsCheck::from_schema_table("content", "summary_item").unwrap();
    assert!(check.value("bad-column", "summary").is_err());
}

// Note: Integration tests with actual database connections should be
// in the tests/ directory using testcontainers.
