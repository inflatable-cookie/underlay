use chrono::{Duration, Utc};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::Row;

use underlay_core::{RawUuid, Uuid};
use underlay_db::QualifiedTableName;

const DEFAULT_AUTH_STATE_TABLE: &str = "auth.auth_state";

fn validate_table_name(table: &str) -> Result<QualifiedTableName, AuthStateError> {
    QualifiedTableName::parse(table)
        .map_err(|_| AuthStateError::InvalidTableName(table.to_string()))
}

/// Generate a 122-bit random RFC 4122 UUIDv4 handle without leaking a timestamp.
///
/// The workspace `uuid` dependency enables `v7` (CSPRNG-backed `getrandom`)
/// without enabling `v4` in standalone production builds. Each `Uuid::new_v7()`
/// carries 7 unmasked `rand_b` CSPRNG bytes in `bytes[9..16]`; combining them
/// produces 16 independent random bytes with zero timestamp or counter bits.
fn new_state_handle() -> Uuid {
    let a = Uuid::new_v7().into_inner().into_bytes();
    let b = Uuid::new_v7().into_inner().into_bytes();
    let c = Uuid::new_v7().into_inner().into_bytes();
    let mut bytes = [0u8; 16];
    bytes[0..7].copy_from_slice(&a[9..16]);
    bytes[7..14].copy_from_slice(&b[9..16]);
    bytes[14..16].copy_from_slice(&c[9..11]);
    // Set RFC 4122 version 4 (0100xxxx) and variant (10xxxxxx).
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from(RawUuid::from_bytes(bytes))
}

#[derive(Debug, Clone)]
pub struct AuthStateStore {
    pool: sqlx::PgPool,
    table: QualifiedTableName,
}

impl AuthStateStore {
    /// Create a store using the default `auth.auth_state` table.
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool,
            table: QualifiedTableName::parse(DEFAULT_AUTH_STATE_TABLE)
                .expect("default auth state table name is a valid SQL identifier"),
        }
    }

    /// Create a store with a custom (schema-qualified) table name, for
    /// consumers whose auth-state table does not live in the `auth` schema.
    ///
    /// The name is validated and quoted via [`QualifiedTableName`].
    pub fn with_table(
        pool: sqlx::PgPool,
        table: impl Into<String>,
    ) -> Result<Self, AuthStateError> {
        let table = validate_table_name(&table.into())?;
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
        let id = new_state_handle();
        let now = Utc::now();
        let expires_at = now + ttl;

        sqlx::query(sqlx::AssertSqlSafe(format!(
            r#"
            INSERT INTO {} (id, user_id, state_type, state, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            self.table.quoted()
        )))
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

        let row = sqlx::query(sqlx::AssertSqlSafe(format!(
            r#"
            SELECT state
            FROM {}
            WHERE id = $1 AND user_id IS NOT DISTINCT FROM $2 AND state_type = $3 AND expires_at > $4
            "#,
            self.table.quoted()
        )))
        .bind(state_id.into_inner())
        .bind(user_id.map(|u| u.into_inner()))
        .bind(state_type)
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(AuthStateError::Db)?;

        row.map(|r| {
            r.try_get::<serde_json::Value, _>("state")
                .map_err(AuthStateError::Db)
        })
        .transpose()
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

        let result = sqlx::query(sqlx::AssertSqlSafe(format!(
            r#"
            UPDATE {}
            SET state = $3
            WHERE id = $1 AND user_id IS NULL AND state_type = $2 AND expires_at > $4
            "#,
            self.table.quoted()
        )))
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
        sqlx::query(sqlx::AssertSqlSafe(format!(
            r#"
            DELETE FROM {}
            WHERE id = $1
            "#,
            self.table.quoted()
        )))
        .bind(state_id.into_inner())
        .execute(&self.pool)
        .await
        .map_err(AuthStateError::Db)?;

        Ok(())
    }

    /// Delete expired rows from this store's table and return the number removed.
    ///
    /// # Errors
    ///
    /// Returns a database error if the deletion fails.
    pub async fn purge_expired(&self) -> Result<u64, AuthStateError> {
        let result = sqlx::query(sqlx::AssertSqlSafe(format!(
            "DELETE FROM {} WHERE expires_at <= $1",
            self.table.quoted()
        )))
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(AuthStateError::Db)?;

        Ok(result.rows_affected())
    }

    pub async fn consume(
        &self,
        state_id: Uuid,
        user_id: Option<Uuid>,
        state_type: &str,
    ) -> Result<Option<serde_json::Value>, AuthStateError> {
        let now = Utc::now();

        let row = sqlx::query(sqlx::AssertSqlSafe(format!(
            r#"
            DELETE FROM {}
            WHERE id = $1 AND user_id IS NOT DISTINCT FROM $2 AND state_type = $3 AND expires_at > $4
            RETURNING state
            "#,
            self.table.quoted()
        )))
        .bind(state_id.into_inner())
        .bind(user_id.map(|u| u.into_inner()))
        .bind(state_type)
        .bind(now)
        .fetch_optional(&self.pool)
        .await
        .map_err(AuthStateError::Db)?;

        row.map(|r| {
            r.try_get::<serde_json::Value, _>("state")
                .map_err(AuthStateError::Db)
        })
        .transpose()
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

#[cfg(test)]
#[path = "tests/integration.rs"]
mod integration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_name_validation_accepts_safe_and_rejects_unsafe() {
        assert!(validate_table_name("auth.auth_state").is_ok());
        assert!(validate_table_name("accounts.auth_state").is_ok());
        for bad in [
            "auth state",
            "auth_state; DROP TABLE x",
            "",
            "auth_state--",
            "s'",
        ] {
            assert!(
                validate_table_name(bad).is_err(),
                "expected {bad:?} rejected"
            );
        }
    }

    #[test]
    fn state_handle_is_random_uuid_v4() {
        let first = new_state_handle();
        let second = new_state_handle();
        assert_ne!(first, second);
        for id in [first, second] {
            let raw = id.into_inner();
            assert_eq!(raw.get_version_num(), 4);
            let bytes = raw.as_bytes();
            assert_eq!(bytes[6] >> 4, 0x4);
            assert_eq!(bytes[8] >> 6, 0b10);
        }
    }
}
