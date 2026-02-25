use crate::DbPool;

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
///
/// We keep this deliberately strict to avoid accidental SQL injection when
/// building `DROP SCHEMA` statements.
pub fn validate_schema_name(schema: &str) -> bool {
    let schema = schema.trim();
    if schema.is_empty() {
        return false;
    }

    let mut chars = schema.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    if !(first.is_ascii_lowercase() || first == '_') {
        return false;
    }

    for ch in chars {
        if !(ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_') {
            return false;
        }
    }

    true
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

        if !validate_schema_name(schema) {
            return Err(sqlx::Error::Protocol(format!(
                "invalid schema name: {schema}"
            )));
        }

        let sql = format!("DROP SCHEMA IF EXISTS {schema} CASCADE");
        sqlx::query(&sql).execute(pool).await?;
    }

    Ok(())
}

#[cfg(test)]
#[path = "tests/schemas_tests.rs"]
mod tests;
