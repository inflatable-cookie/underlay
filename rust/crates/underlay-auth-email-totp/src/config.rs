//! Configuration for email TOTP service.

/// Configuration for the email TOTP service.
#[derive(Debug, Clone)]
pub struct EmailTotpConfig {
    /// Code expiry in minutes (default: 10).
    code_expiry_minutes: i32,

    /// Maximum codes a user can request per hour (default: 5).
    max_codes_per_hour: i32,

    /// Maximum verification attempts per code (default: 5).
    max_attempts: i32,

    /// Verification session expiry in minutes after successful verification (default: 5).
    session_expiry_minutes: i32,

    /// Number of digits in the code (default: 6).
    code_length: usize,
}

impl Default for EmailTotpConfig {
    fn default() -> Self {
        Self {
            code_expiry_minutes: 10,
            max_codes_per_hour: 5,
            max_attempts: 5,
            session_expiry_minutes: 5,
            code_length: 6,
        }
    }
}

impl EmailTotpConfig {
    /// Create a new configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn code_expiry_minutes(&self) -> i32 {
        self.code_expiry_minutes
    }

    pub fn max_codes_per_hour(&self) -> i32 {
        self.max_codes_per_hour
    }

    pub fn max_attempts(&self) -> i32 {
        self.max_attempts
    }

    pub fn session_expiry_minutes(&self) -> i32 {
        self.session_expiry_minutes
    }

    pub fn code_length(&self) -> usize {
        self.code_length
    }

    /// Set the code expiry time in minutes.
    pub fn with_code_expiry_minutes(mut self, minutes: i32) -> Self {
        self.code_expiry_minutes = minutes;
        self
    }

    /// Set the maximum codes per hour.
    pub fn with_max_codes_per_hour(mut self, max: i32) -> Self {
        self.max_codes_per_hour = max;
        self
    }

    /// Set the maximum verification attempts.
    pub fn with_max_attempts(mut self, max: i32) -> Self {
        self.max_attempts = max;
        self
    }

    /// Set the verification session expiry time in minutes.
    pub fn with_session_expiry_minutes(mut self, minutes: i32) -> Self {
        self.session_expiry_minutes = minutes;
        self
    }

    /// Set the code length.
    pub fn with_code_length(mut self, length: usize) -> Self {
        self.code_length = length;
        self
    }
}

#[cfg(test)]
#[path = "tests/config_tests.rs"]
mod tests;
