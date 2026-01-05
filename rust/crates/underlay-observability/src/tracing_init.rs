use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    Pretty,
    Json,
}

#[derive(Debug, Clone)]
pub struct ObservabilityConfig {
    /// If set, used as a fallback when `RUST_LOG` is not present.
    pub level: Option<String>,
    pub format: LogFormat,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            level: None,
            format: LogFormat::Pretty,
        }
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
