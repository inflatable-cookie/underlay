mod schema;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use underlay_core::Uuid;

pub use crate::schema::DOMAIN_EVENTS_SQL;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainEvent {
    pub id: Uuid,
    pub event_type: String,
    pub payload: Value,
    pub occurred_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
mod tests {
    use super::DOMAIN_EVENTS_SQL;

    #[test]
    fn schema_template_is_present() {
        assert!(!DOMAIN_EVENTS_SQL.trim().is_empty());
        let lowered = DOMAIN_EVENTS_SQL.to_ascii_lowercase();
        assert!(lowered.contains("create table"));
    }

    #[test]
    fn new_domain_event_now_sets_expected_fields() {
        let payload = serde_json::json!({"ok": true});
        let before = chrono::Utc::now();
        let event = super::NewDomainEvent::now("test.event", payload.clone());
        let after = chrono::Utc::now();

        assert_eq!(event.event_type, "test.event");
        assert_eq!(event.payload, payload);
        assert!(event.occurred_at >= before);
        assert!(event.occurred_at <= after);
    }
}
