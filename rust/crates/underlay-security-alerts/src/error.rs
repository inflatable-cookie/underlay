use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecurityAlertError {
    #[error("invalid table name")]
    InvalidTableName,

    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
}

pub type SecurityAlertResult<T> = Result<T, SecurityAlertError>;
