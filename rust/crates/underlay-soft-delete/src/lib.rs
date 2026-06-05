//! Soft-delete semantics, conventions, and traits.
//!
//! Underlay standardises soft-delete *naming*, *result semantics*, and provides
//! traits for implementing soft-delete operations.
//!
//! # Trait-based Soft Delete
//!
//! The [`SoftDeletable`] trait enables generic soft-delete and batch soft-delete
//! operations. For simple single-table entities, use the [`impl_soft_deletable!`] macro:
//!
//! ```ignore
//! use underlay_soft_delete::impl_soft_deletable;
//!
//! // Define a marker type for your entity
//! pub struct SummaryItem;
//!
//! // Generate the SoftDeletable implementation
//! impl_soft_deletable!(SummaryItem, "content.summary_item");
//!
//! // Now you can use generic functions:
//! let result = soft_delete::<SummaryItem>(&pool, id).await?;
//! let count = batch_soft_delete::<SummaryItem>(&pool, &ids).await?;
//! ```
//!
//! For complex cascading operations, implement [`SoftDeletable`] manually:
//!
//! ```ignore
//! #[async_trait]
//! impl SoftDeletable for Module {
//!     async fn soft_delete_in_tx(
//!         tx: &mut Transaction<'_, Postgres>,
//!         id: Uuid,
//!         batch_id: Uuid,
//!     ) -> Result<bool, sqlx::Error> {
//!         // Custom cascade logic here
//!     }
//! }
//! ```
//!
//! # Legacy Macros
//!
//! For restore and purge operations, macros are available:
//!
//! ```ignore
//! use underlay_soft_delete::{impl_restore_single, impl_purge_single};
//!
//! impl_restore_single!(restore_summary, "content.summary_item");
//! impl_purge_single!(purge_summary, "content.summary_item");
//! ```

use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreBlockerKind {
    Conflict,
    ParentState,
    InvalidState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreReference {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreFieldConflict {
    pub field_name: String,
    pub candidate_value: Option<String>,
    pub active_occupant: Option<RestoreReference>,
    pub resolution_hints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreBlocker {
    pub kind: RestoreBlockerKind,
    pub entity: RestoreReference,
    pub parent: Option<RestoreReference>,
    pub parent_state: Option<String>,
    pub message: Option<String>,
    pub field_conflicts: Vec<RestoreFieldConflict>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreBatchResult {
    /// No rows were restored for this batch id.
    NotFound,
    /// One or more rows were restored.
    Restored,
    /// Restore is blocked and should not mutate any rows.
    Blocked { blockers: Vec<RestoreBlocker> },
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
// SoftDeletable Trait
// ============================================================================

/// Trait for entities that support soft-delete operations.
///
/// Implement this trait to enable generic [`soft_delete`] and [`batch_soft_delete`]
/// functions for your entity type.
///
/// For simple single-table entities, use [`impl_soft_deletable!`] to generate
/// the implementation. For complex cascading deletes, implement manually.
#[async_trait]
pub trait SoftDeletable: Send + Sync {
    /// Soft delete an entity within an existing transaction.
    ///
    /// Returns `true` if the entity was deleted, `false` if not found or already deleted.
    ///
    /// For cascade deletes, this method should delete the entity and all its
    /// dependent entities within the same transaction.
    async fn soft_delete_in_tx(
        tx: &mut Transaction<'_, Postgres>,
        id: uuid::Uuid,
        batch_id: uuid::Uuid,
    ) -> Result<bool, sqlx::Error>;
}

/// Soft delete a single entity.
///
/// Creates a new delete batch ID and transaction, delegates to the entity's
/// [`SoftDeletable::soft_delete_in_tx`] implementation.
pub async fn soft_delete<T: SoftDeletable>(
    pool: &PgPool,
    id: uuid::Uuid,
) -> Result<SoftDeleteResult, sqlx::Error> {
    let batch_id = new_delete_batch_id();
    let mut tx = pool.begin().await?;

    let deleted = T::soft_delete_in_tx(&mut tx, id, batch_id.into_inner()).await?;

    tx.commit().await?;

    if deleted {
        Ok(SoftDeleteResult::Deleted { batch_id })
    } else {
        Ok(SoftDeleteResult::NotFound)
    }
}

/// Soft delete a single entity with a provided batch ID.
///
/// Use this when you want to control the batch ID (e.g., for grouping related deletes).
pub async fn soft_delete_with_batch_id<T: SoftDeletable>(
    pool: &PgPool,
    id: uuid::Uuid,
    batch_id: DeleteBatchId,
) -> Result<SoftDeleteResult, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let deleted = T::soft_delete_in_tx(&mut tx, id, batch_id.into_inner()).await?;

    tx.commit().await?;

    if deleted {
        Ok(SoftDeleteResult::Deleted { batch_id })
    } else {
        Ok(SoftDeleteResult::NotFound)
    }
}

/// Batch soft delete multiple entities.
///
/// All entities share the same delete batch ID, enabling bulk restore.
/// Returns the count of entities that were successfully deleted.
pub async fn batch_soft_delete<T: SoftDeletable>(
    pool: &PgPool,
    ids: &[uuid::Uuid],
) -> Result<(i64, DeleteBatchId), sqlx::Error> {
    if ids.is_empty() {
        return Ok((0, new_delete_batch_id()));
    }

    let batch_id = new_delete_batch_id();
    let mut tx = pool.begin().await?;
    let mut count: i64 = 0;

    for &id in ids {
        if T::soft_delete_in_tx(&mut tx, id, batch_id.into_inner()).await? {
            count += 1;
        }
    }

    tx.commit().await?;
    Ok((count, batch_id))
}

/// Batch soft delete multiple entities with a provided batch ID.
///
/// Use this when you want to control the batch ID.
pub async fn batch_soft_delete_with_batch_id<T: SoftDeletable>(
    pool: &PgPool,
    ids: &[uuid::Uuid],
    batch_id: DeleteBatchId,
) -> Result<i64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }

    let mut tx = pool.begin().await?;
    let mut count: i64 = 0;

    for &id in ids {
        if T::soft_delete_in_tx(&mut tx, id, batch_id.into_inner()).await? {
            count += 1;
        }
    }

    tx.commit().await?;
    Ok(count)
}

// ============================================================================
// Macros
// ============================================================================

/// Generates a [`SoftDeletable`] implementation for a simple single-table entity.
///
/// This macro creates an implementation that updates a single row in the specified
/// table, setting `deleted_at` and `delete_batch_id`.
///
/// # Example
///
/// ```ignore
/// pub struct SummaryItem;
/// impl_soft_deletable!(SummaryItem, "content.summary_item");
///
/// // Now you can use:
/// let result = soft_delete::<SummaryItem>(&pool, id).await?;
/// let (count, batch_id) = batch_soft_delete::<SummaryItem>(&pool, &ids).await?;
/// ```
#[macro_export]
macro_rules! impl_soft_deletable {
    ($type:ty, $table:literal) => {
        #[async_trait::async_trait]
        impl $crate::SoftDeletable for $type {
            async fn soft_delete_in_tx(
                tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
                id: uuid::Uuid,
                batch_id: uuid::Uuid,
            ) -> Result<bool, sqlx::Error> {
                let result = sqlx::query(concat!(
                    "UPDATE ",
                    $table,
                    " SET deleted_at = NOW(), delete_batch_id = $2 WHERE id = $1 AND deleted_at IS NULL"
                ))
                .bind(id)
                .bind(batch_id)
                .execute(&mut **tx)
                .await?;

                Ok(result.rows_affected() > 0)
            }
        }
    };
}

// ============================================================================
// Legacy Macros for single-table operations
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
/// // pub async fn restore_summary(pool: &sqlx::PgPool, id: Uuid) -> Result<u64, sqlx::Error>
/// ```
#[macro_export]
macro_rules! impl_restore_single {
    ($fn_name:ident, $table:literal) => {
        pub async fn $fn_name(
            pool: &sqlx::PgPool,
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
/// // pub async fn purge_summary(pool: &sqlx::PgPool, id: Uuid) -> Result<u64, sqlx::Error>
/// ```
#[macro_export]
macro_rules! impl_purge_single {
    ($fn_name:ident, $table:literal) => {
        pub async fn $fn_name(pool: &sqlx::PgPool, id: uuid::Uuid) -> Result<u64, sqlx::Error> {
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
#[path = "tests/lib_tests.rs"]
mod tests;
