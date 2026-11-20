//! Postgres-backed rate limiting for multi-instance deployments.
//!
//! Fixed-window counters stored in a shared table so every app replica
//! enforces the same window. This is the documented production backend;
//! [`crate::InMemoryBackend`] is process-local and single-instance only.
//!
//! See `migrations/0001__rate_limit_counters.sql` in this crate for a
//! copy-paste-ready table definition.

use async_trait::async_trait;
use sqlx::{PgPool, Row};
use underlay_db::QualifiedTableName;

use crate::backend::RateLimitBackend;
use crate::config::{RateLimitConfig, RateLimitResult};
use crate::error::{RateLimitError, Result};

const DEFAULT_TABLE: &str = "auth.rate_limit_counters";

fn validate_table_name(table: &str) -> Result<QualifiedTableName> {
    QualifiedTableName::parse(table)
        .map_err(|err| RateLimitError::Config(format!("invalid rate limit table name `{table}`: {err}")))
}

/// Postgres-backed fixed-window rate limiter.
///
/// All operations are single atomic statements (`INSERT ... ON CONFLICT`),
/// so concurrent replicas cannot race a window reset.
#[derive(Debug, Clone)]
pub struct PostgresBackend {
    pool: PgPool,
    table: QualifiedTableName,
}

impl PostgresBackend {
    /// Create a backend using the default `auth.rate_limit_counters` table.
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            table: QualifiedTableName::parse(DEFAULT_TABLE)
                .expect("default rate limit table name is a valid SQL identifier"),
        }
    }

    /// Create a backend with a custom (schema-qualified) table name.
    ///
    /// The name is validated and quoted via [`QualifiedTableName`].
    pub fn with_table(pool: PgPool, table: impl Into<String>) -> Result<Self> {
        let table = validate_table_name(&table.into())?;
        Ok(Self { pool, table })
    }

    fn map_err(err: sqlx::Error) -> RateLimitError {
        RateLimitError::Backend(err.to_string())
    }

    fn to_result(config: &RateLimitConfig, count: u64, elapsed_secs: f64) -> RateLimitResult {
        if count <= config.max_requests() {
            RateLimitResult::allowed(config.max_requests() - count, count)
        } else {
            let window_secs = config.window().as_secs_f64();
            let remaining = (window_secs - elapsed_secs).max(0.0).ceil() as u64;
            RateLimitResult::denied(count, std::time::Duration::from_secs(remaining.max(1)))
        }
    }
}

#[async_trait]
impl RateLimitBackend for PostgresBackend {
    async fn check(&self, key: &str, config: &RateLimitConfig) -> Result<RateLimitResult> {
        let query = format!(
            r#"
            SELECT count, EXTRACT(EPOCH FROM (NOW() - window_started_at))::float8 AS elapsed
            FROM {}
            WHERE key = $1 AND window_started_at > NOW() - make_interval(secs => $2)
            "#,
            self.table.quoted()
        );

        let row = sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(key)
            .bind(config.window().as_secs_f64())
            .fetch_optional(&self.pool)
            .await
            .map_err(Self::map_err)?;

        match row {
            None => Ok(RateLimitResult::allowed(config.max_requests(), 0)),
            Some(row) => {
                let count = row
                    .try_get::<i64, _>("count")
                    .map_err(Self::map_err)?
                    .max(0) as u64;
                let elapsed: f64 = row.try_get::<f64, _>("elapsed").unwrap_or(0.0);
                // check() is non-incrementing: a count already at the limit
                // means the next request would exceed it.
                if count < config.max_requests() {
                    Ok(RateLimitResult::allowed(
                        config.max_requests() - count,
                        count,
                    ))
                } else {
                    let window_secs = config.window().as_secs_f64();
                    let remaining = (window_secs - elapsed).max(0.0).ceil() as u64;
                    Ok(RateLimitResult::denied(
                        count,
                        std::time::Duration::from_secs(remaining.max(1)),
                    ))
                }
            }
        }
    }

    async fn increment(&self, key: &str, config: &RateLimitConfig) -> Result<u64> {
        let table = self.table.quoted();
        let query = format!(
            r#"
            INSERT INTO {table} (key, window_started_at, count)
            VALUES ($1, NOW(), 1)
            ON CONFLICT (key) DO UPDATE SET
                count = CASE
                    WHEN {table}.window_started_at <= NOW() - make_interval(secs => $2)
                    THEN 1
                    ELSE {table}.count + 1
                END,
                window_started_at = CASE
                    WHEN {table}.window_started_at <= NOW() - make_interval(secs => $2)
                    THEN NOW()
                    ELSE {table}.window_started_at
                END
            RETURNING count
            "#
        );

        let row = sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(key)
            .bind(config.window().as_secs_f64())
            .fetch_one(&self.pool)
            .await
            .map_err(Self::map_err)?;

        Ok(row
            .try_get::<i64, _>("count")
            .map_err(Self::map_err)?
            .max(0) as u64)
    }

    async fn reset(&self, key: &str) -> Result<()> {
        let query = format!("DELETE FROM {} WHERE key = $1", self.table.quoted());
        sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(key)
            .execute(&self.pool)
            .await
            .map_err(Self::map_err)?;
        Ok(())
    }

    async fn check_and_increment(
        &self,
        key: &str,
        config: &RateLimitConfig,
    ) -> Result<RateLimitResult> {
        let table = self.table.quoted();
        // Single atomic upsert: no check-then-set race across replicas.
        let query = format!(
            r#"
            INSERT INTO {table} (key, window_started_at, count)
            VALUES ($1, NOW(), 1)
            ON CONFLICT (key) DO UPDATE SET
                count = CASE
                    WHEN {table}.window_started_at <= NOW() - make_interval(secs => $2)
                    THEN 1
                    ELSE {table}.count + 1
                END,
                window_started_at = CASE
                    WHEN {table}.window_started_at <= NOW() - make_interval(secs => $2)
                    THEN NOW()
                    ELSE {table}.window_started_at
                END
            RETURNING count,
                EXTRACT(EPOCH FROM (NOW() - window_started_at))::float8 AS elapsed
            "#
        );

        let row = sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(key)
            .bind(config.window().as_secs_f64())
            .fetch_one(&self.pool)
            .await
            .map_err(Self::map_err)?;

        let count = row
            .try_get::<i64, _>("count")
            .map_err(Self::map_err)?
            .max(0) as u64;
        let elapsed: f64 = row.try_get::<f64, _>("elapsed").unwrap_or(0.0);

        Ok(Self::to_result(config, count, elapsed))
    }
}

#[cfg(test)]
#[path = "tests/postgres_tests.rs"]
mod tests;
