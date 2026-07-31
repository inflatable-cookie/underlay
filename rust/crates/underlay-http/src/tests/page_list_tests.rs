use super::{PageList, PagePaginationParams};

#[test]
fn new_computes_has_more_from_offset() {
    let page: PageList<i32> = PageList::new(vec![1, 2, 3], 10, 0);
    assert!(page.has_more);
    assert_eq!(page.total, 10);

    let last: PageList<i32> = PageList::new(vec![8, 9, 10], 10, 7);
    assert!(!last.has_more);
}

#[test]
fn from_bounded_is_whole_set() {
    let page: PageList<i32> = PageList::from_bounded(vec![1, 2]);
    assert!(!page.has_more);
    assert_eq!(page.total, 2);
}

#[test]
fn wrap_page_list_uses_params_offset() {
    let params = PagePaginationParams { page: 2, limit: 3 };
    let page = params.wrap_page_list(vec![4, 5, 6], 7);
    assert!(page.has_more);

    // Page 3 of 7 with limit 3: offset 6, one item, no more pages.
    let params = PagePaginationParams { page: 3, limit: 3 };
    let page = params.wrap_page_list(vec![7], 7);
    assert!(!page.has_more);
}

#[test]
fn map_preserves_metadata() {
    let page: PageList<i32> = PageList::new(vec![1, 2], 5, 0);
    let mapped = page.map(|v| v.to_string());
    assert_eq!(mapped.data, vec!["1".to_string(), "2".to_string()]);
    assert_eq!(mapped.total, 5);
    assert!(mapped.has_more);
}

#[test]
fn wire_shape_is_data_total_has_more() {
    let page: PageList<i32> = PageList::new(vec![1], 2, 0);
    let json = serde_json::to_value(&page).unwrap();
    assert_eq!(json["data"], serde_json::json!([1]));
    assert_eq!(json["total"], 2);
    assert_eq!(json["has_more"], true);
    assert!(json.get("pagination").is_none());
}
