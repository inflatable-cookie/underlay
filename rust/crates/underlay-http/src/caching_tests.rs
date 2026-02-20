use std::time::Duration;

use axum::http::{header::IF_NONE_MATCH, HeaderMap, HeaderValue};

use crate::caching::{if_none_match_matches, weak_etag_for_bytes, MicroCache};

#[test]
fn weak_etag_is_stable_for_same_bytes() {
    let a = weak_etag_for_bytes(br#"{"a":1}"#);
    let b = weak_etag_for_bytes(br#"{"a":1}"#);
    let c = weak_etag_for_bytes(br#"{"a":2}"#);

    assert_eq!(a, b);
    assert_ne!(a, c);
    assert!(a.starts_with("W/\""));
    assert!(a.ends_with('"'));
}

#[test]
fn if_none_match_matches_exact_or_wildcard() {
    let current = "W/\"abc\"";

    let mut headers = HeaderMap::new();
    headers.insert(IF_NONE_MATCH, HeaderValue::from_static("W/\"abc\""));
    assert!(if_none_match_matches(&headers, current));

    let mut headers = HeaderMap::new();
    headers.insert(
        IF_NONE_MATCH,
        HeaderValue::from_static("W/\"zzz\", W/\"abc\""),
    );
    assert!(if_none_match_matches(&headers, current));

    let mut headers = HeaderMap::new();
    headers.insert(IF_NONE_MATCH, HeaderValue::from_static("*"));
    assert!(if_none_match_matches(&headers, current));

    let mut headers = HeaderMap::new();
    headers.insert(IF_NONE_MATCH, HeaderValue::from_static("W/\"nope\""));
    assert!(!if_none_match_matches(&headers, current));
}

#[test]
fn microcache_expires_entries() {
    let cache = MicroCache::new(Duration::from_millis(10), 10);
    cache.insert("k", 42);
    assert_eq!(cache.get("k"), Some(42));

    std::thread::sleep(Duration::from_millis(20));
    assert_eq!(cache.get("k"), None);
}
