use std::time::Duration;

/// Tunables for [`crate::SessionService`].
#[derive(Debug, Clone)]
pub struct SessionServiceConfig {
    /// Maximum total lifetime of a session regardless of activity.
    /// Default: 30 days.
    pub absolute_session_timeout: Duration,
    /// When true, a client fingerprint (IP/User-Agent) mismatch rejects the
    /// refresh. When false (default), mismatches are logged but allowed,
    /// since they can be legitimate network/browser changes.
    pub refresh_fingerprint_strict: bool,
}

impl Default for SessionServiceConfig {
    fn default() -> Self {
        Self {
            absolute_session_timeout: Duration::from_secs(30 * 24 * 60 * 60),
            refresh_fingerprint_strict: false,
        }
    }
}

impl SessionServiceConfig {
    pub fn with_absolute_session_timeout(mut self, timeout: Duration) -> Self {
        self.absolute_session_timeout = timeout;
        self
    }

    pub fn with_refresh_fingerprint_strict(mut self, strict: bool) -> Self {
        self.refresh_fingerprint_strict = strict;
        self
    }
}
