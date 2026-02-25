mod schema;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use underlay_core::Uuid;

pub use crate::schema::DOMAIN_EVENTS_SQL;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DomainEvent {
    pub id: Uuid,
    pub event_type: String,
    pub payload: Value,
    pub occurred_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewDomainEvent {
    pub event_type: String,
    pub payload: Value,
    pub occurred_at: DateTime<Utc>,
}

impl NewDomainEvent {
    pub fn now(event_type: impl Into<String>, payload: Value) -> Self {
        Self {
            event_type: event_type.into(),
            payload,
            occurred_at: Utc::now(),
        }
    }
}

/// An app-owned storage implementation for persisting domain events.
///
/// Underlay provides the contract and recommended schema; applications provide
/// concrete implementations (e.g. SQLx/Postgres).
#[async_trait]
pub trait DomainEventWriter: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn append(&self, event: NewDomainEvent) -> Result<Uuid, Self::Error>;
}

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
