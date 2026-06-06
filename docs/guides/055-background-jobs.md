# 055 - Background Jobs

> **Underlay Crates**: `underlay-jobs` provides the job contract and runner.
> `underlay-jobs-postgres` provides PostgreSQL persistence, LISTEN/NOTIFY,
> outbox processing, scheduled task runtime, and maintenance tasks.

This guide covers setting up background job processing, including job handlers, scheduling, and database-backed job queues.

## Overview

The `underlay-jobs` crate provides:

- **Core job types** (always available): `Job`, `JobHandler`, `JobRegistry`, `JobRunner`
- **Store contracts**: `JobStore`, dead-letter contracts, event hooks, and scheduler config

The `underlay-jobs-postgres` crate provides:

- **PostgreSQL persistence**: Database-backed job repository with `FOR UPDATE SKIP LOCKED` claiming
- **Cron scheduling runtime**: Recurring task execution via cron expressions
- **Outbox processing**: Durable domain event processing with LISTEN/NOTIFY
- **Maintenance tasks**: Standard cleanup and recovery handlers

## Dependencies

Use the core crate for contracts and the adapter crate when the app needs
PostgreSQL-backed storage/runtime:

```toml
[dependencies]
underlay-jobs = { path = "../underlay/rust/crates/underlay-jobs" }
underlay-jobs-postgres = { path = "../underlay/rust/crates/underlay-jobs-postgres" }
```

`underlay-jobs` no longer exposes `postgres`, `scheduler`, `outbox`, or `full`
feature flags.

## Database Setup

### 1. Copy the Migrations

The crate includes SQL schemas. Copy them to your migrations folder:

```bash
# Using underlay-devtools (recommended)
cd your-api && underlay-devtools sync-migrations

# Or manually
cp underlay/rust/crates/underlay-jobs-postgres/migrations/0001_create_job_tables.sql \
   your-api/migrations/XXXXXX_create_job_tables.sql
cp underlay/rust/crates/underlay-jobs-postgres/migrations/0002_add_job_notify.sql \
   your-api/migrations/XXXXXX_add_job_notify.sql
cp underlay/rust/crates/underlay-jobs-postgres/migrations/0004_add_job_dead_letters.sql \
   your-api/migrations/XXXXXX_add_job_dead_letters.sql
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
platform.job_dead_letter - Failed jobs available for inspection and requeue
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

// With retries and deterministic jitter spread
let config = JobConfig::with_retries_and_jitter(3);

// Long-running job with custom timeout
let config = JobConfig::long_running()
    .with_timeout(Duration::from_secs(3600));

// Full customization
let config = JobConfig {
    max_attempts: 5,
    timeout_seconds: Some(300),
    backoff: BackoffStrategy::Exponential {
        base: Duration::from_secs(10),
        max: Duration::from_secs(3600),
        jitter: None,
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
| `Exponential { base, max, jitter }` | Increasing delays, capped at max, with optional deterministic spread |

Retry jitter is opt-in. Existing `with_retries()` and `long_running_with_retries()` calls keep
their previous timing; use `with_retries_and_jitter()` or
`with_jittered_exponential_backoff()` when you want retry spread.

## Running Jobs

### Setting Up the Runner

```rust
use underlay_jobs::{JobRegistry, JobRunner};
use underlay_jobs_postgres::JobRepository;

// Create registry and register handlers
let mut registry = JobRegistry::new();
registry.register(SendEmailJob { mailer: mailer.clone() });
registry.register(ProcessPaymentJob { stripe: stripe.clone() });

// Create repository
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
use std::time::Duration;
use underlay_jobs::{JobRunner, JobRunnerConfig};
use underlay_jobs_postgres::{JobRepository, PgJobNotifier, PostgresJobRunnerExt};

// Create the runner
let job_repo = JobRepository::new(pool.clone());
let runner = JobRunner::new(job_repo, registry)
    // Fallback poll interval, only used if notifications are missed.
    .with_config(JobRunnerConfig::default().with_poll_interval(Duration::from_secs(30)));

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

let config = JobRunnerConfig::default()
    // Fallback poll interval (notification mode) or poll frequency (polling mode)
    .with_poll_interval(Duration::from_secs(30))
    // 0 = process all available jobs before waiting
    .with_batch_size(0);

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
use serde_json::json;
use underlay_jobs::JobConfig;
use underlay_jobs_postgres::JobRepository;

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

## Dead Letters

When a job fails permanently or exhausts its retry budget, `underlay-jobs` now copies the final
state into `platform.job_dead_letter`. This keeps failed-job inspection and manual requeue out of
the hot queue path.

```rust
use chrono::{Duration as ChronoDuration, Utc};
use underlay_jobs::DeadLetterFilters;
use underlay_jobs_postgres::PgDeadLetterRepository;

let dead_letters = PgDeadLetterRepository::new(pool.clone())
    .list(DeadLetterFilters::new().with_job_type("send_email"))
    .await?;

let retried_job_id = PgDeadLetterRepository::new(pool.clone())
    .retry(dead_letters[0].id)
    .await?;

PgDeadLetterRepository::new(pool.clone())
    .archive_old(Utc::now() - ChronoDuration::days(30))
    .await?;
```

Operational expectations:

- copy and run `0004_add_job_dead_letters.sql` before deploying this batch
- retrying a dead letter creates a fresh `platform.job` row and records the new job id on the dead-letter entry
- archive or purge old dead letters on a retention schedule that matches your incident/debugging needs

## Lifecycle Events

`underlay-jobs` now exposes lightweight synchronous event hooks so apps can attach metrics,
structured logs, tracing, or dashboards without framework lock-in.

```rust
use std::sync::Arc;
use underlay_jobs::{JobEvent, JobEventSink, JobRunner};
use underlay_jobs_postgres::JobRepository;

#[derive(Debug)]
struct MetricsSink;

impl JobEventSink for MetricsSink {
    fn on_event(&self, event: JobEvent) {
        match event {
            JobEvent::Failed { job_type, will_retry, .. } => {
                tracing::warn!(%job_type, will_retry, "job failed");
            }
            JobEvent::DeadLettered { job_id, dead_letter_id, .. } => {
                tracing::error!(%job_id, %dead_letter_id, "job moved to dead letter");
            }
            _ => {}
        }
    }
}

let sink = Arc::new(MetricsSink);
let job_repo = JobRepository::new(pool.clone()).with_event_sink(sink.clone());
let runner = JobRunner::new(job_repo, registry).with_event_sink(sink);
```

Event coverage in this batch:

- `Enqueued`
- `Claimed`
- `Started`
- `Completed`
- `Failed`
- `DeadLettered`

## Scheduled Tasks (Cron)

For recurring tasks, use the PostgreSQL scheduler runtime from
`underlay-jobs-postgres`:

### Defining Scheduled Tasks

```rust
use underlay_jobs::ScheduledTaskDefinition;
use underlay_jobs_postgres::{JobRepository, ScheduledTaskRepository, Scheduler};

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
        .with_config(JobRunnerConfig::default().with_poll_interval(Duration::from_secs(30)));

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
// Built-in tracing remains useful for operator logs.
// Event sinks let apps attach metrics/tracing/dashboards without changing the crate.
// Set RUST_LOG=underlay_jobs=debug for detailed queue diagnostics.
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

The `underlay-jobs-postgres` crate includes pre-built maintenance tasks for
common platform cleanup operations.

### Available Tasks

Import and register the tasks you need:

```rust
use underlay_jobs_postgres::tasks::{
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

## Outbox Pattern for Domain Events

The `underlay-jobs-postgres` outbox module provides reliable domain event
processing using the outbox pattern. This ensures events are delivered even if
downstream systems are temporarily unavailable.

### How It Works

1. **Write**: Application writes domain events to `platform.domain_events` within the same transaction as business data
2. **Process**: Outbox processor claims unprocessed events and calls your handler
3. **Mark**: Successfully processed events are marked with `processed_at` timestamp
4. **Retry**: Failed events remain unprocessed for automatic retry

### Add The Adapter

```toml
[dependencies]
underlay-jobs-postgres = { path = "../underlay/rust/crates/underlay-jobs-postgres" }
```

### Database Setup

Run the domain events migration from `underlay-events`:

```bash
# Using underlay-devtools
underlay-devtools sync-migrations

# The migration creates platform.domain_events table
# and the notify trigger for efficient wake-up
```

### Running the Outbox Processor

```rust
use std::time::Duration;
use underlay_jobs_postgres::outbox::{OutboxConfig, OutboxNotifier, OutboxProcessor};

// Configure the processor
let config = OutboxConfig::default()
    .with_batch_size(50)
    .with_fallback_interval(Duration::from_secs(60));

let processor = OutboxProcessor::new(config);

// Create notifier for LISTEN/NOTIFY
let mut notifier = OutboxNotifier::connect(&pool).await?;

// Run with your event handler
processor.run_with_notifier(&pool, &mut notifier, |event| async move {
    match event.event_type.as_str() {
        "user.created" => {
            // Send welcome email, update analytics, etc.
            println!("New user: {:?}", event.payload);
        }
        "order.placed" => {
            // Notify warehouse, update inventory, etc.
            println!("New order: {:?}", event.payload);
        }
        _ => {
            // Unknown event types are logged but marked processed
            tracing::warn!(event_type = %event.event_type, "Unknown event type");
        }
    }
    Ok(())
}).await?;
```

### Publishing Events

Events are typically published by your domain layer within a transaction:

```rust
use underlay_events::NewDomainEvent;
use serde_json::json;

// Within a transaction that also saves business data
let event = NewDomainEvent::now(
    "user.created",
    json!({
        "user_id": user.id,
        "email": user.email,
        "name": user.display_name,
    }),
);

sqlx::query(
    r#"
    INSERT INTO platform.domain_events (id, event_type, payload, occurred_at)
    VALUES ($1, $2, $3, $4)
    "#,
)
.bind(uuid::Uuid::now_v7())
.bind(&event.event_type)
.bind(&event.payload)
.bind(event.occurred_at)
.execute(&mut *tx)
.await?;

// Both user creation and event are committed together
tx.commit().await?;
```

### OutboxConfig Options

| Option | Default | Description |
|--------|---------|-------------|
| `batch_size` | 100 | Max events processed before yielding |
| `fallback_interval` | 30s | Poll interval when no notifications |

### Error Handling

If your handler returns an error, the event remains unprocessed:

```rust
processor.run_with_notifier(&pool, &mut notifier, |event| async move {
    // This event will be retried
    if some_condition_fails() {
        return Err("Processing failed".into());
    }
    Ok(())
}).await?;
```

Events are retried on the next processing cycle. For events that repeatedly fail, consider adding a retry count column and dead-letter logic in your handler.

### Polling Mode

If LISTEN/NOTIFY isn't available, use polling mode:

```rust
processor.run_polling(&pool, |event| async move {
    // Handle event
    Ok(())
}).await?;
```

## Related Documentation

- [050 - Database & Migrations](./050-database.md) - Database setup
- [120 - Configuration](./120-configuration.md) - Environment configuration
- [140 - Local Development](./140-local-development.md) - Running workers locally
