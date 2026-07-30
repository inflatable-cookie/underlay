//! Shared 500-response builder for failed database operations.
//!
//! The client-facing message is the static `operation` string only. Schema
//! details (table/column/constraint from SQLSTATE diagnostics) stay in the
//! error context, which flows to the error-logging middleware but is never
//! serialized to the wire.

use crate::errors::ApiError;

/// Build a 500 [`ApiError`] for a failed database operation.
///
/// - Wire message: the static `operation` string (e.g. `"Failed to update user"`).
/// - `cause`: the raw error display string (log path only).
/// - `context.diagnostic`: SQLSTATE/table/column/constraint detail when the
///   error is a `sqlx::Error` (log path only).
pub fn internal_db_error<E>(code: &'static str, operation: &str, err: &E) -> ApiError
where
    E: std::any::Any + std::fmt::Display,
{
    let sqlx_err = (err as &dyn std::any::Any)
        .downcast_ref::<sqlx::Error>()
        .or_else(|| {
            (err as &dyn std::any::Any)
                .downcast_ref::<Box<dyn std::error::Error + Send + Sync>>()
                .and_then(|boxed| boxed.as_ref().downcast_ref::<sqlx::Error>())
        });

    let diagnostic = sqlx_err
        .map(|db_err| underlay_db::describe_db_error(operation, db_err))
        .unwrap_or_else(|| format!("{operation}. Cause: {err}"));

    ApiError::internal(code, operation.to_string())
        .with_cause(err)
        .with_context(serde_json::json!({ "diagnostic": diagnostic }))
}
