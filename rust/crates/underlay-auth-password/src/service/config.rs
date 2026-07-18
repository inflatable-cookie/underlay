/// Configuration for password authentication.
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    /// Maximum failed login attempts before lockout.
    max_failed_attempts: u32,
    /// Lockout duration in seconds.
    lockout_duration_seconds: u64,
    /// Rate limit window in seconds.
    rate_limit_window_seconds: u64,
    /// Maximum attempts per rate limit window.
    rate_limit_max_attempts: u32,
    /// Maximum attempts per window keyed on email alone, independent of IP.
    /// Backstop against attackers rotating spoofed or real IPs to reset the
    /// per-`email:ip` counter.
    rate_limit_email_max_attempts: u32,
    /// Minimum password length.
    min_password_length: usize,
    /// Whether to check compromised passwords.
    check_compromised: bool,
    /// Strategy for checking if a password is compromised.
    compromised_password_strategy: CompromisedPasswordStrategy,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            max_failed_attempts: 5,
            lockout_duration_seconds: 900,
            rate_limit_window_seconds: 3600,
            rate_limit_max_attempts: 10,
            rate_limit_email_max_attempts: 30,
            min_password_length: 8,
            check_compromised: false,
            compromised_password_strategy: CompromisedPasswordStrategy::LocalBlocklist,
        }
    }
}

impl PasswordConfig {
    pub fn max_failed_attempts(&self) -> u32 {
        self.max_failed_attempts
    }

    pub fn lockout_duration_seconds(&self) -> u64 {
        self.lockout_duration_seconds
    }

    pub fn rate_limit_window_seconds(&self) -> u64 {
        self.rate_limit_window_seconds
    }

    pub fn rate_limit_max_attempts(&self) -> u32 {
        self.rate_limit_max_attempts
    }

    pub fn rate_limit_email_max_attempts(&self) -> u32 {
        self.rate_limit_email_max_attempts
    }

    pub fn min_password_length(&self) -> usize {
        self.min_password_length
    }

    pub fn check_compromised(&self) -> bool {
        self.check_compromised
    }

    pub fn compromised_password_strategy(&self) -> &CompromisedPasswordStrategy {
        &self.compromised_password_strategy
    }

    pub fn with_max_failed_attempts(mut self, attempts: u32) -> Self {
        self.max_failed_attempts = attempts;
        self
    }

    pub fn with_lockout_duration_seconds(mut self, seconds: u64) -> Self {
        self.lockout_duration_seconds = seconds;
        self
    }

    pub fn with_rate_limit_window_seconds(mut self, seconds: u64) -> Self {
        self.rate_limit_window_seconds = seconds;
        self
    }

    pub fn with_rate_limit_max_attempts(mut self, attempts: u32) -> Self {
        self.rate_limit_max_attempts = attempts;
        self
    }

    pub fn with_rate_limit_email_max_attempts(mut self, attempts: u32) -> Self {
        self.rate_limit_email_max_attempts = attempts;
        self
    }

    pub fn with_min_password_length(mut self, length: usize) -> Self {
        self.min_password_length = length;
        self
    }

    pub fn with_check_compromised(mut self, check: bool) -> Self {
        self.check_compromised = check;
        self
    }

    pub fn with_compromised_password_strategy(
        mut self,
        strategy: CompromisedPasswordStrategy,
    ) -> Self {
        self.compromised_password_strategy = strategy;
        self
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
