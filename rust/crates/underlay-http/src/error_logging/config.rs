use underlay_db::DbPool;

/// Configuration for the error logging middleware.
#[derive(Clone)]
pub struct ErrorLoggingConfig {
    pub(crate) pool: DbPool,
    /// Optional source identifier for the app (e.g., "acme-api", "farmyard-api").
    pub source: Option<String>,
    /// Whether to log 4xx client errors (default: true).
    pub log_client_errors: bool,
    /// Whether to log 5xx server errors (default: true).
    pub log_server_errors: bool,
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
}
