use crate::TotpAlgorithm;

/// Configuration for TOTP generation and verification.
///
/// # Example
///
/// ```
/// use underlay_auth_totp::{TotpConfig, TotpAlgorithm};
///
/// // Default config
/// let config = TotpConfig::default();
///
/// // Custom config with builder pattern
/// let config = TotpConfig::default()
///     .with_issuer("My App")
///     .with_digits(8)
///     .with_period_seconds(60)
///     .with_skew_steps(2);
/// ```
#[derive(Debug, Clone)]
pub struct TotpConfig {
    /// The issuer name displayed in authenticator apps.
    /// Default: "Underlay"
    pub issuer: String,
    /// The TOTP algorithm to use.
    /// Default: SHA1 (widely supported)
    pub algorithm: TotpAlgorithm,
    /// Number of digits in the TOTP code.
    /// Default: 6
    pub digits: u32,
    /// Time period for each code in seconds.
    /// Default: 30
    pub period_seconds: u64,
    /// How many time steps before/after current time are accepted.
    /// Default: 1 (accepts previous, current, and next code)
    pub skew_steps: i64,
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            issuer: "Underlay".to_string(),
            algorithm: TotpAlgorithm::Sha1,
            digits: 6,
            period_seconds: 30,
            skew_steps: 1,
        }
    }
}

impl TotpConfig {
    /// Create a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the issuer name (shown in authenticator apps).
    pub fn with_issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = issuer.into();
        self
    }

    /// Set the TOTP algorithm.
    pub fn with_algorithm(mut self, algorithm: TotpAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Set the number of digits in TOTP codes.
    ///
    /// Common values: 6 (default, most compatible) or 8.
    pub fn with_digits(mut self, digits: u32) -> Self {
        self.digits = digits;
        self
    }

    /// Set the time period for each code in seconds.
    ///
    /// Default is 30 seconds. Some apps use 60 seconds.
    pub fn with_period_seconds(mut self, seconds: u64) -> Self {
        self.period_seconds = seconds;
        self
    }

    /// Set the time skew tolerance in steps.
    ///
    /// A value of 1 (default) accepts the previous, current, and next code.
    /// A value of 0 only accepts the current code (strict).
    /// A value of 2 provides more tolerance for clock drift.
    pub fn with_skew_steps(mut self, steps: i64) -> Self {
        self.skew_steps = steps;
        self
    }
}
