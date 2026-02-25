use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

pub type DbPool = PgPool;

/// Default maximum connections in the pool.
pub const DEFAULT_MAX_CONNECTIONS: u32 = 10;

/// Default minimum connections in the pool.
pub const DEFAULT_MIN_CONNECTIONS: u32 = 1;

/// Default connection acquire timeout in seconds.
pub const DEFAULT_ACQUIRE_TIMEOUT_SECS: u64 = 30;

/// Default idle connection timeout in seconds (10 minutes).
pub const DEFAULT_IDLE_TIMEOUT_SECS: u64 = 600;

/// Database pool configuration.
///
/// # Example
///
/// ```
/// use underlay_db::DbConfig;
///
/// // Simple config with defaults
/// let config = DbConfig::new("postgres://localhost/mydb");
///
/// // Custom config with builder pattern
/// let config = DbConfig::new("postgres://localhost/mydb")
///     .with_max_connections(20)
///     .with_min_connections(5)
///     .with_acquire_timeout_secs(60)
///     .with_idle_timeout_secs(300);
/// ```
#[derive(Debug, Clone)]
pub struct DbConfig {
    /// Database connection URL.
    pub database_url: String,
    /// Maximum number of connections in the pool.
    /// Default: 10
    pub max_connections: u32,
    /// Minimum number of connections to maintain.
    /// Default: 1
    pub min_connections: u32,
    /// Timeout for acquiring a connection from the pool in seconds.
    /// Default: 30
    pub acquire_timeout_secs: u64,
    /// Timeout for idle connections before they are closed in seconds.
    /// Default: 600 (10 minutes)
    pub idle_timeout_secs: u64,
}

impl DbConfig {
    /// Create a new config with default pool settings.
    pub fn new(database_url: impl Into<String>) -> Self {
        Self {
            database_url: database_url.into(),
            max_connections: DEFAULT_MAX_CONNECTIONS,
            min_connections: DEFAULT_MIN_CONNECTIONS,
            acquire_timeout_secs: DEFAULT_ACQUIRE_TIMEOUT_SECS,
            idle_timeout_secs: DEFAULT_IDLE_TIMEOUT_SECS,
        }
    }

    /// Set the maximum number of connections.
    pub fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    /// Set the minimum number of connections to maintain.
    pub fn with_min_connections(mut self, min: u32) -> Self {
        self.min_connections = min;
        self
    }

    /// Set the connection acquire timeout in seconds.
    pub fn with_acquire_timeout_secs(mut self, seconds: u64) -> Self {
        self.acquire_timeout_secs = seconds;
        self
    }

    /// Set the idle connection timeout in seconds.
    ///
    /// Connections idle longer than this will be closed.
    pub fn with_idle_timeout_secs(mut self, seconds: u64) -> Self {
        self.idle_timeout_secs = seconds;
        self
    }
}

/// Create a connection pool with the given configuration.
pub async fn create_pool(config: &DbConfig) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.acquire_timeout_secs))
        .idle_timeout(Duration::from_secs(config.idle_timeout_secs))
        .connect(&config.database_url)
        .await
}

#[cfg(test)]
#[path = "tests/pool_tests.rs"]
mod tests;
