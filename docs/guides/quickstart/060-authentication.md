# 060 - Authentication

This document covers implementing authentication using the Underlay auth system with both development (dev mode) and production (JWT) providers.

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

Your app implements Underlay’s provider boundary (`underlay_auth::AuthProvider`).

For production JWT, use Underlay’s `underlay-auth-jwt` crate (it uses `jsonwebtoken` under the hood) and validate **Ed25519 / EdDSA** tokens.

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
    pub pool: myapp_db::PgPool,
}

impl underlay_auth::HasAuthProvider for AppState {
    fn auth_provider(&self) -> &dyn AuthProvider {
        self.auth_provider.as_ref()
    }
}
```

### Step 2: Auth Provider Selection

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
            std::process::exit(1);
        }
    }
}
```

### Step 3: Protected Routes

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

## Next Step

With authentication configured, proceed to [070-api-handlers](./070-api-handlers.md) to implement HTTP handlers and routing.
