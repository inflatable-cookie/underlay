use super::*;

#[test]
fn test_default_query() {
    let query = SuggestionQuery::new();
    assert!(query.hint_ids().is_empty());
    assert_eq!(query.limit(), 15);
    assert_eq!(query.order(), SuggestionOrder::HintsThenRecent);
}

#[test]
fn test_with_hints() {
    let query = SuggestionQuery::new().with_hints(vec!["a", "b", "c"]);

    assert_eq!(query.hint_ids(), &["a", "b", "c"]);
    assert!(query.has_hints());
    assert_eq!(query.hint_count(), 3);
}

#[test]
fn test_fill_limit() {
    let query = SuggestionQuery::new()
        .with_hints(vec!["a", "b", "c"])
        .with_limit(10);

    assert_eq!(query.fill_limit(3), 7);
    assert_eq!(query.fill_limit(0), 10);
    assert_eq!(query.fill_limit(10), 0);
    assert_eq!(query.fill_limit(15), 0); // saturating
}
