//! Soft-delete semantics and conventions.
//!
//! Underlay standardises soft-delete *naming* and *result semantics*.
//! Actual cascade graphs and SQL statements remain application-owned.
//!
//! # Macros
//!
//! For simple single-table operations, this crate provides macros to reduce boilerplate:
//!
//! ```ignore
//! use underlay_soft_delete::{impl_restore_single, impl_purge_single};
//!
//! // Generates: pub async fn restore_summary(pool: &DbPool, id: Uuid) -> Result<u64, sqlx::Error>
//! impl_restore_single!(restore_summary, "content.summary_item");
//!
//! // Generates: pub async fn purge_summary(pool: &DbPool, id: Uuid) -> Result<u64, sqlx::Error>
//! impl_purge_single!(purge_summary, "content.summary_item");
//! ```
//!
//! For complex cascading operations (deleting across multiple tables), implement
//! the logic manually in your application.

use underlay_core::Uuid;

pub const DELETED_AT_COLUMN: &str = "deleted_at";
pub const DELETE_BATCH_ID_COLUMN: &str = "delete_batch_id";

/// A stable identifier used to correlate a soft-delete operation across multiple tables.
///
/// In reference implementations this is stored in `delete_batch_id`.
pub type DeleteBatchId = Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoftDeleteResult {
    /// The target row does not exist.
    NotFound,
    /// The row exists but is already marked as deleted.
    AlreadyDeleted,
    /// The row was successfully soft-deleted.
    Deleted { batch_id: DeleteBatchId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestoreBatchResult {
    /// No rows were restored for this batch id.
    NotFound,
    /// One or more rows were restored.
    Restored,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PurgeBatchResult {
    /// No rows were found to purge for this batch id.
    NotFound,
    /// One or more rows were permanently deleted.
    Purged,
}

pub fn new_delete_batch_id() -> DeleteBatchId {
    Uuid::new_v7()
}

// ============================================================================
// Macros for single-table operations
// ============================================================================

/// Generates a function to restore a single soft-deleted row by ID.
///
/// The generated function clears `deleted_at` and `delete_batch_id` for the row
/// where `id` matches and `deleted_at IS NOT NULL`.
///
/// # Example
///
/// ```ignore
/// impl_restore_single!(restore_summary, "content.summary_item");
///
/// // Generates:
/// // pub async fn restore_summary(pool: &DbPool, id: Uuid) -> Result<u64, sqlx::Error>
/// ```
#[macro_export]
macro_rules! impl_restore_single {
    ($fn_name:ident, $table:literal) => {
        pub async fn $fn_name(
            pool: &crate::DbPool,
            id: uuid::Uuid,
        ) -> Result<u64, sqlx::Error> {
            let result = sqlx::query(concat!(
                "UPDATE ",
                $table,
                " SET deleted_at = NULL, delete_batch_id = NULL WHERE id = $1 AND deleted_at IS NOT NULL"
            ))
            .bind(id)
            .execute(pool)
            .await?;

            Ok(result.rows_affected())
        }
    };
}

/// Generates a function to permanently delete a single soft-deleted row by ID.
///
/// The generated function deletes the row where `id` matches and `deleted_at IS NOT NULL`.
/// This ensures only already-soft-deleted rows can be purged.
///
/// # Example
///
/// ```ignore
/// impl_purge_single!(purge_summary, "content.summary_item");
///
/// // Generates:
/// // pub async fn purge_summary(pool: &DbPool, id: Uuid) -> Result<u64, sqlx::Error>
/// ```
#[macro_export]
macro_rules! impl_purge_single {
    ($fn_name:ident, $table:literal) => {
        pub async fn $fn_name(pool: &crate::DbPool, id: uuid::Uuid) -> Result<u64, sqlx::Error> {
            let result = sqlx::query(concat!(
                "DELETE FROM ",
                $table,
                " WHERE id = $1 AND deleted_at IS NOT NULL"
            ))
            .bind(id)
            .execute(pool)
            .await?;

            Ok(result.rows_affected())
        }
    };
}

#[cfg(test)]
mod tests {
    use super::{new_delete_batch_id, DELETED_AT_COLUMN, DELETE_BATCH_ID_COLUMN};

    #[test]
    fn column_names_match_convention() {
        assert_eq!(DELETED_AT_COLUMN, "deleted_at");
        assert_eq!(DELETE_BATCH_ID_COLUMN, "delete_batch_id");
    }

    #[test]
    fn batch_id_is_valid_uuid() {
        let id = new_delete_batch_id();
        let parsed = underlay_core::Uuid::parse_str(&id.to_string()).expect("should parse");
        assert_eq!(id, parsed);
    }
}
