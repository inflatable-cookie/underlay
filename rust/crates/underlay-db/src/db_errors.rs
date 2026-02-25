//! SQLx error mapping helpers for API-facing diagnostics.
//!
//! These helpers convert low-level `sqlx::Error` values into structured
//! `underlay_core::AppError` messages that preserve useful operational details:
//!
//! - SQLSTATE code (when available)
//! - table/column/constraint location details
//! - PostgreSQL `DETAIL` and `HINT` fields
//! - actionable suggested fixes for common SQLSTATE classes
//!
//! The resulting `AppError` uses the stable code `infra.db_error`.

use underlay_core::AppError;

fn normalize_space(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

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
        Some("23503") => Some("Suggested fix: verify all referenced IDs exist and are not soft-deleted."),
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

/// Build a detailed, operator-friendly message for a database failure.
pub fn describe_db_error(operation: &str, err: &sqlx::Error) -> String {
    let operation = normalize_operation(operation);

    match err {
        sqlx::Error::Database(db_err) => {
            let sqlstate = db_err.code().map(|value| value.into_owned());
            let pg_err = db_err.try_downcast_ref::<sqlx::postgres::PgDatabaseError>();

            let mut parts = vec![format!("Database error while {}.", operation)];

            if let Some(code) = &sqlstate {
                parts.push(format!("SQLSTATE {}.", code));
            }

            let mut location = Vec::new();
            if let Some(table) = db_err.table() {
                location.push(format!("table={}", table));
            }
            if let Some(column) = pg_err.and_then(|value| value.column()) {
                location.push(format!("column={}", column));
            }
            if let Some(constraint) = db_err.constraint() {
                location.push(format!("constraint={}", constraint));
            }

            if !location.is_empty() {
                parts.push(format!("Location: {}.", location.join(", ")));
            }

            parts.push(format!("Database message: {}.", normalize_space(db_err.message())));

            if let Some(detail) = pg_err.and_then(|value| value.detail()) {
                let detail = normalize_space(detail);
                if !detail.is_empty() {
                    parts.push(format!("Details: {}.", detail));
                }
            }

            if let Some(hint) = pg_err.and_then(|value| value.hint()) {
                let hint = normalize_space(hint);
                if !hint.is_empty() {
                    parts.push(format!("Database hint: {}.", hint));
                }
            }

            if let Some(advice) = sqlstate_hint(sqlstate.as_deref()) {
                parts.push(advice.to_string());
            }

            parts.join(" ")
        }
        sqlx::Error::RowNotFound => format!(
            "Database error while {}. No row matched the query. Suggested fix: verify the target ID exists and is visible in the current scope.",
            operation
        ),
        sqlx::Error::PoolTimedOut => format!(
            "Database error while {}. Database connection pool timed out. Suggested fix: check pool sizing and long-running queries.",
            operation
        ),
        sqlx::Error::PoolClosed => format!(
            "Database error while {}. Database connection pool is closed. Suggested fix: verify service startup and pool lifecycle.",
            operation
        ),
        sqlx::Error::Io(io_err) => format!(
            "Database error while {}. Database I/O error: {}. Suggested fix: check database network connectivity and host availability.",
            operation,
            normalize_space(&io_err.to_string())
        ),
        _ => format!(
            "Database error while {}. Database driver error: {}. Suggested fix: inspect the query and SQL arguments, then retry.",
            operation,
            normalize_space(&err.to_string())
        ),
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
