//! Integration tests for `AuthStateStore` against a real Postgres.
//!
//! These are `#[ignore]`d by default because they need a database. Provide one
//! and run them explicitly:
//!
//! ```bash
//! UNDERLAY_TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/postgres \
//!   cargo test -p underlay-auth-state-postgres -- --ignored
//! ```
//!
//! Without `UNDERLAY_TEST_DATABASE_URL`, `TestDb` falls back to a testcontainer
//! (which needs a Docker API). Either way each test gets an isolated schema.

use chrono::Duration;
use serde::{Deserialize, Serialize};
use serde_json::json;
use underlay_core::Uuid;
use underlay_testing::TestDb;

use crate::{AuthStateError, AuthStateStore};

/// Create the auth-state table in this test's isolated schema and return the
/// schema-qualified name to hand to `AuthStateStore::with_table`. Schema
/// qualification avoids relying on a per-connection `search_path` surviving the
/// pool.
async fn setup_table(db: &TestDb) -> String {
    let table = format!("{}.auth_state", db.schema_name());
    sqlx::query(&format!(
        r#"
        CREATE TABLE {table} (
            id UUID PRIMARY KEY,
            user_id UUID NULL,
            state_type TEXT NOT NULL,
            state JSONB NOT NULL,
            created_at TIMESTAMPTZ NOT NULL,
            expires_at TIMESTAMPTZ NOT NULL
        )
        "#
    ))
    .execute(db.pool())
    .await
    .expect("create auth_state table");
    table
}

fn store(db: &TestDb, table: &str) -> AuthStateStore {
    AuthStateStore::with_table(db.pool().clone(), table).expect("valid table name")
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn user_state_create_load_consume_round_trip() {
    let db = TestDb::new().await;
    let table = setup_table(&db).await;
    let store = store(&db, &table);

    let user = Uuid::new_v7();
    let id = store
        .create_user(user, "email_verify", json!({ "code": "123456" }), Duration::minutes(10))
        .await
        .expect("create_user");

    // Loads for the right user + type.
    let loaded = store
        .load_user(id, user, "email_verify")
        .await
        .expect("load_user");
    assert_eq!(loaded, Some(json!({ "code": "123456" })));

    // Consume returns the state and deletes it.
    let consumed = store
        .consume_user(id, user, "email_verify")
        .await
        .expect("consume_user");
    assert_eq!(consumed, Some(json!({ "code": "123456" })));

    // Second consume finds nothing.
    let again = store
        .consume_user(id, user, "email_verify")
        .await
        .expect("consume_user again");
    assert_eq!(again, None);
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn load_scopes_by_user_and_state_type() {
    let db = TestDb::new().await;
    let table = setup_table(&db).await;
    let store = store(&db, &table);

    let user = Uuid::new_v7();
    let other = Uuid::new_v7();
    let id = store
        .create_user(user, "reset", json!({ "n": 1 }), Duration::minutes(5))
        .await
        .expect("create_user");

    // Wrong user, wrong type, and public lookup all miss.
    assert_eq!(store.load_user(id, other, "reset").await.expect("load"), None);
    assert_eq!(store.load_user(id, user, "other").await.expect("load"), None);
    assert_eq!(store.load_public(id, "reset").await.expect("load"), None);
    // Right user + type hits.
    assert!(store.load_user(id, user, "reset").await.expect("load").is_some());
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn public_state_update_and_typed_round_trip() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Draft {
        step: u32,
        email: String,
    }

    let db = TestDb::new().await;
    let table = setup_table(&db).await;
    let store = store(&db, &table);

    let id = store
        .create_public("signup", Draft { step: 1, email: "a@b.c".into() }, Duration::minutes(30))
        .await
        .expect("create_public");

    store
        .update_public(id, "signup", json!({ "step": 2, "email": "a@b.c" }))
        .await
        .expect("update_public");

    let typed: Option<Draft> = store
        .load_public_typed(id, "signup")
        .await
        .expect("load_public_typed");
    assert_eq!(typed, Some(Draft { step: 2, email: "a@b.c".into() }));

    // Updating a non-existent public state reports invalid/expired.
    let missing = store
        .update_public(Uuid::new_v7(), "signup", json!({}))
        .await;
    assert!(matches!(missing, Err(AuthStateError::InvalidOrExpired)));
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn expired_state_is_not_loaded() {
    let db = TestDb::new().await;
    let table = setup_table(&db).await;
    let store = store(&db, &table);

    // Negative TTL => already expired.
    let id = store
        .create_public("otp", json!({ "code": "0" }), Duration::seconds(-1))
        .await
        .expect("create_public");

    assert_eq!(store.load_public(id, "otp").await.expect("load"), None);
    // Consume of an expired row also yields nothing.
    assert_eq!(store.consume_public(id, "otp").await.expect("consume"), None);
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn delete_removes_the_row() {
    let db = TestDb::new().await;
    let table = setup_table(&db).await;
    let store = store(&db, &table);

    let id = store
        .create_public("token", json!({ "v": true }), Duration::minutes(5))
        .await
        .expect("create_public");
    assert!(store.load_public(id, "token").await.expect("load").is_some());

    store.delete(id).await.expect("delete");
    assert_eq!(store.load_public(id, "token").await.expect("load"), None);
}
