use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SecurityAlertConfig {
    window: Duration,
    cooldown: Duration,
    failed_attempts_threshold: i64,
    distinct_users_threshold: i64,
    lockouts_threshold: i64,
    account_failed_attempts_threshold: i64,
    account_distinct_ips_threshold: i64,
    global_failed_attempts_threshold: i64,
}

impl Default for SecurityAlertConfig {
    fn default() -> Self {
        Self {
            window: Duration::minutes(10),
            cooldown: Duration::minutes(30),
            failed_attempts_threshold: 20,
            distinct_users_threshold: 5,
            lockouts_threshold: 3,
            account_failed_attempts_threshold: 10,
            account_distinct_ips_threshold: 5,
            global_failed_attempts_threshold: 200,
        }
    }
}

impl SecurityAlertConfig {
    pub fn window(&self) -> Duration {
        self.window
    }

    pub fn cooldown(&self) -> Duration {
        self.cooldown
    }

    pub fn failed_attempts_threshold(&self) -> i64 {
        self.failed_attempts_threshold
    }

    pub fn distinct_users_threshold(&self) -> i64 {
        self.distinct_users_threshold
    }

    pub fn lockouts_threshold(&self) -> i64 {
        self.lockouts_threshold
    }

    pub fn account_failed_attempts_threshold(&self) -> i64 {
        self.account_failed_attempts_threshold
    }

    pub fn account_distinct_ips_threshold(&self) -> i64 {
        self.account_distinct_ips_threshold
    }

    pub fn global_failed_attempts_threshold(&self) -> i64 {
        self.global_failed_attempts_threshold
    }

    pub fn with_window(mut self, window: Duration) -> Self {
        self.window = window;
        self
    }

    pub fn with_cooldown(mut self, cooldown: Duration) -> Self {
        self.cooldown = cooldown;
        self
    }

    pub fn with_failed_attempts_threshold(mut self, threshold: i64) -> Self {
        self.failed_attempts_threshold = threshold;
        self
    }

    pub fn with_distinct_users_threshold(mut self, threshold: i64) -> Self {
        self.distinct_users_threshold = threshold;
        self
    }

    pub fn with_lockouts_threshold(mut self, threshold: i64) -> Self {
        self.lockouts_threshold = threshold;
        self
    }

    pub fn with_account_failed_attempts_threshold(mut self, threshold: i64) -> Self {
        self.account_failed_attempts_threshold = threshold;
        self
    }

    pub fn with_account_distinct_ips_threshold(mut self, threshold: i64) -> Self {
        self.account_distinct_ips_threshold = threshold;
        self
    }

    pub fn with_global_failed_attempts_threshold(mut self, threshold: i64) -> Self {
        self.global_failed_attempts_threshold = threshold;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityAlertType {
    LoginFailuresFromIp,
    MultiAccountFailuresFromIp,
    LockoutsFromIp,
    DormantAccountAccessAttempt,
    /// Failed attempts against one account across any number of IPs.
    /// IP rotation cannot keep this counter down.
    LoginFailuresForAccount,
    /// Failed attempts against one account from many distinct IPs
    /// (distributed/rotating attack signature).
    DistributedFailuresForAccount,
    /// Total failed attempts across all accounts and IPs in the window.
    GlobalLoginFailureSurge,
}

impl SecurityAlertType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LoginFailuresFromIp => "login_failures_from_ip",
            Self::MultiAccountFailuresFromIp => "multi_account_failures_from_ip",
            Self::LockoutsFromIp => "lockouts_from_ip",
            Self::DormantAccountAccessAttempt => "dormant_account_access_attempt",
            Self::LoginFailuresForAccount => "login_failures_for_account",
            Self::DistributedFailuresForAccount => "distributed_failures_for_account",
            Self::GlobalLoginFailureSurge => "global_login_failure_surge",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LoginAttemptSignalCounts {
    pub failed_attempts: i64,
    pub distinct_users: i64,
    pub lockouts: i64,
}

/// Failed-attempt signal counts for a single account across all IPs.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AccountSignalCounts {
    pub failed_attempts: i64,
    pub distinct_ips: i64,
}

/// Failed-attempt signal counts across all accounts and IPs.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GlobalSignalCounts {
    pub failed_attempts: i64,
    pub distinct_ips: i64,
}

#[derive(Debug, Clone)]
pub struct SecurityAlertEventInput {
    pub alert_type: SecurityAlertType,
    pub ip_address: String,
    pub window_started_at: DateTime<Utc>,
    pub window_ended_at: DateTime<Utc>,
    pub counts: LoginAttemptSignalCounts,
    pub details: serde_json::Value,
}

/// Alert event input for non-IP scopes (per-account, global).
///
/// `scope_key` is the dedupe key: `account:<user_id>` for account alerts,
/// `global` for global alerts.
#[derive(Debug, Clone)]
pub struct ScopedSecurityAlertEventInput {
    pub alert_type: SecurityAlertType,
    pub scope_key: String,
    pub ip_address: Option<String>,
    pub window_started_at: DateTime<Utc>,
    pub window_ended_at: DateTime<Utc>,
    pub failed_attempts: i64,
    pub details: serde_json::Value,
}
