/// Configuration for password authentication.
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    /// Maximum failed login attempts before lockout.
    pub max_failed_attempts: u32,
    /// Lockout duration in seconds.
    pub lockout_duration_seconds: u64,
    /// Rate limit window in seconds.
    pub rate_limit_window_seconds: u64,
    /// Maximum attempts per rate limit window.
    pub rate_limit_max_attempts: u32,
    /// Minimum password length.
    pub min_password_length: usize,
    /// Whether to check compromised passwords.
    pub check_compromised: bool,
    /// Strategy for checking if a password is compromised.
    pub compromised_password_strategy: CompromisedPasswordStrategy,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            max_failed_attempts: 5,
            lockout_duration_seconds: 900,
            rate_limit_window_seconds: 3600,
            rate_limit_max_attempts: 10,
            min_password_length: 8,
            check_compromised: false,
            compromised_password_strategy: CompromisedPasswordStrategy::LocalBlocklist,
        }
    }
}

/// Strategy for compromised-password checks.
#[derive(Debug, Clone)]
pub enum CompromisedPasswordStrategy {
    /// Offline-only: local blocklist of extremely common passwords.
    ///
    /// This is fast and has no network dependency, but does not detect all breached passwords.
    LocalBlocklist,

    /// Online (optional): HIBP Pwned Passwords k-anonymity range API.
    ///
    /// This sends only the first 5 hex chars of the SHA-1 password hash.
    #[cfg(feature = "hibp")]
    HibpKAnonymity {
        /// Base URL for the API.
        ///
        /// Example: `https://api.pwnedpasswords.com`.
        api_base_url: String,
        /// User agent string for HTTP requests.
        user_agent: String,
    },
}
