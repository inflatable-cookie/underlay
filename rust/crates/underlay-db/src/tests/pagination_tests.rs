use super::*;
use uuid::Uuid;

fn test_uuid() -> Uuid {
    Uuid::parse_str("01234567-89ab-cdef-0123-456789abcdef").unwrap()
}

#[test]
fn test_cursor_encode_decode() {
    let id = test_uuid();
    let cursor = Cursor::new().with_weight(5).with_id(id);

    let encoded = cursor.encode();
    let decoded = Cursor::decode(&encoded).unwrap();

    assert_eq!(decoded.get_weight().unwrap(), 5);
    assert_eq!(decoded.get_id().unwrap(), id);
}

#[test]
fn test_cursor_decode_rejects_oversized_input() {
    let oversized = "A".repeat(8 * 1024 + 1);
    let err = Cursor::decode(&oversized).expect_err("oversized cursor should fail");
    assert!(err.to_string().contains("maximum size"));
}

#[test]
fn test_weight_cursor() {
    let id = test_uuid();
    let cursor = WeightCursor::new(10, id);
    let encoded = cursor.encode();

    let decoded = Cursor::decode(&encoded).unwrap();
    assert_eq!(decoded.get_weight().unwrap(), 10);
    assert_eq!(decoded.get_id().unwrap(), id);
}

#[test]
fn test_pagination_params_defaults() {
    let params = CursorPaginationParams::default();
    assert_eq!(params.page, 1);
    assert_eq!(params.limit, 30);
    assert!(params.cursor.is_none());
    assert_eq!(params.direction, PaginationDirection::Forward);
    assert!(params.include_total);
}

#[test]
fn test_pagination_params_effective_limit() {
    let params = CursorPaginationParams::new().with_limit(200);
    assert_eq!(params.effective_limit(), 100); // Clamped to max

    let params = CursorPaginationParams::new().with_limit(0);
    assert_eq!(params.effective_limit(), 1); // Clamped to min
}

#[test]
fn test_pagination_params_page_offset() {
    let params = CursorPaginationParams::new().with_page(3).with_limit(25);
    let builder = PaginationBuilder::new(params);

    assert_eq!(builder.page_size(), 25);
    assert_eq!(builder.query_offset(), 50);
}

#[test]
fn test_paginated_response_map() {
    let response = PaginatedResponse::new(vec![1, 2, 3])
        .with_has_more(true)
        .with_total(Some(100));

    let mapped = response.map(|x| x * 2);
    assert_eq!(mapped.data, vec![2, 4, 6]);
    assert!(mapped.has_more);
    assert_eq!(mapped.total, Some(100));
}

#[test]
fn test_keyset_operator_forward_desc() {
    let params = CursorPaginationParams::new();
    let builder = PaginationBuilder::new(params);
    assert_eq!(builder.keyset_operator(true), "<");
}

#[test]
fn test_keyset_operator_forward_asc() {
    let params = CursorPaginationParams::new();
    let builder = PaginationBuilder::new(params);
    assert_eq!(builder.keyset_operator(false), ">");
}

#[test]
fn test_keyset_operator_backward_desc() {
    let params = CursorPaginationParams::new().with_direction(PaginationDirection::Backward);
    let builder = PaginationBuilder::new(params);
    assert_eq!(builder.keyset_operator(true), ">");
}

#[test]
fn test_keyset_operator_backward_asc() {
    let params = CursorPaginationParams::new().with_direction(PaginationDirection::Backward);
    let builder = PaginationBuilder::new(params);
    assert_eq!(builder.keyset_operator(false), "<");
}

#[test]
fn test_keyset_condition() {
    let params = CursorPaginationParams::new();
    let builder = PaginationBuilder::new(params);
    assert_eq!(
        builder.keyset_condition("updated_at", 1, true).unwrap(),
        "(\"updated_at\", id) < ($1, $2)"
    );
    assert_eq!(
        builder.keyset_condition("weight", 3, false).unwrap(),
        "(\"weight\", id) > ($3, $4)"
    );
}

#[test]
fn test_keyset_condition_quotes_and_trims_identifier() {
    let builder = PaginationBuilder::new(CursorPaginationParams::new());
    assert_eq!(
        builder.keyset_condition(" _rank ", 2, true).unwrap(),
        "(\"_rank\", id) < ($2, $3)"
    );
}

#[test]
fn test_keyset_helpers_reject_invalid_identifiers() {
    let builder = PaginationBuilder::new(CursorPaginationParams::new());

    assert!(builder
        .keyset_condition("updated_at; DROP TABLE items", 1, true)
        .is_err());
    assert!(builder.keyset_condition("", 1, true).is_err());
    assert!(builder.keyset_order_by("Updated_At", true).is_err());
    assert!(builder.keyset_order_by("updated-at", true).is_err());
}

#[test]
fn test_keyset_order_by() {
    let params = CursorPaginationParams::new();
    let builder = PaginationBuilder::new(params);
    assert_eq!(
        builder.keyset_order_by("updated_at", true).unwrap(),
        "\"updated_at\" DESC, id DESC"
    );
    assert_eq!(
        builder.keyset_order_by("weight", false).unwrap(),
        "\"weight\" ASC, id ASC"
    );
}

#[test]
fn test_keyset_order_by_backward() {
    let params = CursorPaginationParams::new().with_direction(PaginationDirection::Backward);
    let builder = PaginationBuilder::new(params);
    // Backward pagination reverses the order
    assert_eq!(
        builder.keyset_order_by("updated_at", true).unwrap(),
        "\"updated_at\" ASC, id ASC"
    );
}

#[test]
fn test_has_cursor() {
    let params = CursorPaginationParams::new();
    let builder = PaginationBuilder::new(params);
    assert!(!builder.has_cursor());

    let params_with_cursor = CursorPaginationParams::new().with_cursor("abc123");
    let builder_with_cursor = PaginationBuilder::new(params_with_cursor);
    assert!(builder_with_cursor.has_cursor());
}
