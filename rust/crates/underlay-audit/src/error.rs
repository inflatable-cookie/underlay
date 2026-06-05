//! Audit error types.

/// Errors that can occur during audit operations.
#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    /// Invalid table name (potential SQL injection).
    #[error("invalid table name")]
    InvalidTableName,

    /// Database error.
    #[error("database error")]
    Db(#[from] sqlx::Error),
}

/// Result type alias for audit operations.
pub type AuditResult<T> = Result<T, AuditError>;
