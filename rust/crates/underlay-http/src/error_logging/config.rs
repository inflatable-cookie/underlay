use underlay_db::DbPool;

/// Configuration for the error logging middleware.
#[derive(Clone)]
pub struct ErrorLoggingConfig {
    pub(crate) pool: DbPool,
    /// Optional source identifier for the app (e.g., "acme-api", "farmyard-api").
    source: Option<String>,
    /// Whether to log 4xx client errors (default: true).
    log_client_errors: bool,
    /// Whether to log 5xx server errors (default: true).
    log_server_errors: bool,
}

impl ErrorLoggingConfig {
    /// Create a new configuration with the given database pool.
    pub fn new(pool: DbPool) -> Self {
        Self {
            pool,
            source: None,
            log_client_errors: true,
            log_server_errors: true,
        }
    }

    /// Set the source identifier for log entries.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Set whether to log 4xx client errors.
    pub fn with_client_errors(mut self, enabled: bool) -> Self {
        self.log_client_errors = enabled;
        self
    }

    /// Set whether to log 5xx server errors.
    pub fn with_server_errors(mut self, enabled: bool) -> Self {
        self.log_server_errors = enabled;
        self
    }

    /// Return the configured source identifier.
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Return whether 4xx client errors should be logged.
    pub fn log_client_errors(&self) -> bool {
        self.log_client_errors
    }

    /// Return whether 5xx server errors should be logged.
    pub fn log_server_errors(&self) -> bool {
        self.log_server_errors
    }
}

#[cfg(test)]
mod tests {
    use super::ErrorLoggingConfig;

    #[tokio::test]
    async fn builder_methods_set_read_only_values() {
        let pool =
            underlay_db::DbPool::connect_lazy("postgres://underlay:underlay@localhost/underlay")
                .expect("lazy pool should construct");

        let config = ErrorLoggingConfig::new(pool)
            .with_source("test-api")
            .with_client_errors(false)
            .with_server_errors(true);

        assert_eq!(config.source(), Some("test-api"));
        assert!(!config.log_client_errors());
        assert!(config.log_server_errors());
    }
}
