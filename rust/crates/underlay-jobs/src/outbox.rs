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
//! use underlay_jobs::outbox::{OutboxProcessor, OutboxNotifier, OutboxConfig};
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

use std::future::Future;
use std::time::Duration;

use chrono::Utc;
use serde_json::Value;
use sqlx::postgres::PgListener;
use sqlx::{PgPool, Row};
use tracing::{debug, error, info, warn};

/// The PostgreSQL NOTIFY channel for domain event notifications.
pub const DOMAIN_EVENT_NOTIFY_CHANNEL: &str = "underlay_domain_event_notify";

/// SQL schema for the domain event notify trigger.
///
/// Applications should use `underlay-devtools sync-migrations` to copy this
/// to their migrations folder, or include it directly.
pub const DOMAIN_EVENT_NOTIFY_SQL: &str =
    include_str!("../migrations/0003_add_domain_event_notify.sql");

/// Configuration for the outbox processor.
#[derive(Debug, Clone)]
pub struct OutboxConfig {
    /// Maximum number of events to process per batch.
    pub batch_size: usize,
    /// Fallback poll interval when using LISTEN/NOTIFY.
    /// The processor will wake up at least this often even without notifications.
    pub fallback_interval: Duration,
}

impl Default for OutboxConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            fallback_interval: Duration::from_secs(30),
        }
    }
}

impl OutboxConfig {
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

/// Processes domain events from the outbox table.
///
/// The processor claims events using `FOR UPDATE SKIP LOCKED` for
/// safe concurrent processing, and marks them as processed after
/// the handler completes successfully.
#[derive(Debug, Clone)]
pub struct OutboxProcessor {
    config: OutboxConfig,
}

impl Default for OutboxProcessor {
    fn default() -> Self {
        Self::new(OutboxConfig::default())
    }
}

impl OutboxProcessor {
    /// Create a new processor with the given configuration.
    pub fn new(config: OutboxConfig) -> Self {
        Self { config }
    }

    /// Process all pending events, then wait for notifications.
    ///
    /// This is the recommended way to run the outbox processor in production.
    /// Uses LISTEN/NOTIFY for efficient wake-up with a fallback poll.
    ///
    /// The handler function is called for each event. If it returns `Ok(())`,
    /// the event is marked as processed. If it returns an error, the event
    /// remains unprocessed and will be retried on the next batch.
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
        info!(
            fallback_interval_secs = self.config.fallback_interval.as_secs(),
            batch_size = self.config.batch_size,
            "Starting outbox processor with LISTEN/NOTIFY"
        );

        loop {
            // Process all available events
            loop {
                let count = self.process_batch(pool, &handler).await?;
                if count == 0 {
                    break;
                }
            }

            // Wait for notification or fallback timeout
            match notifier.wait(self.config.fallback_interval).await {
                Ok(Some(event_type)) => {
                    debug!(event_type = %event_type, "Woke up from domain event notification");
                }
                Ok(None) => {
                    debug!("Outbox processor woke up from fallback timeout");
                }
                Err(e) => {
                    error!(error = %e, "Outbox notifier error, will retry");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    /// Process a batch of events.
    ///
    /// Returns the number of events processed.
    pub async fn process_batch<F, Fut>(
        &self,
        pool: &PgPool,
        handler: &F,
    ) -> Result<usize, sqlx::Error>
    where
        F: Fn(OutboxEvent) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send,
    {
        let mut processed = 0;

        loop {
            let did_work = self.process_one(pool, handler).await?;
            if !did_work {
                break;
            }

            processed += 1;
            if processed >= self.config.batch_size {
                break;
            }
        }

        Ok(processed)
    }

    /// Process a single event.
    ///
    /// Returns `true` if an event was processed, `false` if the queue was empty.
    async fn process_one<F, Fut>(&self, pool: &PgPool, handler: &F) -> Result<bool, sqlx::Error>
    where
        F: Fn(OutboxEvent) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send,
    {
        let mut tx = pool.begin().await?;

        // Claim the next unprocessed event
        let row = sqlx::query(
            r#"
            SELECT id, event_type, payload, occurred_at
            FROM platform.domain_events
            WHERE processed_at IS NULL
            ORDER BY occurred_at ASC
            LIMIT 1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .fetch_optional(&mut *tx)
        .await?;

        let Some(row) = row else {
            return Ok(false);
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

        // Call the handler
        match handler(event.clone()).await {
            Ok(()) => {
                // Mark as processed
                sqlx::query(
                    r#"
                    UPDATE platform.domain_events
                    SET processed_at = $1
                    WHERE id = $2
                    "#,
                )
                .bind(Utc::now())
                .bind(event.id)
                .execute(&mut *tx)
                .await?;

                tx.commit().await?;
                info!(
                    event_id = %event.id,
                    event_type = %event.event_type,
                    "Outbox event processed successfully"
                );
            }
            Err(e) => {
                // Roll back - event remains unprocessed for retry
                warn!(
                    event_id = %event.id,
                    event_type = %event.event_type,
                    error = %e,
                    "Outbox event handler failed, will retry"
                );
                // Transaction rolls back automatically on drop
            }
        }

        Ok(true)
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
        info!(
            poll_interval_secs = self.config.fallback_interval.as_secs(),
            batch_size = self.config.batch_size,
            "Starting outbox processor in polling mode"
        );

        loop {
            // Process all available events
            loop {
                let count = self.process_batch(pool, &handler).await?;
                if count == 0 {
                    break;
                }
            }

            // Wait for the poll interval
            tokio::time::sleep(self.config.fallback_interval).await;
        }
    }
}

#[cfg(test)]
#[path = "tests/outbox_tests.rs"]
mod tests;
