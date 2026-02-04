# 055 - Background Jobs

> **Underlay Crate**: `underlay-jobs` provides a complete background job system with optional PostgreSQL persistence and cron-based scheduling.

This guide covers setting up background job processing, including job handlers, scheduling, and database-backed job queues.

## Overview

The `underlay-jobs` crate provides:

- **Core job types** (always available): `Job`, `JobHandler`, `JobRegistry`, `JobRunner`
- **PostgreSQL persistence** (`postgres` feature): Database-backed job repository with `FOR UPDATE SKIP LOCKED` claiming
- **Cron scheduling** (`scheduler` feature): Recurring task execution via cron expressions

## Features

The crate uses Cargo features to enable optional functionality:

```toml
[dependencies]
underlay-jobs = { path = "../underlay/rust/crates/underlay-jobs" }  # Core only
underlay-jobs = { path = "../underlay/rust/crates/underlay-jobs", features = ["postgres"] }  # With DB
underlay-jobs = { path = "../underlay/rust/crates/underlay-jobs", features = ["full"] }  # Everything
```

| Feature | Description |
|---------|-------------|
| (default) | Core types, handler trait, registry, in-memory runner |
| `postgres` | PostgreSQL job/task repositories, `FOR UPDATE SKIP LOCKED` claiming |
| `scheduler` | Cron-based scheduled tasks (requires `postgres`) |
| `full` | All features enabled |

## Database Setup

### 1. Copy the Migrations

The crate includes SQL schemas. Copy them to your migrations folder:

```bash
# Using underlay-devtools (recommended)
cd your-api && underlay-devtools sync-migrations

# Or manually
cp underlay/rust/crates/underlay-jobs/migrations/0001_create_job_tables.sql \
   your-api/migrations/XXXXXX_create_job_tables.sql
cp underlay/rust/crates/underlay-jobs/migrations/0002_add_job_notify.sql \
   your-api/migrations/XXXXXX_add_job_notify.sql
```

### 2. Run Migrations

```bash
sqlx migrate run
```

### Schema Overview

The migrations create tables and triggers in the `platform` schema:

```
platform.job            - Individual job instances
platform.scheduled_task - Cron-scheduled recurring task definitions
platform.job_history    - Archive of completed/failed jobs
platform.notify_job_inserted() - Trigger function for LISTEN/NOTIFY
```

Key features:
- Jobs use `claimed_at` + `claimed_by` for distributed locking
- `FOR UPDATE SKIP LOCKED` prevents contention when claiming jobs
- Scheduled tasks track `last_scheduled_at` to prevent duplicate runs
- History table enables auditing without bloating the main job queue
- **LISTEN/NOTIFY** trigger for efficient job wake-up (see below)

## Creating Job Handlers

Implement the `JobHandler` trait for each job type:

```rust
use async_trait::async_trait;
use underlay_jobs::{Job, JobHandler, JobHandlerError};

pub struct SendEmailJob {
    mailer: Arc<Mailer>,
}

#[async_trait]
impl JobHandler for SendEmailJob {
    fn job_type(&self) -> &'static str {
        "send_email"
    }

    async fn handle(&self, job: Job) -> Result<(), JobHandlerError> {
        // Extract payload
        let email: EmailPayload = serde_json::from_value(job.payload)
            .map_err(|e| JobHandlerError::permanent(format!("Invalid payload: {e}")))?;

        // Do the work
        self.mailer.send(&email).await
            .map_err(|e| JobHandlerError::new(format!("Send failed: {e}")))?;

        Ok(())
    }
}
```

### Error Handling

`JobHandlerError` distinguishes retryable from permanent failures:

```rust
// Retryable error (will retry based on JobConfig)
Err(JobHandlerError::new("Temporary network error"))

// Permanent error (will not retry)
Err(JobHandlerError::permanent("Invalid email address"))
```

## Job Configuration

Use `JobConfig` to control job behavior:

```rust
use underlay_jobs::{JobConfig, BackoffStrategy};

// Default: single attempt, no timeout
let config = JobConfig::default();

// With retries
let config = JobConfig::with_retries(3);

// Long-running job with custom timeout
let config = JobConfig::long_running()
    .with_timeout(Duration::from_secs(3600));

// Full customization
let config = JobConfig {
    max_attempts: 5,
    timeout_seconds: Some(300),
    backoff: BackoffStrategy::Exponential {
        initial_delay: Duration::from_secs(10),
        multiplier: 2.0,
        max_delay: Duration::from_secs(3600),
    },
    allow_overlap: false,
    priority: 10,  // Higher = more important
    ..Default::default()
};
```

### Backoff Strategies

| Strategy | Description |
|----------|-------------|
| `None` | Retry immediately |
| `Fixed { delay }` | Wait a fixed duration between retries |
| `Exponential { initial_delay, multiplier, max_delay }` | Increasing delays, capped at max |

## Running Jobs

### Setting Up the Runner

```rust
use underlay_jobs::{JobRunner, JobRegistry, JobRepository};

// Create registry and register handlers
let mut registry = JobRegistry::new();
registry.register(SendEmailJob { mailer: mailer.clone() });
registry.register(ProcessPaymentJob { stripe: stripe.clone() });

// Create repository (requires postgres feature)
let job_repo = JobRepository::new(db_pool.clone());

// Create and run
let runner = JobRunner::new(job_repo, registry);

// Process a batch (useful for testing or cron-triggered runs)
let processed = runner.run_batch(100).await?;
```

### Efficient Job Wake-up with LISTEN/NOTIFY (Recommended)

By default, a job worker must poll the database periodically to check for new jobs. Frequent polling (e.g., every 250ms) wastes database resources when idle. Infrequent polling (e.g., every 30s) delays job processing.

**LISTEN/NOTIFY** solves this by having PostgreSQL notify waiting workers immediately when a job is inserted:

```
┌─────────────┐    INSERT job    ┌─────────────┐
│   API       │ ───────────────► │  PostgreSQL │
│  Server     │                  │             │
└─────────────┘                  │  NOTIFY     │
                                 │  trigger    │
                                 └──────┬──────┘
                                        │
                                        │ pg_notify('underlay_job_notify', ...)
                                        ▼
                                 ┌─────────────┐
                                 │   Worker    │ ◄── LISTEN 'underlay_job_notify'
                                 │  (wakes up) │
                                 └─────────────┘
```

**Benefits:**
- **Instant response**: Jobs are processed within milliseconds of being created
- **Near-zero idle load**: No database traffic when there's no work
- **Simple**: Just uses PostgreSQL built-in features, no external services

### Setting Up LISTEN/NOTIFY

1. **Run the migration** (`0002_add_job_notify.sql`) which creates the notify trigger
2. **Create a notifier** and pass it to the runner:

```rust
use underlay_jobs::{JobRunner, JobRunnerConfig, JobRepository, PgJobNotifier};
use std::time::Duration;

// Create the runner
let job_repo = JobRepository::new(pool.clone());
let runner = JobRunner::new(job_repo, registry)
    .with_config(JobRunnerConfig {
        // Fallback poll interval (only used if notifications are missed)
        poll_interval: Duration::from_secs(30),
        ..Default::default()
    });

// Create notifier (establishes a dedicated connection for LISTEN)
let mut notifier = PgJobNotifier::connect(&pool).await?;

// Run with notification support
runner.run_with_notifier(&mut notifier).await?;
```

### How It Works

1. **Worker starts**: Calls `LISTEN 'underlay_job_notify'` on a dedicated connection
2. **Worker idle**: Waits on the listener (no database queries)
3. **Job inserted**: PostgreSQL trigger sends `NOTIFY 'underlay_job_notify'`
4. **Worker wakes**: Receives notification, queries for claimable jobs
5. **Fallback**: If no notification arrives within `poll_interval`, wakes up anyway

The fallback poll is essential because:
- Notifications can be missed during brief connection drops
- Scheduled jobs (with `scheduled_for` in the future) become ready without notifications
- It provides defense against edge cases

**Recommended fallback interval: 30 seconds** (the default). This balances quick recovery from missed notifications with low database load.

### Worker Configuration

```rust
use underlay_jobs::JobRunnerConfig;

let config = JobRunnerConfig {
    // Fallback poll interval (notification mode) or poll frequency (polling mode)
    poll_interval: Duration::from_secs(30),
    batch_size: 0,  // 0 = process all available jobs before waiting
};

let runner = JobRunner::new(job_repo, registry)
    .with_config(config);
```

### Polling Mode (Legacy)

If you can't use LISTEN/NOTIFY (e.g., non-PostgreSQL backend), use polling mode:

```rust
// Polling mode - checks database every poll_interval
runner.run_forever().await?;
```

For polling mode, use a shorter `poll_interval` (250ms-1s) to balance responsiveness with database load.

### LISTEN/NOTIFY Reliability

PostgreSQL LISTEN/NOTIFY is **production-ready** and powers several popular job queues:

- **Oban** (Elixir) - Highly respected, battle-tested in high-throughput systems
- **Graphile Worker** (Node.js) - Known for reliability
- **pgboss** (Node.js) - Widely used

Key reliability characteristics:

| Property | Behavior |
|----------|----------|
| **Transactional** | NOTIFY only fires when the INSERT transaction commits |
| **Connection-scoped** | If connection drops, you stop receiving notifications |
| **Not persisted** | Notifications sent while disconnected are lost |
| **Payload limit** | 8000 bytes per notification (plenty for job_type) |

The fallback poll handles all edge cases - you get instant response in the common case plus guaranteed correctness.

## Enqueueing Jobs

```rust
use underlay_jobs::{JobRepository, JobConfig};
use serde_json::json;

let job_repo = JobRepository::new(pool);

// Create a job
let job_id = job_repo.create(
    "send_email",
    json!({
        "to": "user@example.com",
        "subject": "Welcome!",
        "template": "welcome"
    }),
    &JobConfig::with_retries(3),
).await?;

// Schedule for later
let job_id = job_repo.create_scheduled(
    "send_reminder",
    json!({ "user_id": user_id }),
    &JobConfig::default(),
    Utc::now() + Duration::hours(24),
).await?;
```

## Scheduled Tasks (Cron)

For recurring tasks, use the scheduler (requires `scheduler` + `postgres` features):

### Defining Scheduled Tasks

```rust
use underlay_jobs::{Scheduler, ScheduledTaskDefinition, JobRepository, ScheduledTaskRepository};

let job_repo = JobRepository::new(pool.clone());
let task_repo = ScheduledTaskRepository::new(pool.clone());
let scheduler = Scheduler::new(job_repo, task_repo);

// Register task definitions
scheduler.register_tasks(&[
    ScheduledTaskDefinition {
        name: "daily_cleanup",
        job_type: "cleanup_expired_sessions",
        schedule: "0 0 3 * * *",  // 3 AM daily
        payload: json!({}),
        max_attempts: 1,
        timeout_seconds: Some(3600),
        allow_overlap: false,
        priority: 0,
    },
    ScheduledTaskDefinition {
        name: "hourly_stats",
        job_type: "aggregate_stats",
        schedule: "0 0 * * * *",  // Every hour
        payload: json!({}),
        max_attempts: 3,
        timeout_seconds: Some(300),
        allow_overlap: false,
        priority: 5,
    },
]).await?;
```

### Running the Scheduler

The scheduler checks for due tasks and creates jobs:

```rust
// Run one scheduling tick (check all tasks, create due jobs)
scheduler.tick().await?;

// Typically run on a timer in your main loop
loop {
    scheduler.tick().await?;
    tokio::time::sleep(Duration::from_secs(60)).await;
}
```

### Cron Expression Format

Standard 6-field cron format: `seconds minutes hours day-of-month month day-of-week`

| Expression | Description |
|------------|-------------|
| `0 0 * * * *` | Every hour on the hour |
| `0 0 3 * * *` | 3:00 AM daily |
| `0 */15 * * * *` | Every 15 minutes |
| `0 0 0 * * MON` | Midnight every Monday |
| `0 0 9-17 * * MON-FRI` | Every hour 9 AM - 5 PM, weekdays |

## Production Patterns

### Separate Worker Process

For production, run the job worker as a separate process:

```rust
// bin/worker.rs
use underlay_jobs::{
    JobRunner, JobRunnerConfig, JobRegistry, JobRepository,
    PgJobNotifier, Scheduler, ScheduledTaskRepository,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = create_db_pool().await?;

    // Register job handlers
    let mut registry = JobRegistry::new();
    registry.register(SendEmailJob::new(&config));
    registry.register(ProcessPaymentJob::new(&config));

    // Create runner with LISTEN/NOTIFY support
    let job_repo = JobRepository::new(pool.clone());
    let runner = JobRunner::new(job_repo, registry)
        .with_config(JobRunnerConfig {
            poll_interval: Duration::from_secs(30),  // Fallback interval
            ..Default::default()
        });

    // Create notifier for efficient wake-up
    let mut notifier = PgJobNotifier::connect(&pool).await?;

    // Set up scheduler for recurring tasks
    let task_repo = ScheduledTaskRepository::new(pool.clone());
    let scheduler = Scheduler::new(
        JobRepository::new(pool.clone()),
        task_repo,
    );
    scheduler.register_tasks(&scheduled_tasks()).await?;

    // Run scheduler tick every minute
    let scheduler_handle = tokio::spawn(async move {
        loop {
            if let Err(e) = scheduler.tick().await {
                tracing::error!("Scheduler tick failed: {e}");
            }
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });

    // Run job worker with LISTEN/NOTIFY
    runner.run_with_notifier(&mut notifier).await?;

    Ok(())
}
```

### Graceful Shutdown

```rust
let runner = JobRunner::new(job_repo, registry);
let mut notifier = PgJobNotifier::connect(&pool).await?;

tokio::select! {
    result = runner.run_with_notifier(&mut notifier) => {
        tracing::info!("Runner stopped: {result:?}");
    }
    _ = shutdown_signal() => {
        tracing::info!("Shutdown requested");
    }
}
```

### Monitoring

Track job metrics in your observability stack:

```rust
// The runner emits tracing events at various levels
// Set RUST_LOG=underlay_jobs=debug for detailed logging

// Key events:
// - INFO: Job completed successfully, Job failed permanently
// - WARN: Job failed (may retry)
// - DEBUG: Job processing started, scheduling decisions
```

## Testing

### Unit Testing Handlers

```rust
#[tokio::test]
async fn test_send_email_handler() {
    let mailer = MockMailer::new();
    let handler = SendEmailJob { mailer };

    let job = Job {
        id: Uuid::new_v7(),
        job_type: "send_email".to_string(),
        payload: json!({
            "to": "test@example.com",
            "subject": "Test"
        }),
        ..Default::default()
    };

    handler.handle(job).await.expect("should succeed");
}
```

### Integration Testing with In-Memory Store

The crate includes test utilities for in-memory job stores:

```rust
#[tokio::test]
async fn test_runner_processes_jobs() {
    let store = Arc::new(MemoryJobStore::default());

    // Enqueue a job
    store.enqueue("test_job", json!({"value": 42}));

    let mut registry = JobRegistry::new();
    registry.register(TestHandler);

    let runner = JobRunner::new(store.clone(), registry);
    let processed = runner.run_batch(10).await.unwrap();

    assert_eq!(processed, 1);
    assert!(store.is_empty());
}
```

## Migration from Custom Job Systems

If migrating from a custom job system:

1. **Keep existing tables** during migration period
2. **Create adapters** that bridge old → new format
3. **Run both systems** in parallel with feature flags
4. **Migrate handlers** one job type at a time
5. **Drain old queue** before removing old tables

## Standard Maintenance Tasks

The `underlay-jobs` crate includes pre-built maintenance tasks for common platform cleanup operations. These are available when using the `postgres` feature.

### Available Tasks

Import and register the tasks you need:

```rust
use underlay_jobs::tasks::{
    // Auth cleanup
    PurgeExpiredSessionsJob,     // Remove expired sessions
    PurgeAuthStatesJob,          // Remove expired auth states
    PurgeLoginAttemptsJob,       // Remove old login attempts
    PurgeRateLimitEntriesJob,    // Remove old rate limit entries
    PurgeEmailTotpCodesJob,      // Remove expired/used TOTP codes
    PurgeVerificationSessionsJob, // Remove expired/used verification sessions

    // Job system maintenance
    ArchiveCompletedJobsJob,     // Move old jobs to history table
    PurgeJobHistoryJob,          // Remove old job history
    RecoverAbandonedJobsJob,     // Reset stalled jobs

    // Log cleanup
    PurgeErrorLogsJob,           // Remove old error logs
    PurgeCapturedEmailsJob,      // Remove old captured emails (dev/test)
};

let mut registry = JobRegistry::new();
registry.register(PurgeExpiredSessionsJob::new(pool.clone()));
registry.register(ArchiveCompletedJobsJob::new(pool.clone()));
// ... register other tasks as needed
```

### Task Configuration

Most tasks support builder methods for customization:

```rust
// Custom retention period
registry.register(
    PurgeLoginAttemptsJob::new(pool.clone())
        .with_retention_days(60)  // Default: 30
);

registry.register(
    PurgeJobHistoryJob::new(pool.clone())
        .with_retention_days(180)  // Default: 90
);

registry.register(
    RecoverAbandonedJobsJob::new(pool.clone())
        .with_stall_timeout_seconds(600)  // Default: 300
);
```

### Recommended Schedules

| Task | Job Type | Recommended Schedule | Notes |
|------|----------|---------------------|-------|
| PurgeExpiredSessionsJob | `purge_expired_sessions` | Every 15 min | Keeps session table clean |
| PurgeAuthStatesJob | `purge_auth_states` | Hourly | Short-lived auth flow entries |
| PurgeLoginAttemptsJob | `purge_login_attempts` | Daily 3 AM | 30-day retention |
| PurgeRateLimitEntriesJob | `purge_rate_limit_entries` | Hourly | 24-hour entries |
| PurgeEmailTotpCodesJob | `purge_email_totp_codes` | Hourly | Expired/used codes |
| PurgeVerificationSessionsJob | `purge_verification_sessions` | Hourly | Expired/used sessions |
| ArchiveCompletedJobsJob | `archive_completed_jobs` | Daily 5 AM | 7-day retention before archive |
| PurgeJobHistoryJob | `purge_job_history` | Weekly Sunday 5 AM | 90-day retention |
| RecoverAbandonedJobsJob | `recover_abandoned_jobs` | Every 5 min | 5-min stall timeout |
| PurgeErrorLogsJob | `purge_error_logs` | Daily 4 AM | 90-day retention |
| PurgeCapturedEmailsJob | `purge_captured_emails` | Daily 4:30 AM | 7-day retention |

### Example: Full Registration

```rust
pub fn scheduled_task_definitions() -> Vec<ScheduledTaskDefinition> {
    vec![
        ScheduledTaskDefinition {
            name: "purge_expired_sessions",
            job_type: "purge_expired_sessions",
            schedule: "0 */15 * * * *",
            payload: serde_json::json!({}),
            config: JobConfig::maintenance(),
        },
        ScheduledTaskDefinition {
            name: "archive_completed_jobs",
            job_type: "archive_completed_jobs",
            schedule: "0 0 5 * * *",
            payload: serde_json::json!({}),
            config: JobConfig::maintenance(),
        },
        ScheduledTaskDefinition {
            name: "recover_abandoned_jobs",
            job_type: "recover_abandoned_jobs",
            schedule: "0 */5 * * * *",
            payload: serde_json::json!({}),
            config: JobConfig::maintenance(),
        },
        // ... add other tasks as needed
    ]
}
```

These tasks use standard Underlay table names (`auth.sessions`, `platform.job`, etc.) and work with any Underlay application that has run the standard migrations.

## Related Documentation

- [050 - Database & Migrations](./050-database.md) - Database setup
- [120 - Configuration](./120-configuration.md) - Environment configuration
- [140 - Local Development](./140-local-development.md) - Running workers locally
