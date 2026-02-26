use super::{DomainEvent, NewDomainEvent, DOMAIN_EVENTS_SQL};
use chrono::{TimeZone, Utc};
use serde_json::json;

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

#[test]
fn domain_event_type_has_required_fields() {
    let id = underlay_core::Uuid::new_v7();
    let occurred = Utc.timestamp_opt(1700000000, 0).unwrap();
    let processed = Utc.timestamp_opt(1700000100, 0).unwrap();

    let event = DomainEvent {
        id,
        event_type: "user.created".to_string(),
        payload: json!({"user_id": "abc123"}),
        occurred_at: occurred,
        processed_at: Some(processed),
    };

    assert_eq!(event.id, id);
    assert_eq!(event.event_type, "user.created");
    assert!(event.processed_at.is_some());
}

#[test]
fn domain_event_with_null_processed_at() {
    let event = DomainEvent {
        id: underlay_core::Uuid::new_v7(),
        event_type: "test".to_string(),
        payload: json!({}),
        occurred_at: Utc::now(),
        processed_at: None,
    };

    assert!(event.processed_at.is_none());
}

#[test]
fn domain_event_serialize_roundtrip() {
    let original = DomainEvent {
        id: underlay_core::Uuid::new_v7(),
        event_type: "test.event".to_string(),
        payload: json!({"nested": {"key": "value"}}),
        occurred_at: Utc::now(),
        processed_at: None,
    };

    let serialized = serde_json::to_string(&original).unwrap();
    let deserialized: DomainEvent = serde_json::from_str(&serialized).unwrap();

    assert_eq!(original.id, deserialized.id);
    assert_eq!(original.event_type, deserialized.event_type);
    assert_eq!(original.payload, deserialized.payload);
}

#[test]
fn new_domain_event_from_parts() {
    let payload = json!({"key": 123});
    let occurred = Utc::now();

    let event = NewDomainEvent {
        event_type: "custom.event".to_string(),
        payload: payload.clone(),
        occurred_at: occurred,
    };

    assert_eq!(event.event_type, "custom.event");
    assert_eq!(event.payload, payload);
    assert_eq!(event.occurred_at, occurred);
}

#[test]
fn domain_event_debug_format() {
    let event = DomainEvent {
        id: underlay_core::Uuid::new_v7(),
        event_type: "debug.test".to_string(),
        payload: json!({}),
        occurred_at: Utc::now(),
        processed_at: None,
    };

    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("DomainEvent"));
    assert!(debug_str.contains("debug.test"));
}

#[test]
fn new_domain_event_debug_format() {
    let event = NewDomainEvent::now("debug.event", json!({}));
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("NewDomainEvent"));
}

#[test]
fn domain_event_traits() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    assert_send::<DomainEvent>();
    assert_sync::<DomainEvent>();
    assert_send::<NewDomainEvent>();
    assert_sync::<NewDomainEvent>();
}

#[test]
fn domain_event_implements_clone() {
    let event1 = DomainEvent {
        id: underlay_core::Uuid::new_v7(),
        event_type: "clone.test".to_string(),
        payload: json!({"cloned": true}),
        occurred_at: Utc::now(),
        processed_at: None,
    };

    let event2 = event1.clone();
    assert_eq!(event1.id, event2.id);
    assert_eq!(event1.event_type, event2.event_type);
}

#[test]
fn complex_payload_serialization() {
    let payload = json!({
        "users": [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ],
        "metadata": {
            "version": "1.0",
            "tags": ["tag1", "tag2"]
        }
    });

    let event = NewDomainEvent::now("complex.event", payload.clone());
    assert_eq!(event.payload, payload);
}
