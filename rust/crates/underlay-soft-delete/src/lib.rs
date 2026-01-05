//! Soft-delete semantics and conventions.
//!
//! Underlay standardises soft-delete *naming* and *result semantics*.
//! Actual cascade graphs and SQL statements remain application-owned.

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

pub fn new_delete_batch_id() -> DeleteBatchId {
    Uuid::new_v7()
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
