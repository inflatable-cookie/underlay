//! Helpers for checking value existence in database tables.
//!
//! These functions provide generic existence checks for common patterns like
//! slug uniqueness validation, scoped uniqueness (e.g., label unique within parent),
//! and update validation (unique excluding current record).
//!
//! # Security Note
//!
//! The `schema`, `table`, and column parameters are interpolated into SQL queries.
//! They should only accept known-good values from your application code, never
//! from user input. Consider using constants or enums for these values.
//!
//! # Examples
//!
//! ## Simple existence check
//!
//! ```rust,ignore
//! use underlay_db::{value_exists, value_exists_excluding};
//!
//! // Check if a slug exists
//! let exists = value_exists(&pool, "learning", "pathway", "slug", "my-slug").await?;
//!
//! // Check for update (exclude current record)
//! let exists = value_exists_excluding(
//!     &pool, "learning", "pathway", "slug", "my-slug", existing_id
//! ).await?;
//! ```
//!
//! ## Builder pattern for composite constraints
//!
//! ```rust,ignore
//! use underlay_db::ExistsCheck;
//!
//! // Pathway: slug + year composite uniqueness
//! let exists = ExistsCheck::new("learning", "pathway")
//!     .value("slug", slug)
//!     .nullable_value("year", year)  // uses IS NOT DISTINCT FROM
//!     .check(&pool)
//!     .await?;
//!
//! // Module: slug + pathway_id + start_year (for updates)
//! let exists = ExistsCheck::new("learning", "module")
//!     .value("slug", slug)
//!     .scope("pathway_id", pathway_id)
//!     .value_i32("start_year", start_year)
//!     .excluding(current_id)
//!     .check(&pool)
//!     .await?;
//! ```

use sqlx::PgPool;
use uuid::Uuid;

// =============================================================================
// ExistsCheck Builder
// =============================================================================

/// Builder for flexible existence checks with composite constraints.
///
/// Supports multiple conditions, nullable comparisons (IS NOT DISTINCT FROM),
/// and excluding specific records for update validation.
///
/// # Example
///
/// ```rust,ignore
/// // Simple: slug uniqueness
/// let exists = ExistsCheck::new("content", "summary_item")
///     .value("slug", "my-slug")
///     .check(&pool)
///     .await?;
///
/// // Composite: slug + year (with nullable year)
/// let exists = ExistsCheck::new("learning", "pathway")
///     .value("slug", "intro-pathway")
///     .nullable_value("year", Some(2024))
///     .excluding(pathway_id)
///     .check(&pool)
///     .await?;
///
/// // Multi-scope: slug + pathway_id + start_year
/// let exists = ExistsCheck::new("learning", "module")
///     .value("slug", "module-1")
///     .scope("pathway_id", pathway_id)
///     .value_i32("start_year", 2024)
///     .check(&pool)
///     .await?;
/// ```
#[derive(Debug, Clone)]
pub struct ExistsCheck<'a> {
    schema: &'a str,
    table: &'a str,
    conditions: Vec<Condition<'a>>,
    exclude_id: Option<Uuid>,
    include_deleted: bool,
}

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
enum Condition<'a> {
    /// column = $N (string)
    StringEquals { column: &'a str, value: String },
    /// column = $N (i32)
    IntEquals { column: &'a str, value: i32 },
    /// column = $N (Uuid)
    UuidEquals { column: &'a str, value: Uuid },
    /// column IS NOT DISTINCT FROM $N (Option<i32>)
    NullableIntEquals { column: &'a str, value: Option<i32> },
}

impl<'a> ExistsCheck<'a> {
    /// Create a new existence check for a table.
    ///
    /// # Arguments
    /// * `schema` - Schema name (e.g., "learning", "content")
    /// * `table` - Table name (e.g., "pathway", "module")
    pub fn new(schema: &'a str, table: &'a str) -> Self {
        Self {
            schema,
            table,
            conditions: Vec::new(),
            exclude_id: None,
            include_deleted: false,
        }
    }

    /// Add a string equality condition.
    ///
    /// Generates: `column = $N`
    pub fn value(mut self, column: &'a str, value: impl Into<String>) -> Self {
        self.conditions.push(Condition::StringEquals {
            column,
            value: value.into(),
        });
        self
    }

    /// Add an integer equality condition.
    ///
    /// Generates: `column = $N`
    pub fn value_i32(mut self, column: &'a str, value: i32) -> Self {
        self.conditions.push(Condition::IntEquals { column, value });
        self
    }

    /// Add a UUID equality condition (alias for `scope`).
    ///
    /// Generates: `column = $N`
    pub fn scope(mut self, column: &'a str, value: Uuid) -> Self {
        self.conditions
            .push(Condition::UuidEquals { column, value });
        self
    }

    /// Add a nullable integer condition using IS NOT DISTINCT FROM.
    ///
    /// Generates: `column IS NOT DISTINCT FROM $N`
    ///
    /// This handles NULL comparisons correctly - two NULLs are considered equal.
    pub fn nullable_value(mut self, column: &'a str, value: Option<i32>) -> Self {
        self.conditions
            .push(Condition::NullableIntEquals { column, value });
        self
    }

    /// Exclude a specific record ID from the check.
    ///
    /// Used for update validation to allow the current record to keep its value.
    ///
    /// Generates: `AND id <> $N`
    pub fn excluding(mut self, id: Uuid) -> Self {
        self.exclude_id = Some(id);
        self
    }

    /// Include soft-deleted records in the check.
    ///
    /// By default, `ExistsCheck` adds `AND deleted_at IS NULL` to exclude
    /// soft-deleted records. Call this method to check all records including
    /// deleted ones.
    ///
    /// Use this for tables without soft-delete or when you explicitly need
    /// to check against deleted records (e.g., preventing slug reuse).
    pub fn include_deleted(mut self) -> Self {
        self.include_deleted = true;
        self
    }

    /// Execute the existence check.
    ///
    /// Returns `true` if a matching record exists (respecting soft-delete).
    pub async fn check(self, pool: &PgPool) -> Result<bool, sqlx::Error> {
        if self.conditions.is_empty() {
            // No conditions means we'd match everything - probably a bug
            return Ok(false);
        }

        let mut param_idx = 1u32;
        let mut where_parts = Vec::new();

        // Build condition clauses
        for condition in &self.conditions {
            let clause = match condition {
                Condition::StringEquals { column, .. }
                | Condition::IntEquals { column, .. }
                | Condition::UuidEquals { column, .. } => {
                    let clause = format!("{} = ${}", column, param_idx);
                    param_idx += 1;
                    clause
                }
                Condition::NullableIntEquals { column, .. } => {
                    let clause = format!("{} IS NOT DISTINCT FROM ${}", column, param_idx);
                    param_idx += 1;
                    clause
                }
            };
            where_parts.push(clause);
        }

        // Add exclusion clause if present
        if self.exclude_id.is_some() {
            where_parts.push(format!("id <> ${}", param_idx));
            param_idx += 1;
        }
        let _ = param_idx; // silence unused warning

        // Add soft-delete condition unless explicitly including deleted records
        if !self.include_deleted {
            where_parts.push("deleted_at IS NULL".to_string());
        }

        let where_clause = where_parts.join(" AND ");
        let query = format!(
            "SELECT EXISTS(SELECT 1 FROM {}.{} WHERE {})",
            self.schema, self.table, where_clause
        );

        // Build and execute query with bindings
        let mut sqlx_query = sqlx::query_scalar::<_, bool>(&query);

        for condition in &self.conditions {
            sqlx_query = match condition {
                Condition::StringEquals { value, .. } => sqlx_query.bind(value.as_str()),
                Condition::IntEquals { value, .. } => sqlx_query.bind(*value),
                Condition::UuidEquals { value, .. } => sqlx_query.bind(*value),
                Condition::NullableIntEquals { value, .. } => sqlx_query.bind(*value),
            };
        }

        if let Some(exclude_id) = self.exclude_id {
            sqlx_query = sqlx_query.bind(exclude_id);
        }

        sqlx_query.fetch_one(pool).await
    }
}

// =============================================================================
// Legacy Helper Functions (kept for backwards compatibility)
// =============================================================================

/// Check if a value exists in a table column.
///
/// Checks for existence while respecting soft-delete (assumes `deleted_at` column).
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `schema` - Schema name (e.g., "learning", "content")
/// * `table` - Table name (e.g., "pathway", "module")
/// * `column` - Column to check (e.g., "slug", "key")
/// * `value` - Value to check for
///
/// # Example
/// ```rust,ignore
/// let exists = value_exists(&pool, "learning", "pathway", "slug", "my-slug").await?;
/// if exists {
///     return Err("Slug already in use");
/// }
/// ```
pub async fn value_exists(
    pool: &PgPool,
    schema: &str,
    table: &str,
    column: &str,
    value: &str,
) -> Result<bool, sqlx::Error> {
    ExistsCheck::new(schema, table)
        .value(column, value)
        .check(pool)
        .await
}

/// Check if a value exists in a table column, excluding a specific ID.
///
/// Used for update validation to ensure uniqueness while excluding the current record.
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `schema` - Schema name (e.g., "learning", "content")
/// * `table` - Table name (e.g., "pathway", "module")
/// * `column` - Column to check (e.g., "slug", "key")
/// * `value` - Value to check for
/// * `exclude_id` - ID of record to exclude (typically the record being updated)
///
/// # Example
/// ```rust,ignore
/// let exists = value_exists_excluding(
///     &pool, "learning", "pathway", "slug", "new-slug", current_pathway_id
/// ).await?;
/// if exists {
///     return Err("Slug already in use by another pathway");
/// }
/// ```
pub async fn value_exists_excluding(
    pool: &PgPool,
    schema: &str,
    table: &str,
    column: &str,
    value: &str,
    exclude_id: Uuid,
) -> Result<bool, sqlx::Error> {
    ExistsCheck::new(schema, table)
        .value(column, value)
        .excluding(exclude_id)
        .check(pool)
        .await
}

/// Check if a value exists within a parent scope.
///
/// Used for fields that must be unique within a parent entity (e.g., section label
/// unique within a module, area number unique within a section).
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `schema` - Schema name (e.g., "learning")
/// * `table` - Table name (e.g., "section")
/// * `column` - Column to check (e.g., "label")
/// * `value` - Value to check for
/// * `scope_column` - Foreign key column for scoping (e.g., "module_id")
/// * `scope_id` - ID of the parent entity
///
/// # Example
/// ```rust,ignore
/// // Check if section label "A" exists within module
/// let exists = value_exists_in_scope(
///     &pool, "learning", "section", "label", "A", "module_id", module_id
/// ).await?;
/// ```
pub async fn value_exists_in_scope(
    pool: &PgPool,
    schema: &str,
    table: &str,
    column: &str,
    value: &str,
    scope_column: &str,
    scope_id: Uuid,
) -> Result<bool, sqlx::Error> {
    ExistsCheck::new(schema, table)
        .value(column, value)
        .scope(scope_column, scope_id)
        .check(pool)
        .await
}

/// Check if a value exists within a parent scope, excluding a specific ID.
///
/// Used for update validation of scoped unique fields.
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `schema` - Schema name (e.g., "learning")
/// * `table` - Table name (e.g., "section")
/// * `column` - Column to check (e.g., "label")
/// * `value` - Value to check for
/// * `scope_column` - Foreign key column for scoping (e.g., "module_id")
/// * `scope_id` - ID of the parent entity
/// * `exclude_id` - ID of record to exclude (typically the record being updated)
///
/// # Example
/// ```rust,ignore
/// // Check if section label "A" exists within module, excluding current section
/// let exists = value_exists_in_scope_excluding(
///     &pool, "learning", "section", "label", "A", "module_id", module_id, section_id
/// ).await?;
/// ```
#[allow(clippy::too_many_arguments)]
pub async fn value_exists_in_scope_excluding(
    pool: &PgPool,
    schema: &str,
    table: &str,
    column: &str,
    value: &str,
    scope_column: &str,
    scope_id: Uuid,
    exclude_id: Uuid,
) -> Result<bool, sqlx::Error> {
    ExistsCheck::new(schema, table)
        .value(column, value)
        .scope(scope_column, scope_id)
        .excluding(exclude_id)
        .check(pool)
        .await
}

/// Check if a numeric value exists within a parent scope.
///
/// Similar to `value_exists_in_scope` but for integer columns (e.g., area number).
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `schema` - Schema name (e.g., "learning")
/// * `table` - Table name (e.g., "area")
/// * `column` - Column to check (e.g., "number")
/// * `value` - Numeric value to check for
/// * `scope_column` - Foreign key column for scoping (e.g., "section_id")
/// * `scope_id` - ID of the parent entity
///
/// # Example
/// ```rust,ignore
/// // Check if area number 1 exists within section
/// let exists = number_exists_in_scope(
///     &pool, "learning", "area", "number", 1, "section_id", section_id
/// ).await?;
/// ```
pub async fn number_exists_in_scope(
    pool: &PgPool,
    schema: &str,
    table: &str,
    column: &str,
    value: i32,
    scope_column: &str,
    scope_id: Uuid,
) -> Result<bool, sqlx::Error> {
    ExistsCheck::new(schema, table)
        .value_i32(column, value)
        .scope(scope_column, scope_id)
        .check(pool)
        .await
}

/// Check if a numeric value exists within a parent scope, excluding a specific ID.
///
/// # Example
/// ```rust,ignore
/// let exists = number_exists_in_scope_excluding(
///     &pool, "learning", "area", "number", 1, "section_id", section_id, area_id
/// ).await?;
/// ```
#[allow(clippy::too_many_arguments)]
pub async fn number_exists_in_scope_excluding(
    pool: &PgPool,
    schema: &str,
    table: &str,
    column: &str,
    value: i32,
    scope_column: &str,
    scope_id: Uuid,
    exclude_id: Uuid,
) -> Result<bool, sqlx::Error> {
    ExistsCheck::new(schema, table)
        .value_i32(column, value)
        .scope(scope_column, scope_id)
        .excluding(exclude_id)
        .check(pool)
        .await
}

#[cfg(test)]
#[path = "existence_tests.rs"]
mod tests;
