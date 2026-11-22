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

#[test]
fn describe_db_error_pool_and_io_messages_omit_operational_hints() {
    let io_err = sqlx::Error::Io(std::io::Error::new(
        std::io::ErrorKind::ConnectionRefused,
        "db-primary.internal:5432 refused",
    ));
    let cases = [sqlx::Error::PoolTimedOut, sqlx::Error::PoolClosed, io_err];

    for err in &cases {
        let message = describe_db_error("loading profile", err);
        assert!(message.starts_with("Database error while loading profile."));
        assert!(message.contains("Retry the operation"));
        assert!(!message.contains("Suggested fix"));
        assert!(!message.contains("pool sizing"));
        assert!(!message.contains("long-running queries"));
        assert!(!message.contains("service startup"));
        assert!(!message.contains("pool lifecycle"));
        assert!(!message.contains("network connectivity"));
        assert!(!message.contains("host availability"));
        assert!(!message.contains("db-primary.internal"));
        assert!(!message.contains("5432"));
    }
}

#[derive(Debug)]
struct FakeDbError {
    message: String,
    code: Option<String>,
    table: Option<String>,
    constraint: Option<String>,
}

impl std::fmt::Display for FakeDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for FakeDbError {}

impl sqlx::error::DatabaseError for FakeDbError {
    fn message(&self) -> &str {
        &self.message
    }

    fn code(&self) -> Option<std::borrow::Cow<'_, str>> {
        self.code.as_deref().map(std::borrow::Cow::Borrowed)
    }

    fn table(&self) -> Option<&str> {
        self.table.as_deref()
    }

    fn constraint(&self) -> Option<&str> {
        self.constraint.as_deref()
    }

    fn as_error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
        self
    }

    fn as_error_mut(&mut self) -> &mut (dyn std::error::Error + Send + Sync + 'static) {
        self
    }

    fn into_error(self: Box<Self>) -> Box<dyn std::error::Error + Send + Sync + 'static> {
        self
    }

    fn kind(&self) -> sqlx::error::ErrorKind {
        sqlx::error::ErrorKind::UniqueViolation
    }
}

#[test]
fn map_db_error_sanitizes_database_schema_and_detail_from_client_message() {
    let db_err = FakeDbError {
        message: "duplicate key value violates unique constraint \"secret_users_email_key\""
            .to_string(),
        code: Some("23505".to_string()),
        table: Some("secret_users".to_string()),
        constraint: Some("secret_users_email_key".to_string()),
    };
    let app_err = map_db_error("creating account", sqlx::Error::Database(Box::new(db_err)));

    assert_eq!(app_err.code, "infra.db_error");
    assert!(app_err.message.contains("creating account"));
    assert!(app_err.message.contains("uniqueness constraints"));
    assert!(!app_err.message.contains("secret_users"));
    assert!(!app_err.message.contains("secret_users_email_key"));
    assert!(!app_err.message.contains("duplicate key"));
    assert!(!app_err.message.contains("23505"));
}

#[test]
fn map_db_error_sanitizes_io_and_driver_errors_from_client_message() {
    let io_err = sqlx::Error::Io(std::io::Error::new(
        std::io::ErrorKind::ConnectionRefused,
        "10.0.0.42:5432 refused",
    ));
    let io_msg = map_db_error("saving item", io_err).message;
    assert!(io_msg.contains("saving item"));
    assert!(!io_msg.contains("10.0.0.42"));
    assert!(!io_msg.contains("5432"));

    let driver_err = sqlx::Error::Protocol("internal protocol frame secret_col".to_string());
    let driver_msg = map_db_error("saving item", driver_err).message;
    assert!(driver_msg.contains("saving item"));
    assert!(!driver_msg.contains("internal protocol frame"));
    assert!(!driver_msg.contains("secret_col"));
}
