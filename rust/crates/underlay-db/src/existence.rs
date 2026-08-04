//! Helpers for checking value existence in database tables.
//!
//! These helpers provide generic existence checks for common patterns like slug
//! uniqueness validation, scoped uniqueness, and update validation.
//!
//! # Security Note
//!
//! Existence helpers accept `QualifiedTableName` and `SqlIdentifier` so
//! identifier validation happens before SQL construction.
//!
//! # Examples
//!
//! ## Simple existence check
//!
//! ```rust,ignore
//! use underlay_db::{
//!     value_exists_typed, value_exists_excluding_typed, QualifiedTableName, SqlIdentifier,
//! };
//!
//! let table = QualifiedTableName::from_schema_table("learning", "pathway")?;
//! let slug = SqlIdentifier::parse("slug")?;
//!
//! // Check if a slug exists
//! let exists = value_exists_typed(&pool, &table, &slug, "my-slug").await?;
//!
//! // Check for update (exclude current record)
//! let exists =
//!     value_exists_excluding_typed(&pool, &table, &slug, "my-slug", existing_id).await?;
//! ```
//!
//! ## Builder pattern for composite constraints
//!
//! ```rust,ignore
//! use underlay_db::TypedExistsCheck;
//!
//! // Pathway: slug + year composite uniqueness
//! let exists = TypedExistsCheck::from_schema_table("learning", "pathway")?
//!     .value("slug", slug)?
//!     .nullable_value("year", year)?
//!     .active_only()
//!     .check(&pool)
//!     .await?;
//!
//! // Module: slug + pathway_id + start_year (for updates)
//! let exists = TypedExistsCheck::from_schema_table("learning", "module")?
//!     .value("slug", slug)?
//!     .scope("pathway_id", pathway_id)?
//!     .value_i32("start_year", start_year)?
//!     .excluding(current_id)
//!     .active_only()
//!     .check(&pool)
//!     .await?;
//! ```

use sqlx::PgPool;
use uuid::Uuid;

use crate::{IdentifierError, QualifiedTableName, SqlIdentifier};

// =============================================================================
// TypedExistsCheck Builder
// =============================================================================

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
enum TypedCondition {
    /// column = $N (string)
    StringEquals {
        column: SqlIdentifier,
        value: String,
    },
    /// column = $N (i32)
    IntEquals { column: SqlIdentifier, value: i32 },
    /// column = $N (Uuid)
    UuidEquals { column: SqlIdentifier, value: Uuid },
    /// column IS NOT DISTINCT FROM $N (Option<i32>)
    NullableIntEquals {
        column: SqlIdentifier,
        value: Option<i32>,
    },
}

/// Typed builder for flexible existence checks with composite constraints.
///
/// This is the preferred builder for new code. It validates table and column
/// identifiers at construction time, then uses quoted identifiers and bound
/// values when building SQL.
#[derive(Debug, Clone)]
pub struct TypedExistsCheck {
    table: QualifiedTableName,
    conditions: Vec<TypedCondition>,
    exclude_id: Option<Uuid>,
    active_only: bool,
}

impl TypedExistsCheck {
    /// Create a new existence check for a validated table.
    pub fn new(table: QualifiedTableName) -> Self {
        Self {
            table,
            conditions: Vec::new(),
            exclude_id: None,
            active_only: false,
        }
    }

    /// Parse schema and table names into a typed existence check.
    pub fn from_schema_table(schema: &str, table: &str) -> Result<Self, IdentifierError> {
        Ok(Self::new(QualifiedTableName::from_schema_table(
            schema, table,
        )?))
    }

    /// Parse a qualified table name into a typed existence check.
    pub fn parse_table(table: &str) -> Result<Self, IdentifierError> {
        Ok(Self::new(QualifiedTableName::parse(table)?))
    }

    /// Add a string equality condition.
    pub fn value(
        mut self,
        column: impl AsRef<str>,
        value: impl Into<String>,
    ) -> Result<Self, IdentifierError> {
        self.conditions.push(TypedCondition::StringEquals {
            column: SqlIdentifier::parse(column)?,
            value: value.into(),
        });
        Ok(self)
    }

    /// Add an integer equality condition.
    pub fn value_i32(
        mut self,
        column: impl AsRef<str>,
        value: i32,
    ) -> Result<Self, IdentifierError> {
        self.conditions.push(TypedCondition::IntEquals {
            column: SqlIdentifier::parse(column)?,
            value,
        });
        Ok(self)
    }

    /// Add a UUID equality condition.
    pub fn scope(mut self, column: impl AsRef<str>, value: Uuid) -> Result<Self, IdentifierError> {
        self.conditions.push(TypedCondition::UuidEquals {
            column: SqlIdentifier::parse(column)?,
            value,
        });
        Ok(self)
    }

    /// Add a nullable integer condition using IS NOT DISTINCT FROM.
    pub fn nullable_value(
        mut self,
        column: impl AsRef<str>,
        value: Option<i32>,
    ) -> Result<Self, IdentifierError> {
        self.conditions.push(TypedCondition::NullableIntEquals {
            column: SqlIdentifier::parse(column)?,
            value,
        });
        Ok(self)
    }

    /// Exclude a specific record ID from the check.
    pub fn excluding(mut self, id: Uuid) -> Self {
        self.exclude_id = Some(id);
        self
    }

    /// Restrict the check to rows that follow Underlay's `deleted_at IS NULL`
    /// active-record convention.
    pub fn active_only(mut self) -> Self {
        self.active_only = true;
        self
    }

    fn query(&self) -> String {
        typed_exists_query(
            &self.table,
            &self.conditions,
            self.exclude_id.is_some(),
            self.active_only,
        )
    }

    /// Execute the existence check.
    ///
    /// Returns `true` if a matching record exists.
    pub async fn check(self, pool: &PgPool) -> Result<bool, sqlx::Error> {
        if self.conditions.is_empty() {
            return Ok(false);
        }

        let query = self.query();
        let mut sqlx_query = sqlx::query_scalar::<_, bool>(sqlx::AssertSqlSafe(query));

        for condition in &self.conditions {
            sqlx_query = match condition {
                TypedCondition::StringEquals { value, .. } => sqlx_query.bind(value.as_str()),
                TypedCondition::IntEquals { value, .. } => sqlx_query.bind(*value),
                TypedCondition::UuidEquals { value, .. } => sqlx_query.bind(*value),
                TypedCondition::NullableIntEquals { value, .. } => sqlx_query.bind(*value),
            };
        }

        if let Some(exclude_id) = self.exclude_id {
            sqlx_query = sqlx_query.bind(exclude_id);
        }

        sqlx_query.fetch_one(pool).await
    }
}

fn typed_exists_query(
    table: &QualifiedTableName,
    conditions: &[TypedCondition],
    excluding: bool,
    active_only: bool,
) -> String {
    let mut param_idx = 1u32;
    let mut where_parts = Vec::new();

    for condition in conditions {
        let clause = match condition {
            TypedCondition::StringEquals { column, .. }
            | TypedCondition::IntEquals { column, .. }
            | TypedCondition::UuidEquals { column, .. } => {
                let clause = format!("{} = ${}", column.quoted(), param_idx);
                param_idx += 1;
                clause
            }
            TypedCondition::NullableIntEquals { column, .. } => {
                let clause = format!("{} IS NOT DISTINCT FROM ${}", column.quoted(), param_idx);
                param_idx += 1;
                clause
            }
        };
        where_parts.push(clause);
    }

    if excluding {
        where_parts.push(format!("id <> ${}", param_idx));
    }

    if active_only {
        where_parts.push("deleted_at IS NULL".to_string());
    }

    let where_clause = where_parts.join(" AND ");
    format!(
        "SELECT EXISTS(SELECT 1 FROM {} WHERE {})",
        table.quoted(),
        where_clause
    )
}

// =============================================================================
// Typed Helper Functions
// =============================================================================

fn value_exists_query(
    table: &QualifiedTableName,
    column: &SqlIdentifier,
    excluding: bool,
) -> String {
    let exclude_clause = if excluding { " AND id <> $2" } else { "" };
    format!(
        "SELECT EXISTS(SELECT 1 FROM {} WHERE {} = $1{})",
        table.quoted(),
        column.quoted(),
        exclude_clause
    )
}

/// Check if a string value exists in a typed table and column.
pub async fn value_exists_typed(
    pool: &PgPool,
    table: &QualifiedTableName,
    column: &SqlIdentifier,
    value: &str,
) -> Result<bool, sqlx::Error> {
    let query = value_exists_query(table, column, false);
    sqlx::query_scalar::<_, bool>(sqlx::AssertSqlSafe(query))
        .bind(value)
        .fetch_one(pool)
        .await
}

/// Check if a string value exists in a typed table and column, excluding one ID.
pub async fn value_exists_excluding_typed(
    pool: &PgPool,
    table: &QualifiedTableName,
    column: &SqlIdentifier,
    value: &str,
    exclude_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let query = value_exists_query(table, column, true);
    sqlx::query_scalar::<_, bool>(sqlx::AssertSqlSafe(query))
        .bind(value)
        .bind(exclude_id)
        .fetch_one(pool)
        .await
}

#[cfg(test)]
#[path = "tests/existence_tests.rs"]
mod tests;
