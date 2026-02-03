use tracing_subscriber::EnvFilter;

/// Log output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogFormat {
    /// Human-readable format with colors (default).
    #[default]
    Pretty,
    /// JSON format for log aggregation systems.
    Json,
}

/// Configuration for observability (logging, tracing).
///
/// # Example
///
/// ```
/// use underlay_observability::{ObservabilityConfig, LogFormat};
///
/// // Default config (pretty format, info level fallback)
/// let config = ObservabilityConfig::default();
///
/// // Custom config
/// let config = ObservabilityConfig::default()
///     .with_level("debug")
///     .with_format(LogFormat::Json);
///
/// // Production config
/// let config = ObservabilityConfig::new()
///     .with_level("warn")
///     .with_json();
/// ```
#[derive(Debug, Clone, Default)]
pub struct ObservabilityConfig {
    /// Fallback log level when `RUST_LOG` is not present.
    /// Default: "info"
    pub level: Option<String>,
    /// Log output format.
    /// Default: Pretty
    pub format: LogFormat,
}

impl ObservabilityConfig {
    /// Create a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the fallback log level (used when `RUST_LOG` is not set).
    ///
    /// Valid values: "trace", "debug", "info", "warn", "error"
    pub fn with_level(mut self, level: impl Into<String>) -> Self {
        self.level = Some(level.into());
        self
    }

    /// Set the log output format.
    pub fn with_format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }

    /// Use pretty (human-readable) log format.
    pub fn with_pretty(mut self) -> Self {
        self.format = LogFormat::Pretty;
        self
    }

    /// Use JSON log format (for log aggregation).
    pub fn with_json(mut self) -> Self {
        self.format = LogFormat::Json;
        self
    }
}

/// Initialise `tracing` for an application.
///
/// Conventions:
/// - Prefer `RUST_LOG` for runtime filtering.
/// - Fall back to `config.level` if `RUST_LOG` is absent.
/// - Use Pretty format by default; use JSON format for log aggregation.
pub fn init_tracing(config: ObservabilityConfig) {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let level = config.level.as_deref().unwrap_or("info");
        EnvFilter::new(level)
    });

    match config.format {
        LogFormat::Pretty => {
            tracing_subscriber::fmt()
                .with_env_filter(env_filter)
                .with_target(true)
                .init();
        }
        LogFormat::Json => {
            tracing_subscriber::fmt()
                .json()
                .with_env_filter(env_filter)
                .with_target(true)
                .init();
        }
    }
}
