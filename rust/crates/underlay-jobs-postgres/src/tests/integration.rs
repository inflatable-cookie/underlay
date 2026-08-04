//! Integration test for the Postgres job repository against a real Postgres.
//!
//! `#[ignore]`d by default (needs a database). Run with:
//!
//! ```bash
//! UNDERLAY_TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/postgres \
//!   cargo test -p underlay-jobs-postgres --lib -- --ignored
//! ```
//!
//! This adapter hardcodes the `platform` schema, so per-test schema isolation
//! does not apply. The whole job lifecycle is exercised in a single test that
//! rebuilds the `platform` schema from the crate's own migrations, avoiding any
//! cross-test contention on `platform.job`.
//!
//! **WARNING — destructive:** this test runs `DROP SCHEMA platform CASCADE`
//! against whatever database `UNDERLAY_TEST_DATABASE_URL` points at. Only ever
//! point it at a **throwaway** database (the CI service container, a local
//! scratch container) — never at a database whose `platform` schema you care
//! about.

use serde_json::json;
use sqlx::Executor;

use underlay_jobs::{JobConfig, JobFilters, JobHandlerError, JobStatus, JobStore};
use underlay_testing::TestDb;

use crate::JobRepository;

// 0003 (domain-event notify) is intentionally omitted: it targets an app-owned
// `platform.domain_events` table that this crate does not create and that the
// job lifecycle does not touch.
const MIGRATIONS: &[&str] = &[
    include_str!("../../migrations/0001_create_job_tables.sql"),
    include_str!("../../migrations/0002_add_job_notify.sql"),
    include_str!("../../migrations/0004_add_job_dead_letters.sql"),
];

async fn setup() -> JobRepository {
    let db = TestDb::new().await;
    let pool = db.pool().clone();

    // The migrations create (and hardcode) the `platform` schema; rebuild it
    // from scratch so the run is deterministic. `raw_sql` uses the simple
    // protocol, which the multi-statement + `$$` function bodies require.
    pool.execute(sqlx::raw_sql("DROP SCHEMA IF EXISTS platform CASCADE"))
        .await
        .expect("drop platform schema");
    for migration in MIGRATIONS {
        pool.execute(sqlx::raw_sql(*migration))
            .await
            .expect("run job migration");
    }

    // TestDb is dropped here, but the pool is cloned and kept alive by the
    // returned repository; the container (if any) lives as long as the pool.
    JobRepository::new(pool)
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn job_lifecycle_create_claim_succeed_fail_cancel() {
    let repo = setup().await;
    let config = JobConfig::new();

    // --- create + get ---
    let id = repo
        .create("email.send", json!({ "to": "a@b.c" }), &config)
        .await
        .expect("create");

    let job = repo.get(id).await.expect("get").expect("job exists");
    assert_eq!(job.job_type, "email.send");
    assert_eq!(job.status, JobStatus::Pending);

    // --- claim (fetch_next) transitions to running + increments attempts ---
    let claimed = repo
        .fetch_next(&["email.send".to_string()])
        .await
        .expect("fetch_next")
        .expect("a claimable job");
    assert_eq!(claimed.id, id);
    assert_eq!(claimed.status, JobStatus::Running);

    let running = repo.get(id).await.expect("get").expect("job");
    assert_eq!(running.status, JobStatus::Running);
    assert_eq!(running.attempts, 1);

    // Nothing else is claimable now.
    assert!(repo
        .fetch_next(&["email.send".to_string()])
        .await
        .expect("fetch_next")
        .is_none());

    // --- mark success ---
    repo.mark_success(id).await.expect("mark_success");
    assert_eq!(
        repo.get(id).await.expect("get").expect("job").status,
        JobStatus::Succeeded
    );

    // --- a job that exhausts its single attempt ends terminal (failed) ---
    let fail_id = repo
        .create("email.send", json!({}), &config)
        .await
        .expect("create");
    let fail_job = repo
        .fetch_next(&["email.send".to_string()])
        .await
        .expect("fetch_next")
        .expect("claimable");
    repo.mark_failure(&fail_job, JobHandlerError::new("smtp down"), &config)
        .await
        .expect("mark_failure");
    let failed = repo.get(fail_id).await.expect("get").expect("job");
    assert!(
        matches!(failed.status, JobStatus::Failed),
        "expected Failed, got {:?}",
        failed.status
    );

    // --- cancel a pending job ---
    let cancel_id = repo
        .create("report.build", json!({}), &config)
        .await
        .expect("create");
    repo.cancel(cancel_id).await.expect("cancel");
    assert_eq!(
        repo.get(cancel_id).await.expect("get").expect("job").status,
        JobStatus::Cancelled
    );

    // --- list filters by type and status ---
    let emails = repo
        .list(JobFilters {
            job_type: Some("email.send".to_string()),
            limit: 50,
            ..Default::default()
        })
        .await
        .expect("list");
    assert_eq!(emails.len(), 2);

    let cancelled = repo
        .list(JobFilters {
            status: Some(JobStatus::Cancelled),
            limit: 50,
            ..Default::default()
        })
        .await
        .expect("list");
    assert_eq!(cancelled.len(), 1);
    assert_eq!(cancelled[0].id, cancel_id);
}
