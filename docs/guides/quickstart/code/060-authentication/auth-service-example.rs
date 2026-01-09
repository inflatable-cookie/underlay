//! Auth service implementation example
//!
//! This file demonstrates how to integrate all Underlay auth methods:
//! - JWT authentication
//! - Password authentication with Argon2id
//! - TOTP two-factor authentication
//! - WebAuthn/PassKey authentication
//! - Google OAuth2 with repository integration
//!
//! Copy this to your app's auth crate as a starting point.

use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
use underlay_auth::{
    AuthError, AuthProvider, AuthResult, Credential, CredentialMetadata, CredentialRepository,
    HasAuthProvider, Principal, RoleSet, User, UserRepository, UserStatus,
};
use underlay_auth_jwt::{JwtConfig, JwtService};
use underlay_auth_oauth::{
    GoogleOAuthAppService, GoogleOAuthConfig, GoogleOAuthService, OAuthCallbackRequest,
    OAuthLoginResult, OAuthLoginState,
};
use underlay_auth_password::{
    Argon2Hasher, FailedLoginAttempt, PasswordAuthError, PasswordAuthRepository,
    PasswordAuthResult, PasswordAuthService, PasswordConfig,
};
use underlay_auth_totp::{TotpConfig, TotpService, TwoFactorCode, TwoFactorVerified};
use underlay_auth_webauthn::{
    StartPasskeyRegistrationRequest, StoredPasskey, WebAuthnConfig, WebAuthnService,
};
use underlay_core::Uuid;

// ============================================================================
// 1. Principal Types (your domain-specific types)
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UserId(pub Uuid);

#[derive(Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub display_name: String,
    pub totp_secret_encrypted: Option<String>,
    pub totp_verified: bool,
    pub webauthn_credentials: Vec<StoredPasskey>,
    pub backup_code_hashes: Vec<String>,
    pub google_oauth_connected: bool,
    pub status: UserStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<underlay_auth::User> for User {
    fn from(u: underlay_auth::User) -> Self {
        Self {
            id: UserId(u.id),
            email: u.email,
            display_name: u.display_name,
            totp_secret_encrypted: None,
            totp_verified: false,
            webauthn_credentials: vec![],
            backup_code_hashes: vec![],
            google_oauth_connected: false,
            status: u.status,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

// ============================================================================
// 2. App State with All Auth Services
// ============================================================================

#[derive(Clone)]
pub struct AppState {
    pub jwt_service: Arc<JwtService>,
    pub dev_auth_enabled: bool,
    pub password_service: Arc<PasswordAuthService<impl PasswordAuthRepository + Send + Sync>>,
    pub totp_service: Arc<TotpService>,
    pub webauthn_service: Arc<WebAuthnService>,
    pub oauth_service: Arc<GoogleOAuthAppService<GoogleOAuthService>>,
    pub pool: Arc<sqlx::PgPool>,
}

impl HasAuthProvider for AppState {
    fn auth_provider(&self) -> &dyn AuthProvider {
        self.jwt_service.as_ref()
    }
}

// ============================================================================
// 3. Auth Provider Implementation
// ============================================================================

#[derive(Clone)]
pub struct AppAuthProvider {
    state: Arc<AppState>,
}

impl AppAuthProvider {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl AuthProvider for AppAuthProvider {
    async fn authenticate_bearer(&self, bearer_token: &str) -> AuthResult<Principal> {
        let claims = self
            .state
            .jwt_service
            .verify_access_token(bearer_token)
            .map_err(|e| e.into())?;

        Ok(Principal {
            user_id: claims.common.subject,
            roles: RoleSet::new(claims.roles),
        })
    }
}

// ============================================================================
// 4. Password Auth Repository Implementation
// ============================================================================

#[derive(Clone)]
pub struct PgAuthRepository {
    pool: Arc<sqlx::PgPool>,
}

impl PgAuthRepository {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PasswordAuthRepository for PgAuthRepository {
    async fn find_user_by_email(&self, email: &str) -> PasswordAuthResult<Option<User>> {
        let email = email.trim().to_lowercase();
        sqlx::query_as!(
            underlay_auth::User,
            r#"SELECT id, email, display_name, status, created_at, updated_at
               FROM auth_users WHERE LOWER(email) = $1"#,
            email
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))
        .map(|r| r.map(|u| u.into()))
    }

    async fn find_user_by_id(&self, user_id: Uuid) -> PasswordAuthResult<Option<User>> {
        sqlx::query_as!(
            underlay_auth::User,
            r#"SELECT id, email, display_name, status, created_at, updated_at
               FROM auth_users WHERE id = $1"#,
            user_id
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))
        .map(|r| r.map(|u| u.into()))
    }

    async fn find_password_credential(
        &self,
        user_id: Uuid,
    ) -> PasswordAuthResult<Option<Credential>> {
        sqlx::query_as!(
            Credential,
            r#"SELECT id, user_id, credential_type, secret_encrypted, metadata,
                      verified, created_at, updated_at, last_used_at
               FROM auth_credentials
               WHERE user_id = $1 AND credential_type = 'password'"#,
            user_id
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))
    }

    async fn create_password_credential(
        &self,
        user_id: Uuid,
        password_hash: &str,
    ) -> PasswordAuthResult<Credential> {
        let cred = sqlx::query_as!(
            Credential,
            r#"INSERT INTO auth_credentials
               (user_id, credential_type, secret_encrypted, metadata, verified,
                created_at, updated_at, last_used_at)
               VALUES ($1, 'password', $2, $3, true, NOW(), NOW(), NOW())
               RETURNING id, user_id, credential_type, secret_encrypted, metadata,
                         verified, created_at, updated_at, last_used_at"#,
            user_id,
            password_hash,
            serde_json::json!({ "algorithm": "argon2id", "memory_kb": 65536, "iterations": 3, "parallelism": 4 })
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))?;

        Ok(cred)
    }

    async fn update_password_credential(
        &self,
        credential_id: Uuid,
        password_hash: &str,
    ) -> PasswordAuthResult<()> {
        sqlx::query!(
            r#"UPDATE auth_credentials
               SET secret_encrypted = $1, updated_at = NOW()
               WHERE id = $2"#,
            password_hash,
            credential_id
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn delete_password_credential(&self, credential_id: Uuid) -> PasswordAuthResult<()> {
        sqlx::query!(
            r#"DELETE FROM auth_credentials WHERE id = $1"#,
            credential_id
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn record_failed_login(
        &self,
        user_id: Uuid,
        max_failed_attempts: u32,
        lockout_duration_seconds: u64,
    ) -> PasswordAuthResult<FailedLoginAttempt> {
        let now = chrono::Utc::now();

        sqlx::query!(
            r#"INSERT INTO auth_failed_logins (user_id, attempted_at, reason)
               VALUES ($1, NOW(), 'wrong_password')"#,
            user_id
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))?;

        let count: i64 = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM auth_failed_logins
               WHERE user_id = $1 AND attempted_at > NOW() - INTERVAL '1 hour'"#,
            user_id
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))?;

        let count = count as u32;
        let lockout_remaining_seconds = if count >= max_failed_attempts {
            let until = now + chrono::Duration::seconds(lockout_duration_seconds as i64);
            sqlx::query!(
                r#"UPDATE auth_users SET locked_until = $1 WHERE id = $2"#,
                until,
                user_id
            )
            .execute(&*self.pool)
            .await
            .map_err(|e| PasswordAuthError::Internal(e.to_string()))?;

            Some(lockout_duration_seconds)
        } else {
            None
        };

        Ok(FailedLoginAttempt {
            count,
            lockout_remaining_seconds,
        })
    }

    async fn reset_failed_logins(&self, user_id: Uuid) -> PasswordAuthResult<()> {
        sqlx::query!(
            r#"DELETE FROM auth_failed_logins WHERE user_id = $1"#,
            user_id
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))?;

        sqlx::query!(
            r#"UPDATE auth_users SET locked_until = NULL WHERE id = $1"#,
            user_id
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn get_failed_login_count(&self, user_id: Uuid) -> PasswordAuthResult<u32> {
        let count: i64 = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM auth_failed_logins
               WHERE user_id = $1 AND attempted_at > NOW() - INTERVAL '1 hour'"#,
            user_id
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))?;

        Ok(count as u32)
    }

    async fn get_lockout_remaining_seconds(
        &self,
        user_id: Uuid,
    ) -> PasswordAuthResult<Option<u64>> {
        let remaining: Option<i64> = sqlx::query_scalar!(
            r#"SELECT EXTRACT(EPOCH FROM (locked_until - NOW()))
               FROM auth_users WHERE id = $1 AND locked_until > NOW()"#,
            user_id
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))?;

        Ok(remaining.map(|r| r.max(1) as u64))
    }

    async fn check_rate_limit(
        &self,
        key: &str,
        max_attempts: u32,
        window_seconds: u64,
    ) -> PasswordAuthResult<(bool, u64)> {
        let window_start = chrono::Utc::now() - chrono::Duration::seconds(window_seconds as i64);

        let count: i64 = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM auth_rate_limits
               WHERE key = $1 AND window_start > $2"#,
            key,
            window_start
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))?;

        if count >= max_attempts as i64 {
            let retry_after: i64 = sqlx::query_scalar!(
                r#"SELECT EXTRACT(EPOCH FROM (MAX(window_start) + $1 * INTERVAL '1 second' - NOW()))
                   FROM auth_rate_limits WHERE key = $2"#,
                window_seconds,
                key
            )
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| PasswordAuthError::Internal(e.to_string()))?;

            return Ok((false, retry_after.max(60) as u64));
        }

        sqlx::query!(
            r#"INSERT INTO auth_rate_limits (key, count, window_start, expires_at)
               VALUES ($1, 1, NOW(), NOW() + $2 * INTERVAL '1 second')
               ON CONFLICT (key, window_start)
               DO UPDATE SET count = auth_rate_limits.count + 1"#,
            key,
            window_seconds
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| PasswordAuthError::Internal(e.to_string()))?;

        Ok((true, 0))
    }
}

// ============================================================================
// 5. TOTP Service Helpers
// ============================================================================

impl AppState {
    /// Generate TOTP setup for user (QR code for authenticator app)
    pub async fn totp_generate_setup(
        &self,
        user: &User,
    ) -> AuthResult<underlay_auth_totp::TotpSetup> {
        let setup = self.totp_service.setup(&user.email, 8)?; // 8 backup codes

        // Store encrypted secret and backup code hashes
        // (In real app: persist to database)
        Ok(setup)
    }

    /// Verify TOTP code during login
    pub async fn totp_verify(
        &self,
        user: &User,
        code: &str,
        now: SystemTime,
    ) -> AuthResult<TwoFactorVerified> {
        let Some(ref secret) = user.totp_secret_encrypted else {
            return Err(AuthError::TwoFactorNotEnabled);
        };

        self.totp_service
            .verify_second_factor(
                secret,
                None, // No replay protection in this example
                TwoFactorCode::Totp(code),
                &user.backup_code_hashes,
                now,
            )
            .map_err(|e| e.into())
    }

    /// Verify backup code
    pub async fn totp_verify_backup_code(&self, user: &User, code: &str) -> AuthResult<usize> {
        self.totp_service
            .verify_backup_code(code, &user.backup_code_hashes)
            .map_err(|e| e.into())
    }
}

// ============================================================================
// 6. WebAuthn Service Helpers
// ============================================================================

impl AppState {
    /// Start PassKey registration
    pub async fn webauthn_start_registration(
        &self,
        user_id: Uuid,
        user_name: &str,
        display_name: &str,
    ) -> AuthResult<underlay_auth_webauthn::StartPasskeyRegistrationResponse> {
        self.webauthn_service
            .start_passkey_registration_http(
                StartPasskeyRegistrationRequest {
                    user_id,
                    user_name: user_name.to_string(),
                    display_name: display_name.to_string(),
                    exclude_credential_ids: None,
                },
                |state| {
                    // TODO: Persist PasskeyRegistration state to Redis/database
                    // Return state_id
                    Ok(uuid::Uuid::new_v4().to_string())
                },
            )
            .await
    }

    /// Finish PassKey registration
    pub async fn webauthn_finish_registration(
        &self,
        state_id: &str,
        credential_json: &str,
    ) -> AuthResult<StoredPasskey> {
        let credential: serde_json::Value = serde_json::from_str(credential_json)
            .map_err(|_| AuthError::BadRequest("invalid credential format".into()))?;

        self.webauthn_service
            .finish_passkey_registration_http(
                underlay_auth_webauthn::FinishPasskeyRegistrationRequest {
                    state_id: state_id.to_string(),
                    credential,
                },
                |state_id| {
                    // TODO: Load PasskeyRegistration from Redis/database
                    Err(AuthError::BadRequest("state not found".into()))
                },
            )
            .await
            .map(|r| r.stored_passkey)
    }

    /// Start PassKey authentication
    pub async fn webauthn_start_authentication(
        &self,
        allowed_credential_ids: &[String],
    ) -> AuthResult<underlay_auth_webauthn::StartPasskeyAuthenticationResponse> {
        self.webauthn_service
            .start_passkey_authentication_http(
                underlay_auth_webauthn::StartPasskeyAuthenticationRequest {
                    allowed_credentials: allowed_credential_ids.to_vec(),
                },
                |state| {
                    // TODO: Persist PasskeyAuthentication state
                    Ok(uuid::Uuid::new_v4().to_string())
                },
            )
            .await
    }

    /// Finish PassKey authentication
    pub async fn webauthn_finish_authentication(
        &self,
        state_id: &str,
        credential_json: &str,
    ) -> AuthResult<underlay_auth_webauthn::AuthenticationResult> {
        let credential: serde_json::Value = serde_json::from_str(credential_json)
            .map_err(|_| AuthError::BadRequest("invalid credential format".into()))?;

        self.webauthn_service
            .finish_passkey_authentication_http(
                underlay_auth_webauthn::FinishPasskeyAuthenticationRequest {
                    state_id: state_id.to_string(),
                    credential,
                },
                |state_id| {
                    // TODO: Load PasskeyAuthentication from Redis/database
                    Err(AuthError::BadRequest("state not found".into()))
                },
            )
            .await
            .map(|r| r.result)
    }
}

// ============================================================================
// 7. OAuth Service Helpers
// ============================================================================

impl AppState {
    /// Start Google OAuth login
    pub fn oauth_start_login(&self) -> AuthResult<underlay_auth_oauth::OAuthStart> {
        self.oauth_service.initiate_google_login()
    }

    /// Start Google OAuth with custom state (session-based)
    pub fn oauth_start_login_with(
        &self,
        csrf_state: &str,
        pkce_verifier: &str,
    ) -> AuthResult<String> {
        self.oauth_service
            .initiate_google_login_with(csrf_state, pkce_verifier)
    }

    /// Handle complete OAuth callback (user creation + credential linking)
    pub async fn oauth_handle_callback<R>(
        &self,
        repo: &R,
        request: OAuthCallbackRequest,
        stored_state: OAuthLoginState,
    ) -> AuthResult<OAuthLoginResult>
    where
        R: UserRepository + CredentialRepository,
    {
        // Encrypt refresh token for storage
        let encrypt = |token: &str| {
            // Use your encryption library (e.g., AES-256-GCM)
            Ok(format!("encrypted:{}", token))
        };

        self.oauth_service
            .handle_google_callback(repo, request, stored_state, encrypt)
            .await
    }

    /// Refresh Google OAuth token
    pub async fn oauth_refresh_token(
        &self,
        refresh_token: &str,
    ) -> AuthResult<underlay_auth_oauth::TokenSet> {
        self.oauth_service.refresh_google_token(refresh_token).await
    }

    /// Disconnect Google OAuth from user account
    pub async fn oauth_disconnect<R>(&self, repo: &R, user_id: Uuid) -> AuthResult<()>
    where
        R: UserRepository + CredentialRepository,
    {
        self.oauth_service.disconnect_google(repo, user_id).await
    }
}

// ============================================================================
// 8. Factory Function
// ============================================================================

pub async fn create_app_state(pool: sqlx::PgPool) -> Result<Arc<AppState>, anyhow::Error> {
    let pool = Arc::new(pool);

    // JWT Service
    let jwt_config =
        JwtConfig::from_env().map_err(|e| anyhow::anyhow!("JWT config error: {}", e))?;
    let jwt_service = Arc::new(
        JwtService::new(jwt_config).map_err(|e| anyhow::anyhow!("JWT service error: {}", e))?,
    );

    // Password Service with repository
    let repo = Arc::new(PgAuthRepository::new(pool.clone()));
    let hasher = Arc::new(Argon2Hasher::new());
    let password_service = Arc::new(PasswordAuthService::new(
        repo.clone(),
        hasher.clone(),
        hasher,
        None,
    ));

    // TOTP Service
    let totp_service = Arc::new(TotpService::new(None));

    // WebAuthn Service
    let webauthn_service = Arc::new(
        WebAuthnService::new(WebAuthnConfig {
            rp_id: "myapp.com".to_string(),
            rp_origin: "https://myapp.com".to_string(),
            rp_name: "My App".to_string(),
        })
        .map_err(|e| anyhow::anyhow!("WebAuthn service error: {}", e))?,
    );

    // OAuth Service
    let google_oauth =
        GoogleOAuthService::from_env().map_err(|e| anyhow::anyhow!("OAuth config error: {}", e))?;
    let oauth_service = Arc::new(GoogleOAuthAppService::new(google_oauth));

    // Dev mode check
    let dev_auth_enabled = std::env::var("NURSERY_DEV_AUTH")
        .map(|v| v == "true")
        .unwrap_or(false);

    Ok(Arc::new(AppState {
        jwt_service,
        dev_auth_enabled,
        password_service,
        totp_service,
        webauthn_service,
        oauth_service,
        pool,
    }))
}

// ============================================================================
// 9. Usage Example: Auth Handlers
// ============================================================================

use axum::{Extension, Json};
use underlay_core::SingleResponse;

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
    #[serde(default)]
    totp_code: Option<String>,
    #[serde(default)]
    backup_code: Option<String>,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    access_token: String,
    refresh_token: String,
    requires_2fa: bool,
}

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Json<SingleResponse<LoginResponse>> {
    // 1. Verify password
    let user = match state
        .password_service
        .verify_login(&req.email, &req.password)
        .await
    {
        Ok(user) => user,
        Err(e) => {
            tracing::warn!(email = %req.email, error = ?e, "Login failed");
            return Json(SingleResponse {
                data: LoginResponse {
                    access_token: String::new(),
                    refresh_token: String::new(),
                    requires_2fa: false,
                },
            });
        }
    };

    // 2. Check if 2FA is required
    if user.totp_verified && req.totp_code.is_none() && req.backup_code.is_none() {
        return Json(SingleResponse {
            data: LoginResponse {
                access_token: String::new(),
                refresh_token: String::new(),
                requires_2fa: true,
            },
        });
    }

    // 3. Verify 2FA if provided
    // ... (implement TOTP verification)

    // 4. Issue session tokens
    // let (access, refresh) = state.jwt_service.issue_tokens(...).await?;

    // Placeholder response
    Json(SingleResponse {
        data: LoginResponse {
            access_token: "placeholder".to_string(),
            refresh_token: "placeholder".to_string(),
            requires_2fa: false,
        },
    })
}

#[derive(serde::Deserialize)]
pub struct TotpVerifyRequest {
    email: String,
    code: String,
}

pub async fn login_totp_verify(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<TotpVerifyRequest>,
) -> Json<SingleResponse<LoginResponse>> {
    // 1. Look up user
    // let user = find_user_by_email(&req.email).await;

    // 2. Verify TOTP code
    // let verified = state.totp_verify(&user, &req.code, SystemTime::now()).await?;

    // 3. On success, issue session tokens
    // let (access, refresh) = state.jwt_service.issue_tokens(...).await?;

    // Placeholder response
    Json(SingleResponse {
        data: LoginResponse {
            access_token: "placeholder".to_string(),
            refresh_token: "placeholder".to_string(),
            requires_2fa: false,
        },
    })
}

// ============================================================================
// 10. Password Auth Handlers (Complete Example)
// ============================================================================

use axum::{TypedHeader, headers};
use std::net::SocketAddr;

#[derive(serde::Deserialize)]
pub struct PasswordLoginRequest {
    email: String,
    password: String,
}

#[derive(serde::Serialize)]
pub struct PasswordLoginResponse {
    user_id: Uuid,
    email: String,
    display_name: String,
}

pub async fn password_login(
    Extension(state): Extension<Arc<AppState>>,
    TypedHeader(headers::UserAgent(user_agent)): Option<TypedHeader<headers::UserAgent>>,
    remote_addr: Option<Extension<SocketAddr>>,
    Json(req): Json<PasswordLoginRequest>,
) -> Json<SingleResponse<PasswordLoginResponse>> {
    let ip = remote_addr.map(|addr| addr.to_string());

    match state
        .password_service
        .verify_login_with_context(&req.email, &req.password, ip.as_deref())
        .await
    {
        Ok(user) => {
            tracing::info!(user_id = %user.id, email = %user.email, "Login successful");
            Json(SingleResponse {
                data: PasswordLoginResponse {
                    user_id: user.id,
                    email: user.email,
                    display_name: user.display_name,
                },
            })
        }
        Err(e) => {
            tracing::warn!(email = %req.email, error = ?e, "Login failed");
            Json(SingleResponse {
                data: PasswordLoginResponse {
                    user_id: Uuid::nil(),
                    email: String::new(),
                    display_name: String::new(),
                },
            })
        }
    }
}

#[derive(serde::Deserialize)]
pub struct SetPasswordRequest {
    current_password: Option<String>,
    new_password: String,
}

#[derive(serde::Serialize)]
pub struct SetPasswordResponse {
    success: bool,
    message: String,
}

pub async fn set_password(
    Authenticated(principal): Authenticated<Principal>,
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<SetPasswordRequest>,
) -> Json<SingleResponse<SetPasswordResponse>> {
    let user_id = principal.user_id;

    let result = match req.current_password {
        Some(current) => {
            state
                .password_service
                .change_password(user_id, &current, &req.new_password)
                .await
        }
        None => state
            .password_service
            .set_password(user_id, &req.new_password)
            .await
            .map(|_| ()),
    };

    match result {
        Ok(_) => {
            tracing::info!(user_id = %user_id, "Password updated");
            Json(SingleResponse {
                data: SetPasswordResponse {
                    success: true,
                    message: "Password updated successfully".to_string(),
                },
            })
        }
        Err(e) => {
            tracing::error!(user_id = %user_id, error = ?e, "Password update failed");
            let message = match e {
                PasswordAuthError::WrongPassword => "Current password is incorrect",
                PasswordAuthError::PasswordTooWeak(feedback) => &feedback,
                PasswordAuthError::PasswordCompromised => {
                    "Password has been found in a data breach"
                }
                PasswordAuthError::PasswordSameAsCurrent => {
                    "New password must be different from current"
                }
                _ => "Failed to update password",
            };
            Json(SingleResponse {
                data: SetPasswordResponse {
                    success: false,
                    message: message.to_string(),
                },
            })
        }
    }
}

#[derive(serde::Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
}

pub async fn request_password_reset(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<PasswordResetRequest>,
) -> Json<SingleResponse<()>> {
    // 1. Look up user
    let user = match state.password_service.verify_login(&req.email, "").await {
        Ok(user) => user,
        Err(_) => {
            // Don't reveal whether email exists
            return Json(SingleResponse { data: () });
        }
    };

    // 2. Generate reset token
    // let reset_token = generate_reset_token();

    // 3. Store reset token with expiry
    // let expires_at = Utc::now() + Duration::hours(1);

    // 4. Send email (implement email sending)
    // send_password_reset_email(&user.email, &reset_token);

    tracing::info!(user_id = %user.id, "Password reset requested");

    Json(SingleResponse { data: () })
}

// ============================================================================
// 11. Password Auth Error Type (re-export)
// ============================================================================

pub use underlay_auth_password::errors::PasswordAuthError;
