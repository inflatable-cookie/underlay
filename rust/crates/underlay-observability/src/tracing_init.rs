use tracing_subscriber::{fmt, EnvFilter};

/// Runtime environment for the application.
///
/// Used to automatically select appropriate logging format:
/// - Local/Dev: Pretty text format for readability
/// - Staging/Prod/Test: JSON format for log aggregation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Environment {
    /// Local development environment (legacy alias; prefer `Effigy` for the
    /// shared container dev stack).
    #[default]
    Local,
    /// The shared effigy container dev stack — the single canonical dev
    /// environment across Underlay consumers. Behaves like `Local` at every
    /// dev gate, but is unambiguous in logs and config.
    Effigy,
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
    /// - "effigy" -> Effigy (the shared container dev stack)
    /// - "dev", "development" -> Dev
    /// - "staging", "stage" -> Staging
    /// - "prod", "production" -> Prod
    /// - "test" -> Test
    ///
    /// Unknown values default to Prod (fail closed: never enable
    /// development behavior such as permissive CORS, optional encryption
    /// keys, or dev seeds because of a typo).
    pub fn parse(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "local" => Environment::Local,
            "effigy" => Environment::Effigy,
            "dev" | "development" => Environment::Dev,
            "staging" | "stage" => Environment::Staging,
            "prod" | "production" => Environment::Prod,
            "test" => Environment::Test,
            _ => Environment::Prod,
        }
    }

    /// Resolve the runtime environment from process env vars (fail closed).
    ///
    /// Reads `primary_var` first, then the deprecated `legacy_var` if given.
    /// Unset (or unknown) resolves to `Prod`: development behavior is never
    /// enabled by omission. This is the single env-resolution point — apps
    /// should call it instead of parsing env vars themselves.
    ///
    /// When the value comes from the legacy var, a deprecation warning is
    /// printed to stderr once per process (resolve can run before tracing
    /// is initialised).
    pub fn resolve(primary_var: &str, legacy_var: Option<&str>) -> Self {
        match std::env::var(primary_var) {
            Ok(value) => Self::parse(&value),
            Err(_) => match legacy_var.and_then(|var| std::env::var(var).ok()) {
                Some(value) => {
                    if let Some(legacy) = legacy_var {
                        warn_legacy_env_var_once(primary_var, legacy);
                    }
                    Self::parse(&value)
                }
                None => Environment::Prod,
            },
        }
    }

    /// Raw environment name from process env, for config overlay selection
    /// (`config/<name>.toml`).
    ///
    /// Reads `primary_var` first, then the deprecated `legacy_var`, and
    /// returns the first non-empty trimmed value, or `None`. Unlike
    /// [`Environment::resolve`] this does NOT normalize through the enum:
    /// overlay names are arbitrary strings (`uat`, `production`, …) that do
    /// not map 1:1 to variants. Pair the two — both read the same vars in
    /// the same order, so the overlay name and the behavior env cannot
    /// diverge.
    pub fn resolve_name(primary_var: &str, legacy_var: Option<&str>) -> Option<String> {
        std::env::var(primary_var)
            .ok()
            .or_else(|| legacy_var.and_then(|var| std::env::var(var).ok()))
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
    }

    /// Returns true if this is a local or dev environment.
    pub fn is_development(&self) -> bool {
        matches!(self, Environment::Local | Environment::Effigy | Environment::Dev)
    }

    /// Returns true for environments where local-development behavior is
    /// allowed (dev seeds, CORS origin mirroring with credentials, loopback
    /// bind defaults): the local/effigy dev stacks and test runs.
    pub fn is_local_dev(&self) -> bool {
        matches!(self, Environment::Local | Environment::Effigy | Environment::Test)
    }

    /// Returns the recommended log format for this environment.
    pub fn default_log_format(&self) -> LogFormat {
        match self {
            Environment::Local | Environment::Effigy | Environment::Dev => LogFormat::Pretty,
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

pub(crate) fn legacy_env_var_warning(primary_var: &str, legacy_var: &str) -> String {
    format!(
        "warning: environment resolved from deprecated {legacy_var}; set {primary_var} instead"
    )
}

fn warn_legacy_env_var_once(primary_var: &str, legacy_var: &str) {
    use std::sync::atomic::{AtomicBool, Ordering};
    static WARNED: AtomicBool = AtomicBool::new(false);
    if !WARNED.swap(true, Ordering::Relaxed) {
        eprintln!("{}", legacy_env_var_warning(primary_var, legacy_var));
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Local => write!(f, "local"),
            Environment::Effigy => write!(f, "effigy"),
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
    level: Option<String>,
    /// Log output format.
    /// Default: Pretty
    format: LogFormat,
    /// Runtime environment (optional, for context in logs).
    environment: Option<Environment>,
}

impl ObservabilityConfig {
    /// Create a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn level(&self) -> Option<&str> {
        self.level.as_deref()
    }

    pub fn format(&self) -> LogFormat {
        self.format
    }

    pub fn environment(&self) -> Option<Environment> {
        self.environment
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
        let level = config.level().unwrap_or("info");
        EnvFilter::new(level)
    });

    match config.format() {
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

#[cfg(test)]
#[path = "tests/tracing_init_tests.rs"]
mod tests;
