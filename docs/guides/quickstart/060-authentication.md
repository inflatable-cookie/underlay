# 060 - Authentication

This document covers implementing authentication using the Underlay auth system with both development (dev mode) and production (JWT) providers.

## Auth Module Structure

The auth crate is organized into three modules following the Underlay pattern:

```
apps/nursery/crates/auth/src/
├── lib.rs           # Module declarations and exports
├── principal.rs     # UserId, UserPrincipal, UserRole types
├── provider.rs      # AuthError, AuthProvider trait
└── underlay.rs      # DevBearerUuidAuthProvider, converters
```

## Key Concepts

### 1. Principal Module

Defines domain-specific auth types in `principal.rs`:

```rust
use farmyard_core::Uuid;
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

Defines the auth boundary trait in `provider.rs`:

```rust
use async_trait::async_trait;
use underlay_auth::{AuthProvider, AuthResult, Principal, RoleSet};

pub enum AuthError {
    InvalidToken,
    TokenExpired,
    Unauthorized,
}

#[async_trait]
impl AuthProvider for MyAuthProvider {
    async fn authenticate_bearer(&self, token: &str) -> AuthResult<Principal> {
        // Validate token and return principal
    }
}
```

### 3. Underlay Module

Connects to Underlay and provides dev mode in `underlay.rs`:

```rust
use farmyard_core::Uuid;

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
pub use provider::{AuthError, AuthProvider};
pub use underlay::{user_principal_from_underlay, DevBearerUuidAuthProvider};
```

## Dev vs Prod Modes

| Mode | Provider | Security | Use Case |
|------|----------|----------|----------|
| Development | `DevBearerUuidAuthProvider` | ⚠️ NONE | Local development only |
| Production | `UnderlayJwtAuthProvider` | ✅ Secure | Production deployment |

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
use myapp_auth::{DevBearerUuidAuthProvider, UnderlayJwtAuthProvider};

fn create_auth_provider() -> Arc<dyn underlay_auth::AuthProvider> {
    let dev_auth_enabled = std::env::var("NURSERY_DEV_AUTH")
        .map(|v| v == "true")
        .unwrap_or(false);

    match UnderlayJwtAuthProvider::from_env() {
        Some(provider) => Arc::new(provider),
        None if dev_auth_enabled => {
            tracing::warn!("DEV AUTH ENABLED - NEVER USE IN PRODUCTION");
            Arc::new(DevBearerUuidAuthProvider)
        }
        None => {
            tracing::error!(
                "Auth not configured. Set JWT env vars or NURSERY_DEV_AUTH=true"
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
use underlay_http::SingleResponse;

use crate::state::AppState;

pub async fn list_artists(
    State(state): State<AppState>,
    Authenticated(principal): Authenticated<myapp_auth::UserPrincipal>,
) -> Json<SingleResponse<Vec<ArtistDto>>> {
    tracing::info!(user_id = %principal.user_id.0, "Listing artists");
    todo!()
}
```

## Configuration

### Environment Variables

Create `apps/nursery/.env`:

```bash
# === Authentication ===

# Production JWT Configuration
# Generate keys using: openssl genpkey -algorithm Ed25519 -out private.pem
AUTH_JWT_PRIVATE_KEY=your-base64-encoded-private-key
AUTH_JWT_PUBLIC_KEY=your-base64-encoded-public-key

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

```bash
openssl genpkey -algorithm Ed25519 -out private.pem
openssl pkey -in private.pem -pubout -out public.pem
base64 -w0 private.pem
base64 -w0 public.pem
```

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
