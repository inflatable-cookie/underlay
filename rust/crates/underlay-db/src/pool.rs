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
#[derive(Clone)]
pub struct DbConfig {
    /// Database connection URL.
    database_url: String,
    /// Maximum number of connections in the pool.
    /// Default: 10
    max_connections: u32,
    /// Minimum number of connections to maintain.
    /// Default: 1
    min_connections: u32,
    /// Timeout for acquiring a connection from the pool in seconds.
    /// Default: 30
    acquire_timeout_secs: u64,
    /// Timeout for idle connections before they are closed in seconds.
    /// Default: 600 (10 minutes)
    idle_timeout_secs: u64,
}

impl std::fmt::Debug for DbConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DbConfig")
            .field("database_url", &"[REDACTED]")
            .field("max_connections", &self.max_connections)
            .field("min_connections", &self.min_connections)
            .field("acquire_timeout_secs", &self.acquire_timeout_secs)
            .field("idle_timeout_secs", &self.idle_timeout_secs)
            .finish()
    }
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

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }

    pub fn min_connections(&self) -> u32 {
        self.min_connections
    }

    pub fn acquire_timeout_secs(&self) -> u64 {
        self.acquire_timeout_secs
    }

    pub fn idle_timeout_secs(&self) -> u64 {
        self.idle_timeout_secs
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
        .max_connections(config.max_connections())
        .min_connections(config.min_connections())
        .acquire_timeout(Duration::from_secs(config.acquire_timeout_secs()))
        .idle_timeout(Duration::from_secs(config.idle_timeout_secs()))
        .connect(config.database_url())
        .await
}

#[cfg(test)]
#[path = "tests/pool_tests.rs"]
mod tests;
