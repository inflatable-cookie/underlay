use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use axum::http::{
    header::{IF_MATCH, IF_NONE_MATCH},
    HeaderMap, HeaderValue,
};

use crate::caching::{
    if_match_matches, if_none_match_matches, weak_etag_for_bytes, MicroCache, SingleFlight,
};

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
fn if_match_matches_exact_or_wildcard() {
    let current = "W/\"abc\"";

    let mut headers = HeaderMap::new();
    headers.insert(IF_MATCH, HeaderValue::from_static("W/\"abc\""));
    let strong = "\"abc\"";
    let mut headers = HeaderMap::new();
    headers.insert(IF_MATCH, HeaderValue::from_static("\"abc\""));
    assert!(if_match_matches(&headers, strong));

    // RFC 9110 strong comparison: weak ETags never match If-Match.
    let mut headers = HeaderMap::new();
    headers.insert(IF_MATCH, HeaderValue::from_static("W/\"zzz\", W/\"abc\""));
    assert!(!if_match_matches(&headers, current));
    assert!(!if_match_matches(&headers, strong));

    let mut headers = HeaderMap::new();
    headers.insert(IF_MATCH, HeaderValue::from_static("*"));
    assert!(if_match_matches(&headers, current));

    let mut headers = HeaderMap::new();
    headers.insert(IF_MATCH, HeaderValue::from_static("W/\"nope\""));
    assert!(!if_match_matches(&headers, current));
}

#[test]
fn etag_matchers_inspect_all_header_values() {
    let mut headers = HeaderMap::new();
    headers.append(IF_NONE_MATCH, HeaderValue::from_static("W/\"first\""));
    headers.append(IF_NONE_MATCH, HeaderValue::from_static("W/\"second\""));
    assert!(if_none_match_matches(&headers, "W/\"second\""));

    let mut headers = HeaderMap::new();
    headers.append(IF_MATCH, HeaderValue::from_static("\"first\""));
    headers.append(IF_MATCH, HeaderValue::from_static("\"second\""));
    assert!(if_match_matches(&headers, "\"second\""));
}

#[tokio::test]
async fn singleflight_leader_cancellation_does_not_deadlock_waiters() {
    let sf = Arc::new(SingleFlight::<usize>::new());
    let sf_leader = Arc::clone(&sf);
    let leader = tokio::spawn(async move {
        sf_leader
            .run("cancel-key", || async {
                tokio::time::sleep(Duration::from_secs(60)).await;
                1
            })
            .await
    });

    tokio::time::sleep(Duration::from_millis(10)).await;
    let sf_waiter = Arc::clone(&sf);
    let waiter = tokio::spawn(async move { sf_waiter.run("cancel-key", || async { 42 }).await });

    tokio::time::sleep(Duration::from_millis(10)).await;
    leader.abort();

    let result = tokio::time::timeout(Duration::from_secs(2), waiter)
        .await
        .expect("waiter should not deadlock after leader cancellation")
        .expect("waiter task should join");
    assert_eq!(result, 42);
}

#[test]
fn microcache_expires_entries() {
    let cache = MicroCache::new(Duration::from_millis(50), 10);
    cache.insert("k", 42);
    assert_eq!(cache.get("k"), Some(42));

    std::thread::sleep(Duration::from_millis(80));
    assert_eq!(cache.get("k"), None);
}

#[test]
fn microcache_recovers_from_poisoned_entries_lock() {
    let cache = MicroCache::new(Duration::from_secs(1), 10);
    cache.test_poison_entries_lock();

    cache.insert("k", 42);
    assert_eq!(cache.get("k"), Some(42));

    cache.invalidate("k");
    assert_eq!(cache.get("k"), None);
}

#[tokio::test]
async fn singleflight_coalesces_same_key() {
    let sf = Arc::new(SingleFlight::<usize>::new());
    let executions = Arc::new(AtomicUsize::new(0));

    let mut tasks = Vec::new();
    for _ in 0..12 {
        let sf = Arc::clone(&sf);
        let executions = Arc::clone(&executions);
        tasks.push(tokio::spawn(async move {
            sf.run("same-key", || async move {
                executions.fetch_add(1, Ordering::SeqCst);
                tokio::time::sleep(Duration::from_millis(20)).await;
                7
            })
            .await
        }));
    }

    for task in tasks {
        let value = task.await.expect("singleflight task should join");
        assert_eq!(value, 7);
    }
    assert_eq!(executions.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn singleflight_allows_distinct_keys() {
    let sf = Arc::new(SingleFlight::<usize>::new());
    let executions = Arc::new(AtomicUsize::new(0));

    let (a, b) = tokio::join!(
        {
            let sf = Arc::clone(&sf);
            let executions = Arc::clone(&executions);
            async move {
                sf.run("k1", || async move {
                    executions.fetch_add(1, Ordering::SeqCst);
                    1
                })
                .await
            }
        },
        {
            let sf = Arc::clone(&sf);
            let executions = Arc::clone(&executions);
            async move {
                sf.run("k2", || async move {
                    executions.fetch_add(1, Ordering::SeqCst);
                    2
                })
                .await
            }
        }
    );

    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(executions.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn singleflight_waiter_falls_back_if_sender_drops() {
    let sf = Arc::new(SingleFlight::<usize>::new());
    sf.test_insert_inflight_key("drop-key").await;

    let sf_for_cleanup = Arc::clone(&sf);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(5)).await;
        sf_for_cleanup.test_remove_inflight_key("drop-key").await;
    });

    let value = sf
        .run("drop-key", || async move {
            tokio::time::sleep(Duration::from_millis(1)).await;
            99
        })
        .await;

    assert_eq!(value, 99);
}
