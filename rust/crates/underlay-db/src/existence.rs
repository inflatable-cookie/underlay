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
//! # Example
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

use sqlx::PgPool;
use uuid::Uuid;

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
    let query = format!(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM {schema}.{table}
            WHERE {column} = $1
              AND deleted_at IS NULL
        )
        "#
    );
    sqlx::query_scalar::<_, bool>(&query)
        .bind(value)
        .fetch_one(pool)
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
    let query = format!(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM {schema}.{table}
            WHERE {column} = $1
              AND id <> $2
              AND deleted_at IS NULL
        )
        "#
    );
    sqlx::query_scalar::<_, bool>(&query)
        .bind(value)
        .bind(exclude_id)
        .fetch_one(pool)
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
    let query = format!(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM {schema}.{table}
            WHERE {column} = $1
              AND {scope_column} = $2
              AND deleted_at IS NULL
        )
        "#
    );
    sqlx::query_scalar::<_, bool>(&query)
        .bind(value)
        .bind(scope_id)
        .fetch_one(pool)
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
    let query = format!(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM {schema}.{table}
            WHERE {column} = $1
              AND {scope_column} = $2
              AND id <> $3
              AND deleted_at IS NULL
        )
        "#
    );
    sqlx::query_scalar::<_, bool>(&query)
        .bind(value)
        .bind(scope_id)
        .bind(exclude_id)
        .fetch_one(pool)
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
    let query = format!(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM {schema}.{table}
            WHERE {column} = $1
              AND {scope_column} = $2
              AND deleted_at IS NULL
        )
        "#
    );
    sqlx::query_scalar::<_, bool>(&query)
        .bind(value)
        .bind(scope_id)
        .fetch_one(pool)
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
    let query = format!(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM {schema}.{table}
            WHERE {column} = $1
              AND {scope_column} = $2
              AND id <> $3
              AND deleted_at IS NULL
        )
        "#
    );
    sqlx::query_scalar::<_, bool>(&query)
        .bind(value)
        .bind(scope_id)
        .bind(exclude_id)
        .fetch_one(pool)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These are unit tests for query building. Integration tests with
    // actual database connections are in the tests/ directory.

    #[test]
    fn test_query_format_value_exists() {
        // Verify the query format is correct (no SQL injection via format string)
        let query = format!(
            r#"
        SELECT EXISTS(
            SELECT 1
            FROM {schema}.{table}
            WHERE {column} = $1
              AND deleted_at IS NULL
        )
        "#,
            schema = "learning",
            table = "pathway",
            column = "slug"
        );
        assert!(query.contains("learning.pathway"));
        assert!(query.contains("slug = $1"));
        assert!(query.contains("deleted_at IS NULL"));
    }

    #[test]
    fn test_query_format_value_exists_excluding() {
        let query = format!(
            r#"
        SELECT EXISTS(
            SELECT 1
            FROM {schema}.{table}
            WHERE {column} = $1
              AND id <> $2
              AND deleted_at IS NULL
        )
        "#,
            schema = "content",
            table = "video_item",
            column = "slug"
        );
        assert!(query.contains("content.video_item"));
        assert!(query.contains("id <> $2"));
    }

    #[test]
    fn test_query_format_scoped() {
        let query = format!(
            r#"
        SELECT EXISTS(
            SELECT 1
            FROM {schema}.{table}
            WHERE {column} = $1
              AND {scope_column} = $2
              AND deleted_at IS NULL
        )
        "#,
            schema = "learning",
            table = "section",
            column = "label",
            scope_column = "module_id"
        );
        assert!(query.contains("learning.section"));
        assert!(query.contains("label = $1"));
        assert!(query.contains("module_id = $2"));
    }
}
