use chrono::{DateTime, Utc};
use underlay_core::Uuid;

/// A token pair issued together.
#[derive(Clone)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

impl std::fmt::Debug for Tokens {
    /// Both fields are live bearer tokens, so neither is rendered. Presence is
    /// still visible so an empty pair remains diagnosable.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tokens")
            .field("access_token", &redacted(&self.access_token))
            .field("refresh_token", &redacted(&self.refresh_token))
            .finish()
    }
}

/// Renders a secret as a fixed marker that reports presence but not value.
fn redacted(value: &str) -> &'static str {
    if value.is_empty() {
        "[EMPTY]"
    } else {
        "[REDACTED]"
    }
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

#[cfg(test)]
mod debug_redaction_tests {
    use super::Tokens;

    #[test]
    fn debug_redacts_both_bearer_tokens() {
        let tokens = Tokens {
            access_token: "header.access-payload.signature".to_string(),
            refresh_token: "header.refresh-payload.signature".to_string(),
        };

        let rendered = format!("{tokens:?}");

        assert!(!rendered.contains("access-payload"));
        assert!(!rendered.contains("refresh-payload"));
        assert_eq!(rendered.matches("[REDACTED]").count(), 2);
    }

    #[test]
    fn debug_distinguishes_an_empty_token_from_a_redacted_one() {
        let tokens = Tokens {
            access_token: String::new(),
            refresh_token: "header.refresh-payload.signature".to_string(),
        };

        let rendered = format!("{tokens:?}");

        assert!(rendered.contains("[EMPTY]"));
        assert_eq!(rendered.matches("[REDACTED]").count(), 1);
    }
}
