use std::borrow::Borrow;

use crate::{DbPool, IdentifierError, SqlIdentifier};

#[derive(Debug, Clone, Copy)]
pub struct DestructiveGuard {
    allowed: bool,
}

impl DestructiveGuard {
    pub fn disallow() -> Self {
        Self { allowed: false }
    }

    pub fn allow() -> Self {
        Self { allowed: true }
    }

    pub fn is_allowed(&self) -> bool {
        self.allowed
    }
}

/// Validate a Postgres schema identifier.
pub fn validate_schema_name(schema: &str) -> bool {
    SqlIdentifier::parse(schema).is_ok()
}

/// Parse a Postgres schema identifier into the shared typed SQL identifier.
pub fn parse_schema_name(schema: &str) -> Result<SqlIdentifier, IdentifierError> {
    SqlIdentifier::parse(schema)
}

/// Drop typed schema identifiers (CASCADE) in a safe, guarded way.
///
/// This helper is intended for local/dev reset tooling.
pub async fn drop_schema_identifiers<S, I>(
    pool: &DbPool,
    guard: DestructiveGuard,
    schemas: I,
) -> Result<(), sqlx::Error>
where
    I: IntoIterator<Item = S>,
    S: Borrow<SqlIdentifier>,
{
    if !guard.is_allowed() {
        return Err(sqlx::Error::Protocol(
            "destructive operations are not allowed".into(),
        ));
    }

    for schema in schemas {
        drop_schema_identifier(pool, schema.borrow()).await?;
    }

    Ok(())
}

async fn drop_schema_identifier(pool: &DbPool, schema: &SqlIdentifier) -> Result<(), sqlx::Error> {
    let sql = format!("DROP SCHEMA IF EXISTS {} CASCADE", schema.quoted());
    sqlx::query(sqlx::AssertSqlSafe(sql)).execute(pool).await?;
    Ok(())
}

/// Drop schemas (CASCADE) in a safe, guarded way.
///
/// This helper is intended for local/dev reset tooling.
pub async fn drop_schemas<S, I>(
    pool: &DbPool,
    guard: DestructiveGuard,
    schemas: I,
) -> Result<(), sqlx::Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    if !guard.is_allowed() {
        return Err(sqlx::Error::Protocol(
            "destructive operations are not allowed".into(),
        ));
    }

    for schema in schemas {
        let schema = schema.as_ref();
        let schema = parse_schema_name(schema)
            .map_err(|err| sqlx::Error::Protocol(format!("invalid schema name: {err}")))?;
        drop_schema_identifier(pool, &schema).await?;
    }

    Ok(())
}

#[cfg(test)]
#[path = "tests/schemas_tests.rs"]
mod tests;
