use super::{new_delete_batch_id, DELETED_AT_COLUMN, DELETE_BATCH_ID_COLUMN};

#[allow(dead_code)]
mod macro_expansion_tests {
    crate::impl_restore_single!(restore_test_row, "test.soft_deleted");
    crate::impl_purge_single!(purge_test_row, "test.soft_deleted");
}

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

#[test]
fn classify_update_reports_every_soft_delete_outcome() {
    let batch_id = new_delete_batch_id();

    assert_eq!(
        super::__private::classify_update(0, false, batch_id),
        super::SoftDeleteResult::NotFound
    );
    assert_eq!(
        super::__private::classify_update(0, true, batch_id),
        super::SoftDeleteResult::AlreadyDeleted
    );
    assert_eq!(
        super::__private::classify_update(1, true, batch_id),
        super::SoftDeleteResult::Deleted { batch_id }
    );
    assert_eq!(
        super::__private::classify_update(3, false, batch_id),
        super::SoftDeleteResult::Deleted { batch_id }
    );
}

#[allow(dead_code)]
mod trait_signature_tests {
    //! Compile-level proof that a manual implementation can report
    //! `AlreadyDeleted`, which `soft_delete` / `soft_delete_with_batch_id`
    //! return unchanged.

    use sqlx::{Postgres, Transaction};

    use crate::{DeleteBatchId, SoftDeletable, SoftDeleteResult, Uuid};

    struct AlreadyDeletedEntity;

    #[crate::__async_trait::async_trait]
    impl SoftDeletable for AlreadyDeletedEntity {
        async fn soft_delete_in_tx(
            _tx: &mut Transaction<'_, Postgres>,
            _id: Uuid,
            _batch_id: DeleteBatchId,
        ) -> Result<SoftDeleteResult, sqlx::Error> {
            Ok(SoftDeleteResult::AlreadyDeleted)
        }
    }
}
