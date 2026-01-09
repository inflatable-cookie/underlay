use chrono::{Duration, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::Row;

use underlay_core::Uuid;

#[derive(Debug, Clone)]
pub struct AuthStateStore {
    pool: sqlx::PgPool,
}

impl AuthStateStore {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    /// Create a short-lived auth state row.
    ///
    /// This stores state in `accounts.auth_state` and returns the generated UUID.
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

        sqlx::query(
            r#"
            INSERT INTO accounts.auth_state (id, user_id, state_type, state, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
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

        let row = sqlx::query(
            r#"
            SELECT state
            FROM accounts.auth_state
            WHERE id = $1 AND user_id IS NOT DISTINCT FROM $2 AND state_type = $3 AND expires_at > $4
            "#,
        )
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

        let result = sqlx::query(
            r#"
            UPDATE accounts.auth_state
            SET state = $3
            WHERE id = $1 AND user_id IS NULL AND state_type = $2 AND expires_at > $4
            "#,
        )
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
        sqlx::query(
            r#"
            DELETE FROM accounts.auth_state
            WHERE id = $1
            "#,
        )
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
pub enum AuthStateError {
    #[error("invalid or expired auth state")]
    InvalidOrExpired,
    #[error("failed to encode auth state")]
    Encode(#[source] serde_json::Error),
    #[error("failed to decode auth state")]
    Decode(#[source] serde_json::Error),
    #[error("database error")]
    Db(#[source] sqlx::Error),
}

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

pub async fn run_migrations(pool: &sqlx::PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    MIGRATOR.run(pool).await
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthStateRow {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub state_type: String,
    pub state: serde_json::Value,
}
