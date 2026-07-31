//! Transactional weight-based reordering for admin list endpoints.
//!
//! Canonical contract: clients submit the complete ordered id list for a
//! scope; the server diffs against the current set inside one transaction
//! and either rewrites weights in a single statement or returns a
//! [`ReorderConflict`] the client can merge and retry (see
//! `ts/src/patterns/reorder-conflict.ts`).
//!
//! Replaces per-row UPDATE loops, which are non-transactional and corrupt
//! weights on partial failure.

use sqlx::PgPool;
use uuid::Uuid;

use crate::identifiers::{QualifiedTableName, SqlIdentifier};

/// The submitted set no longer matches the persisted set: items were
/// added or removed concurrently. The client should merge these into its
/// working order and retry the save.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ReorderConflict {
    /// Ids present in the submission but not in the persisted set.
    pub added_ids: Vec<Uuid>,
    /// Ids present in the persisted set but missing from the submission.
    pub removed_ids: Vec<Uuid>,
}

impl std::fmt::Display for ReorderConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "reorder set changed: {} added, {} removed",
            self.added_ids.len(),
            self.removed_ids.len()
        )
    }
}

impl std::error::Error for ReorderConflict {}

/// Errors from [`reorder_scoped`].
#[derive(Debug, thiserror::Error)]
pub enum ReorderError {
    #[error(transparent)]
    Conflict(#[from] ReorderConflict),
    #[error("reorder submission contains duplicate ids")]
    DuplicateIds,
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
}

/// Optional scoping for a reorder: a parent column and value (e.g.
/// `pathway_id = $x`) plus an optional soft-delete column to exclude
/// (`deleted_at IS NULL`).
#[derive(Debug, Clone, Copy)]
pub struct ReorderScope<'a> {
    pub parent_column: Option<(&'a SqlIdentifier, Uuid)>,
    pub not_deleted_column: Option<&'a SqlIdentifier>,
}

impl<'a> ReorderScope<'a> {
    pub fn none() -> Self {
        Self {
            parent_column: None,
            not_deleted_column: None,
        }
    }

    pub fn parent(column: &'a SqlIdentifier, value: Uuid) -> Self {
        Self {
            parent_column: Some((column, value)),
            not_deleted_column: None,
        }
    }

    pub fn exclude_deleted(mut self, column: &'a SqlIdentifier) -> Self {
        self.not_deleted_column = Some(column);
        self
    }
}

/// Reorder the weight column of a table within a scope, transactionally.
///
/// - Locks the current set (`FOR UPDATE`), diffs it against `ordered_ids`.
/// - On set mismatch returns [`ReorderError::Conflict`] (no writes).
/// - On match rewrites weights to 1..=N in a single `UPDATE ... FROM
///   unnest ... WITH ORDINALITY` statement.
///
/// Identifiers must come through [`QualifiedTableName`]/[`SqlIdentifier`],
/// so table and column names are validated, never interpolated raw.
///
/// Returns the number of rows updated.
pub async fn reorder_scoped(
    pool: &PgPool,
    table: &QualifiedTableName,
    id_column: &SqlIdentifier,
    weight_column: &SqlIdentifier,
    scope: ReorderScope<'_>,
    ordered_ids: &[Uuid],
) -> Result<u64, ReorderError> {
    // Reject duplicates before touching the database.
    let mut seen = std::collections::HashSet::with_capacity(ordered_ids.len());
    if ordered_ids.iter().any(|id| !seen.insert(*id)) {
        return Err(ReorderError::DuplicateIds);
    }

    let table_q = table.quoted();
    let id_q = id_column.quoted();
    let weight_q = weight_column.quoted();

    let mut scope_sql = String::new();
    if let Some((col, _)) = scope.parent_column {
        scope_sql.push_str(&format!("{} = $1", col.quoted()));
    }
    if let Some(col) = scope.not_deleted_column {
        if !scope_sql.is_empty() {
            scope_sql.push_str(" AND ");
        }
        scope_sql.push_str(&format!("{} IS NULL", col.quoted()));
    }
    let where_sql = if scope_sql.is_empty() {
        String::new()
    } else {
        format!("WHERE {scope_sql}")
    };
    let and_scope_sql = if scope_sql.is_empty() {
        String::new()
    } else {
        format!("AND {scope_sql}")
    };

    let mut tx = pool.begin().await?;

    // Lock the current set so concurrent writers serialize on the diff.
    let current: Vec<Uuid> = sqlx::query_scalar(&format!(
        "SELECT {id_q} FROM {table_q} {where_sql} ORDER BY {weight_q} ASC, {id_q} ASC FOR UPDATE"
    ))
    .bind(scope.parent_column.map(|(_, v)| v))
    .fetch_all(&mut *tx)
    .await?;

    let submitted: std::collections::HashSet<Uuid> = ordered_ids.iter().copied().collect();
    let persisted: std::collections::HashSet<Uuid> = current.iter().copied().collect();

    if submitted != persisted {
        let added_ids: Vec<Uuid> = submitted.difference(&persisted).copied().collect();
        let removed_ids: Vec<Uuid> = persisted.difference(&submitted).copied().collect();
        return Err(ReorderError::Conflict(ReorderConflict {
            added_ids,
            removed_ids,
        }));
    }

    let rows = if ordered_ids.is_empty() {
        0
    } else {
        let result = if let Some((_, parent_value)) = scope.parent_column {
            // Scoped: $1 = parent value, $2 = ordered ids.
            sqlx::query(&format!(
                "UPDATE {table_q} SET {weight_q} = u.ord \
                 FROM (SELECT * FROM unnest($2::uuid[]) WITH ORDINALITY) AS u(id, ord) \
                 WHERE {table_q}.{id_q} = u.id AND {scope_sql}"
            ))
            .bind(parent_value)
            .bind(ordered_ids)
            .execute(&mut *tx)
            .await?
        } else {
            // Unscoped: $1 = ordered ids.
            sqlx::query(&format!(
                "UPDATE {table_q} SET {weight_q} = u.ord \
                 FROM (SELECT * FROM unnest($1::uuid[]) WITH ORDINALITY) AS u(id, ord) \
                 WHERE {table_q}.{id_q} = u.id {and_scope_sql}"
            ))
            .bind(ordered_ids)
            .execute(&mut *tx)
            .await?
        };
        result.rows_affected()
    };

    tx.commit().await?;
    Ok(rows)
}

#[cfg(test)]
#[path = "tests/reorder_tests.rs"]
mod tests;
