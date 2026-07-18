//! Integration tests for the audit writer/query helpers against a real Postgres.
//!
//! `#[ignore]`d by default (needs a database). Run with:
//!
//! ```bash
//! UNDERLAY_TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/postgres \
//!   cargo test -p underlay-audit -- --ignored
//! ```

use serde_json::json;
use uuid::Uuid;

use underlay_testing::TestDb;

use crate::entry::{AuditAction, AuditEntry};
use crate::query::{
    count_audit_logs_from_table, get_audit_log_by_id_from_table, list_audit_logs_from_table,
    AuditLogFilters,
};
use crate::tables::AuditTable;
use crate::writer::append_audit_log_to_table;

struct Fixture {
    db: TestDb,
    table: AuditTable,
}

/// Create the audit-log table in this test's isolated schema (matches the
/// crate's documented example migration).
async fn setup() -> Fixture {
    let db = TestDb::new().await;
    let schema = db.schema_name().to_string();

    sqlx::query(&format!(
        r#"
        CREATE TABLE {schema}.audit_log (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            user_id UUID,
            action TEXT NOT NULL,
            resource_type TEXT NOT NULL,
            resource_id UUID NOT NULL,
            details JSONB NOT NULL DEFAULT '{{}}',
            correlation_id TEXT,
            ip_address TEXT
        )
        "#
    ))
    .execute(db.pool())
    .await
    .expect("create audit_log");

    let table = AuditTable::parse(format!("{schema}.audit_log")).expect("valid table");
    Fixture { db, table }
}

fn filters() -> AuditLogFilters {
    AuditLogFilters {
        limit: 50,
        ..Default::default()
    }
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn append_returns_stored_row_and_get_by_id_round_trips() {
    let fx = setup().await;
    let user = Uuid::now_v7();
    let resource = Uuid::now_v7();

    let entry = AuditEntry::new(Some(user), AuditAction::Update, "invoice", resource)
        .with_details(json!({ "field": "total" }))
        .with_correlation_id("corr-1")
        .with_ip_address("203.0.113.9");

    let row = append_audit_log_to_table(fx.db.pool(), &fx.table, entry)
        .await
        .expect("append");

    assert_eq!(row.user_id, Some(user));
    assert_eq!(row.action, "update");
    assert_eq!(row.resource_type, "invoice");
    assert_eq!(row.resource_id, resource);
    assert_eq!(row.details, json!({ "field": "total" }));
    assert_eq!(row.correlation_id.as_deref(), Some("corr-1"));

    let fetched = get_audit_log_by_id_from_table(fx.db.pool(), &fx.table, row.id)
        .await
        .expect("get_by_id");
    assert!(fetched.is_some());
    assert_eq!(fetched.unwrap().id, row.id);

    // Unknown id yields None.
    let missing = get_audit_log_by_id_from_table(fx.db.pool(), &fx.table, Uuid::now_v7())
        .await
        .expect("get_by_id missing");
    assert!(missing.is_none());
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn list_and_count_apply_filters() {
    let fx = setup().await;
    let alice = Uuid::now_v7();
    let bob = Uuid::now_v7();

    for (user, action, resource_type) in [
        (Some(alice), AuditAction::Create, "invoice"),
        (Some(alice), AuditAction::Delete, "invoice"),
        (Some(bob), AuditAction::Create, "customer"),
        (None, AuditAction::Login, "session"),
    ] {
        append_audit_log_to_table(
            fx.db.pool(),
            &fx.table,
            AuditEntry::new(user, action, resource_type, Uuid::now_v7()),
        )
        .await
        .expect("append");
    }

    // No filter -> all four.
    let all = list_audit_logs_from_table(fx.db.pool(), &fx.table, filters())
        .await
        .expect("list all");
    assert_eq!(all.len(), 4);
    assert_eq!(
        count_audit_logs_from_table(fx.db.pool(), &fx.table, &filters())
            .await
            .unwrap(),
        4
    );

    // Filter by user.
    let by_user = AuditLogFilters {
        user_id: Some(alice),
        ..filters()
    };
    let alice_rows = list_audit_logs_from_table(fx.db.pool(), &fx.table, by_user.clone())
        .await
        .expect("list by user");
    assert_eq!(alice_rows.len(), 2);
    assert!(alice_rows.iter().all(|r| r.user_id == Some(alice)));
    assert_eq!(
        count_audit_logs_from_table(fx.db.pool(), &fx.table, &by_user)
            .await
            .unwrap(),
        2
    );

    // Filter by action.
    let by_action = AuditLogFilters {
        action: Some("create".to_string()),
        ..filters()
    };
    let creates = list_audit_logs_from_table(fx.db.pool(), &fx.table, by_action.clone())
        .await
        .expect("list by action");
    assert_eq!(creates.len(), 2);
    assert!(creates.iter().all(|r| r.action == "create"));

    // Filter by resource_type.
    let by_resource = AuditLogFilters {
        resource_type: Some("customer".to_string()),
        ..filters()
    };
    assert_eq!(
        count_audit_logs_from_table(fx.db.pool(), &fx.table, &by_resource)
            .await
            .unwrap(),
        1
    );
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn list_orders_newest_first_and_paginates() {
    let fx = setup().await;
    let user = Uuid::now_v7();

    for _ in 0..5 {
        append_audit_log_to_table(
            fx.db.pool(),
            &fx.table,
            AuditEntry::new(Some(user), AuditAction::Update, "doc", Uuid::now_v7()),
        )
        .await
        .expect("append");
    }

    let page = list_audit_logs_from_table(
        fx.db.pool(),
        &fx.table,
        AuditLogFilters {
            limit: 2,
            offset: 0,
            ..Default::default()
        },
    )
    .await
    .expect("list page");
    assert_eq!(page.len(), 2);
    // Newest first.
    assert!(page[0].occurred_at >= page[1].occurred_at);

    let next = list_audit_logs_from_table(
        fx.db.pool(),
        &fx.table,
        AuditLogFilters {
            limit: 2,
            offset: 2,
            ..Default::default()
        },
    )
    .await
    .expect("list next page");
    assert_eq!(next.len(), 2);
    // Disjoint from the first page.
    assert!(next.iter().all(|r| !page.iter().any(|p| p.id == r.id)));
}
