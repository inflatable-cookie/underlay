use super::*;

#[test]
fn outbox_config_defaults() {
    let config = OutboxConfig::default();
    assert_eq!(config.batch_size, 100);
    assert_eq!(config.fallback_interval, Duration::from_secs(30));
}

#[test]
fn outbox_config_builder() {
    let config = OutboxConfig::default()
        .with_batch_size(50)
        .with_fallback_interval(Duration::from_secs(60));

    assert_eq!(config.batch_size, 50);
    assert_eq!(config.fallback_interval, Duration::from_secs(60));
}

#[test]
fn outbox_event_debug() {
    let event = OutboxEvent {
        id: uuid::Uuid::nil(),
        event_type: "test.event".to_string(),
        payload: serde_json::json!({"key": "value"}),
        occurred_at: Utc::now(),
    };
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("OutboxEvent"));
    assert!(debug_str.contains("test.event"));
}

#[test]
fn domain_event_notify_sql_present() {
    assert!(!DOMAIN_EVENT_NOTIFY_SQL.is_empty());
    assert!(DOMAIN_EVENT_NOTIFY_SQL.contains("notify_domain_event_inserted"));
}
