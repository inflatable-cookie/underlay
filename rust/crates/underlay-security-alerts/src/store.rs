use chrono::{DateTime, Duration, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::error::SecurityAlertResult;
use crate::tables::{LoginAttemptsTable, SecurityAlertEventsTable};
use crate::types::{
    AccountSignalCounts, GlobalSignalCounts, LoginAttemptSignalCounts,
    ScopedSecurityAlertEventInput, SecurityAlertEventInput, SecurityAlertType,
};

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

    let row = sqlx::query(sqlx::AssertSqlSafe(query))
        .bind(ip_address)
        .bind(since)
        .fetch_one(pool)
        .await?;

    Ok(LoginAttemptSignalCounts {
        failed_attempts: row
            .try_get::<Option<i64>, _>("failed_attempts")?
            .unwrap_or(0),
        distinct_users: row
            .try_get::<Option<i64>, _>("distinct_users")?
            .unwrap_or(0),
        lockouts: row.try_get::<Option<i64>, _>("lockouts")?.unwrap_or(0),
    })
}

/// Load failed-attempt counts for one account across all IPs.
pub async fn load_account_signal_counts_from_table(
    pool: &PgPool,
    login_attempts_table: &LoginAttemptsTable,
    user_id: Uuid,
    since: DateTime<Utc>,
) -> SecurityAlertResult<AccountSignalCounts> {
    let login_attempts_table = login_attempts_table.quoted();

    let query = format!(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE success = FALSE) AS failed_attempts,
            COUNT(DISTINCT ip_address) FILTER (WHERE success = FALSE) AS distinct_ips
        FROM {}
        WHERE user_id = $1
          AND attempted_at >= $2
        "#,
        login_attempts_table
    );

    let row = sqlx::query(sqlx::AssertSqlSafe(query))
        .bind(user_id)
        .bind(since)
        .fetch_one(pool)
        .await?;

    Ok(AccountSignalCounts {
        failed_attempts: row
            .try_get::<Option<i64>, _>("failed_attempts")?
            .unwrap_or(0),
        distinct_ips: row.try_get::<Option<i64>, _>("distinct_ips")?.unwrap_or(0),
    })
}

/// Load failed-attempt counts across all accounts and IPs in the window.
pub async fn load_global_signal_counts_from_table(
    pool: &PgPool,
    login_attempts_table: &LoginAttemptsTable,
    since: DateTime<Utc>,
) -> SecurityAlertResult<GlobalSignalCounts> {
    let login_attempts_table = login_attempts_table.quoted();

    let query = format!(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE success = FALSE) AS failed_attempts,
            COUNT(DISTINCT ip_address) FILTER (WHERE success = FALSE) AS distinct_ips
        FROM {}
        WHERE attempted_at >= $1
        "#,
        login_attempts_table
    );

    let row = sqlx::query(sqlx::AssertSqlSafe(query))
        .bind(since)
        .fetch_one(pool)
        .await?;

    Ok(GlobalSignalCounts {
        failed_attempts: row
            .try_get::<Option<i64>, _>("failed_attempts")?
            .unwrap_or(0),
        distinct_ips: row.try_get::<Option<i64>, _>("distinct_ips")?.unwrap_or(0),
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

    let row = sqlx::query(sqlx::AssertSqlSafe(query))
        .bind(alert_type.as_str())
        .bind(ip_address)
        .bind(since)
        .fetch_one(pool)
        .await?;

    Ok(row.try_get::<Option<i64>, _>("count")?.unwrap_or(0) > 0)
}

/// Cooldown check for non-IP alert scopes, keyed on `scope_key`
/// (requires the `0002__security_alert_scopes` migration).
pub async fn has_recent_scoped_alert_in_table(
    pool: &PgPool,
    alert_events_table: &SecurityAlertEventsTable,
    alert_type: SecurityAlertType,
    scope_key: &str,
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
          AND scope_key = $2
          AND created_at >= $3
        "#,
        alert_events_table
    );

    let row = sqlx::query(sqlx::AssertSqlSafe(query))
        .bind(alert_type.as_str())
        .bind(scope_key)
        .bind(since)
        .fetch_one(pool)
        .await?;

    Ok(row.try_get::<Option<i64>, _>("count")?.unwrap_or(0) > 0)
}

/// Insert an account/global alert event
/// (requires the `0002__security_alert_scopes` migration).
pub async fn insert_scoped_alert_event_into_table(
    pool: &PgPool,
    alert_events_table: &SecurityAlertEventsTable,
    input: &ScopedSecurityAlertEventInput,
) -> SecurityAlertResult<Uuid> {
    let alert_events_table = alert_events_table.quoted();

    let query = format!(
        r#"
        INSERT INTO {} (
            alert_type,
            scope_key,
            ip_address,
            window_started_at,
            window_ended_at,
            failed_attempts,
            details
        )
        VALUES ($1, $2, $3::inet, $4, $5, $6, $7)
        RETURNING id
        "#,
        alert_events_table
    );

    let row = sqlx::query(sqlx::AssertSqlSafe(query))
        .bind(input.alert_type.as_str())
        .bind(&input.scope_key)
        .bind(input.ip_address.as_deref())
        .bind(input.window_started_at)
        .bind(input.window_ended_at)
        .bind(input.failed_attempts)
        .bind(&input.details)
        .fetch_one(pool)
        .await?;

    Ok(row.try_get("id")?)
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

    let row = sqlx::query(sqlx::AssertSqlSafe(query))
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

    Ok(row.try_get("id")?)
}

/// Atomically check the cooldown window and insert an IP-scoped alert event.
///
/// Uses a transaction-scoped Postgres advisory lock keyed on the table, alert
/// type, and normalized IP address together with a conditional `INSERT ... WHERE
/// NOT EXISTS`, so concurrent requests cannot insert duplicate alerts within
/// the cooldown window.
///
/// Returns `Ok(Some(id))` when the event was inserted, or `Ok(None)` when an
/// alert of the same type for the same IP was already recorded within `cooldown`.
pub async fn insert_alert_event_if_not_in_cooldown(
    pool: &PgPool,
    alert_events_table: &SecurityAlertEventsTable,
    input: &SecurityAlertEventInput,
    cooldown: Duration,
    now: DateTime<Utc>,
) -> SecurityAlertResult<Option<Uuid>> {
    let table_quoted = alert_events_table.quoted();
    let lock_prefix = format!("{table_quoted}:ip:{}", input.alert_type.as_str());
    let since = now - cooldown;

    let mut tx = pool.begin().await?;

    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1 || ':' || host($2::inet), 0))")
        .bind(&lock_prefix)
        .bind(input.ip_address.as_str())
        .execute(&mut *tx)
        .await?;

    let query = format!(
        r#"
        INSERT INTO {table_quoted} (
            alert_type,
            ip_address,
            window_started_at,
            window_ended_at,
            failed_attempts,
            distinct_user_count,
            lockout_count,
            details
        )
        SELECT $1, $2::inet, $3, $4, $5, $6, $7, $8
        WHERE NOT EXISTS (
            SELECT 1
            FROM {table_quoted}
            WHERE alert_type = $1
              AND ip_address = $2::inet
              AND created_at >= $9
        )
        RETURNING id
        "#
    );

    let row = sqlx::query(sqlx::AssertSqlSafe(query))
        .bind(input.alert_type.as_str())
        .bind(input.ip_address.as_str())
        .bind(input.window_started_at)
        .bind(input.window_ended_at)
        .bind(input.counts.failed_attempts)
        .bind(input.counts.distinct_users)
        .bind(input.counts.lockouts)
        .bind(&input.details)
        .bind(since)
        .fetch_optional(&mut *tx)
        .await?;

    let id = match row {
        Some(row) => Some(row.try_get("id")?),
        None => None,
    };

    tx.commit().await?;
    Ok(id)
}

/// Atomically check the cooldown window and insert a scoped (account/global)
/// alert event (requires the `0002__security_alert_scopes` migration).
///
/// Uses a transaction-scoped Postgres advisory lock keyed on the table, alert
/// type, and `scope_key` together with a conditional `INSERT ... WHERE NOT
/// EXISTS`, so concurrent requests cannot insert duplicate alerts within the
/// cooldown window.
///
/// Returns `Ok(Some(id))` when the event was inserted, or `Ok(None)` when an
/// alert of the same type and `scope_key` was already recorded within `cooldown`.
pub async fn insert_scoped_alert_event_if_not_in_cooldown(
    pool: &PgPool,
    alert_events_table: &SecurityAlertEventsTable,
    input: &ScopedSecurityAlertEventInput,
    cooldown: Duration,
    now: DateTime<Utc>,
) -> SecurityAlertResult<Option<Uuid>> {
    let table_quoted = alert_events_table.quoted();
    let lock_key = format!(
        "{table_quoted}:scope:{}:{}",
        input.alert_type.as_str(),
        input.scope_key
    );
    let since = now - cooldown;

    let mut tx = pool.begin().await?;

    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 0))")
        .bind(&lock_key)
        .execute(&mut *tx)
        .await?;

    let query = format!(
        r#"
        INSERT INTO {table_quoted} (
            alert_type,
            scope_key,
            ip_address,
            window_started_at,
            window_ended_at,
            failed_attempts,
            details
        )
        SELECT $1, $2, $3::inet, $4, $5, $6, $7
        WHERE NOT EXISTS (
            SELECT 1
            FROM {table_quoted}
            WHERE alert_type = $1
              AND scope_key = $2
              AND created_at >= $8
        )
        RETURNING id
        "#
    );

    let row = sqlx::query(sqlx::AssertSqlSafe(query))
        .bind(input.alert_type.as_str())
        .bind(&input.scope_key)
        .bind(input.ip_address.as_deref())
        .bind(input.window_started_at)
        .bind(input.window_ended_at)
        .bind(input.failed_attempts)
        .bind(&input.details)
        .bind(since)
        .fetch_optional(&mut *tx)
        .await?;

    let id = match row {
        Some(row) => Some(row.try_get("id")?),
        None => None,
    };

    tx.commit().await?;
    Ok(id)
}
