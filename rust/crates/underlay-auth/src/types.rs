//! Core authentication types for Underlay-based apps.
//!
//! These types define the canonical shapes for users, credentials, sessions,
//! and audit events. Applications can use these directly or wrap them in
//! their own domain-specific types.

use serde::{Deserialize, Serialize};
use underlay_core::Uuid;

use crate::AuthError;

/// User account status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    /// User account is active and can authenticate.
    Active,
    /// User account is temporarily suspended.
    Suspended,
    /// User account is permanently deleted (soft deleted).
    Deleted,
}

/// A user account.
///
/// This is an app-agnostic representation. Applications may extend this
/// with additional fields (e.g., `artist_id` for Songsprout, `role` for Acowtancy).
///
/// Note: `display_name` is optional because identity/personalization fields
/// should live in `account.user_profile`, not in auth. Auth handles
/// authentication only (email, password hash, verification status).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct User {
    /// Unique user identifier.
    pub id: Uuid,
    /// User's email address (unique per app).
    pub email: String,
    /// User's display name (optional - prefer account.user_profile for identity).
    pub display_name: Option<String>,
    /// Account status.
    pub status: UserStatus,
    /// When the account was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the account was last updated.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    /// Check if the user can authenticate.
    pub fn can_authenticate(&self) -> bool {
        self.status == UserStatus::Active
    }

    /// Check if the user is suspended.
    pub fn is_suspended(&self) -> bool {
        self.status == UserStatus::Suspended
    }

    /// Check if the user is deleted.
    pub fn is_deleted(&self) -> bool {
        self.status == UserStatus::Deleted
    }
}

/// Credential type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialType {
    /// Password-based authentication.
    Password,
    /// Time-based One-Time Password (TOTP).
    Totp,
    /// PassKey / WebAuthn credential.
    Passkey,
    /// OAuth2 connection (e.g., Google).
    OAuthGoogle,
}

/// Metadata associated with a credential.
///
/// The contents vary by credential type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum CredentialMetadata {
    /// Password credential metadata.
    #[serde(rename_all = "snake_case")]
    Password {
        /// Hashing algorithm used.
        algorithm: String,
        /// Memory cost in KB.
        memory_kb: u32,
        /// Number of iterations.
        iterations: u32,
        /// Parallelism factor.
        parallelism: u32,
    },
    /// TOTP credential metadata.
    #[serde(rename_all = "snake_case")]
    Totp {
        /// Issuer name for QR code.
        issuer: String,
        /// Hash algorithm (SHA1, SHA256, SHA512).
        algorithm: String,
        /// Number of digits in code.
        digits: u8,
        /// Time period in seconds (usually 30).
        period: u8,
    },
    /// PassKey credential metadata.
    #[serde(rename_all = "snake_case")]
    Passkey {
        /// Base64-encoded credential ID.
        credential_id: String,
        /// Supported transports.
        transports: Vec<String>,
        /// Last used counter.
        last_counter: u32,
    },
    /// OAuth connection metadata.
    #[serde(rename_all = "snake_case")]
    OAuthGoogle {
        /// Google user ID.
        google_user_id: String,
        /// Granted scopes.
        scopes: Vec<String>,
    },
}

/// A credential used for authentication.
///
/// Users can have multiple credentials (e.g., password + TOTP + PassKey).
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Credential {
    /// Unique credential identifier.
    pub id: Uuid,
    /// Owning user.
    pub user_id: Uuid,
    /// Type of credential.
    pub credential_type: CredentialType,
    /// Encrypted credential data (format varies by type).
    pub secret_encrypted: String,
    /// Type-specific metadata.
    pub metadata: CredentialMetadata,
    /// Whether the credential has been verified (e.g., TOTP confirmed).
    pub verified: bool,
    /// When the credential was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the credential was last updated.
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// When the credential was last used.
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Debug for Credential {
    /// `secret_encrypted` is the stored credential material. It is ciphertext
    /// rather than a plaintext secret, but it is still the protected artefact
    /// this row exists to hold, so diagnostics render its presence and length
    /// instead of its value. Every other field is identifying or temporal and
    /// stays visible.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credential")
            .field("id", &self.id)
            .field("user_id", &self.user_id)
            .field("credential_type", &self.credential_type)
            .field(
                "secret_encrypted",
                &format_args!("[REDACTED; {} bytes]", self.secret_encrypted.len()),
            )
            .field("metadata", &self.metadata)
            .field("verified", &self.verified)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .field("last_used_at", &self.last_used_at)
            .finish()
    }
}

/// Session status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    /// Session is active.
    Active,
    /// Session has been revoked.
    Revoked,
    /// Session has expired.
    Expired,
}

/// An active user session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Session {
    /// Unique session identifier.
    pub id: Uuid,
    /// Owning user.
    pub user_id: Uuid,
    /// Fingerprint of access token (for lookup without storing token).
    pub access_token_fingerprint: String,
    /// Fingerprint of refresh token.
    pub refresh_token_fingerprint: String,
    /// When the access token expires.
    pub access_token_expires_at: chrono::DateTime<chrono::Utc>,
    /// When the refresh token expires.
    pub refresh_token_expires_at: chrono::DateTime<chrono::Utc>,
    /// When the session was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the session was last used.
    pub last_used_at: chrono::DateTime<chrono::Utc>,
    /// Client IP address.
    pub ip_address: Option<String>,
    /// Client user agent.
    pub user_agent: Option<String>,
    /// Session status.
    pub status: SessionStatus,
    /// Reason for revocation (if revoked).
    pub revocation_reason: Option<String>,
    /// When the session was revoked (if revoked).
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Session {
    /// Check if the session is valid for authentication.
    pub fn is_valid(&self) -> bool {
        self.status == SessionStatus::Active && self.access_token_expires_at > chrono::Utc::now()
    }

    /// Check if the session has expired.
    pub fn is_expired(&self) -> bool {
        self.access_token_expires_at <= chrono::Utc::now()
    }

    /// Check if the session is revoked.
    pub fn is_revoked(&self) -> bool {
        self.status == SessionStatus::Revoked
    }
}

/// Authentication event type for audit logging.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthEventType {
    // Registration
    Register,
    RegisterAttempt,

    // Login
    LoginAttempt,
    LoginPassword,
    LoginTotp,
    LoginPasskey,
    LoginOAuth,
    LoginSuccess,
    LoginFailure,

    // Logout
    Logout,
    LogoutAll,

    // Session
    SessionRefresh,
    SessionRevoke,
    SessionExpire,

    // 2FA
    TwoFactorEnable,
    TwoFactorDisable,
    TwoFactorSetup,
    TwoFactorRecovery,

    // PassKey
    PasskeyRegister,
    PasskeyDelete,

    // OAuth
    OAuthConnect,
    OAuthDisconnect,
    OAuthLink,

    // Password
    PasswordChange,
    PasswordReset,

    // Account
    AccountSuspend,
    AccountActivate,
    AccountDelete,

    // Threat signals
    RateLimitExceeded,
    TokenTheftDetected,
}

/// An authentication event for audit logging.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthEvent {
    /// Unique event identifier.
    pub id: Uuid,
    /// Type of event.
    pub event_type: AuthEventType,
    /// Related user (null for system events).
    pub user_id: Option<Uuid>,
    /// Related session (null if no session).
    pub session_id: Option<Uuid>,
    /// Client IP address.
    pub ip_address: Option<String>,
    /// Client user agent.
    pub user_agent: Option<String>,
    /// Whether the event succeeded.
    pub success: bool,
    /// Event-specific details.
    pub details: serde_json::Value,
    /// When the event occurred.
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

/// Backup code for account recovery.
#[derive(Clone, PartialEq, Eq)]
pub struct BackupCode {
    /// The code (plain text, to be hashed before storage).
    pub code: String,
    /// Whether the code has been used.
    pub used: bool,
}

impl std::fmt::Debug for BackupCode {
    /// `code` is a plaintext single-use recovery credential, so it is never
    /// rendered. `used` stays visible because whether a code is spent is the
    /// diagnostic callers need.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BackupCode")
            .field("code", &"[REDACTED]")
            .field("used", &self.used)
            .finish()
    }
}

/// Builder for creating auth events.
#[derive(Debug, Clone, Default)]
pub struct AuthEventBuilder {
    event_type: Option<AuthEventType>,
    user_id: Option<Uuid>,
    session_id: Option<Uuid>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    success: bool,
    details: serde_json::Map<String, serde_json::Value>,
}

impl AuthEventBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the event.
    ///
    /// Returns an error instead of panicking when required fields are missing.
    pub fn try_build(self) -> Result<AuthEvent, AuthError> {
        let event_type = self
            .event_type
            .ok_or_else(|| AuthError::BadRequest("event_type is required".to_string()))?;

        Ok(AuthEvent {
            id: Uuid::new_v7(),
            event_type,
            user_id: self.user_id,
            session_id: self.session_id,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            success: self.success,
            details: serde_json::Value::Object(self.details),
            occurred_at: chrono::Utc::now(),
        })
    }

    /// Set the event type.
    pub fn event_type(mut self, event_type: AuthEventType) -> Self {
        self.event_type = Some(event_type);
        self
    }

    /// Set the user.
    pub fn user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Set the session.
    pub fn session_id(mut self, session_id: Uuid) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Set the IP address.
    pub fn ip_address(mut self, ip_address: impl Into<String>) -> Self {
        self.ip_address = Some(ip_address.into());
        self
    }

    /// Set the user agent.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Set whether the event succeeded.
    pub fn success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    /// Add event details.
    pub fn detail<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Serialize,
    {
        let key = key.into();
        let value = serde_json::to_value(value).unwrap_or_default();
        self.details.insert(key, value);
        self
    }

    /// Build the event.
    pub fn build(self) -> Result<AuthEvent, AuthError> {
        self.try_build()
    }
}

#[cfg(test)]
mod backup_code_debug_tests {
    use super::BackupCode;

    #[test]
    fn debug_redacts_the_plaintext_recovery_code() {
        let rendered = format!(
            "{:?}",
            BackupCode {
                code: "recovery-code-value".to_string(),
                used: false,
            }
        );

        assert!(!rendered.contains("recovery-code-value"));
        assert!(rendered.contains("[REDACTED]"));
        assert!(rendered.contains("used: false"));
    }
}
