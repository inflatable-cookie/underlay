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
