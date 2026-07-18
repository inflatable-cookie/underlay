//! Distributed-enforcement integration test for the Postgres rate-limit
//! backend, against a real Postgres.
//!
//! `#[ignore]`d by default (needs a database). Run with:
//!
//! ```bash
//! UNDERLAY_TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/postgres \
//!   cargo test -p underlay-ratelimit --features postgres --lib -- --ignored
//! ```
//!
//! This is the test the `g08.008` card promised: two backend instances (two
//! simulated app replicas) sharing one database must enforce a single window.

use std::time::Duration;

use underlay_testing::TestDb;

use crate::{PostgresBackend, RateLimitBackend, RateLimitConfig};

async fn setup() -> (TestDb, String) {
    let db = TestDb::new().await;
    let table = format!("{}.rate_limit_counters", db.schema_name());
    // Same shape as migrations/0001__rate_limit_counters.sql, in the isolated
    // test schema.
    sqlx::query(&format!(
        r#"
        CREATE TABLE {table} (
            key TEXT PRIMARY KEY,
            window_started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            count BIGINT NOT NULL DEFAULT 0
        )
        "#
    ))
    .execute(db.pool())
    .await
    .expect("create rate_limit_counters");
    (db, table)
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn two_instances_enforce_one_shared_window() {
    let (db, table) = setup().await;

    // Two backend instances over the same database = two app replicas.
    let a = PostgresBackend::with_table(db.pool().clone(), &table).expect("backend a");
    let b = PostgresBackend::with_table(db.pool().clone(), &table).expect("backend b");
    let config = RateLimitConfig::new(5, Duration::from_secs(60));
    let key = "login:203.0.113.7";

    // Five requests spread across BOTH instances are allowed…
    for i in 0..5u64 {
        let backend: &dyn RateLimitBackend = if i % 2 == 0 { &a } else { &b };
        let res = backend
            .check_and_increment(key, &config)
            .await
            .expect("check_and_increment");
        assert!(res.allowed, "request {i} should be allowed");
    }

    // …then the shared window is exhausted no matter which replica asks.
    let via_a = a
        .check_and_increment(key, &config)
        .await
        .expect("check_and_increment");
    assert!(!via_a.allowed, "6th request via instance A must be denied");
    assert!(via_a.reset_after.is_some());

    let via_b = b
        .check_and_increment(key, &config)
        .await
        .expect("check_and_increment");
    assert!(!via_b.allowed, "7th request via instance B must be denied");

    // Non-incrementing check agrees with the shared state.
    let peek = a.check(key, &config).await.expect("check");
    assert!(!peek.allowed);

    // A different key is an independent window.
    let other = b
        .check_and_increment("login:198.51.100.9", &config)
        .await
        .expect("check_and_increment");
    assert!(other.allowed);
}
