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

#[test]
fn test_with_hints_deduplicates_and_enforces_max_recent_hints() {
    let mut raw: Vec<String> = vec!["dup".to_string(), "first".to_string(), "dup".to_string()];
    raw.extend((0..MAX_RECENT_HINTS + 20).map(|i| format!("hint-{i}")));

    let query = SuggestionQuery::new().with_hints(raw);
    assert_eq!(query.hint_count(), MAX_RECENT_HINTS);
    assert_eq!(query.hint_ids()[0], "dup");
    assert_eq!(query.hint_ids()[1], "first");
    assert_eq!(query.hint_ids()[2], "hint-0");
}
