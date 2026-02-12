use super::*;

#[test]
fn map_db_error_sets_standard_code() {
    let err = sqlx::Error::RowNotFound;
    let app_err = map_db_error("Database error loading entity", err);

    assert_eq!(app_err.code, "infra.db_error");
    assert!(app_err.message.contains("loading entity"));
}

#[test]
fn map_db_error_ref_sets_standard_code() {
    let err = sqlx::Error::PoolClosed;
    let app_err = map_db_error_ref("Database error listing entities", &err);

    assert_eq!(app_err.code, "infra.db_error");
    assert!(app_err.message.contains("listing entities"));
    assert!(app_err.message.contains("pool is closed"));
}

#[test]
fn describe_db_error_normalizes_operation_prefix() {
    let err = sqlx::Error::RowNotFound;
    let message = describe_db_error("Database error checking module.", &err);

    assert!(message.starts_with("Database error while checking module."));
    assert!(!message.contains("Database error while Database error"));
}

#[test]
fn describe_db_error_falls_back_for_empty_operation() {
    let err = sqlx::Error::PoolTimedOut;
    let message = describe_db_error("   ", &err);

    assert!(message.contains("running database operation"));
    assert!(message.contains("pool timed out"));
}
