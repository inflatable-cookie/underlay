use chrono::{DateTime, Duration, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::error::SecurityAlertResult;
use crate::tables::{LoginAttemptsTable, SecurityAlertEventsTable};
use crate::types::{LoginAttemptSignalCounts, SecurityAlertEventInput, SecurityAlertType};

pub async fn load_ip_signal_counts_from_table(
    pool: &PgPool,
    login_attempts_table: &LoginAttemptsTable,
    ip_address: &str,
    since: DateTime<Utc>,
) -> SecurityAlertResult<LoginAttemptSignalCounts> {
    let login_attempts_table = login_attempts_table.quoted();

    let query = format!(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE success = FALSE) AS failed_attempts,
            COUNT(DISTINCT user_id) FILTER (WHERE success = FALSE) AS distinct_users,
            COUNT(*) FILTER (WHERE success = FALSE AND failure_reason = 'account_locked') AS lockouts
        FROM {}
        WHERE ip_address = $1::inet
          AND attempted_at >= $2
        "#,
        login_attempts_table
    );

    let row = sqlx::query(&query)
        .bind(ip_address)
        .bind(since)
        .fetch_one(pool)
        .await?;

    Ok(LoginAttemptSignalCounts {
        failed_attempts: row.get::<Option<i64>, _>("failed_attempts").unwrap_or(0),
        distinct_users: row.get::<Option<i64>, _>("distinct_users").unwrap_or(0),
        lockouts: row.get::<Option<i64>, _>("lockouts").unwrap_or(0),
    })
}

pub async fn has_recent_alert_in_table(
    pool: &PgPool,
    alert_events_table: &SecurityAlertEventsTable,
    alert_type: SecurityAlertType,
    ip_address: &str,
    cooldown: Duration,
    now: DateTime<Utc>,
) -> SecurityAlertResult<bool> {
    let alert_events_table = alert_events_table.quoted();

    let since = now - cooldown;
    let query = format!(
        r#"
        SELECT COUNT(*) AS count
        FROM {}
        WHERE alert_type = $1
          AND ip_address = $2::inet
          AND created_at >= $3
        "#,
        alert_events_table
    );

    let row = sqlx::query(&query)
        .bind(alert_type.as_str())
        .bind(ip_address)
        .bind(since)
        .fetch_one(pool)
        .await?;

    Ok(row.get::<Option<i64>, _>("count").unwrap_or(0) > 0)
}

pub async fn insert_alert_event_into_table(
    pool: &PgPool,
    alert_events_table: &SecurityAlertEventsTable,
    input: &SecurityAlertEventInput,
) -> SecurityAlertResult<Uuid> {
    let alert_events_table = alert_events_table.quoted();

    let query = format!(
        r#"
        INSERT INTO {} (
            alert_type,
            ip_address,
            window_started_at,
            window_ended_at,
            failed_attempts,
            distinct_user_count,
            lockout_count,
            details
        )
        VALUES ($1, $2::inet, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
        alert_events_table
    );

    let row = sqlx::query(&query)
        .bind(input.alert_type.as_str())
        .bind(input.ip_address.as_str())
        .bind(input.window_started_at)
        .bind(input.window_ended_at)
        .bind(input.counts.failed_attempts)
        .bind(input.counts.distinct_users)
        .bind(input.counts.lockouts)
        .bind(&input.details)
        .fetch_one(pool)
        .await?;

    Ok(row.get("id"))
}
