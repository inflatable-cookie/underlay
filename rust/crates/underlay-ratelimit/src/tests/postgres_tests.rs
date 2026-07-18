use super::*;
use crate::config::RateLimitConfig;
use std::time::Duration;

// Note: full multi-instance enforcement is covered by an integration test
// against a live Postgres in CI (shared window across simulated instances).
// These unit tests cover the pure logic: table-name validation and the
// count -> RateLimitResult mapping that decides allow/deny and retry-after.

#[test]
fn table_name_validation_accepts_safe_and_rejects_unsafe() {
    assert!(validate_table_name("auth.rate_limit_counters").is_ok());
    assert!(validate_table_name("rl_counters").is_ok());

    for bad in [
        "rate limits",            // space
        "counters; DROP TABLE x", // injection
        "counters--",             // comment
        "",                       // empty
        "counters'",              // quote
    ] {
        assert!(
            validate_table_name(bad).is_err(),
            "expected {bad:?} to be rejected"
        );
    }
}

#[test]
fn to_result_allows_below_limit() {
    let config = RateLimitConfig::new(5, Duration::from_secs(60));
    let result = PostgresBackend::to_result(&config, 3, 10.0);
    assert!(result.is_allowed());
    assert_eq!(result.remaining, 2);
    assert_eq!(result.count, 3);
}

#[test]
fn to_result_allows_at_limit() {
    let config = RateLimitConfig::new(5, Duration::from_secs(60));
    let result = PostgresBackend::to_result(&config, 5, 10.0);
    assert!(result.is_allowed());
    assert_eq!(result.remaining, 0);
}

#[test]
fn to_result_denies_above_limit_with_retry_after() {
    let config = RateLimitConfig::new(5, Duration::from_secs(60));
    // 20s into a 60s window: ~40s remaining.
    let result = PostgresBackend::to_result(&config, 6, 20.0);
    assert!(result.is_denied());
    assert_eq!(result.remaining, 0);
    let retry = result.retry_after_secs();
    assert!((39..=41).contains(&retry), "retry_after was {retry}");
}

#[test]
fn to_result_denied_retry_after_is_at_least_one() {
    let config = RateLimitConfig::new(5, Duration::from_secs(60));
    // Window already elapsed; clamp keeps retry-after >= 1.
    let result = PostgresBackend::to_result(&config, 6, 120.0);
    assert!(result.is_denied());
    assert_eq!(result.retry_after_secs(), 1);
}
