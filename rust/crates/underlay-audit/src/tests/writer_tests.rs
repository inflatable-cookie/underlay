use super::{AUDIT_WRITE_SEMAPHORE, MAX_CONCURRENT_ASYNC_AUDIT_WRITES};
use crate::{append_audit_log_to_table_async, AuditAction, AuditEntry, AuditTable};

#[test]
fn valid_table_names() {
    assert!(AuditTable::parse("platform.audit_log").is_ok());
    assert!(AuditTable::parse("audit_log").is_ok());
}

#[test]
fn invalid_table_names() {
    assert!(AuditTable::parse("audit; DROP TABLE users").is_err());
    assert!(AuditTable::parse("audit-log").is_err());
}

#[tokio::test]
async fn async_writer_drops_when_semaphore_is_saturated() {
    // Drain every permit so the async writer must drop rather than spawn.
    let permits = AUDIT_WRITE_SEMAPHORE
        .try_acquire_many(MAX_CONCURRENT_ASYNC_AUDIT_WRITES as u32)
        .expect("all permits should be available");
    assert_eq!(AUDIT_WRITE_SEMAPHORE.available_permits(), 0);

    let pool = sqlx::PgPool::connect_lazy("postgres://underlay:underlay@localhost/underlay")
        .expect("lazy pool should construct");
    let table = AuditTable::parse("platform.audit_log").expect("valid table");
    let entry = AuditEntry::new(None, AuditAction::Create, "item", uuid::Uuid::now_v7());

    append_audit_log_to_table_async(pool, table, entry);
    assert_eq!(AUDIT_WRITE_SEMAPHORE.available_permits(), 0);

    drop(permits);
    assert_eq!(
        AUDIT_WRITE_SEMAPHORE.available_permits(),
        MAX_CONCURRENT_ASYNC_AUDIT_WRITES
    );
}
