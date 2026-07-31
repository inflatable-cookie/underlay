//! Integration tests for `reorder_scoped`. These run against a real
//! Postgres (UNDERLAY_TEST_DATABASE_URL) and are `#[ignore]`d by default,
//! matching the other DB-backed suites in this workspace.

use super::*;
use crate::identifiers::QualifiedTableName;

async fn test_pool() -> Option<PgPool> {
    let url = std::env::var("UNDERLAY_TEST_DATABASE_URL").ok()?;
    let pool = PgPool::connect(&url).await.ok()?;
    sqlx::query("DROP TABLE IF EXISTS test_reorder_items")
        .execute(&pool)
        .await
        .ok()?;
    sqlx::query(
        "CREATE TABLE test_reorder_items (
            id uuid PRIMARY KEY,
            parent_id uuid NOT NULL,
            weight int NOT NULL,
            deleted_at timestamptz
        )",
    )
    .execute(&pool)
    .await
    .ok()?;
    Some(pool)
}

fn col(name: &str) -> SqlIdentifier {
    SqlIdentifier::parse(name).unwrap()
}

async fn insert(pool: &PgPool, parent: Uuid, weight: i32) -> Uuid {
    let id = Uuid::now_v7();
    sqlx::query("INSERT INTO test_reorder_items (id, parent_id, weight) VALUES ($1, $2, $3)")
        .bind(id)
        .bind(parent)
        .bind(weight)
        .execute(pool)
        .await
        .unwrap();
    id
}

#[tokio::test]
#[ignore]
async fn scoped_reorder_rewrites_weights_in_one_statement() {
    let Some(pool) = test_pool().await else {
        return;
    };
    let parent = Uuid::now_v7();
    let a = insert(&pool, parent, 1).await;
    let b = insert(&pool, parent, 2).await;
    let c = insert(&pool, parent, 3).await;

    let table = QualifiedTableName::parse("public.test_reorder_items").unwrap();
    let id_col = col("id");
    let weight_col = col("weight");
    let parent_col = col("parent_id");
    let deleted_col = col("deleted_at");

    let rows = reorder_scoped(
        &pool,
        &table,
        &id_col,
        &weight_col,
        ReorderScope::parent(&parent_col, parent).exclude_deleted(&deleted_col),
        &[c, a, b],
    )
    .await
    .unwrap();
    assert_eq!(rows, 3);

    let ordered: Vec<(Uuid, i32)> = sqlx::query_as(
        "SELECT id, weight FROM test_reorder_items WHERE parent_id = $1 ORDER BY weight ASC",
    )
    .bind(parent)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(ordered, vec![(c, 1), (a, 2), (b, 3)]);
}

#[tokio::test]
#[ignore]
async fn set_mismatch_returns_conflict_with_added_and_removed() {
    let Some(pool) = test_pool().await else {
        return;
    };
    let parent = Uuid::now_v7();
    let a = insert(&pool, parent, 1).await;
    let b = insert(&pool, parent, 2).await;
    let stranger = Uuid::now_v7();

    let table = QualifiedTableName::parse("public.test_reorder_items").unwrap();
    let id_col = col("id");
    let weight_col = col("weight");
    let parent_col = col("parent_id");

    let err = reorder_scoped(
        &pool,
        &table,
        &id_col,
        &weight_col,
        ReorderScope::parent(&parent_col, parent),
        &[a, stranger],
    )
    .await
    .unwrap_err();

    match err {
        ReorderError::Conflict(conflict) => {
            assert_eq!(conflict.added_ids, vec![stranger]);
            assert_eq!(conflict.removed_ids, vec![b]);
        }
        other => panic!("expected conflict, got {other:?}"),
    }

    // No weights were touched.
    let weights: Vec<i32> = sqlx::query_scalar(
        "SELECT weight FROM test_reorder_items WHERE parent_id = $1 ORDER BY weight ASC",
    )
    .bind(parent)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(weights, vec![1, 2]);
}

#[tokio::test]
#[ignore]
async fn duplicates_are_rejected_before_writing() {
    let Some(pool) = test_pool().await else {
        return;
    };
    let parent = Uuid::now_v7();
    let a = insert(&pool, parent, 1).await;

    let table = QualifiedTableName::parse("public.test_reorder_items").unwrap();
    let id_col = col("id");
    let weight_col = col("weight");
    let parent_col = col("parent_id");

    let err = reorder_scoped(
        &pool,
        &table,
        &id_col,
        &weight_col,
        ReorderScope::parent(&parent_col, parent),
        &[a, a],
    )
    .await
    .unwrap_err();
    assert!(matches!(err, ReorderError::DuplicateIds));
}
