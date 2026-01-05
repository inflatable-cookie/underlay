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
pub async fn drop_schemas(
    pool: &DbPool,
    guard: DestructiveGuard,
    schemas: &[&str],
) -> Result<(), sqlx::Error> {
    if !guard.is_allowed() {
        return Err(sqlx::Error::Protocol(
            "destructive operations are not allowed".into(),
        ));
    }

    for schema in schemas {
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
mod tests {
    use super::validate_schema_name;

    #[test]
    fn schema_validation_allows_simple_names() {
        assert!(validate_schema_name("learning"));
        assert!(validate_schema_name("infra"));
        assert!(validate_schema_name("content_library"));
        assert!(validate_schema_name("_private"));
    }

    #[test]
    fn schema_validation_rejects_mixed_case_and_symbols() {
        assert!(!validate_schema_name("Learning"));
        assert!(!validate_schema_name("learning; drop schema public"));
        assert!(!validate_schema_name("learning-content"));
        assert!(!validate_schema_name("learning.content"));
        assert!(!validate_schema_name(""));
        assert!(!validate_schema_name(" "));
    }
}
