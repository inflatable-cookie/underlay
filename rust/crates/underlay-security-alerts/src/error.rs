use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecurityAlertError {
    #[error("invalid table name")]
    InvalidTableName,

    #[error("invalid security alert config `{field}`: {reason}")]
    InvalidConfig {
        field: &'static str,
        reason: &'static str,
    },

    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
}

pub type SecurityAlertResult<T> = Result<T, SecurityAlertError>;
