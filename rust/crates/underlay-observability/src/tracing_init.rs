use tracing_subscriber::{fmt, EnvFilter};

/// Runtime environment for the application.
///
/// Used to automatically select appropriate logging format:
/// - Local/Dev: Pretty text format for readability
/// - Staging/Prod/Test: JSON format for log aggregation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Environment {
    /// Local development environment.
    #[default]
    Local,
    /// Development/integration environment.
    Dev,
    /// Staging/pre-production environment.
    Staging,
    /// Production environment.
    Prod,
    /// Test environment (unit/integration tests).
    Test,
}

impl Environment {
    /// Parse an environment string (case-insensitive).
    ///
    /// Recognized values:
    /// - "local" -> Local
    /// - "dev", "development" -> Dev
    /// - "staging", "stage" -> Staging
    /// - "prod", "production" -> Prod
    /// - "test" -> Test
    ///
    /// Unknown values default to Dev.
    pub fn parse(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "local" => Environment::Local,
            "dev" | "development" => Environment::Dev,
            "staging" | "stage" => Environment::Staging,
            "prod" | "production" => Environment::Prod,
            "test" => Environment::Test,
            _ => Environment::Dev,
        }
    }

    /// Returns true if this is a local or dev environment.
    pub fn is_development(&self) -> bool {
        matches!(self, Environment::Local | Environment::Dev)
    }

    /// Returns the recommended log format for this environment.
    pub fn default_log_format(&self) -> LogFormat {
        match self {
            Environment::Local | Environment::Dev => LogFormat::Pretty,
            Environment::Staging | Environment::Prod | Environment::Test => LogFormat::Json,
        }
    }
}

impl std::str::FromStr for Environment {
    type Err = std::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(value))
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Local => write!(f, "local"),
            Environment::Dev => write!(f, "dev"),
            Environment::Staging => write!(f, "staging"),
            Environment::Prod => write!(f, "prod"),
            Environment::Test => write!(f, "test"),
        }
    }
}

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
/// use underlay_observability::{ObservabilityConfig, LogFormat, Environment};
///
/// // Default config (pretty format, info level fallback)
/// let config = ObservabilityConfig::default();
///
/// // Custom config
/// let config = ObservabilityConfig::default()
///     .with_level("debug")
///     .with_format(LogFormat::Json);
///
/// // Environment-based config (recommended)
/// let config = ObservabilityConfig::for_environment(Environment::Prod)
///     .with_level("info");
///
/// // Production config (explicit)
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
    /// Runtime environment (optional, for context in logs).
    pub environment: Option<Environment>,
}

impl ObservabilityConfig {
    /// Create a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a config with environment-appropriate defaults.
    ///
    /// - Local/Dev: Pretty format
    /// - Staging/Prod/Test: JSON format
    pub fn for_environment(env: Environment) -> Self {
        Self {
            level: None,
            format: env.default_log_format(),
            environment: Some(env),
        }
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

    /// Set the environment (for context in logs).
    pub fn with_environment(mut self, env: Environment) -> Self {
        self.environment = Some(env);
        self
    }
}

/// Initialise `tracing` for an application.
///
/// Conventions:
/// - Prefer `RUST_LOG` for runtime filtering.
/// - Fall back to `config.level` if `RUST_LOG` is absent.
/// - Use Pretty format for local/dev; JSON format for staging/prod.
///
/// # Pretty Format (Local/Dev)
///
/// Human-readable output with colors, suitable for terminal viewing.
///
/// # JSON Format (Staging/Prod)
///
/// Structured JSON output optimized for log aggregation systems:
/// - Includes current span context
/// - Emits span close events for timing
/// - Flattens event fields for easier querying
pub fn init_tracing(config: ObservabilityConfig) {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let level = config.level.as_deref().unwrap_or("info");
        EnvFilter::new(level)
    });

    match config.format {
        LogFormat::Pretty => {
            fmt()
                .pretty()
                .with_env_filter(env_filter)
                .with_target(true)
                .init();
        }
        LogFormat::Json => {
            // Enhanced JSON format for log aggregation systems
            fmt()
                .json()
                .with_env_filter(env_filter)
                .with_target(true)
                .with_current_span(true)
                .with_span_events(fmt::format::FmtSpan::CLOSE)
                .flatten_event(true)
                .init();
        }
    }
}

/// Initialise `tracing` with environment-based defaults.
///
/// This is a convenience function that selects the appropriate log format
/// based on the runtime environment:
/// - Local/Dev: Pretty text format for readability
/// - Staging/Prod/Test: JSON format for log aggregation
///
/// # Example
///
/// ```ignore
/// use underlay_observability::{init_tracing_for_env, Environment};
///
/// let env = Environment::parse(&std::env::var("APP_ENV").unwrap_or_default());
/// init_tracing_for_env(env, "info");
/// ```
pub fn init_tracing_for_env(env: Environment, default_level: &str) {
    init_tracing(ObservabilityConfig::for_environment(env).with_level(default_level));
}
