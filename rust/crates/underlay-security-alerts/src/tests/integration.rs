//! Integration tests for the security-alert store against a real Postgres.
//!
//! `#[ignore]`d by default (needs a database). Run with:
//!
//! ```bash
//! UNDERLAY_TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/postgres \
//!   cargo test -p underlay-security-alerts -- --ignored
//! ```

use chrono::{Duration, Utc};
use serde_json::json;
use uuid::Uuid;

use underlay_testing::TestDb;

use crate::store::{
    has_recent_alert_in_table, has_recent_scoped_alert_in_table, insert_alert_event_into_table,
    insert_scoped_alert_event_into_table, load_account_signal_counts_from_table,
    load_global_signal_counts_from_table, load_ip_signal_counts_from_table,
};
use crate::tables::{LoginAttemptsTable, SecurityAlertEventsTable};
use crate::types::{
    LoginAttemptSignalCounts, ScopedSecurityAlertEventInput, SecurityAlertEventInput,
    SecurityAlertType,
};

struct Fixture {
    db: TestDb,
    login_attempts: LoginAttemptsTable,
    alert_events: SecurityAlertEventsTable,
}

/// Create the login-attempts and alert-events tables in this test's isolated
/// schema, matching the crate's example migrations (`0001`/`0002`).
async fn setup() -> Fixture {
    let db = TestDb::new().await;
    let schema = db.schema_name().to_string();

    sqlx::query(&format!(
        r#"
        CREATE TABLE {schema}.login_attempts (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NULL,
            ip_address INET NOT NULL,
            success BOOLEAN NOT NULL,
            failure_reason TEXT NULL,
            attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    ))
    .execute(db.pool())
    .await
    .expect("create login_attempts");

    sqlx::query(&format!(
        r#"
        CREATE TABLE {schema}.security_alert_events (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            alert_type TEXT NOT NULL,
            scope_key TEXT NOT NULL DEFAULT '',
            ip_address INET NULL,
            window_started_at TIMESTAMPTZ NOT NULL,
            window_ended_at TIMESTAMPTZ NOT NULL,
            failed_attempts BIGINT NOT NULL DEFAULT 0,
            distinct_user_count BIGINT NOT NULL DEFAULT 0,
            lockout_count BIGINT NOT NULL DEFAULT 0,
            details JSONB NOT NULL DEFAULT '{{}}'::jsonb
        )
        "#
    ))
    .execute(db.pool())
    .await
    .expect("create security_alert_events");

    let login_attempts =
        LoginAttemptsTable::parse(format!("{schema}.login_attempts")).expect("valid table");
    let alert_events = SecurityAlertEventsTable::parse(format!("{schema}.security_alert_events"))
        .expect("valid table");

    Fixture {
        db,
        login_attempts,
        alert_events,
    }
}

async fn record_attempt(
    fx: &Fixture,
    user_id: Option<Uuid>,
    ip: &str,
    success: bool,
    failure_reason: Option<&str>,
) {
    sqlx::query(&format!(
        "INSERT INTO {}.login_attempts (user_id, ip_address, success, failure_reason) \
         VALUES ($1, $2::inet, $3, $4)",
        fx.db.schema_name()
    ))
    .bind(user_id)
    .bind(ip)
    .bind(success)
    .bind(failure_reason)
    .execute(fx.db.pool())
    .await
    .expect("insert login attempt");
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn ip_signal_counts_aggregate_failures_users_and_lockouts() {
    let fx = setup().await;
    let ip = "203.0.113.7";
    let user_a = Uuid::now_v7();
    let user_b = Uuid::now_v7();

    record_attempt(&fx, Some(user_a), ip, false, Some("bad_password")).await;
    record_attempt(&fx, Some(user_b), ip, false, Some("account_locked")).await;
    record_attempt(&fx, Some(user_a), ip, true, None).await; // success ignored
    record_attempt(&fx, Some(user_a), "198.51.100.1", false, None).await; // other IP ignored

    let counts = load_ip_signal_counts_from_table(
        fx.db.pool(),
        &fx.login_attempts,
        ip,
        Utc::now() - Duration::hours(1),
    )
    .await
    .expect("ip counts");

    assert_eq!(counts.failed_attempts, 2);
    assert_eq!(counts.distinct_users, 2);
    assert_eq!(counts.lockouts, 1);
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn account_and_global_counts_scope_correctly() {
    let fx = setup().await;
    let user = Uuid::now_v7();

    record_attempt(&fx, Some(user), "203.0.113.7", false, None).await;
    record_attempt(&fx, Some(user), "198.51.100.1", false, None).await;
    record_attempt(&fx, Some(Uuid::now_v7()), "192.0.2.5", false, None).await;

    let since = Utc::now() - Duration::hours(1);

    let account =
        load_account_signal_counts_from_table(fx.db.pool(), &fx.login_attempts, user, since)
            .await
            .expect("account counts");
    assert_eq!(account.failed_attempts, 2);
    assert_eq!(account.distinct_ips, 2);

    let global = load_global_signal_counts_from_table(fx.db.pool(), &fx.login_attempts, since)
        .await
        .expect("global counts");
    assert_eq!(global.failed_attempts, 3);
    assert_eq!(global.distinct_ips, 3);
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn ip_alert_insert_and_cooldown() {
    let fx = setup().await;
    let now = Utc::now();
    let ip = "203.0.113.7";

    assert!(!has_recent_alert_in_table(
        fx.db.pool(),
        &fx.alert_events,
        SecurityAlertType::LoginFailuresFromIp,
        ip,
        Duration::minutes(30),
        now,
    )
    .await
    .expect("cooldown check"));

    let id = insert_alert_event_into_table(
        fx.db.pool(),
        &fx.alert_events,
        &SecurityAlertEventInput {
            alert_type: SecurityAlertType::LoginFailuresFromIp,
            ip_address: ip.to_string(),
            window_started_at: now - Duration::minutes(5),
            window_ended_at: now,
            counts: LoginAttemptSignalCounts {
                failed_attempts: 8,
                distinct_users: 3,
                lockouts: 1,
            },
            details: json!({ "note": "burst" }),
        },
    )
    .await
    .expect("insert alert");
    assert_ne!(id, Uuid::nil());

    // Now within cooldown.
    assert!(has_recent_alert_in_table(
        fx.db.pool(),
        &fx.alert_events,
        SecurityAlertType::LoginFailuresFromIp,
        ip,
        Duration::minutes(30),
        now,
    )
    .await
    .expect("cooldown check"));

    // A different alert type on the same IP is not in cooldown.
    assert!(!has_recent_alert_in_table(
        fx.db.pool(),
        &fx.alert_events,
        SecurityAlertType::LockoutsFromIp,
        ip,
        Duration::minutes(30),
        now,
    )
    .await
    .expect("cooldown check"));
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn scoped_alert_insert_and_cooldown() {
    let fx = setup().await;
    let now = Utc::now();
    let scope = format!("account:{}", Uuid::now_v7());

    let id = insert_scoped_alert_event_into_table(
        fx.db.pool(),
        &fx.alert_events,
        &ScopedSecurityAlertEventInput {
            alert_type: SecurityAlertType::LoginFailuresForAccount,
            scope_key: scope.clone(),
            ip_address: None,
            window_started_at: now - Duration::minutes(5),
            window_ended_at: now,
            failed_attempts: 12,
            details: json!({}),
        },
    )
    .await
    .expect("insert scoped alert");
    assert_ne!(id, Uuid::nil());

    assert!(has_recent_scoped_alert_in_table(
        fx.db.pool(),
        &fx.alert_events,
        SecurityAlertType::LoginFailuresForAccount,
        &scope,
        Duration::minutes(30),
        now,
    )
    .await
    .expect("scoped cooldown"));

    // A different scope key is not in cooldown.
    assert!(!has_recent_scoped_alert_in_table(
        fx.db.pool(),
        &fx.alert_events,
        SecurityAlertType::LoginFailuresForAccount,
        "account:other",
        Duration::minutes(30),
        now,
    )
    .await
    .expect("scoped cooldown"));
}
