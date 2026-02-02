//! Task and project database operations.
//!
//! Example domain queries demonstrating common patterns.

use chrono::{DateTime, NaiveDate, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::DbPool;

// ============================================================================
// Projects
// ============================================================================

/// Row type for acme.projects table.
#[derive(Debug, Clone, FromRow)]
pub struct ProjectRow {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create a new project.
pub async fn create_project(
    pool: &DbPool,
    id: Uuid,
    owner_id: Uuid,
    name: &str,
    description: Option<&str>,
) -> Result<ProjectRow, sqlx::Error> {
    sqlx::query_as::<_, ProjectRow>(
        r#"
        INSERT INTO acme.projects (id, owner_id, name, description)
        VALUES ($1, $2, $3, $4)
        RETURNING id, owner_id, name, description, status, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(owner_id)
    .bind(name)
    .bind(description)
    .fetch_one(pool)
    .await
}

/// Get a project by ID.
pub async fn get_project(pool: &DbPool, id: Uuid) -> Result<Option<ProjectRow>, sqlx::Error> {
    sqlx::query_as::<_, ProjectRow>(
        r#"
        SELECT id, owner_id, name, description, status, created_at, updated_at
        FROM acme.projects
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// List projects for a user.
pub async fn list_projects_for_user(
    pool: &DbPool,
    owner_id: Uuid,
    include_archived: bool,
) -> Result<Vec<ProjectRow>, sqlx::Error> {
    if include_archived {
        sqlx::query_as::<_, ProjectRow>(
            r#"
            SELECT id, owner_id, name, description, status, created_at, updated_at
            FROM acme.projects
            WHERE owner_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(owner_id)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, ProjectRow>(
            r#"
            SELECT id, owner_id, name, description, status, created_at, updated_at
            FROM acme.projects
            WHERE owner_id = $1 AND status = 'active'
            ORDER BY created_at DESC
            "#,
        )
        .bind(owner_id)
        .fetch_all(pool)
        .await
    }
}

/// Update a project.
pub async fn update_project(
    pool: &DbPool,
    id: Uuid,
    name: Option<&str>,
    description: Option<Option<&str>>,
    status: Option<&str>,
) -> Result<Option<ProjectRow>, sqlx::Error> {
    sqlx::query_as::<_, ProjectRow>(
        r#"
        UPDATE acme.projects
        SET
            name = COALESCE($2, name),
            description = CASE WHEN $3 THEN $4 ELSE description END,
            status = COALESCE($5, status),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, owner_id, name, description, status, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(description.is_some())
    .bind(description.flatten())
    .bind(status)
    .fetch_optional(pool)
    .await
}

/// Delete a project.
pub async fn delete_project(pool: &DbPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM acme.projects WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

// ============================================================================
// Tasks
// ============================================================================

/// Row type for acme.tasks table.
#[derive(Debug, Clone, FromRow)]
pub struct TaskRow {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub due_date: Option<NaiveDate>,
    pub completed_at: Option<DateTime<Utc>>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create a new task.
pub async fn create_task(
    pool: &DbPool,
    id: Uuid,
    project_id: Uuid,
    title: &str,
    description: Option<&str>,
    priority: &str,
    due_date: Option<NaiveDate>,
) -> Result<TaskRow, sqlx::Error> {
    // Get the next position
    let next_position: i32 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(MAX(position), -1) + 1
        FROM acme.tasks
        WHERE project_id = $1
        "#,
    )
    .bind(project_id)
    .fetch_one(pool)
    .await?;

    sqlx::query_as::<_, TaskRow>(
        r#"
        INSERT INTO acme.tasks (id, project_id, title, description, priority, due_date, position)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, project_id, title, description, status, priority, due_date, completed_at, position, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(project_id)
    .bind(title)
    .bind(description)
    .bind(priority)
    .bind(due_date)
    .bind(next_position)
    .fetch_one(pool)
    .await
}

/// Get a task by ID.
pub async fn get_task(pool: &DbPool, id: Uuid) -> Result<Option<TaskRow>, sqlx::Error> {
    sqlx::query_as::<_, TaskRow>(
        r#"
        SELECT id, project_id, title, description, status, priority, due_date, completed_at, position, created_at, updated_at
        FROM acme.tasks
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// List tasks for a project.
pub async fn list_tasks_for_project(
    pool: &DbPool,
    project_id: Uuid,
    include_completed: bool,
) -> Result<Vec<TaskRow>, sqlx::Error> {
    if include_completed {
        sqlx::query_as::<_, TaskRow>(
            r#"
            SELECT id, project_id, title, description, status, priority, due_date, completed_at, position, created_at, updated_at
            FROM acme.tasks
            WHERE project_id = $1
            ORDER BY position
            "#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, TaskRow>(
            r#"
            SELECT id, project_id, title, description, status, priority, due_date, completed_at, position, created_at, updated_at
            FROM acme.tasks
            WHERE project_id = $1 AND status NOT IN ('completed', 'cancelled')
            ORDER BY position
            "#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
    }
}

/// Update a task.
pub async fn update_task(
    pool: &DbPool,
    id: Uuid,
    title: Option<&str>,
    description: Option<Option<&str>>,
    status: Option<&str>,
    priority: Option<&str>,
    due_date: Option<Option<NaiveDate>>,
) -> Result<Option<TaskRow>, sqlx::Error> {
    // Handle completed_at based on status change
    let completed_at_expr = match status {
        Some("completed") => "NOW()",
        Some(_) => "NULL",
        None => "completed_at",
    };

    let query = format!(
        r#"
        UPDATE acme.tasks
        SET
            title = COALESCE($2, title),
            description = CASE WHEN $3 THEN $4 ELSE description END,
            status = COALESCE($5, status),
            priority = COALESCE($6, priority),
            due_date = CASE WHEN $7 THEN $8 ELSE due_date END,
            completed_at = {},
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, project_id, title, description, status, priority, due_date, completed_at, position, created_at, updated_at
        "#,
        completed_at_expr
    );

    sqlx::query_as::<_, TaskRow>(&query)
        .bind(id)
        .bind(title)
        .bind(description.is_some())
        .bind(description.flatten())
        .bind(status)
        .bind(priority)
        .bind(due_date.is_some())
        .bind(due_date.flatten())
        .fetch_optional(pool)
        .await
}

/// Delete a task.
pub async fn delete_task(pool: &DbPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM acme.tasks WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

/// Reorder tasks within a project.
pub async fn reorder_tasks(
    pool: &DbPool,
    project_id: Uuid,
    task_ids: &[Uuid],
) -> Result<(), sqlx::Error> {
    for (position, task_id) in task_ids.iter().enumerate() {
        sqlx::query(
            r#"
            UPDATE acme.tasks
            SET position = $3, updated_at = NOW()
            WHERE id = $1 AND project_id = $2
            "#,
        )
        .bind(task_id)
        .bind(project_id)
        .bind(position as i32)
        .execute(pool)
        .await?;
    }
    Ok(())
}

// ============================================================================
// Task Comments
// ============================================================================

/// Row type for acme.task_comments table.
#[derive(Debug, Clone, FromRow)]
pub struct TaskCommentRow {
    pub id: Uuid,
    pub task_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create a comment on a task.
pub async fn create_task_comment(
    pool: &DbPool,
    id: Uuid,
    task_id: Uuid,
    author_id: Uuid,
    body: &str,
) -> Result<TaskCommentRow, sqlx::Error> {
    sqlx::query_as::<_, TaskCommentRow>(
        r#"
        INSERT INTO acme.task_comments (id, task_id, author_id, body)
        VALUES ($1, $2, $3, $4)
        RETURNING id, task_id, author_id, body, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(task_id)
    .bind(author_id)
    .bind(body)
    .fetch_one(pool)
    .await
}

/// List comments for a task.
pub async fn list_task_comments(
    pool: &DbPool,
    task_id: Uuid,
) -> Result<Vec<TaskCommentRow>, sqlx::Error> {
    sqlx::query_as::<_, TaskCommentRow>(
        r#"
        SELECT id, task_id, author_id, body, created_at, updated_at
        FROM acme.task_comments
        WHERE task_id = $1
        ORDER BY created_at
        "#,
    )
    .bind(task_id)
    .fetch_all(pool)
    .await
}
