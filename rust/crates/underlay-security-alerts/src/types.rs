use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SecurityAlertConfig {
    pub window: Duration,
    pub cooldown: Duration,
    pub failed_attempts_threshold: i64,
    pub distinct_users_threshold: i64,
    pub lockouts_threshold: i64,
}

impl Default for SecurityAlertConfig {
    fn default() -> Self {
        Self {
            window: Duration::minutes(10),
            cooldown: Duration::minutes(30),
            failed_attempts_threshold: 20,
            distinct_users_threshold: 5,
            lockouts_threshold: 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityAlertType {
    LoginFailuresFromIp,
    MultiAccountFailuresFromIp,
    LockoutsFromIp,
}

impl SecurityAlertType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LoginFailuresFromIp => "login_failures_from_ip",
            Self::MultiAccountFailuresFromIp => "multi_account_failures_from_ip",
            Self::LockoutsFromIp => "lockouts_from_ip",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LoginAttemptSignalCounts {
    pub failed_attempts: i64,
    pub distinct_users: i64,
    pub lockouts: i64,
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
