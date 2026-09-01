use serde::{Deserialize, Serialize};

use crate::{Credential, User};

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenSet {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub expires_in_seconds: Option<u64>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OAuthStart {
    pub authorization_url: String,
    pub csrf_state: String,
    pub pkce_verifier: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OAuthCallbackRequest {
    pub code: String,
    pub state: String,
}

impl std::fmt::Debug for OAuthCallbackRequest {
    /// `code` is the single-use OAuth authorization code and is exchangeable
    /// for tokens until it is redeemed, so it is never rendered. `state` is a
    /// CSRF correlator and stays visible for callback debugging.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OAuthCallbackRequest")
            .field("code", &redacted(&self.code))
            .field("state", &self.state)
            .finish()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OAuthLoginState {
    pub csrf_state: String,
    pub pkce_verifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthLoginResult {
    pub user: User,
    pub is_new_user: bool,
    pub credential: Credential,
    pub token_set: TokenSet,
    pub userinfo: GoogleUserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
}

impl std::fmt::Debug for TokenSet {
    /// The three token fields are live provider credentials, so none is
    /// rendered. Scope, type, and lifetime stay visible because they are the
    /// fields an operator actually debugs.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenSet")
            .field("access_token", &redacted(&self.access_token))
            .field(
                "refresh_token",
                &self.refresh_token.as_deref().map(redacted),
            )
            .field("id_token", &self.id_token.as_deref().map(redacted))
            .field("expires_in_seconds", &self.expires_in_seconds)
            .field("scope", &self.scope)
            .field("token_type", &self.token_type)
            .finish()
    }
}

impl std::fmt::Debug for OAuthStart {
    /// `pkce_verifier` must stay secret between authorization and token
    /// exchange, so it is never rendered.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OAuthStart")
            .field("authorization_url", &self.authorization_url)
            .field("csrf_state", &self.csrf_state)
            .field("pkce_verifier", &redacted(&self.pkce_verifier))
            .finish()
    }
}

impl std::fmt::Debug for OAuthLoginState {
    /// `pkce_verifier` must stay secret between authorization and token
    /// exchange, so it is never rendered.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OAuthLoginState")
            .field("csrf_state", &self.csrf_state)
            .field("pkce_verifier", &redacted(&self.pkce_verifier))
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

#[cfg(test)]
mod debug_redaction_tests {
    use super::{OAuthLoginState, OAuthStart, TokenSet};

    fn token_set() -> TokenSet {
        TokenSet {
            access_token: "ya29.access-secret".to_string(),
            refresh_token: Some("1//refresh-secret".to_string()),
            id_token: Some("header.id-secret.signature".to_string()),
            expires_in_seconds: Some(3599),
            scope: Some("openid email".to_string()),
            token_type: Some("Bearer".to_string()),
        }
    }

    #[test]
    fn debug_redacts_every_provider_token() {
        let rendered = format!("{:?}", token_set());

        assert!(!rendered.contains("access-secret"));
        assert!(!rendered.contains("refresh-secret"));
        assert!(!rendered.contains("id-secret"));
        assert_eq!(rendered.matches("[REDACTED]").count(), 3);
        assert!(rendered.contains("openid email"));
        assert!(rendered.contains("3599"));
    }

    #[test]
    fn debug_omits_optional_tokens_that_are_not_set() {
        let mut tokens = token_set();
        tokens.refresh_token = None;
        tokens.id_token = None;

        let rendered = format!("{tokens:?}");

        assert_eq!(rendered.matches("[REDACTED]").count(), 1);
        assert_eq!(rendered.matches("None").count(), 2);
    }

    #[test]
    fn debug_redacts_the_pkce_verifier_but_keeps_csrf_state() {
        let start = OAuthStart {
            authorization_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            csrf_state: "csrf-state-value".to_string(),
            pkce_verifier: "pkce-verifier-secret".to_string(),
        };

        let rendered = format!("{start:?}");

        assert!(!rendered.contains("pkce-verifier-secret"));
        assert!(rendered.contains("[REDACTED]"));
        assert!(rendered.contains("csrf-state-value"));
    }

    #[test]
    fn callback_request_debug_redacts_the_authorization_code() {
        let request = super::OAuthCallbackRequest {
            code: "4/0AX4XfWh-authorization-code".to_string(),
            state: "csrf-state-value".to_string(),
        };

        let rendered = format!("{request:?}");

        assert!(!rendered.contains("4/0AX4XfWh-authorization-code"));
        assert!(rendered.contains("[REDACTED]"));
        assert!(rendered.contains("csrf-state-value"));
    }

    #[test]
    fn login_result_debug_redacts_the_nested_credential_and_tokens() {
        use crate::{Credential, User};
        use underlay_auth::{CredentialMetadata, CredentialType, UserStatus};
        use underlay_core::Uuid;

        let now = chrono::Utc::now();
        let result = super::OAuthLoginResult {
            user: User {
                id: Uuid::new_v7(),
                email: "person@example.com".to_string(),
                display_name: None,
                status: UserStatus::Active,
                created_at: now,
                updated_at: now,
            },
            is_new_user: false,
            credential: Credential {
                id: Uuid::new_v7(),
                user_id: Uuid::new_v7(),
                credential_type: CredentialType::OAuthGoogle,
                secret_encrypted: "enc:v1:stored-ciphertext".to_string(),
                metadata: CredentialMetadata::OAuthGoogle {
                    google_user_id: "sub-123".to_string(),
                    scopes: Vec::new(),
                },
                verified: true,
                created_at: now,
                updated_at: now,
                last_used_at: None,
            },
            token_set: token_set(),
            userinfo: super::GoogleUserInfo {
                sub: "sub-123".to_string(),
                email: None,
                email_verified: None,
                name: None,
                given_name: None,
                family_name: None,
                picture: None,
                locale: None,
            },
        };

        let rendered = format!("{result:?}");

        assert!(!rendered.contains("enc:v1:stored-ciphertext"));
        assert!(!rendered.contains("access-secret"));
        assert!(!rendered.contains("refresh-secret"));
        assert!(rendered.contains("person@example.com"));
    }

    #[test]
    fn login_state_debug_redacts_the_pkce_verifier() {
        let state = OAuthLoginState {
            csrf_state: "csrf-state-value".to_string(),
            pkce_verifier: "pkce-verifier-secret".to_string(),
        };

        let rendered = format!("{state:?}");

        assert!(!rendered.contains("pkce-verifier-secret"));
        assert!(rendered.contains("[REDACTED]"));
    }
}
