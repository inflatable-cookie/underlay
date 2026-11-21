//! Outbox pattern implementation for reliable domain event processing.
//!
//! The outbox pattern ensures reliable event delivery by:
//! 1. Writing events to a database table within the same transaction as business data
//! 2. Processing events asynchronously with guaranteed delivery
//! 3. Using PostgreSQL LISTEN/NOTIFY for efficient wake-up
//!
//! # Usage
//!
//! ```ignore
//! use underlay_jobs_postgres::outbox::{OutboxProcessor, OutboxNotifier, OutboxConfig};
//!
//! // Create processor with custom config
//! let config = OutboxConfig::default()
//!     .with_batch_size(50)
//!     .with_fallback_interval(Duration::from_secs(60));
//! let processor = OutboxProcessor::new(config);
//!
//! // Create notifier for LISTEN/NOTIFY
//! let mut notifier = OutboxNotifier::connect(&pool).await?;
//!
//! // Run with efficient wake-up
//! processor.run_with_notifier(&pool, &mut notifier, |event| async move {
//!     // Handle the event
//!     println!("Processing event: {}", event.event_type);
//!     Ok(())
//! }).await?;
//! ```

use std::collections::HashMap;
use std::future::{self, Future};
use std::pin::{pin, Pin};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use serde_json::Value;
use sqlx::postgres::PgListener;
use sqlx::{PgPool, Row};
use tokio::time::Instant;
use tracing::{debug, error, info, warn};

/// The PostgreSQL NOTIFY channel for domain event notifications.
pub const DOMAIN_EVENT_NOTIFY_CHANNEL: &str = "underlay_domain_event_notify";

/// SQL schema for the domain event notify trigger.
///
/// Applications should use `underlay-devtools sync-migrations` to copy this
/// to their migrations folder, or include it directly.
pub const DOMAIN_EVENT_NOTIFY_SQL: &str =
    include_str!("../migrations/0003_add_domain_event_notify.sql");

/// Maximum number of failing event IDs tracked in memory for poison-pill backoff.
const MAX_TRACKED_FAILURES: usize = 1024;

/// Configuration for the outbox processor.
#[derive(Debug, Clone)]
pub struct OutboxConfig {
    /// Maximum number of events to process per batch.
    batch_size: usize,
    /// Fallback poll interval when using LISTEN/NOTIFY.
    /// The processor will wake up at least this often even without notifications.
    fallback_interval: Duration,
    /// Maximum time allowed for a handler while its event is claimed.
    handler_timeout: Duration,
    /// Initial per-event backoff after a handler failure so poison-pill events
    /// do not starve newer events.
    failure_backoff: Duration,
    /// Upper bound for exponential per-event failure backoff.
    max_failure_backoff: Duration,
}

impl Default for OutboxConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            fallback_interval: Duration::from_secs(30),
            handler_timeout: Duration::from_secs(30),
            failure_backoff: Duration::from_secs(5),
            max_failure_backoff: Duration::from_secs(300),
        }
    }
}

impl OutboxConfig {
    pub fn batch_size(&self) -> usize {
        self.batch_size
    }

    pub fn fallback_interval(&self) -> Duration {
        self.fallback_interval
    }

    /// Return the per-event handler timeout.
    pub fn handler_timeout(&self) -> Duration {
        self.handler_timeout
    }

    /// Return the initial per-event failure backoff.
    pub fn failure_backoff(&self) -> Duration {
        self.failure_backoff
    }

    /// Return the maximum per-event failure backoff.
    pub fn max_failure_backoff(&self) -> Duration {
        self.max_failure_backoff
    }

    /// Create a new config with the specified batch size.
    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    /// Set the fallback poll interval.
    pub fn with_fallback_interval(mut self, interval: Duration) -> Self {
        self.fallback_interval = interval;
        self
    }

    /// Set the per-event handler timeout before the event becomes retryable.
    pub fn with_handler_timeout(mut self, timeout: Duration) -> Self {
        self.handler_timeout = timeout;
        self
    }

    /// Set the initial per-event failure backoff.
    pub fn with_failure_backoff(mut self, backoff: Duration) -> Self {
        self.failure_backoff = backoff;
        self
    }

    /// Set the maximum per-event failure backoff.
    pub fn with_max_failure_backoff(mut self, max_backoff: Duration) -> Self {
        self.max_failure_backoff = max_backoff;
        self
    }
}

/// A domain event from the outbox.
#[derive(Debug, Clone)]
pub struct OutboxEvent {
    /// Unique event ID.
    pub id: uuid::Uuid,
    /// Event type (e.g., "user.created", "order.placed").
    pub event_type: String,
    /// Event payload as JSON.
    pub payload: Value,
    /// When the event occurred.
    pub occurred_at: chrono::DateTime<Utc>,
}

/// Listens for domain event notifications via PostgreSQL LISTEN/NOTIFY.
pub struct OutboxNotifier {
    listener: PgListener,
}

impl OutboxNotifier {
    /// Connect and start listening for domain event notifications.
    pub async fn connect(pool: &PgPool) -> Result<Self, sqlx::Error> {
        let mut listener = PgListener::connect_with(pool).await?;
        listener.listen(DOMAIN_EVENT_NOTIFY_CHANNEL).await?;
        debug!(
            channel = DOMAIN_EVENT_NOTIFY_CHANNEL,
            "Outbox notifier listening"
        );
        Ok(Self { listener })
    }

    /// Wait for a notification or timeout.
    ///
    /// Returns `Some(event_type)` if a notification was received,
    /// or `None` if the timeout elapsed.
    pub async fn wait(&mut self, timeout: Duration) -> Result<Option<String>, sqlx::Error> {
        match tokio::time::timeout(timeout, self.listener.recv()).await {
            Ok(Ok(notification)) => {
                debug!(
                    channel = %notification.channel(),
                    payload = %notification.payload(),
                    "Received domain event notification"
                );
                Ok(Some(notification.payload().to_string()))
            }
            Ok(Err(e)) => Err(e),
            Err(_) => Ok(None), // Timeout
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct FailedEventState {
    attempts: u32,
    retry_after: Instant,
}

#[derive(Debug, Default)]
struct FailureTracker {
    entries: HashMap<uuid::Uuid, FailedEventState>,
}

impl FailureTracker {
    fn active_skip_ids(&self, now: Instant) -> Vec<uuid::Uuid> {
        self.entries
            .iter()
            .filter_map(|(&id, state)| (state.retry_after > now).then_some(id))
            .collect()
    }

    fn record_failure(
        &mut self,
        id: uuid::Uuid,
        now: Instant,
        base: Duration,
        max_backoff: Duration,
    ) -> (u32, Duration) {
        let attempts = self
            .entries
            .get(&id)
            .map_or(1, |state| state.attempts.saturating_add(1));

        let max_backoff = max_backoff.max(base);
        let shift = attempts.saturating_sub(1).min(16);
        let factor = 1u32 << shift;
        let delay = base.saturating_mul(factor).min(max_backoff);

        if !self.entries.contains_key(&id) && self.entries.len() >= MAX_TRACKED_FAILURES {
            self.evict_one(now);
        }

        self.entries.insert(
            id,
            FailedEventState {
                attempts,
                retry_after: now + delay,
            },
        );
        (attempts, delay)
    }

    fn record_success(&mut self, id: uuid::Uuid) {
        self.entries.remove(&id);
    }

    fn evict_one(&mut self, now: Instant) {
        if let Some(expired_id) = self
            .entries
            .iter()
            .find_map(|(&id, state)| (state.retry_after <= now).then_some(id))
        {
            self.entries.remove(&expired_id);
            return;
        }
        if let Some(oldest_id) = self
            .entries
            .iter()
            .min_by_key(|(_, state)| state.retry_after)
            .map(|(&id, _)| id)
        {
            self.entries.remove(&oldest_id);
        }
    }
}

struct BatchReport {
    processed: usize,
    exhausted: bool,
    shutdown_requested: bool,
}

enum ProcessOneOutcome {
    Empty,
    Succeeded(uuid::Uuid),
    Failed {
        event_id: uuid::Uuid,
        event_type: String,
        error: String,
    },
}

trait OutboxBackend: Sync {
    type Error;

    fn process_next<'a, F, Fut>(
        &'a self,
        skip_ids: &'a [uuid::Uuid],
        timeout: Duration,
        handler: &'a F,
    ) -> impl Future<Output = Result<ProcessOneOutcome, Self::Error>> + Send + 'a
    where
        F: Fn(OutboxEvent) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'a;
}

impl OutboxBackend for PgPool {
    type Error = sqlx::Error;

    async fn process_next<'a, F, Fut>(
        &'a self,
        skip_ids: &'a [uuid::Uuid],
        timeout: Duration,
        handler: &'a F,
    ) -> Result<ProcessOneOutcome, Self::Error>
    where
        F: Fn(OutboxEvent) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'a,
    {
        let mut tx = self.begin().await?;

        // Claim the next unprocessed event that is not currently skipped/backed off.
        let row = sqlx::query(
            r#"
            SELECT id, event_type, payload, occurred_at
            FROM platform.domain_events
            WHERE processed_at IS NULL
              AND NOT (id = ANY($1))
            ORDER BY occurred_at ASC
            LIMIT 1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(skip_ids)
        .fetch_optional(&mut *tx)
        .await?;

        let Some(row) = row else {
            return Ok(ProcessOneOutcome::Empty);
        };

        let event = OutboxEvent {
            id: row.get("id"),
            event_type: row.get("event_type"),
            payload: row.get("payload"),
            occurred_at: row.get("occurred_at"),
        };

        debug!(
            event_id = %event.id,
            event_type = %event.event_type,
            "Processing outbox event"
        );

        let event_id = event.id;
        let event_type = event.event_type.clone();
        let failure = match tokio::time::timeout(timeout, async { handler(event).await }).await {
            Ok(Ok(())) => {
                // Mark as processed
                sqlx::query(
                    r#"
                    UPDATE platform.domain_events
                    SET processed_at = $1
                    WHERE id = $2
                    "#,
                )
                .bind(Utc::now())
                .bind(event_id)
                .execute(&mut *tx)
                .await?;

                tx.commit().await?;
                info!(
                    %event_id,
                    %event_type,
                    "Outbox event processed successfully"
                );
                return Ok(ProcessOneOutcome::Succeeded(event_id));
            }
            Ok(Err(error)) => error.to_string(),
            Err(_) => format!("handler timed out after {timeout:?}"),
        };

        tx.rollback().await?;
        Ok(ProcessOneOutcome::Failed {
            event_id,
            event_type,
            error: failure,
        })
    }
}

#[async_trait]
trait OutboxWakeup: Send {
    type Error: std::fmt::Display;

    async fn wait_notification(&mut self, timeout: Duration)
        -> Result<Option<String>, Self::Error>;
}

#[async_trait]
impl OutboxWakeup for OutboxNotifier {
    type Error = sqlx::Error;

    async fn wait_notification(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<String>, Self::Error> {
        self.wait(timeout).await
    }
}

/// Processes domain events from the outbox table.
///
/// The processor claims events using `FOR UPDATE SKIP LOCKED` for
/// safe concurrent processing, and marks them as processed after
/// the handler completes successfully.
///
/// The outbox schema has no persistent claim fields. Each in-flight event
/// holds one row lock and pool connection through its handler, with a
/// 30-second default handler deadline. Reserve other pool connections if
/// handlers also use the pool. Handlers must yield to Tokio for the deadline
/// to take effect and be idempotent because a retry may repeat a side effect.
///
/// Failing events are tracked in memory with exponential backoff and skipped
/// during subsequent claims within the backoff window so a single poison-pill
/// event cannot starve newer events.
#[derive(Debug, Clone)]
pub struct OutboxProcessor {
    config: OutboxConfig,
    failures: Arc<Mutex<FailureTracker>>,
}

impl Default for OutboxProcessor {
    fn default() -> Self {
        Self::new(OutboxConfig::default())
    }
}

impl OutboxProcessor {
    /// Create a new processor with the given configuration.
    pub fn new(config: OutboxConfig) -> Self {
        Self {
            config,
            failures: Arc::new(Mutex::new(FailureTracker::default())),
        }
    }

    /// Process all pending events, then wait for notifications.
    ///
    /// This is the recommended way to run the outbox processor in production.
    /// Uses LISTEN/NOTIFY for efficient wake-up with a fallback poll.
    ///
    /// The handler function is called for each event. If it returns `Ok(())`,
    /// the event is marked as processed. If it returns an error, the event
    /// remains unprocessed, is backed off so newer events are not starved,
    /// and will be retried after its backoff elapses. A timed-out handler
    /// future is dropped and its event is also backed off and retried later.
    pub async fn run_with_notifier<F, Fut>(
        &self,
        pool: &PgPool,
        notifier: &mut OutboxNotifier,
        handler: F,
    ) -> Result<(), sqlx::Error>
    where
        F: Fn(OutboxEvent) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send,
    {
        self.run_with_notifier_until_shutdown(pool, notifier, handler, future::pending())
            .await
    }

    /// Process pending events using LISTEN/NOTIFY until the supplied shutdown
    /// future resolves.
    ///
    /// A shutdown wakes an idle processor immediately. If a batch is in
    /// progress, the processor settles the current event before shutting down.
    pub async fn run_with_notifier_until_shutdown<F, Fut>(
        &self,
        pool: &PgPool,
        notifier: &mut OutboxNotifier,
        handler: F,
        shutdown: impl Future<Output = ()>,
    ) -> Result<(), sqlx::Error>
    where
        F: Fn(OutboxEvent) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send,
    {
        self.run_with_notifier_backend_until_shutdown(pool, notifier, handler, shutdown)
            .await
    }

    async fn run_with_notifier_backend_until_shutdown<B, N, F, Fut>(
        &self,
        backend: &B,
        notifier: &mut N,
        handler: F,
        shutdown: impl Future<Output = ()>,
    ) -> Result<(), B::Error>
    where
        B: OutboxBackend,
        N: OutboxWakeup,
        F: Fn(OutboxEvent) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send,
    {
        info!(
            fallback_interval_secs = self.config.fallback_interval().as_secs(),
            batch_size = self.config.batch_size(),
            "Starting outbox processor with LISTEN/NOTIFY"
        );

        let mut shutdown = pin!(shutdown);
        let mut should_wait = false;

        loop {
            if should_wait {
                tokio::select! {
                    biased;
                    () = &mut shutdown => return Ok(()),
                    wait_res = notifier.wait_notification(self.config.fallback_interval()) => {
                        match wait_res {
                            Ok(Some(event_type)) => {
                                debug!(event_type = %event_type, "Woke up from domain event notification");
                            }
                            Ok(None) => {
                                debug!("Outbox processor woke up from fallback timeout");
                            }
                            Err(e) => {
                                error!(error = %e, "Outbox notifier error, will retry");
                                tokio::select! {
                                    biased;
                                    () = &mut shutdown => return Ok(()),
                                    () = tokio::time::sleep(Duration::from_secs(1)) => {}
                                }
                            }
                        }
                    }
                }
            } else {
                tokio::select! {
                    biased;
                    () = &mut shutdown => return Ok(()),
                    () = future::ready(()) => {}
                }
            }

            let report = self
                .process_batch_with(backend, &handler, shutdown.as_mut())
                .await?;
            if report.shutdown_requested {
                return Ok(());
            }
            should_wait = report.exhausted;
        }
    }

    /// Process a batch of events.
    ///
    /// Returns the number of events processed successfully. Failing events are
    /// skipped for the remainder of the batch and backed off so newer events
    /// continue to make progress.
    pub async fn process_batch<F, Fut>(
        &self,
        pool: &PgPool,
        handler: &F,
    ) -> Result<usize, sqlx::Error>
    where
        F: Fn(OutboxEvent) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send,
    {
        let shutdown = pin!(future::pending::<()>());
        let report = self.process_batch_with(pool, handler, shutdown).await?;
        Ok(report.processed)
    }

    async fn process_batch_with<B, F, Fut, S>(
        &self,
        backend: &B,
        handler: &F,
        mut shutdown: Pin<&mut S>,
    ) -> Result<BatchReport, B::Error>
    where
        B: OutboxBackend,
        F: Fn(OutboxEvent) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send,
        S: Future<Output = ()>,
    {
        let batch_limit = self.config.batch_size();
        if batch_limit == 0 {
            return Ok(BatchReport {
                processed: 0,
                exhausted: true,
                shutdown_requested: false,
            });
        }

        let mut skip_ids = {
            let tracker = self
                .failures
                .lock()
                .expect("outbox failure tracker lock poisoned");
            tracker.active_skip_ids(Instant::now())
        };

        let mut processed = 0usize;
        let mut attempted = 0usize;
        let mut exhausted = false;

        while attempted < batch_limit {
            tokio::select! {
                biased;
                () = shutdown.as_mut() => {
                    return Ok(BatchReport {
                        processed,
                        exhausted: false,
                        shutdown_requested: true,
                    });
                }
                () = future::ready(()) => {}
            }

            let outcome = backend
                .process_next(&skip_ids, self.config.handler_timeout(), handler)
                .await?;

            match outcome {
                ProcessOneOutcome::Empty => {
                    exhausted = true;
                    break;
                }
                ProcessOneOutcome::Succeeded(event_id) => {
                    self.failures
                        .lock()
                        .expect("outbox failure tracker lock poisoned")
                        .record_success(event_id);
                    processed += 1;
                    attempted += 1;
                }
                ProcessOneOutcome::Failed {
                    event_id,
                    event_type,
                    error,
                } => {
                    let (attempts, delay) = self
                        .failures
                        .lock()
                        .expect("outbox failure tracker lock poisoned")
                        .record_failure(
                            event_id,
                            Instant::now(),
                            self.config.failure_backoff(),
                            self.config.max_failure_backoff(),
                        );
                    warn!(
                        %event_id,
                        %event_type,
                        attempts,
                        retry_backoff_ms = delay.as_millis(),
                        error = %error,
                        "Outbox event handler failed, backing off and continuing batch"
                    );
                    skip_ids.push(event_id);
                    attempted += 1;
                }
            }
        }

        Ok(BatchReport {
            processed,
            exhausted,
            shutdown_requested: false,
        })
    }

    /// Run the processor in polling mode (without LISTEN/NOTIFY).
    ///
    /// This is useful for environments where LISTEN/NOTIFY isn't available.
    /// The processor will poll at the configured `fallback_interval`.
    pub async fn run_polling<F, Fut>(&self, pool: &PgPool, handler: F) -> Result<(), sqlx::Error>
    where
        F: Fn(OutboxEvent) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send,
    {
        self.run_polling_until_shutdown(pool, handler, future::pending())
            .await
    }

    /// Run the processor in polling mode until the supplied shutdown future resolves.
    ///
    /// A shutdown wakes an idle processor immediately. If a batch is in
    /// progress, the processor settles that batch before shutting down.
    pub async fn run_polling_until_shutdown<F, Fut>(
        &self,
        pool: &PgPool,
        handler: F,
        shutdown: impl Future<Output = ()>,
    ) -> Result<(), sqlx::Error>
    where
        F: Fn(OutboxEvent) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send,
    {
        self.run_polling_backend_until_shutdown(pool, handler, shutdown)
            .await
    }

    async fn run_polling_backend_until_shutdown<B, F, Fut>(
        &self,
        backend: &B,
        handler: F,
        shutdown: impl Future<Output = ()>,
    ) -> Result<(), B::Error>
    where
        B: OutboxBackend,
        F: Fn(OutboxEvent) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send,
    {
        info!(
            poll_interval_secs = self.config.fallback_interval().as_secs(),
            batch_size = self.config.batch_size(),
            "Starting outbox processor in polling mode"
        );

        let mut shutdown = pin!(shutdown);
        let mut should_sleep = false;

        loop {
            tokio::select! {
                biased;
                () = &mut shutdown => return Ok(()),
                () = async {
                    if should_sleep {
                        tokio::time::sleep(self.config.fallback_interval()).await;
                    }
                } => {}
            }

            let report = self
                .process_batch_with(backend, &handler, shutdown.as_mut())
                .await?;
            if report.shutdown_requested {
                return Ok(());
            }
            should_sleep = report.exhausted;
        }
    }
}

#[cfg(test)]
#[path = "tests/outbox_tests.rs"]
mod tests;
