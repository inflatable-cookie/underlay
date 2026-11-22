//! SQLx error mapping helpers for safe API-facing responses.
//!
//! These helpers log detailed PostgreSQL/SQLx diagnostics internally via
//! `tracing` while returning sanitized, generic client-safe `AppError`
//! messages that omit raw table/column/constraint names, `DETAIL`, `HINT`,
//! and raw driver error strings.
//!
//! The resulting `AppError` uses the stable code `infra.db_error`.

use underlay_core::AppError;

fn normalize_operation(operation: &str) -> String {
    let trimmed = operation.trim().trim_end_matches('.');
    let without_prefix = trimmed
        .strip_prefix("Database error")
        .map(str::trim)
        .unwrap_or(trimmed);

    if without_prefix.is_empty() {
        "running database operation".to_string()
    } else {
        without_prefix.to_string()
    }
}

fn sqlstate_hint(code: Option<&str>) -> Option<&'static str> {
    match code {
        Some("23503") => {
            Some("Suggested fix: verify all referenced IDs exist and are not soft-deleted.")
        }
        Some("23505") => Some(
            "Suggested fix: check uniqueness constraints and update the conflicting field value.",
        ),
        Some("23502") => Some(
            "Suggested fix: ensure all required fields are present before writing to the database.",
        ),
        Some("22001") => {
            Some("Suggested fix: shorten the input value to match the column length limit.")
        }
        Some("22P02") => {
            Some("Suggested fix: validate UUID/number/date formats before sending the query.")
        }
        Some("23514") => Some(
            "Suggested fix: inspect the failed check constraint and adjust the payload accordingly.",
        ),
        _ => None,
    }
}

/// Log full SQLx diagnostics internally and build a client-safe message for a
/// database failure.
pub fn describe_db_error(operation: &str, err: &sqlx::Error) -> String {
    let operation = normalize_operation(operation);

    match err {
        sqlx::Error::Database(db_err) => {
            let sqlstate = db_err.code().map(|value| value.into_owned());
            let pg_err = db_err.try_downcast_ref::<sqlx::postgres::PgDatabaseError>();

            tracing::warn!(
                operation = %operation,
                sqlstate = ?sqlstate,
                table = ?db_err.table(),
                column = ?pg_err.and_then(|value| value.column()),
                constraint = ?db_err.constraint(),
                db_message = %db_err.message(),
                detail = ?pg_err.and_then(|value| value.detail()),
                hint = ?pg_err.and_then(|value| value.hint()),
                "database error"
            );

            let mut parts = vec![format!("Database error while {}.", operation)];
            if let Some(advice) = sqlstate_hint(sqlstate.as_deref()) {
                parts.push(advice.to_string());
            } else {
                parts.push(
                    "Suggested fix: verify the input data and retry the operation.".to_string(),
                );
            }

            parts.join(" ")
        }
        sqlx::Error::RowNotFound => {
            tracing::warn!(operation = %operation, "database query returned no rows");
            format!(
                "Database error while {}. No row matched the query. Suggested fix: verify the target ID exists and is visible in the current scope.",
                operation
            )
        }
        sqlx::Error::PoolTimedOut => {
            tracing::error!(operation = %operation, "database connection pool timed out");
            format!(
                "Database error while {}. Database connection pool timed out. Retry the operation or contact support if the problem persists.",
                operation
            )
        }
        sqlx::Error::PoolClosed => {
            tracing::error!(operation = %operation, "database connection pool is closed");
            format!(
                "Database error while {}. Database connection pool is closed. Retry the operation or contact support if the problem persists.",
                operation
            )
        }
        sqlx::Error::Io(io_err) => {
            tracing::error!(operation = %operation, error = %io_err, "database I/O error");
            format!(
                "Database error while {}. Database connection failed. Retry the operation or contact support if the problem persists.",
                operation
            )
        }
        _ => {
            tracing::error!(operation = %operation, error = %err, "database driver error");
            format!(
                "Database error while {}. An unexpected database error occurred. Suggested fix: retry the operation or contact support if the problem persists.",
                operation
            )
        }
    }
}

/// Map an owned SQLx error into the standard `infra.db_error` application error.
pub fn map_db_error(operation: &str, err: sqlx::Error) -> AppError {
    AppError::new("infra.db_error", describe_db_error(operation, &err))
}

/// Map a borrowed SQLx error into the standard `infra.db_error` application error.
pub fn map_db_error_ref(operation: &str, err: &sqlx::Error) -> AppError {
    AppError::new("infra.db_error", describe_db_error(operation, err))
}

#[cfg(test)]
#[path = "tests/db_errors_tests.rs"]
mod tests;
