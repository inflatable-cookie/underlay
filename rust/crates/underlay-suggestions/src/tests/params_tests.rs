use super::*;

#[test]
fn test_empty_params() {
    let params = SuggestionParams::new();
    assert!(!params.wants_suggestions());
    assert!(params.recent_hints().is_empty());
}

#[test]
fn test_from_query_string() {
    let params = SuggestionParams::from_query_string("suggestions=true&recentHints=id1,id2,id3");
    assert!(params.wants_suggestions());
    assert_eq!(params.recent_hints(), vec!["id1", "id2", "id3"]);
}

#[test]
fn test_from_query_string_no_hints() {
    let params = SuggestionParams::from_query_string("suggestions=true");
    assert!(params.wants_suggestions());
    assert!(params.recent_hints().is_empty());
}

#[test]
fn test_from_query_string_empty() {
    let params = SuggestionParams::from_query_string("");
    assert!(!params.wants_suggestions());
    assert!(params.recent_hints().is_empty());
}

#[test]
fn test_with_hints() {
    let params = SuggestionParams::with_hints(vec!["a", "b", "c"]);
    assert!(params.wants_suggestions());
    assert_eq!(params.recent_hints(), vec!["a", "b", "c"]);
}

#[test]
fn test_whitespace_handling() {
    let params = SuggestionParams::from_query_string("suggestions=true&recentHints=id1, id2 ,id3");
    assert_eq!(params.recent_hints(), vec!["id1", "id2", "id3"]);
}

#[test]
fn test_empty_hint_filtering() {
    let params = SuggestionParams::from_query_string("suggestions=true&recentHints=id1,,id3");
    assert_eq!(params.recent_hints(), vec!["id1", "id3"]);
}
