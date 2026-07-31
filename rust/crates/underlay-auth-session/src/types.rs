use chrono::{DateTime, Utc};
use underlay_core::Uuid;

/// A token pair issued together.
#[derive(Debug, Clone)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

/// Client fingerprint bound to a session (advisory by default).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SessionFingerprint {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl SessionFingerprint {
    pub fn new(ip_address: Option<String>, user_agent: Option<String>) -> Self {
        Self {
            ip_address,
            user_agent,
        }
    }

    /// Human-readable description of the first mismatching field, if any.
    /// Missing values on either side are not treated as mismatches.
    pub fn mismatch_description(&self, current: &SessionFingerprint) -> Option<String> {
        if let (Some(stored), Some(current)) = (&self.ip_address, &current.ip_address) {
            if stored != current {
                return Some(format!("ip changed {stored} -> {current}"));
            }
        }
        if let (Some(stored), Some(current)) = (&self.user_agent, &current.user_agent) {
            if stored != current {
                return Some("user-agent changed".to_string());
            }
        }
        None
    }
}

/// The full persisted session record. Mirrors the canonical
/// `auth.sessions` shape used by the reference consumers; repositories map
/// their own schema onto this.
#[derive(Debug, Clone)]
pub struct SessionRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub roles: Vec<String>,
    pub is_active: bool,

    pub access_token_fingerprint: String,
    pub refresh_token_fingerprint: String,

    pub refresh_token_id: Uuid,
    pub refresh_token_version: i32,

    pub access_token_expires_at: DateTime<Utc>,
    pub refresh_token_expires_at: DateTime<Utc>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,

    pub ip_address: Option<String>,
    pub user_agent: Option<String>,

    pub revoked_reason: Option<String>,
    pub revoked_at: Option<DateTime<Utc>>,
}

impl SessionRecord {
    pub fn fingerprint(&self) -> SessionFingerprint {
        SessionFingerprint {
            ip_address: self.ip_address.clone(),
            user_agent: self.user_agent.clone(),
        }
    }
}
