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

### 1. Copy the Migration

The crate includes a SQL schema. Copy it to your migrations folder:

```bash
# Using underlay-devtools (recommended)
cd your-api && underlay-devtools sync-migrations

# Or manually
cp underlay/rust/crates/underlay-jobs/migrations/0001_create_job_tables.sql \
   your-api/crates/db/migrations/XXXXXX_create_job_tables.sql
```

### 2. Run Migrations

```bash
cd your-api/crates/db && sqlx migrate run
```

### Schema Overview

The migration creates three tables in the `platform` schema:

```
platform.job           - Individual job instances
platform.scheduled_task - Cron-scheduled recurring task definitions
platform.job_history   - Archive of completed/failed jobs
```

Key features:
- Jobs use `claimed_at` + `claimed_by` for distributed locking
- `FOR UPDATE SKIP LOCKED` prevents contention when claiming jobs
- Scheduled tasks track `last_scheduled_at` to prevent duplicate runs
- History table enables auditing without bloating the main job queue

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

// Process forever (typical for worker process)
runner.run_forever().await?;

// Or process a batch (useful for testing or cron-triggered runs)
let processed = runner.run_batch(100).await?;
```

### Worker Configuration

```rust
use underlay_jobs::JobRunnerConfig;

let config = JobRunnerConfig {
    poll_interval: Duration::from_millis(250),  // How often to check for jobs
    batch_size: 0,  // 0 = unlimited per poll cycle
};

let runner = JobRunner::new(job_repo, registry)
    .with_config(config);
```

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
#[tokio::main]
async fn main() -> Result<()> {
    let pool = create_db_pool().await?;

    let mut registry = JobRegistry::new();
    registry.register(SendEmailJob::new(&config));
    registry.register(ProcessPaymentJob::new(&config));

    let job_repo = JobRepository::new(pool.clone());
    let runner = JobRunner::new(job_repo, registry);

    // Run the scheduler alongside the worker
    let task_repo = ScheduledTaskRepository::new(pool.clone());
    let scheduler = Scheduler::new(
        JobRepository::new(pool.clone()),
        task_repo,
    );

    // Register scheduled tasks
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

    // Run job worker
    runner.run_forever().await?;

    Ok(())
}
```

### Graceful Shutdown

```rust
let runner = JobRunner::new(job_repo, registry);

tokio::select! {
    result = runner.run_forever() => {
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

## Related Documentation

- [050 - Database & Migrations](./050-database.md) - Database setup
- [120 - Configuration](./120-configuration.md) - Environment configuration
- [140 - Local Development](./140-local-development.md) - Running workers locally
