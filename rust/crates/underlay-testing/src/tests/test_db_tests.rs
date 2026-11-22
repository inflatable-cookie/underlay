use super::*;

// Docker-backed tests are ignored by default.
// Run with: oxide test -p underlay-testing --features db -- --ignored

#[tokio::test]
#[ignore]
async fn test_db_creates_isolated_schema() {
    let db = TestDb::new().await;

    // Schema should exist
    let (exists,): (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM information_schema.schemata WHERE schema_name = $1)",
    )
    .bind(db.schema_name())
    .fetch_one(db.pool())
    .await
    .expect("check schema exists");

    assert!(exists, "Test schema should exist");
}

#[tokio::test]
#[ignore]
async fn test_db_load_fixture() {
    let db = TestDb::new().await;

    // Create a table via fixture
    db.load_fixture("CREATE TABLE items (id SERIAL PRIMARY KEY, name TEXT NOT NULL)")
        .await
        .expect("load fixture");

    // Insert some data
    db.load_fixture("INSERT INTO items (name) VALUES ('test1'), ('test2')")
        .await
        .expect("insert data");

    // Verify data
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*)::BIGINT FROM items")
        .fetch_one(db.pool())
        .await
        .expect("count items");

    assert_eq!(count.0, 2);
}

#[tokio::test]
#[ignore]
async fn test_db_isolation() {
    // Create two test databases
    let db1 = TestDb::new().await;
    let db2 = TestDb::new().await;

    // They should have different schemas
    assert_ne!(db1.schema_name(), db2.schema_name());

    // Create table in db1
    db1.load_fixture("CREATE TABLE isolated_table (id INT)")
        .await
        .expect("create table in db1");

    // Table should not exist in db2's schema
    let exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM information_schema.tables \
             WHERE table_schema = $1 AND table_name = 'isolated_table')",
    )
    .bind(db2.schema_name())
    .fetch_one(db2.pool())
    .await
    .expect("check table exists in db2");

    assert!(!exists.0, "Table should not exist in db2's schema");
}

#[test]
fn unique_test_schema_is_typed_and_stable_for_public_name() {
    let schema = unique_test_schema();

    assert!(schema.as_str().starts_with("test_"));
    assert_eq!(schema.quoted(), format!("\"{}\"", schema.as_str()));
}

/// Simulates `testcontainers::Container`, whose `start()` and `Drop` both run
/// `Runtime::block_on` and would panic if executed on a Tokio runtime thread.
struct BlockOnResource {
    dropped: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl BlockOnResource {
    fn start(dropped: std::sync::Arc<std::sync::atomic::AtomicBool>) -> Self {
        assert!(
            tokio::runtime::Handle::try_current().is_err(),
            "startup must run outside any Tokio runtime context"
        );
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("build nested runtime");
        rt.block_on(async {});
        Self { dropped }
    }
}

impl Drop for BlockOnResource {
    fn drop(&mut self) {
        assert!(
            tokio::runtime::Handle::try_current().is_err(),
            "drop must run outside any Tokio runtime context"
        );
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("build nested runtime in Drop");
        rt.block_on(async {});
        self.dropped
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

#[tokio::test]
async fn blocking_handle_keeps_start_and_drop_off_tokio_runtime() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    let dropped = Arc::new(AtomicBool::new(false));
    let dropped_ref = Arc::clone(&dropped);

    let (value, handle) = BlockingHandle::spawn(move || {
        let resource = BlockOnResource::start(dropped_ref);
        ("postgres://127.0.0.1:5432/postgres".to_string(), resource)
    })
    .await;

    assert_eq!(value, "postgres://127.0.0.1:5432/postgres");
    assert!(!dropped.load(Ordering::SeqCst));

    // Dropping inside an active `#[tokio::test]` runtime must not panic and
    // must wait for off-runtime teardown to complete.
    drop(handle);
    assert!(dropped.load(Ordering::SeqCst));
}

#[tokio::test]
async fn blocking_handle_tears_down_off_runtime_when_setup_fails() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    let dropped = Arc::new(AtomicBool::new(false));
    let dropped_ref = Arc::clone(&dropped);

    let task = tokio::spawn(async move {
        let (_url, _handle) = BlockingHandle::spawn(move || {
            let resource = BlockOnResource::start(dropped_ref);
            ("postgres://127.0.0.1:5432/postgres".to_string(), resource)
        })
        .await;
        panic!("simulated database setup failure after container start");
    });

    let err = task.await.expect_err("task should propagate setup panic");
    assert!(err.is_panic());
    assert!(
        dropped.load(Ordering::SeqCst),
        "container resource must still tear down off-runtime on setup unwind"
    );
}
