use chrono::{Duration, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::Row;

use underlay_core::Uuid;

const DEFAULT_AUTH_STATE_TABLE: &str = "auth.auth_state";

/// Validate a schema-qualified table name before it is interpolated into SQL.
/// Allows only `[A-Za-z0-9_.]+`.
fn validate_table_name(table: &str) -> Result<(), AuthStateError> {
    let valid = !table.is_empty()
        && table
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.');
    if valid {
        Ok(())
    } else {
        Err(AuthStateError::InvalidTableName(table.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct AuthStateStore {
    pool: sqlx::PgPool,
    table: String,
}

impl AuthStateStore {
    /// Create a store using the default `auth.auth_state` table.
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool,
            table: DEFAULT_AUTH_STATE_TABLE.to_string(),
        }
    }

    /// Create a store with a custom (schema-qualified) table name, for
    /// consumers whose auth-state table does not live in the `auth` schema.
    ///
    /// The name is validated to `[A-Za-z0-9_.]+` because it is interpolated
    /// into SQL.
    pub fn with_table(pool: sqlx::PgPool, table: impl Into<String>) -> Result<Self, AuthStateError> {
        let table = table.into();
        validate_table_name(&table)?;
        Ok(Self { pool, table })
    }

    /// Create a short-lived auth state row.
    ///
    /// This stores state in `auth.auth_state` and returns the generated UUID.
    pub async fn create(
        &self,
        user_id: Option<Uuid>,
        state_type: &str,
        state: serde_json::Value,
        ttl: Duration,
    ) -> Result<Uuid, AuthStateError> {
        let id = Uuid::new_v7();
        let now = Utc::now();
        let expires_at = now + ttl;

        sqlx::query(&format!(
            r#"
            INSERT INTO {} (id, user_id, state_type, state, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            self.table
        ))
        .bind(id.into_inner())
        .bind(user_id.map(|u| u.into_inner()))
        .bind(state_type)
        .bind(state)
        .bind(now)
        .bind(expires_at)
        .execute(&self.pool)
        .await
        .map_err(AuthStateError::Db)?;

        Ok(id)
    }

    pub async fn create_user<T: Serialize>(
        &self,
        user_id: Uuid,
        state_type: &str,
        state: T,
        ttl: Duration,
    ) -> Result<Uuid, AuthStateError> {
        let state = serde_json::to_value(state).map_err(AuthStateError::Encode)?;
        self.create(Some(user_id), state_type, state, ttl).await
    }

    pub async fn create_public<T: Serialize>(
        &self,
        state_type: &str,
        state: T,
        ttl: Duration,
    ) -> Result<Uuid, AuthStateError> {
        let state = serde_json::to_value(state).map_err(AuthStateError::Encode)?;
        self.create(None, state_type, state, ttl).await
    }

    pub async fn load(
        &self,
        state_id: Uuid,
        user_id: Option<Uuid>,
        state_type: &str,
    ) -> Result<Option<serde_json::Value>, AuthStateError> {
        let now = Utc::now();

        let row = sqlx::query(&format!(
            r#"
            SELECT state
            FROM {}
            WHERE id = $1 AND user_id IS NOT DISTINCT FROM $2 AND state_type = $3 AND expires_at > $4
            "#,
            self.table
        ))
        .bind(state_id.into_inner())
        .bind(user_id.map(|u| u.into_inner()))
        .bind(state_type)
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(AuthStateError::Db)?;

        Ok(row.map(|r| r.get::<serde_json::Value, _>("state")))
    }

    pub async fn load_user(
        &self,
        state_id: Uuid,
        user_id: Uuid,
        state_type: &str,
    ) -> Result<Option<serde_json::Value>, AuthStateError> {
        self.load(state_id, Some(user_id), state_type).await
    }

    pub async fn load_public(
        &self,
        state_id: Uuid,
        state_type: &str,
    ) -> Result<Option<serde_json::Value>, AuthStateError> {
        self.load(state_id, None, state_type).await
    }

    pub async fn update_public(
        &self,
        state_id: Uuid,
        state_type: &str,
        state: serde_json::Value,
    ) -> Result<(), AuthStateError> {
        let now = Utc::now();

        let result = sqlx::query(&format!(
            r#"
            UPDATE {}
            SET state = $3
            WHERE id = $1 AND user_id IS NULL AND state_type = $2 AND expires_at > $4
            "#,
            self.table
        ))
        .bind(state_id.into_inner())
        .bind(state_type)
        .bind(state)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(AuthStateError::Db)?;

        if result.rows_affected() == 0 {
            return Err(AuthStateError::InvalidOrExpired);
        }

        Ok(())
    }

    pub async fn delete(&self, state_id: Uuid) -> Result<(), AuthStateError> {
        sqlx::query(&format!(
            r#"
            DELETE FROM {}
            WHERE id = $1
            "#,
            self.table
        ))
        .bind(state_id.into_inner())
        .execute(&self.pool)
        .await
        .map_err(AuthStateError::Db)?;

        Ok(())
    }

    pub async fn consume(
        &self,
        state_id: Uuid,
        user_id: Option<Uuid>,
        state_type: &str,
    ) -> Result<Option<serde_json::Value>, AuthStateError> {
        let state = self.load(state_id, user_id, state_type).await?;
        if state.is_none() {
            return Ok(None);
        }

        self.delete(state_id).await?;
        Ok(state)
    }

    pub async fn consume_public(
        &self,
        state_id: Uuid,
        state_type: &str,
    ) -> Result<Option<serde_json::Value>, AuthStateError> {
        self.consume(state_id, None, state_type).await
    }

    pub async fn consume_user(
        &self,
        state_id: Uuid,
        user_id: Uuid,
        state_type: &str,
    ) -> Result<Option<serde_json::Value>, AuthStateError> {
        self.consume(state_id, Some(user_id), state_type).await
    }

    pub async fn consume_public_typed<T: DeserializeOwned>(
        &self,
        state_id: Uuid,
        state_type: &str,
    ) -> Result<Option<T>, AuthStateError> {
        let Some(value) = self.consume_public(state_id, state_type).await? else {
            return Ok(None);
        };

        let parsed = serde_json::from_value(value).map_err(AuthStateError::Decode)?;
        Ok(Some(parsed))
    }

    pub async fn load_public_typed<T: DeserializeOwned>(
        &self,
        state_id: Uuid,
        state_type: &str,
    ) -> Result<Option<T>, AuthStateError> {
        let Some(value) = self.load_public(state_id, state_type).await? else {
            return Ok(None);
        };

        let parsed = serde_json::from_value(value).map_err(AuthStateError::Decode)?;
        Ok(Some(parsed))
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AuthStateError {
    #[error("invalid or expired auth state")]
    InvalidOrExpired,
    #[error("failed to encode auth state")]
    Encode(#[source] serde_json::Error),
    #[error("failed to decode auth state")]
    Decode(#[source] serde_json::Error),
    #[error("database error")]
    Db(#[source] sqlx::Error),
    #[error("invalid auth-state table name: {0}")]
    InvalidTableName(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthStateRow {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub state_type: String,
    pub state: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_name_validation_accepts_safe_and_rejects_unsafe() {
        assert!(validate_table_name("auth.auth_state").is_ok());
        assert!(validate_table_name("accounts.auth_state").is_ok());
        for bad in ["auth state", "auth_state; DROP TABLE x", "", "auth_state--", "s'"] {
            assert!(validate_table_name(bad).is_err(), "expected {bad:?} rejected");
        }
    }
}
