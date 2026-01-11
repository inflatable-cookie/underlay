# 060 - Authentication

> **Reference Implementation**: This guide includes patterns from Acowtancy, a production application built with Underlay. These serve as working examples of best practices.

This document covers implementing authentication using the Underlay auth system. Underlay provides multiple authentication methods:

- **JWT tokens** (production) with Ed25519/EdDSA
- **Dev mode** for local development
- **Password** authentication with Argon2id
- **TOTP** two-factor authentication
- **WebAuthn/PassKey** passwordless authentication
- **OAuth2** (Google) social login

## Auth Module Structure

The auth crate is organized into three modules following the Underlay pattern:

```
apps/nursery/crates/auth/src/
├── lib.rs           # Module declarations and exports
├── principal.rs     # UserId, UserPrincipal, UserRole types
├── provider.rs      # JWT provider wrapper (Ed25519 / EdDSA)
└── underlay.rs      # DevBearerUuidAuthProvider, converters
```

## Key Concepts

### 1. Principal Module

Defines domain-specific auth types in `principal.rs`:

```rust
use underlay_core::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
    Guest,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPrincipal {
    pub user_id: UserId,
    pub roles: Vec<UserRole>,
    pub email: Option<String>,
    pub display_name: Option<String>,
}
```

### 2. Provider Module

Your app implements Underlay's provider boundary (`underlay_auth::AuthProvider`).

For production JWT, use Underlay's `underlay-auth-jwt` crate (it uses `jsonwebtoken` under the hood) and validate **Ed25519 / EdDSA** tokens.

```rust
use async_trait::async_trait;
use underlay_auth::{AuthProvider, AuthResult, Principal, RoleSet};
use underlay_auth_jwt::JwtService;

#[derive(Clone)]
pub struct JwtAuthProvider {
    jwt: JwtService,
}

impl JwtAuthProvider {
    pub fn new(jwt: JwtService) -> Self {
        Self { jwt }
    }
}

#[async_trait]
impl AuthProvider for JwtAuthProvider {
    async fn authenticate_bearer(&self, bearer_token: &str) -> AuthResult<Principal> {
        let claims = self
            .jwt
            .verify_access_token(bearer_token)
            .map_err(|e| e.into())?;

        Ok(Principal {
            user_id: claims.common.subject,
            roles: RoleSet::new(claims.roles),
        })
    }
}
```

### 3. Underlay Module

Connects to Underlay and provides dev mode in `underlay.rs`:

```rust
use underlay_core::Uuid;

use crate::{UserId, UserPrincipal, UserRole};

pub struct DevBearerUuidAuthProvider;

#[async_trait::async_trait]
impl underlay_auth::AuthProvider for DevBearerUuidAuthProvider {
    async fn authenticate_bearer(
        &self,
        bearer_token: &str,
    ) -> underlay_auth::AuthResult<underlay_auth::Principal> {
        let user_id =
            Uuid::parse_str(bearer_token).map_err(|_| underlay_auth::AuthError::InvalidToken)?;

        Ok(underlay_auth::Principal {
            user_id,
            roles: underlay_auth::RoleSet::new(["admin"]),
        })
    }
}

pub fn user_principal_from_underlay(principal: underlay_auth::Principal) -> UserPrincipal {
    let roles: Vec<UserRole> = principal
        .roles
        .iter()
        .filter_map(|r| match r.to_ascii_lowercase().as_str() {
            "admin" => Some(UserRole::Admin),
            "user" => Some(UserRole::User),
            _ => None,
        })
        .collect();

    UserPrincipal {
        user_id: UserId(principal.user_id),
        roles,
        email: None,
        display_name: None,
    }
}
```

## lib.rs

Export the public API:

```rust
mod principal;
mod provider;
mod underlay;

pub use principal::{UserId, UserPrincipal, UserRole};

// Session types
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AuthSession {
    pub user_id: UserId,
    pub session_id: underlay_core::Uuid,
    pub access_token: String,
    pub refresh_token: String,
}

// Re-export Underlay boundary types (optional convenience)
pub use underlay_auth::{AuthError, AuthProvider};

// Providers
pub use provider::JwtAuthProvider;
pub use underlay::{user_principal_from_underlay, DevBearerUuidAuthProvider};
```

## Dev vs Prod Modes

| Mode | Provider | Security | Use Case |
|------|----------|----------|----------|
| Development | `DevBearerUuidAuthProvider` | ⚠️ NONE | Local development only |
| Production | `JwtAuthProvider` (backed by `underlay-auth-jwt`) | ✅ Secure | Production deployment |

**Critical:** Never enable dev mode in production.

## Implementation Steps

### Step 1: AppState with Auth

In `apps/nursery/crates/api/src/main.rs`:

```rust
use myapp_auth::{AuthProvider, user_principal_from_underlay};

#[derive(Clone)]
pub struct AppState {
    pub auth_provider: Arc<dyn AuthProvider>,
    pub auth_state: underlay_auth_state::AuthStateStore,
    pub pool: myapp_db::PgPool,
}

impl underlay_auth::HasAuthProvider for AppState {
    fn auth_provider(&self) -> &dyn AuthProvider {
        self.auth_provider.as_ref()
    }
}
```

### Step 2: Auth Service (Login & Session)

Create `apps/nursery/crates/auth/src/service.rs` to handle login logic and token issuance:

```rust
use std::sync::Arc;
use underlay_auth::{AuthError, AuthResult};
use underlay_auth_jwt::JwtService;
use farmyard_core::Uuid; // or underlay_core::Uuid

use crate::{AuthSession, UserId};
// Import your PasswordAuth, TotpAuthService etc.

pub struct MyAppAuthService {
    jwt: JwtService,
    // Add other services/repos here
    // password: Arc<PasswordAuth>,
    // pool: sqlx::PgPool,
}

impl MyAppAuthService {
    pub fn new(jwt: JwtService) -> Self {
        Self { jwt }
    }

    pub async fn create_session(&self, user_id: Uuid, roles: Vec<String>) -> AuthResult<AuthSession> {
        let session_id = Uuid::new_v7();

        // Issue access token (short lived)
        let (access_token, _) = self.jwt
            .issue_access_token(user_id, session_id, roles.clone())
            .map_err(AuthError::from)?;

        // Issue refresh token (long lived)
        let (refresh_token, _) = self.jwt
            .issue_refresh_token(user_id, session_id, None, 1) // version 1
            .map_err(AuthError::from)?;

        // In a real app, you MUST persist the session in DB here using `sessions` table
        // (see 050-database.md for schema).
        // See `underlay-auth` docs for implementing session validation.

        Ok(AuthSession {
            user_id: UserId(user_id),
            session_id,
            access_token,
            refresh_token,
        })
    }

    // Example login combining Password + Session
    /*
    pub async fn login(&self, email: &str, password: &str) -> AuthResult<AuthSession> {
        let user = self.password.verify_login(email, password).await?;
        let roles = vec!["user".to_string()]; // fetch roles from DB
        self.create_session(user.id, roles).await
    }
    */
}
```

### Step 3: Auth Provider Selection

```rust
use myapp_auth::{DevBearerUuidAuthProvider, JwtAuthProvider};
use underlay_auth_jwt::{JwtConfig, JwtService};

fn create_auth_provider() -> Arc<dyn underlay_auth::AuthProvider> {
    let dev_auth_enabled = std::env::var("NURSERY_DEV_AUTH")
        .map(|v| v == "true")
        .unwrap_or(false);

    match JwtConfig::from_env().ok().and_then(|cfg| JwtService::new(cfg).ok()) {
        Some(jwt) => Arc::new(JwtAuthProvider::new(jwt)),
        None if dev_auth_enabled => {
            tracing::warn!("DEV AUTH ENABLED - NEVER USE IN PRODUCTION");
            Arc::new(DevBearerUuidAuthProvider)
        }
        None => {
            tracing::error!(
                "Auth not configured. Set AUTH_JWT_* env vars or NURSERY_DEV_AUTH=true"
            );
            // In a real app, you might want to panic or exit, but for dev we might allow continuing
            // if we are not handling any requests yet.
            std::process::exit(1);
        }
    }
}
```

### Step 4: Protected Routes

```rust
use axum::{
    Json,
    extract::{Path, State},
};
use underlay_auth::Authenticated;
use underlay_core::SingleResponse;

use crate::state::AppState;

pub async fn list_artists(
    State(state): State<AppState>,
    Authenticated(principal): Authenticated<myapp_auth::UserPrincipal>,
) -> Json<SingleResponse<Vec<ArtistDto>>> {
    tracing::info!(user_id = %principal.user_id.0, "Listing artists");

    // Placeholder implementation for the quickstart.
    Json(SingleResponse { data: Vec::new() })
}
```

---

## TOTP Two-Factor Authentication

Underlay provides `underlay-auth-totp` for Time-based One-Time Password support.

### Adding TOTP to Your App

```toml
# apps/nursery/crates/auth/Cargo.toml
[dependencies]
underlay-auth-totp = { path = "../../../underlay/rust/crates/underlay-auth-totp" }
underlay-auth-state = { path = "../../../underlay/rust/crates/underlay-auth-state" }
```

### TOTP Service Setup

```rust
use underlay_auth_totp::{TotpSetup, TotpError};
use underlay_auth_state::AuthStateStore; // Add this

pub struct TotpAuthService {
    totp: TotpService,
    state: AuthStateStore, // Add this
}

impl TotpAuthService {
    pub fn new(state: AuthStateStore) -> Self { // Update constructor
        let config = TotpConfig {
            issuer: "MyApp".to_string(),
            digits: 6,
            period_seconds: 30,
            skew_steps: 1,
            algorithm: underlay_auth_totp::TotpAlgorithm::Sha1,
        };
        Self {
            totp: TotpService::new(Some(config)),
            state,
        }
    }
}
```

### Generating TOTP Setup (QR Code for User)

```rust
use underlay_auth_totp::{TotpSetup, TotpError};

impl TotpAuthService {
    /// Generate TOTP secret, provisioning URI, and QR code for user setup
    pub fn generate_setup(
        &self,
        account_name: &str,  // e.g., "user@example.com"
        backup_code_count: usize,  // e.g., 8
    ) -> Result<TotpSetup, AuthError> {
        self.totp
            .setup(account_name, backup_code_count)
            .map_err(|e| e.into())
    }
}
```

The `TotpSetup` struct contains:
- `secret`: Base32-encoded secret for manual entry
- `otpauth_uri`: `otpauth://totp/...` URI for authenticator apps
- `qr_svg`: SVG QR code image (render directly in UI)
- `backup_codes`: One-time use codes (show once, user must save)
- `backup_code_hashes`: Hashed codes for storage/verification
- `metadata`: Credential metadata for your credential store

### Verifying TOTP Codes

```rust
use underlay_auth_totp::{TwoFactorCode, TwoFactorVerified};

impl TotpAuthService {
    /// Verify a TOTP code or backup code during login
    pub fn verify_second_factor(
        &self,
        secret_base32: &str,
        last_counter: Option<u64>,
        code: &str,
        backup_code_hashes: &[String],
        now: SystemTime,
    ) -> AuthResult<TwoFactorVerified> {
        self.totp
            .verify_second_factor(
                secret_base32,
                last_counter,
                TwoFactorCode::Totp(code),
                backup_code_hashes,
                now,
            )
    }

    /// Verify just a backup code
    pub fn verify_backup_code(
        &self,
        input: &str,
        stored_hashes: &[String],
    ) -> AuthResult<usize> {
        self.totp
            .verify_backup_code(input, stored_hashes)
            .map_err(|e| e.into())
    }
}
```

### Database Schema for TOTP

Add to your auth schema (see `docs/architecture/050-auth-database-schema.md`):

```sql
-- TOTP secrets are stored in auth_credentials with type = 'totp'
-- secret_encrypted: AES-256-GCM encrypted secret
-- metadata: { issuer, algorithm, digits, period }

-- Backup codes stored separately (one-time use)
CREATE TABLE auth_backup_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth_users(id) ON DELETE CASCADE,
    code_hash VARCHAR(255) NOT NULL UNIQUE,
    used_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_auth_backup_codes_user_id ON auth_backup_codes(user_id);
```

### TOTP Login Flow

```
1. User submits email/password
2. If user has TOTP credential:
   - Return 2FA required response
   - Client shows TOTP input field
3. User submits TOTP code or backup code
4. Verify using TotpAuthService::verify_second_factor()
5. On success, issue session tokens
```

---

## WebAuthn / PassKey Authentication

Underlay provides `underlay-auth-webauthn` for passwordless authentication using WebAuthn/PassKeys.

### Adding WebAuthn to Your App

```toml
# apps/nursery/crates/auth/Cargo.toml
[dependencies]
underlay-auth-webauthn = { path = "../../../underlay/rust/crates/underlay-auth-webauthn" }
```

### WebAuthn Service Setup

```rust
use underlay_auth_webauthn::{WebAuthnService, WebAuthnConfig};
use underlay_auth_state::AuthStateStore;
use chrono::Duration;

pub struct WebAuthnAuthService {
    webauthn: WebAuthnService,
    state: AuthStateStore,
}

impl WebAuthnAuthService {
    pub fn new(rp_id: &str, rp_origin: &str, rp_name: &str, state: AuthStateStore) -> AuthResult<Self> {
        let config = WebAuthnConfig {
            rp_id: rp_id.to_string(),
            rp_origin: rp_origin.to_string(),
            rp_name: rp_name.to_string(),
        };
        let webauthn = WebAuthnService::new(config)?;
        Ok(Self { webauthn, state })
    }
}
```

### PassKey Registration (User adds a PassKey)

```rust
use underlay_auth_webauthn::{
    StartPasskeyRegistrationRequest, StartPasskeyRegistrationResponse,
    FinishPasskeyRegistrationRequest, FinishPasskeyRegistrationResponse,
    StoredPasskey,
};
use underlay_core::Uuid;

impl WebAuthnAuthService {
    /// Step 1: Start registration
    pub async fn start_registration(
        &self,
        user_id: Uuid,
        user_name: &str,
        display_name: &str,
    ) -> AuthResult<StartPasskeyRegistrationResponse> {
        let (options, state) = self.webauthn.start_passkey_registration(
            user_id,
            user_name,
            display_name,
            None, // exclude credentials
        )?;

        let encoded = WebAuthnService::encode_registration_state(&state)?;
        
        let state_id = self.state.create_user(
            user_id,
            "passkey_registration",
            serde_json::Value::String(encoded),
            Duration::minutes(15),
        ).await?;

        Ok(StartPasskeyRegistrationResponse {
            options,
            state_id: state_id.to_string(),
        })
    }

    /// Step 2: Finish registration
    pub async fn finish_registration(
        &self,
        user_id: Uuid,
        state_id: &str,
        credential_json: &str,
    ) -> AuthResult<FinishPasskeyRegistrationResponse> {
        let state_uuid = Uuid::parse_str(state_id).map_err(|_| AuthError::BadRequest("invalid state id".into()))?;
        
        let value = self.state
            .consume_user(user_id, state_uuid, "passkey_registration")
            .await?
            .ok_or_else(|| AuthError::BadRequest("invalid or expired registration state".into()))?;

        let encoded = value.as_str().ok_or_else(|| AuthError::Internal("invalid state format".into()))?;
        let state = WebAuthnService::decode_registration_state(encoded)?;

        // Parse credential JSON
        let credential = serde_json::from_str(credential_json)
             .map_err(|_| AuthError::BadRequest("invalid credential format".into()))?;

        // Finish logic (returns passkey)
        let passkey = self.webauthn.finish_passkey_registration(&state, &credential)?;
        
        // Store the passkey in the database
        let stored_passkey = self.webauthn.stored_passkey_from_passkey(\u0026passkey)?;
        let secret = serde_json::to_string(\u0026stored_passkey)
            .map_err(|_| AuthError::Internal("failed to encode passkey".into()))?;
        let metadata = WebAuthnService::credential_metadata_from_stored_passkey(\u0026stored_passkey);
        
        // Save to your repository (example assumes a credential repository pattern)
        let created = credential_repo.create(
            user_id,
            CredentialType::Passkey,
            \u0026secret,
            \u0026metadata,
        ).await?;
        
        Ok(FinishPasskeyRegistrationResponse {
            credential_id: created.id.to_string(),
        })
    }
}
```

### PassKey Authentication (Login)

```rust
use underlay_auth_webauthn::{
    StartPasskeyAuthenticationRequest, StartPasskeyAuthenticationResponse,
    FinishPasskeyAuthenticationRequest, FinishPasskeyAuthenticationResponse,
};

impl WebAuthnAuthService {
    /// Step 1: Start authentication
    pub async fn start_authentication(
        &self,
        allowed_credential_ids: &[String],
    ) -> AuthResult<StartPasskeyAuthenticationResponse> {
        // Convert Base64url strings to descriptors/passkeys if needed.
        // Simplified:
        let allowed_credentials = vec![]; // Populate from input

        let (options, state) = self.webauthn.start_passkey_authentication(allowed_credentials)?;
        let encoded = WebAuthnService::encode_authentication_state(&state)?;

        let state_id = self.state.create_public(
            "passkey_authentication",
            serde_json::Value::String(encoded),
            Duration::minutes(5),
        ).await?;

        Ok(StartPasskeyAuthenticationResponse {
            options,
            state_id: state_id.to_string(),
        })
    }

    /// Step 2: Finish authentication
    pub async fn finish_authentication(
        &self,
        state_id: &str,
        credential_json: &str,
    ) -> AuthResult<FinishPasskeyAuthenticationResponse> {
        let state_uuid = Uuid::parse_str(state_id).map_err(|_| AuthError::BadRequest("invalid state id".into()))?;

        let value = self.state
            .consume_public(state_uuid, "passkey_authentication")
            .await?
            .ok_or_else(|| AuthError::BadRequest("invalid or expired auth state".into()))?;

        let encoded = value.as_str().ok_or_else(|| AuthError::Internal("invalid state format".into()))?;
        let state = WebAuthnService::decode_authentication_state(encoded)?;

        let credential = serde_json::from_str(credential_json)
            .map_err(|_| AuthError::BadRequest("invalid credential format".into()))?;

        let result = self.webauthn.finish_passkey_authentication(&credential, &state)?;
        
        // Verify the credential against your database
        let credential_id = WebAuthnService::authentication_result_credential_id_base64url(\u0026result)?;
        
        let (user_id, _passkey_credential_id, stored_passkey) = credential_repo
            .find_passkey_by_credential_id(\u0026credential_id)
            .await?
            .ok_or(AuthError::PassKeyCredentialNotFound)?;
        
        // Update credential counter to prevent replay attacks
        self.webauthn.update_passkey_credential(\u0026result, \u0026stored_passkey)?;
        credential_repo.update_counter(user_id, result.counter()).await?;
        
        // Issue a session for the authenticated user
        let session = self.issue_session(user_id).await?;
        
        Ok(FinishPasskeyAuthenticationResponse {
            user_id: user_id.to_string(),
            access_token: session.access_token,
            refresh_token: session.refresh_token,
        })
    }
}
```

### Storing PassKeys

```rust
impl WebAuthnAuthService {
    /// Convert Passkey to storable format
    pub fn encode_passkey(&self, passkey: &Passkey) -> AuthResult<String> {
        self.webauthn.encode_passkey(passkey)
    }

    /// Load Passkey from storage
    pub fn decode_passkey(&self, encoded: &str) -> AuthResult<Passkey> {
        self.webauthn.decode_passkey(encoded)
    }

    /// Create stored passkey struct for database
    pub fn stored_passkey_from_passkey(&self, passkey: &Passkey) -> AuthResult<StoredPasskey> {
        self.webauthn.stored_passkey_from_passkey(passkey)
    }
}
```

### Database Schema for WebAuthn

```sql
-- PassKeys stored in auth_credentials with type = 'passkey'
-- secret_encrypted: CBOR-encoded credential data (or JSON via webauthn-rs serialization)
-- metadata: { credential_id, transports, last_counter }

-- The underlay-auth-webauthn crate uses JSON serialization:
-- StoredPasskey { credential_id, passkey_json, counter }
```

### WebAuthn Login Flow

```
Registration:
1. User navigates to security settings
2. Client calls POST /auth/passkey/register/start
3. Server returns challenge + state_id
4. Browser WebAuthn API creates credential
5. Client POSTs credential to /auth/passkey/register/finish
6. Server verifies and stores passkey

Login:
1. User clicks "Sign in with PassKey"
2. Client calls POST /auth/passkey/auth/start (with allowed credentials)
3. Server returns challenge + state_id
4. Browser WebAuthn API signs challenge
5. Client POSTs credential to /auth/passkey/auth/finish
6. Server verifies and creates session
```

---

## Multi-Step Authentication Flows

Many authentication methods require multiple request-response cycles. Examples include:

- **Password + TOTP**: User enters password first, then 2FA code in a second step
- **WebAuthn registration**: Start creates challenge, finish verifies credential
- **OAuth**: Redirect to provider, then callback processes result

These flows share a common pattern: **temporary state management with TTL**.

### State Management Pattern

Underlay provides `AuthStateStore` (from `underlay-auth-state`) for managing short-lived authentication state between request steps.

**Key concepts:**

1. **Create state** with TTL when starting a flow
2. **Return state ID** to client for the next request
3. **Consume state** (one-time use) when finishing the flow
4. **Automatic expiry** prevents replay attacks and cleans up abandoned flows

### AuthStateStore API

```rust
use underlay_auth_state::AuthStateStore;
use underlay_core::Uuid;
use chrono::Duration;
use serde_json::Value;

pub struct AuthStateStore {
    pool: sqlx::PgPool,
}

impl AuthStateStore {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    /// Create new state (public or user-specific)
    pub async fn create(
        &self,
        user_id: Option<Uuid>,  // None for public (unauthenticated) flows
        state_type: &str,        // e.g., "login_2fa", "passkey_registration"
        state: Value,            // Arbitrary JSON state
        ttl: Duration,           // How long state is valid
    ) -> Result<Uuid, AuthStateError> {
        // Returns state_id to pass to client
    }

    /// Load state without consuming (for multiple attempts)
    pub async fn load_public(
        &self,
        state_id: Uuid,
        state_type: &str,
    ) -> Result<Option<Value>, AuthStateError> {
        // Returns state if valid and not expired
    }

    /// Update state (e.g., increment attempt counter)
    pub async fn update_public(
        &self,
        state_id: Uuid,
        state_type: &str,
        state: Value,
    ) -> Result<(), AuthStateError> {
        // Updates existing state
    }

    /// Consume state (one-time use, deletes after retrieval)
    pub async fn consume_public(
        &self,
        state_id: Uuid,
        state_type: &str,
    ) -> Result<Option<Value>, AuthStateError> {
        // Loads and deletes state atomically
    }

    /// Consume user-specific state (validates user_id matches)
    pub async fn consume_user(
        &self,
        state_id: Uuid,
        user_id: Uuid,
        state_type: &str,
    ) -> Result<Option<Value>, AuthStateError> {
        // Validates user_id before consuming
    }

    /// Delete state explicitly
    pub async fn delete(&self, state_id: Uuid) -> Result<(), AuthStateError> {
        // Cleanup on error/cancellation
    }
}
```

### Database Schema

Auth state requires a table (see Underlay migrations):

```sql
CREATE TABLE auth.auth_state (
    id UUID PRIMARY KEY,
    user_id UUID NULL,                -- NULL for public flows
    state_type VARCHAR(64) NOT NULL,  -- Flow identifier
    state JSONB NOT NULL,             -- Arbitrary JSON
    expires_at TIMESTAMPTZ NOT NULL,  -- TTL
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_auth_state_expires_at ON auth.auth_state(expires_at);
CREATE INDEX idx_auth_state_user_id ON auth.auth_state(user_id) WHERE user_id IS NOT NULL;
```

**Note**: Run the Underlay migration sync tool to copy the canonical migrations into your app:

```bash
cargo run --manifest-path /path/to/underlay/Cargo.toml \
  -p underlay-devtools --bin underlay-devtools -- \
  sync-migrations --target /path/to/your-app/migrations
```

### Example: Password + TOTP Login (Multi-Step)

This is a production-ready pattern from Acowtancy showing how to split login into two steps when 2FA is required.

**Step 1: Login Start (Password Verification)**

```rust
use serde::{Deserialize, Serialize};
use underlay_auth_state::AuthStateStore;
use underlay_core::Uuid;
use chrono::Duration;

#[derive(Debug, Clone)]
pub enum LoginStartOutcome {
    /// Login complete (no 2FA required)
    Complete { session: AuthSession, role: String },
    /// 2FA required - client must call login_finish with TOTP code
    TwoFactorRequired { login_state_id: Uuid },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginTwoFactorState {
    user_id: String,
    client_fingerprint: String,
    attempts: u32,
}

pub async fn login_start_with_password(
    auth_state: &AuthStateStore,
    email: &str,
    password: &str,
    client_fingerprint: &str,  // e.g., user-agent hash
) -> AuthResult<LoginStartOutcome> {
    // 1. Find user by email
    let user = find_user_by_email(email).await?
        .ok_or(AuthError::WrongCredentials)?;

    // 2. Verify password
    let password_credential = find_password_credential(user.id).await?
        .ok_or(AuthError::WrongCredentials)?;
    
    verify_password(password, &password_credential.secret_encrypted).await?;

    // 3. Check if user has TOTP enabled
    if has_totp_enabled(user.id).await? {
        // Create temporary state for 2FA flow
        let state_id = auth_state.create(
            None,  // Public flow (user not yet authenticated)
            "login_2fa",
            serde_json::to_value(LoginTwoFactorState {
                user_id: user.id.to_string(),
                client_fingerprint: client_fingerprint.to_string(),
                attempts: 0,
            })?,
            Duration::minutes(5),  // Short TTL for security
        ).await?;

        return Ok(LoginStartOutcome::TwoFactorRequired {
            login_state_id: state_id,
        });
    }

    // 4. No 2FA required - create session immediately
    let roles = get_user_roles(user.id).await?;
    let (tokens, session) = create_session(user.id, roles).await?;

    Ok(LoginStartOutcome::Complete {
        session: AuthSession {
            user,
            session,
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
        },
        role: get_user_role(user.id).await?,
    })
}
```

**Step 2: Login Finish (TOTP Verification)**

```rust
pub async fn login_finish_with_totp(
    auth_state: &AuthStateStore,
    login_state_id: Uuid,
    code: &str,
    client_fingerprint: &str,
) -> AuthResult<(AuthSession, String)> {
    // 1. Load state (not consumed yet - allows retry on wrong code)
    let state_value = auth_state.load_public(login_state_id, "login_2fa").await?
        .ok_or(AuthError::BadRequest("Invalid or expired login state".into()))?;

    let mut state: LoginTwoFactorState = serde_json::from_value(state_value)?;

    // 2. Validate client fingerprint matches (prevent state hijacking)
    if state.client_fingerprint != client_fingerprint {
        auth_state.delete(login_state_id).await?;
        return Err(AuthError::BadRequest("Invalid or expired login state".into()));
    }

    // 3. Check rate limiting (max 5 attempts)
    if state.attempts >= 5 {
        auth_state.delete(login_state_id).await?;
        return Err(AuthError::RateLimited { retry_after_seconds: 60 });
    }

    let user_id = Uuid::parse_str(&state.user_id)?;
    let user = find_user_by_id(user_id).await?
        .ok_or(AuthError::UserNotFound)?;

    // 4. Verify TOTP code
    let totp_details = find_totp_details(user.id).await?
        .ok_or(AuthError::TwoFactorNotSetUp)?;

    if let Err(err) = verify_totp_code(&totp_details, code).await {
        // Increment attempt counter and update state
        state.attempts += 1;
        auth_state.update_public(
            login_state_id,
            "login_2fa",
            serde_json::to_value(&state)?,
        ).await?;

        if state.attempts >= 5 {
            auth_state.delete(login_state_id).await?;
            return Err(AuthError::RateLimited { retry_after_seconds: 60 });
        }

        return Err(err);
    }

    // 5. Success - consume state (one-time use prevents replay)
    auth_state.consume_public(login_state_id, "login_2fa").await?;

    // 6. Create session
    let role = get_user_role(user.id).await?;
    let roles = roles_for_user(&role);
    let (tokens, session) = create_session(user.id, roles).await?;

    Ok((
        AuthSession {
            user,
            session,
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
        },
        role,
    ))
}
```

**Frontend Integration (SvelteKit)**

```typescript
// Step 1: Start login
const response = await api.auth.loginStart({
  email,
  password,
  clientFingerprint: navigator.userAgent,
});

if (response.outcome === 'twoFactorRequired') {
  // Show TOTP input form
  setLoginStateId(response.loginStateId);
  setShowTotpInput(true);
} else {
  // Login complete - set cookies and redirect
  setCookies(response.session);
  goto('/dashboard');
}

// Step 2: Finish login with TOTP
const finishResponse = await api.auth.loginFinish({
  loginStateId,
  code: totpCode,
  clientFingerprint: navigator.userAgent,
});

setCookies(finishResponse.session);
goto('/dashboard');
```

### 2FA Enforcement Logic

You can enforce 2FA for specific user roles or security levels:

```rust
pub async fn should_require_2fa(user_id: Uuid) -> AuthResult<bool> {
    let role = get_user_role(user_id).await?;
    
    // Require 2FA for admin users
    if role == "admin" || role == "super_admin" {
        return Ok(true);
    }

    // Require 2FA if user has enabled it
    Ok(has_totp_enabled(user_id).await?)
}

pub async fn login_start_with_password(
    // ... params
) -> AuthResult<LoginStartOutcome> {
    // ... verify password ...

    if should_require_2fa(user.id).await? {
        // Create 2FA state
        // ...
    }

    // ... create session ...
}
```

### State Types Convention

Use descriptive `state_type` values for different flows:

- `login_2fa` - Password + TOTP login continuation
- `passkey_registration` - WebAuthn registration challenge
- `passkey_authentication` - WebAuthn login challenge
- `passkey_discoverable_authentication` - Discoverable credential flow
- `oauth_google` - Google OAuth CSRF/PKCE state
- `totp_setup` - TOTP enrollment continuation
- `password_reset` - Password reset token

### Security Best Practices

1. **Short TTLs**: Use 5-15 minutes for sensitive flows (login, registration)
2. **One-time use**: Always `consume` state when completing a flow
3. **Client fingerprinting**: Include user-agent or other client identifiers in state to prevent hijacking
4. **Rate limiting**: Track attempts in state to prevent brute force
5. **State cleanup**: Failed attempts should delete state after max attempts
6. **Type validation**: Always validate `state_type` matches expected flow
7. **User validation**: For user-specific flows, validate `user_id` in state matches authenticated user

### Common Pitfalls

**Don't:** Store sensitive data (passwords, plaintext secrets) in auth state
**Do:** Store only identifiers and flow control data

**Don't:** Reuse state IDs across different flows
**Do:** Use unique state_type for each flow

**Don't:** Forget to delete state on errors
**Do:** Clean up state in error paths to prevent leaks

**Don't:** Use long TTLs (hours/days)
**Do:** Keep TTLs short (minutes) and let users restart if expired

### Advanced: Multi-Device Flows

For flows that span devices (e.g., QR code scan), use public state with polling:

```rust
// Device A: Generate QR code with state_id
let state_id = auth_state.create(
    None,
    "device_pairing",
    serde_json::to_value(DevicePairingState {
        status: "pending",
        device_a_fingerprint: fingerprint,
    })?,
    Duration::minutes(5),
).await?;

// Device B: Complete pairing
auth_state.update_public(
    state_id,
    "device_pairing",
    serde_json::to_value(DevicePairingState {
        status: "completed",
        device_b_id: Some(device_id),
    })?,
).await?;

// Device A: Poll for completion
loop {
    let state = auth_state.load_public(state_id, "device_pairing").await?;
    if state.status == "completed" {
        auth_state.consume_public(state_id, "device_pairing").await?;
        break;
    }
    tokio::time::sleep(Duration::seconds(2)).await;
}
```

---

## OAuth2 (Google) Authentication

Underlay provides `underlay-auth-oauth` for Google Sign-In with two service layers:

1. **`GoogleOAuthService`** - Low-level OAuth2 protocol helpers
2. **`GoogleOAuthAppService`** - Higher-level service with repository integration for user/credential management

### Adding OAuth to Your App

```toml
# apps/nursery/crates/auth/Cargo.toml
[dependencies]
underlay-auth-oauth = { path = "../../../underlay/rust/crates/underlay-auth-oauth" }
```

### OAuth Service Setup

```rust
use underlay_auth_oauth::{GoogleOAuthService, GoogleOAuthConfig, GoogleOAuthAppService};

pub struct OAuthAuthService {
    // Low-level service for token operations
    google: GoogleOAuthService,
    // Higher-level service with user/credential management
    app: GoogleOAuthAppService<GoogleOAuthService>,
}

impl OAuthAuthService {
    pub fn new() -> AuthResult<Self> {
        let google = GoogleOAuthService::from_env()?;
        let app = GoogleOAuthAppService::new(google.clone());
        Ok(Self { google, app })
    }

    pub fn from_config(config: GoogleOAuthConfig) -> AuthResult<Self> {
        let google = GoogleOAuthService::new(config)?;
        let app = GoogleOAuthAppService::new(google.clone());
        Ok(Self { google, app })
    }
}
```

### Low-Level OAuth (Token Operations)

Use `GoogleOAuthService` directly when you need fine-grained control:

```rust
impl OAuthAuthService {
    /// Step 1: Start OAuth flow - returns authorization URL and state
    pub fn start_login(&self) -> AuthResult<underlay_auth_oauth::OAuthStart> {
        self.google.start_login()
    }

    /// Step 2: Start with custom state (for session-based flows)
    pub fn start_login_with(
        &self,
        csrf_state: &str,
        pkce_verifier: &str,
    ) -> AuthResult<String> {
        self.google.start_login_with(csrf_state, pkce_verifier)
    }

    /// Step 3: Exchange authorization code for tokens
    pub async fn exchange_code(
        &self,
        code: &str,
        pkce_verifier: &str,
    ) -> AuthResult<underlay_auth_oauth::TokenSet> {
        self.google.exchange_code(code, pkce_verifier).await
    }

    /// Step 4: Refresh access token
    pub async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> AuthResult<underlay_auth_oauth::TokenSet> {
        self.google.refresh(refresh_token).await
    }

    /// Step 5: Get user info from Google
    pub async fn fetch_userinfo(
        &self,
        access_token: &str,
    ) -> AuthResult<underlay_auth_oauth::GoogleUserInfo> {
        self.google.fetch_userinfo(access_token).await
    }
}
```

### Higher-Level OAuth (User & Credential Management)

Use `GoogleOAuthAppService` for complete OAuth callback handling including user creation and credential linking:

```rust
use underlay_auth_oauth::{
    OAuthCallbackRequest, OAuthLoginState, OAuthLoginResult,
    CredentialRepository, UserRepository,
};
use underlay_core::Uuid;

impl OAuthAuthService {
    /// Handle the complete OAuth callback flow
    ///
    /// This method:
    /// 1. Validates CSRF state
    /// 2. Exchanges code for tokens
    /// 3. Fetches user info from Google
    /// 4. Verifies email is verified (if configured)
    /// 5. Creates user if doesn't exist (or links to existing)
    /// 6. Creates OAuth credential
    ///
    /// Returns the user, whether they were newly created, and the credential.
    pub async fn handle_google_callback<R>(
        &self,
        repo: &R,
        request: OAuthCallbackRequest,
        stored_state: OAuthLoginState,
        encrypt_secret: impl FnOnce(&str) -> AuthResult<String>,
    ) -> AuthResult<OAuthLoginResult>
    where
        R: UserRepository + CredentialRepository,
    {
        self.app
            .handle_google_callback(repo, request, stored_state, encrypt_secret)
            .await
    }

    /// Disconnect Google OAuth from a user account
    ///
    /// Removes the OAuth credential from the user's account.
    pub async fn disconnect_google<R>(
        &self,
        repo: &R,
        user_id: Uuid,
    ) -> AuthResult<()>
    where
        R: UserRepository + CredentialRepository,
    {
        self.app.disconnect_google(repo, user_id).await
    }
}
```

### OAuth Handler Example (Complete Flow)

```rust
use axum::{
    Json, Query, redirect,
    extract::State,
    response::IntoResponse,
};
use serde::Deserialize;
use underlay_auth_oauth::{OAuthCallbackRequest, OAuthLoginState};
use underlay_core::Uuid;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct OAuthCallbackQuery {
    code: String,
    state: String,
}

pub async fn oauth_start(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Generate PKCS wrapper for the exchange
    let pkce_verifier = underlay_auth_oauth::generate_pkce_verifier();
    
    // Create a state ID to track this login attempt
    let state_id = Uuid::new_v7();
    let csrf_state = state_id.to_string();

    let state_data = OAuthLoginState {
        csrf_state: csrf_state.clone(),
        pkce_verifier,
    };

    // Store state in DB (valid for 10 minutes)
    state.auth_state.create_public(
        "oauth_login",
        serde_json::to_value(&state_data).unwrap(),
        chrono::Duration::minutes(10)
    ).await.expect("Failed to store auth state");

    // Start login with our custom state (UUID)
    // The UUID acts as the CSRF token passed to Google
    let url = state.oauth.start_login_with(&csrf_state, &state_data.pkce_verifier)
        .expect("Failed to start oauth flow");

    redirect(url)
}

pub async fn oauth_callback(
    State(state): State<Arc<AppState>>,
    Query(query): Query<OAuthCallbackQuery>,
) -> impl IntoResponse {
    // Load stored state
    // We assume query.state is the UUID we sent
    let state_id = Uuid::parse_str(&query.state).map_err(|_| AuthError::BadRequest("Invalid state".into()))?;
    
    let value = state.auth_state
        .consume_public(state_id, "oauth_login")
        .await?
        .ok_or_else(|| AuthError::BadRequest("invalid or expired oauth state".into()))?;
        
    let stored_state: OAuthLoginState = serde_json::from_value(value)
        .map_err(|_| AuthError::Internal("invalid state format".into()))?;

    let request = OAuthCallbackRequest {
        code: query.code,
        state: query.state,
    };

    // Encrypt refresh token for storage
    let encrypt = |token: &str| {
        // Use your encryption library (e.g., AES-256-GCM)
        Ok(format!("encrypted:{}", token))
    };

    // Handle complete callback flow
    let result = state
        .oauth
        .handle_google_callback(&state.pool, request, stored_state, encrypt)
        .await
        .map_err(|e| {
            tracing::error!(?e, "OAuth callback failed");
            e
        })?;

    // Create session for the user
    let session = create_session_for_user(result.user.id, \u0026state).await?;

    // State already consumed via consume_public() above - no need to clean up
    
    // Redirect to app with session
    redirect(format!(
        \"{}/auth/callback?token={}\u0026new_user={}\",
        FRONTEND_URL,
        session.access_token,
        result.is_new_user
    ))
}

/// Disconnect Google OAuth
pub async fn oauth_disconnect(
    State(state): State<Arc<AppState>>,
    Authenticated(principal): Authenticated<UserPrincipal>,
) -> Json<SingleResponse<()>> {
    state.oauth.disconnect_google(&state.pool, principal.user_id.0).await?;

    Json(SingleResponse { data: () })
}
```

### OAuth Callback Types

```rust
// Request from Google callback
pub struct OAuthCallbackRequest {
    pub code: String,   // Authorization code from Google
    pub state: String,  // CSRF state we generated
}

// Stored state (persist between start and callback)
pub struct OAuthLoginState {
    pub csrf_state: String,       // For CSRF validation
    pub pkce_verifier: String,    // For token exchange
}

// Callback result
pub struct OAuthLoginResult {
    pub user: User,                      // The user (new or existing)
    pub is_new_user: bool,               // Whether this is a new account
    pub credential: Credential,           // The OAuth credential created
    pub token_set: TokenSet,             // OAuth tokens
    pub userinfo: GoogleUserInfo,        // User profile from Google
}

// Google user profile
pub struct GoogleUserInfo {
    pub sub: String,                     // Google user ID
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,          // Avatar URL
    pub locale: Option<String>,
}
```

### Database Schema for OAuth

OAuth credentials are stored in the standard `auth_credentials` table:

```sql
-- OAuth connections stored in auth_credentials with type = 'oauth_google'
-- secret_encrypted: encrypted access_token or refresh_token
-- metadata: { google_user_id, scopes }

-- Find user's Google connection
SELECT * FROM auth_credentials
WHERE user_id = $1 AND type = 'oauth_google';

-- Find user by Google account
SELECT u.* FROM auth_users u
JOIN auth_credentials c ON u.id = c.user_id
WHERE c.type = 'oauth_google'
  AND (c.metadata->>'google_user_id') = $1;
```

### Environment Variables for OAuth

```bash
# Google OAuth
AUTH_GOOGLE_CLIENT_ID=your-client-id.apps.googleusercontent.com
AUTH_GOOGLE_CLIENT_SECRET=your-client-secret
AUTH_GOOGLE_REDIRECT_URI=https://yourapp.com/auth/oauth/google/callback
```

### Google OAuth Setup

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select existing
3. Enable "Google OAuth" API
4. Configure OAuth consent screen
5. Create OAuth 2.0 credentials (Web application)
6. Add authorized redirect URIs
7. Copy Client ID and Secret to environment variables

### Email Verification

By default, `GoogleOAuthAppService` requires `email_verified=true` from Google. You can disable this:

```rust
let mut app = GoogleOAuthAppService::new(google);
app.require_verified_email = false;  // Allow unverified emails
```

### Configuring Scopes

Default scopes: `openid`, `email`, `profile`. Configure custom scopes:

```rust
let config = GoogleOAuthConfig {
    client_id: "...".to_string(),
    client_secret: "...".to_string(),
    redirect_uri: "https://...".to_string(),
    scopes: vec!["openid", "email", "profile", "https://www.googleapis.com/auth/drive".to_string()],
};
```

---

## Password Authentication

Underlay provides `underlay-auth-password` for secure password authentication using Argon2id hashing with configurable security policies.

### Adding Password Auth to Your App

```toml
# apps/nursery/crates/auth/Cargo.toml
[dependencies]
underlay-auth-password = { path = "../../../underlay/rust/crates/underlay-auth-password" }
```

### Password Auth Components

The password crate provides:

- **`PasswordAuthService<R, H, V>`** - Main service for password operations
- **`PasswordAuthRepository`** - Trait for database operations (implement this)
- **`PasswordConfig`** - Security policy configuration
- **`Argon2Hasher`** - Default hasher using Argon2id
- **`PasswordStrengthAnalyzer`** - Password quality validation

### Password Config

Configure security policies:

```rust
use underlay_auth_password::{PasswordConfig, CompromisedPasswordStrategy};

let config = PasswordConfig {
    max_failed_attempts: 5,           // Lockout after 5 failures
    lockout_duration_seconds: 900,    // 15 minute lockout
    rate_limit_window_seconds: 3600,  // 1 hour rate limit window
    rate_limit_max_attempts: 10,      // Max 10 attempts per window
    min_password_length: 8,           // Minimum 8 characters
    check_compromised: true,          // Check against blocklist
    compromised_password_strategy: CompromisedPasswordStrategy::LocalBlocklist,
    // Or for HIBP integration:
    // compromised_password_strategy: CompromisedPasswordStrategy::HibpKAnonymity {
    //     api_base_url: "https://api.pwnedpasswords.com".to_string(),
    //     user_agent: "myapp".to_string(),
    // },
};
```

### Password Auth Repository

> **Note**: The example below shows the repository trait interface with SQL query placeholders.  
> For complete working SQL implementations including proper error handling, row mapping, and schema-qualified queries, see the Acowtancy reference implementation at `farmyard/crates/auth/src/local.rs` (lines 1168-1300+).

Implement the `PasswordAuthRepository` trait for your database:

```rust
use async_trait::async_trait;
use underlay_auth_password::{PasswordAuthRepository, PasswordAuthResult, FailedLoginAttempt};
use underlay_core::Uuid;
use underlay_auth::{User, Credential, CredentialMetadata};

#[derive(Clone)]
pub struct PgAuthRepository {
    pool: Arc<sqlx::PgPool>,
}

#[async_trait]
impl PasswordAuthRepository for PgAuthRepository {
    async fn find_user_by_email(&self, email: &str) -> PasswordAuthResult<Option<User>> {
        let email = email.trim().to_lowercase();
        // Your SQL query here
        Ok(None)
    }

    async fn find_user_by_id(&self, user_id: Uuid) -> PasswordAuthResult<Option<User>> {
        // Your SQL query here
        Ok(None)
    }

    async fn find_password_credential(
        &self,
        user_id: Uuid,
    ) -> PasswordAuthResult<Option<Credential>> {
        // Query: SELECT * FROM auth_credentials
        //        WHERE user_id = $1 AND type = 'password'
        Ok(None)
    }

    async fn create_password_credential(
        &self,
        user_id: Uuid,
        password_hash: &str,
    ) -> PasswordAuthResult<Credential> {
        // Insert into auth_credentials with type = 'password'
        Ok(...)
    }

    async fn update_password_credential(
        &self,
        credential_id: Uuid,
        password_hash: &str,
    ) -> PasswordAuthResult<()> {
        // UPDATE auth_credentials SET secret_encrypted = $1 WHERE id = $2
        Ok(())
    }

    async fn delete_password_credential(
        &self,
        credential_id: Uuid,
    ) -> PasswordAuthResult<()> {
        // DELETE FROM auth_credentials WHERE id = $1
        Ok(())
    }

    async fn record_failed_login(
        &self,
        user_id: Uuid,
        max_failed_attempts: u32,
        lockout_duration_seconds: u64,
    ) -> PasswordAuthResult<FailedLoginAttempt> {
        // Increment failure count, apply lockout if threshold reached
        // Return (count, lockout_remaining_seconds)
        Ok(FailedLoginAttempt {
            count: 1,
            lockout_remaining_seconds: None,
        })
    }

    async fn reset_failed_logins(&self, user_id: Uuid) -> PasswordAuthResult<()> {
        // DELETE FROM auth_failed_logins WHERE user_id = $1
        Ok(())
    }

    async fn get_failed_login_count(&self, user_id: Uuid) -> PasswordAuthResult<u32> {
        // SELECT COUNT(*) FROM auth_failed_logins WHERE user_id = $1
        Ok(0)
    }

    async fn get_lockout_remaining_seconds(
        &self,
        user_id: Uuid,
    ) -> PasswordAuthResult<Option<u64>> {
        // SELECT (locked_until - NOW()) FROM auth_users
        // WHERE user_id = $1 AND locked_until > NOW()
        Ok(None)
    }

    async fn check_rate_limit(
        &self,
        key: &str,
        max_attempts: u32,
        window_seconds: u64,
    ) -> PasswordAuthResult<(bool, u64)> {
        // Rate limiting implementation (Redis sorted sets, etc.)
        Ok((true, 0))
    }
}
```

### Password Service Setup

```rust
use underlay_auth_password::{PasswordAuthService, PasswordConfig, Argon2Hasher};
use std::sync::Arc;

pub struct PasswordAuth {
    service: Arc<PasswordAuthService<impl PasswordAuthRepository + Send + Sync>>,
}

impl PasswordAuth {
    pub fn new(
        repository: Arc<impl PasswordAuthRepository + Send + Sync>,
        config: Option<PasswordConfig>,
    ) -> Self {
        let hasher = Arc::new(Argon2Hasher::new());
        let service = PasswordAuthService::new(
            repository,
            hasher.clone(),
            hasher,
            config,
        );
        Self { service: Arc::new(service) }
    }
}
```

### Password Operations

```rust
impl PasswordAuth {
    /// Set or update a user's password
    pub async fn set_password(
        &self,
        user_id: Uuid,
        password: &str,
    ) -> AuthResult<Credential> {
        self.service
            .set_password(user_id, password)
            .await
            .map_err(|e| e.into())
    }

    /// Verify login credentials
    pub async fn verify_login(
        &self,
        email: &str,
        password: &str,
    ) -> AuthResult<User> {
        self.service
            .verify_login(email, password)
            .await
            .map_err(|e| e.into())
    }

    /// Verify login with IP context (for rate limiting)
    pub async fn verify_login_with_context(
        &self,
        email: &str,
        password: &str,
        ip: Option<&str>,
    ) -> AuthResult<User> {
        self.service
            .verify_login_with_context(email, password, ip)
            .await
            .map_err(|e| e.into())
    }

    /// Change password (requires current password)
    pub async fn change_password(
        &self,
        user_id: Uuid,
        current_password: &str,
        new_password: &str,
    ) -> AuthResult<()> {
        self.service
            .change_password(user_id, current_password, new_password)
            .await
            .map_err(|e| e.into())
    }

    /// Reset password (admin/internal use, bypasses current password)
    pub async fn reset_password(
        &self,
        user_id: Uuid,
        new_password: &str,
    ) -> AuthResult<()> {
        self.service
            .reset_password(user_id, new_password)
            .await
            .map_err(|e| e.into())
    }
}
```

### Password Login Handler Example

```rust
use axum::{Json, Extension};
use underlay_core::SingleResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PasswordLoginRequest {
    email: String,
    password: String,
    #[serde(default)]
    ip: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PasswordLoginResponse {
    user_id: Uuid,
    email: String,
    display_name: String,
}

pub async fn password_login(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<PasswordLoginRequest>,
) -> Json<SingleResponse<PasswordLoginResponse>> {
    match state
        .password_service
        .verify_login_with_context(
            &req.email,
            &req.password,
            req.ip.as_deref(),
        )
        .await
    {
        Ok(user) => Json(SingleResponse {
            data: PasswordLoginResponse {
                user_id: user.id,
                email: user.email,
                display_name: user.display_name,
            },
        }),
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

#[derive(Deserialize)]
pub struct SetPasswordRequest {
    current_password: Option<String>,  // None for initial set
    new_password: String,
}

pub async fn set_password(
    Authenticated(principal): Authenticated<UserPrincipal>,
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<SetPasswordRequest>,
) -> Json<SingleResponse<()>> {
    let result = match req.current_password {
        Some(current) => {
            state
                .password_service
                .change_password(principal.user_id.0, &current, &req.new_password)
                .await
        }
        None => {
            state
                .password_service
                .set_password(principal.user_id.0, &req.new_password)
                .await
                .map(|_| ())
        }
    };

    match result {
        Ok(_) => Json(SingleResponse { data: () }),
        Err(e) => {
            tracing::error!(user_id = %principal.user_id.0, error = ?e, "Password operation failed");
            Json(SingleResponse {
                data: (),  // In real app, return error in response
            })
        }
    }
}
```

### Database Schema for Password Auth

```sql
-- Password credentials stored in auth_credentials with type = 'password'
-- secret_encrypted: Argon2id hash
-- metadata: { algorithm: "argon2id", memory_kb, iterations, parallelism }

-- Failed login tracking
CREATE TABLE auth_failed_logins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth_users(id) ON DELETE CASCADE,
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address VARCHAR(45),
    user_agent TEXT,
    reason VARCHAR(50),  -- 'wrong_password', 'account_locked', etc.
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_auth_failed_logins_user_id ON auth_failed_logins(user_id);
CREATE INDEX idx_auth_failed_logins_ip ON auth_failed_logins(ip_address);
CREATE INDEX idx_auth_failed_logins_created_at ON auth_failed_logins(created_at);

-- Rate limiting (using sliding window)
CREATE TABLE auth_rate_limits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key VARCHAR(255) NOT NULL,
    count INTEGER NOT NULL,
    window_start TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    UNIQUE(key, window_start)
);

CREATE INDEX idx_auth_rate_limits_key ON auth_rate_limits(key);
CREATE INDEX idx_auth_rate_limits_expires ON auth_rate_limits(expires_at);
```

### Password Security Features

| Feature | Description |
|---------|-------------|
| **Argon2id** | Memory-hard hashing, resistant to GPU/ASIC attacks |
| **Rate Limiting** | Configurable attempts per time window |
| **Account Lockout** | Temporary lockout after failed attempts |
| **Password Strength** | Minimum length + complexity validation |
| **Compromised Check** | Optional HIBP k-anonymity integration |
| **Replay Protection** | TOTP counter tracking for replay detection |
| **Constant-Time Comparison** | Timing attack resistant |

### Error Types

```rust
use underlay_auth_password::PasswordAuthError;

match result {
    Ok(user) => { /* success */ }
    Err(PasswordAuthError::WrongPassword) => { /* invalid credentials */ }
    Err(PasswordAuthError::AccountLocked { retry_after_seconds }) => {
        // User is locked out, tell them to wait
    }
    Err(PasswordAuthError::RateLimited { retry_after_seconds }) => {
        // Too many attempts, rate limited
    }
    Err(PasswordAuthError::PasswordTooWeak(feedback)) => {
        // Password doesn't meet requirements
    }
    Err(PasswordAuthError::PasswordCompromised) => {
        // Password found in breach database
    }
    Err(PasswordAuthError::PasswordSameAsCurrent) => {
        // New password can't be the same as current
    }
    _ => { /* other errors */ }
}
```

---

## Configuration

### Environment Variables

Create `apps/nursery/.env`:

```bash
# === Authentication ===

# Production JWT Configuration (Ed25519 / EdDSA)
#
# Underlay JWT expects:
# - AUTH_JWT_PRIVATE_KEY: base64-encoded PKCS#8 DER (Ed25519 private key)
# - AUTH_JWT_PUBLIC_KEY: base64url (or base64) of raw Ed25519 public key bytes (32 bytes)
#
# See `docs/guides/quickstart/code/060-authentication/generate-jwt-env.rs` for a generator.
AUTH_JWT_PRIVATE_KEY=...
AUTH_JWT_PUBLIC_KEY=...

# Token Configuration
AUTH_JWT_ISSUER=myapp
AUTH_JWT_AUDIENCE=myapp-api
AUTH_ACCESS_TOKEN_LIFETIME_MINUTES=15
AUTH_REFRESH_TOKEN_LIFETIME_DAYS=30
AUTH_JWT_LEEWAY_SECONDS=30

# === Dev Mode (LOCAL DEVELOPMENT ONLY) ===
NURSERY_DEV_AUTH=false

# === TOTP (optional - defaults provided) ===
# TOTP_ISSUER=MyApp
# TOTP_DIGITS=6
# TOTP_PERIOD=30
# TOTP_SKEW=1

# === Auth State Migrations (required for multi-step flows) ===
#
# Underlay uses `auth.auth_state` to store short-lived state for multi-step auth flows
# like 2FA login continuation, passkeys, and OAuth callbacks.
#
# Underlay owns the canonical migration(s) for this table.
# To keep each app using a *single* sqlx migrator, copy Underlay migrations into your
# app's migrations folder using:
#
#   cargo run --manifest-path /path/to/libraries/underlay/Cargo.toml -p underlay-devtools --bin underlay-devtools -- \
#     sync-migrations --target /path/to/your-app/migrations
#
# Re-run this whenever you update Underlay.

# === OAuth token encryption (recommended if storing refresh tokens) ===
#
# If you store OAuth refresh tokens (e.g. Google) in your database, encrypt them at rest.
# Underlay provides `underlay-auth-oauth::OAuthTokenCipher` which expects:
# - AUTH_OAUTH_SECRET_KEY: base64/base64url of 32 random bytes (AES-256-GCM key)
#
# Generate one with:
#   openssl rand -base64 32
# or:
#   node -e "console.log(require('crypto').randomBytes(32).toString('base64'))"
AUTH_OAUTH_SECRET_KEY=...

# === Google OAuth (optional) ===
AUTH_GOOGLE_CLIENT_ID=...
AUTH_GOOGLE_CLIENT_SECRET=...
AUTH_GOOGLE_REDIRECT_URI=https://myapp.com/auth/oauth/google/callback
```

### Key Generation

Because `underlay-auth-jwt` expects specific key encodings (PKCS#8 DER for the private key, raw 32-byte public key), the easiest way to generate correct env values is via the helper in:

- `docs/guides/quickstart/code/060-authentication/generate-jwt-env.rs`

Copy that file into your app repo as a small Rust bin target, for example:

- `apps/nursery/crates/auth/src/bin/generate-jwt-env.rs`

Then run:

```bash
cd apps/nursery
cargo run -p myapp-auth --bin generate-jwt-env
```

This prints `AUTH_JWT_PRIVATE_KEY=...` and `AUTH_JWT_PUBLIC_KEY=...` ready to paste into `apps/nursery/.env`.

## JWT Claims and Validation

`underlay-auth-jwt` verifies **Ed25519 / EdDSA** tokens using `jsonwebtoken`.

### Required JWT claims

Access tokens are expected to include:

- `exp` (expiry)
- `iss` (issuer)
- `sub` (subject user id)
- `nbf` (not-before)
- `tuse` (token use), must equal `"access"`

If `AUTH_JWT_AUDIENCE` is set, `aud` is also required and validated.

### Validation behavior

- Algorithm is fixed to `EdDSA` (allowlist enforced by the validator).
- `iss` is required and must match `AUTH_JWT_ISSUER`.
- `aud` is validated if configured.
- `nbf` is validated.
- `exp` is validated with `AUTH_JWT_LEEWAY_SECONDS` applied.
- Keypair mismatch fails fast during service initialization.

## Security Checklist

- [ ] Dev auth provider requires explicit `NURSERY_DEV_AUTH=true`
- [ ] JWT keys are loaded from environment, not hardcoded
- [ ] Token expiration is set (access: 15min, refresh: 30days)
- [ ] HTTPS required in production
- [ ] Sensitive data not logged in auth operations
- [ ] Rate limiting on auth endpoints
- [ ] Audit logging for login attempts
- [ ] TOTP secrets encrypted at rest (use AES-256-GCM)
- [ ] Backup codes hashed (SHA-256) before storage
- [ ] OAuth tokens encrypted at rest
- [ ] WebAuthn credential IDs indexed for lookup
- [ ] PassKey counter regression detected and rejected

## See Also

**Related Guides:**
- **[065-session-management.md](./065-session-management.md)** - Complete login/logout flows, cookie management, session refresh
- **[067-authorization.md](./067-authorization.md)** - RBAC patterns, role extraction, protected routes
- **[070-api-handlers.md](./070-api-handlers.md)** - HTTP handlers for auth endpoints, error handling
- **[050-database.md](./050-database.md)** - Database setup, migrations, auth schema
- **[075-validation.md](./075-validation.md)** - Input validation for login forms

**Implementation Checklist:**
1. Set up auth database schema (see [050-database.md](./050-database.md))
2. Configure JWT keys and environment variables (this guide)
3. Implement login/logout handlers (see [070-api-handlers.md](./070-api-handlers.md))
4. Add session management (see [065-session-management.md](./065-session-management.md))
5. Add authorization guards (see [067-authorization.md](./067-authorization.md))
6. Build frontend login forms (see [100-frontend-bloom.md](./100-frontend-bloom.md))

## Next Steps

With authentication configured, proceed to [070-api-handlers](./070-api-handlers.md) to implement HTTP handlers and routing.
