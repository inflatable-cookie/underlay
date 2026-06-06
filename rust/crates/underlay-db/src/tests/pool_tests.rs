use super::*;

#[test]
fn db_config_defaults() {
    let config = DbConfig::new("postgres://localhost/test");
    assert_eq!(config.max_connections(), DEFAULT_MAX_CONNECTIONS);
    assert_eq!(config.min_connections(), DEFAULT_MIN_CONNECTIONS);
    assert_eq!(config.acquire_timeout_secs(), DEFAULT_ACQUIRE_TIMEOUT_SECS);
    assert_eq!(config.idle_timeout_secs(), DEFAULT_IDLE_TIMEOUT_SECS);
}

#[test]
fn db_config_builder_pattern() {
    let config = DbConfig::new("postgres://localhost/test")
        .with_max_connections(50)
        .with_min_connections(5)
        .with_acquire_timeout_secs(60)
        .with_idle_timeout_secs(300);

    assert_eq!(config.max_connections(), 50);
    assert_eq!(config.min_connections(), 5);
    assert_eq!(config.acquire_timeout_secs(), 60);
    assert_eq!(config.idle_timeout_secs(), 300);
}

#[test]
fn db_config_stores_database_url() {
    let url = "postgres://localhost:5432/mydb";
    let config = DbConfig::new(url);
    assert_eq!(config.database_url(), url);
}

#[test]
fn db_config_from_string() {
    let config = DbConfig::new("postgresql://user:pass@localhost/db");
    assert_eq!(config.database_url(), "postgresql://user:pass@localhost/db");
}

#[test]
fn db_config_clone_works() {
    let config = DbConfig::new("postgres://localhost/test").with_max_connections(20);
    let cloned = config.clone();
    assert_eq!(cloned.database_url(), config.database_url());
    assert_eq!(cloned.max_connections(), config.max_connections());
    assert_eq!(cloned.min_connections(), config.min_connections());
}

#[test]
fn db_config_debug_redacts_database_url() {
    let config = DbConfig::new("postgres://user:secretpassword@localhost/test");
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("[REDACTED]"));
    assert!(!debug_str.contains("secretpassword"));
    assert!(!debug_str.contains("postgres://"));
    assert!(debug_str.contains("max_connections"));
}
