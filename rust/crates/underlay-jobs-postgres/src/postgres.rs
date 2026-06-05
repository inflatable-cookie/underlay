//! PostgreSQL-backed job store implementation.

mod create;
mod failure;
mod job_store;
mod maintenance;
mod query;
mod status;

use sqlx::PgPool;
use thiserror::Error;

use underlay_core::Uuid;
use underlay_jobs::{JobEventHub, JobEventSink};

pub(super) fn to_raw(id: Uuid) -> uuid::Uuid {
    id.0
}

/// Repository errors.
#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Job not found: {0}")]
    NotFound(Uuid),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, RepoError>;

/// Repository for job operations.
pub struct JobRepository {
    pool: PgPool,
    events: JobEventHub,
}

impl JobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            events: JobEventHub::new(),
        }
    }

    pub fn with_event_sink(mut self, sink: std::sync::Arc<dyn JobEventSink>) -> Self {
        self.events = self.events.with_sink(sink);
        self
    }
}
