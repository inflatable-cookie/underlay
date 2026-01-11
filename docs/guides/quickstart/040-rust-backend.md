# 040 - Rust Backend (Nursery Pattern)

This document covers setting up the Rust API backend following the Nursery pattern. The backend uses a **workspace structure** with specialized crates for different concerns.

## Workspace Structure

```
apps/nursery/
├── Cargo.toml                    # Workspace manifest
├── Cargo.lock                    # Locked dependencies
│
├── crates/
│   ├── core/                     # Cross-cutting primitives (NO domain entities)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # Public exports
│   │       ├── error.rs          # AppError, AppResult
│   │       ├── id.rs             # IdGenerator, Uuid wrapper
│   │       ├── pagination.rs     # PageRequest, Pagination
│   │       └── time.rs           # NowProvider trait
│   │
│   ├── auth/                     # Authentication boundary
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # Public exports
│   │       ├── principal.rs      # UserId, UserRole, UserPrincipal
│   │       ├── provider.rs       # AuthError, AuthProvider traits
│   │       └── underlay.rs       # Underlay integration, DevBearerUuidAuthProvider
│   │
│   ├── infra/                    # Infrastructure
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config.rs         # Configuration
│   │       └── tracing.rs        # Tracing setup
│   │
│   ├── db/                       # Database utilities
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # Pool, migrations
│   │       └── migrations/       # SQL migration files
│   │
│   └── api/                      # HTTP server, handlers (domain-specific in domain crates)
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs           # Application entry point
│           └── (handlers inline or in domain crates)
│
└── migrations/                   # Database migrations
    └── *.sql
```

## Step 1: Create Workspace Manifest

Create `apps/nursery/Cargo.toml`:

```toml
[workspace]
members = [
  "crates/core",
  "crates/infra",
  "crates/db",
  "crates/api",
  "crates/auth",
]

[workspace.package]
version = "0.1.0"
edition = "2021"

[workspace.dependencies]
# === Rust Standard Library ===
anyhow = "1"
async-trait = "0.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"

# === Async Runtime ===
tokio = { version = "1", features = ["macros", "rt-multi-thread", "signal"] }

# === Web Framework ===
axum = "0.7"
tower-http = { version = "0.6", features = ["trace", "request-id", "propagate-header", "cors"] }

# === Database ===
sqlx = { version = "0.8.6", features = ["runtime-tokio-rustls", "postgres", "uuid", "migrate", "chrono"] }

# === Authentication ===
# JWT is provided by Underlay via `underlay-auth-jwt` (Ed25519 / EdDSA).

# === Observability ===
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt", "env-filter", "json"] }

# === Configuration ===
dotenvy = "0.15"

# === Underlay (local dev via relative paths) ===
# Monorepo: this file is `apps/nursery/Cargo.toml`, so `../../libs/underlay/...`.
# Multi-repo: see `030-underlay-integration.md` and use `../underlay/...` in `myapp-nursery/Cargo.toml`.
underlay-core = { path = "../../libs/underlay/rust/crates/underlay-core" }
underlay-http = { path = "../../libs/underlay/rust/crates/underlay-http" }
underlay-auth = { path = "../../libs/underlay/rust/crates/underlay-auth" }
underlay-auth-jwt = { path = "../../libs/underlay/rust/crates/underlay-auth-jwt" }
underlay-db = { path = "../../libs/underlay/rust/crates/underlay-db" }
underlay-observability = { path = "../../libs/underlay/rust/crates/underlay-observability" }
underlay-metrics = { path = "../../libs/underlay/rust/crates/underlay-metrics" }
underlay-soft-delete = { path = "../../libs/underlay/rust/crates/underlay-soft-delete" }
```

## Step 2: Create Core Crate

Create `apps/nursery/crates/core/Cargo.toml`:

```toml
[package]
name = "myapp-core"
version.workspace = true
edition.workspace = true

[dependencies]
underlay-core = { workspace = true }
```

Create `apps/nursery/crates/core/src/lib.rs`:

```rust
//! Core primitives for the application.
//!
//! This crate provides cross-cutting primitives used throughout the application:
//! - UUID-backed identifier types
//! - Common error type and result alias
//! - Pagination helpers shared across domains
//!
//! NOTE: Domain entities (e.g., User, Product, Order) belong in their own
//! domain crates (e.g., `accounts`, `billing`), NOT in this crate.

pub mod error;
pub mod id;
pub mod pagination;
pub mod time;

pub use crate::error::{AppError, AppResult};
pub use crate::id::{IdGenerator, Uuid};
pub use crate::pagination::{PageRequest, Pagination};
pub use crate::time::NowProvider;
```

Create `apps/nursery/crates/core/src/error.rs`:

```rust
use thiserror::Error;

/// Common error type for the application.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(&'static str),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(&'static str),

    #[error("Forbidden: {0}")]
    Forbidden(&'static str),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for operations that can fail with AppError.
pub type AppResult<T> = Result<T, AppError>;
```

Create `apps/nursery/crates/core/src/id.rs`:

```rust
use underlay_core::Uuid;

/// UUID wrapper type for application identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(pub Uuid);

impl Id {
    /// Create a new ID with a fresh UUIDv7.
    pub fn new() -> Self {
        Self(Uuid::new_v7())
    }

    /// Get the underlying UUID.
    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<Id> for Uuid {
    fn from(value: Id) -> Self {
        value.0
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Id {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

/// Trait for types that can generate IDs.
pub trait IdGenerator {
    type Id;

    fn generate(&self) -> Self::Id;
}

/// Default ID generator using UUIDv7.
#[derive(Debug, Default)]
pub struct UuidGenerator;

impl IdGenerator for UuidGenerator {
    type Id = Id;

    fn generate(&self) -> Self::Id {
        Id::new()
    }
}
```

Create `apps/nursery/crates/core/src/pagination.rs`:

```rust
use serde::{Deserialize, Serialize};

/// Pagination parameters for list queries.
#[derive(Debug, Clone, Copy, Default)]
pub struct PageRequest {
    pub page: u64,
    pub page_size: u64,
}

impl PageRequest {
    /// Calculate the offset for database queries.
    pub fn offset(&self) -> u64 {
        self.page.saturating_sub(1) * self.page_size
    }

    /// Get the limit (page size).
    pub fn limit(&self) -> u64 {
        self.page_size.max(1).min(100)
    }
}

/// Pagination metadata included in list responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
    pub total_items: u64,
    pub total_pages: u64,
}

impl Pagination {
    /// Create pagination metadata from request and total count.
    pub fn new(request: PageRequest, total_items: u64) -> Self {
        let total_pages = if request.page_size == 0 {
            0
        } else {
            (total_items + request.page_size - 1) / request.page_size
        };

        Self {
            page: request.page,
            page_size: request.page_size,
            total_items,
            total_pages,
        }
    }

    /// Check if there is a next page.
    pub fn has_next(&self) -> bool {
        self.page < self.total_pages
    }

    /// Check if there is a previous page.
    pub fn has_previous(&self) -> bool {
        self.page > 1
    }
}
```

## Step 3: Create Auth Crate

Create `apps/nursery/crates/auth/Cargo.toml`:

```toml
[package]
name = "myapp-auth"
version.workspace = true
edition.workspace = true

[dependencies]
underlay-core = { workspace = true }
underlay-auth = { workspace = true }
underlay-auth-jwt = { workspace = true }
async-trait = { workspace = true }
serde = { version = "1", features = ["derive"] }
thiserror = "2"
```

Create `apps/nursery/crates/auth/src/lib.rs`:

```rust
//! Authentication boundary for the application.
//!
//! This crate defines the canonical authentication boundary,
//! following the patterns in the Underlay auth documentation.

mod principal;
mod provider;
mod underlay;

pub use principal::{UserId, UserRole, UserPrincipal};
pub use provider::{AuthError, AuthProvider};
pub use underlay::{user_principal_from_underlay, DevBearerUuidAuthProvider, JwtAuthProvider};
```

Create `apps/nursery/crates/auth/src/principal.rs`:

```rust
//! Principal types for authentication.

use underlay_core::Uuid;
use serde::{Deserialize, Serialize};

/// Strongly-typed user identifier (UUIDv7 wrapper).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub Uuid);

/// Roles understood by the application.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
    // Add other roles as needed
}

/// Canonical representation of the logged-in user at the auth boundary.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPrincipal {
    pub user_id: UserId,
    pub roles: Vec<UserRole>,
    pub email: Option<String>,
    pub display_name: Option<String>,
}

impl UserPrincipal {
    /// Check if user has a specific role.
    pub fn has_role(&self, role: UserRole) -> bool {
        self.roles.contains(&role)
    }
}
```

Create `apps/nursery/crates/auth/src/provider.rs`:

```rust
//! Authentication provider traits and errors.

use thiserror::Error;

/// Result type for authentication operations.
pub type AuthResult<T> = Result<T, AuthError>;

/// Authentication errors.
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,

    #[error("Token claims are invalid")]
    InvalidClaims,

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Trait for authentication providers.
#[async_trait::async_trait]
pub trait AuthProvider: Send + Sync {
    /// Authenticate a bearer token and return the principal.
    async fn authenticate_bearer(
        &self,
        bearer_token: &str,
    ) -> AuthResult<super::UserPrincipal>;
}
```

Production JWT is implemented by Underlay’s `underlay-auth-jwt` crate.

- Algorithm: **Ed25519 / EdDSA**
- Library: `jsonwebtoken` (via `underlay-auth-jwt`)
- Key formats: `AUTH_JWT_PRIVATE_KEY` is base64 PKCS#8 DER; `AUTH_JWT_PUBLIC_KEY` is base64url/base64 raw 32-byte public key

You do not need to implement a `jwt.rs` module for the quickstart. Instead, wrap `underlay_auth_jwt::JwtService` in an `underlay_auth::AuthProvider` (see `apps/nursery/crates/auth/src/underlay.rs` below and `docs/guides/quickstart/060-authentication.md`).

Create `apps/nursery/crates/auth/src/underlay.rs`:

```rust
//! Underlay auth integration.

use async_trait::async_trait;
use underlay_auth::{AuthProvider as UnderlayAuthProvider, AuthResult, Principal, RoleSet};
use underlay_core::Uuid;

use crate::{UserId, UserPrincipal, UserRole};

/// Dev auth provider for local development only.
///
/// WARNING: This accepts ANY UUID as a valid token and grants the "user" role.
/// NEVER enable this in production.
#[derive(Debug, Default, Clone, Copy)]
pub struct DevBearerUuidAuthProvider;

#[async_trait]
impl UnderlayAuthProvider for DevBearerUuidAuthProvider {
    async fn authenticate_bearer(&self, bearer_token: &str) -> AuthResult<Principal> {
        let user_id = Uuid::parse_str(bearer_token)
            .map_err(|_| underlay_auth::AuthError::InvalidToken)?;

        Ok(Principal {
            user_id,
            roles: RoleSet::new(["user"]),
        })
    }
}

/// Convert Underlay Principal to app UserPrincipal.
pub fn user_principal_from_underlay(principal: Principal) -> UserPrincipal {
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

/// Production JWT auth provider.
///
/// This uses Underlay’s `underlay-auth-jwt` crate, which implements Ed25519 / EdDSA
/// JWT issuance + verification using `jsonwebtoken`.
#[derive(Clone)]
pub struct JwtAuthProvider {
    jwt: underlay_auth_jwt::JwtService,
}

impl JwtAuthProvider {
    pub fn from_env() -> Result<Self, underlay_auth_jwt::JwtError> {
        let config = underlay_auth_jwt::JwtConfig::from_env()?;
        let jwt = underlay_auth_jwt::JwtService::new(config)?;
        Ok(Self { jwt })
    }
}

#[async_trait]
impl UnderlayAuthProvider for JwtAuthProvider {
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

## Step 4: Create Infrastructure Crate

Create `apps/nursery/crates/infra/Cargo.toml`:

```toml
[package]
name = "myapp-infra"
version.workspace = true
edition.workspace = true

[dependencies]
underlay-observability = { workspace = true }
underlay-metrics = { workspace = true }
config = "0.14"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt", "env-filter", "json"] }
serde = { version = "1", features = ["derive"] }
```

Create `apps/nursery/crates/infra/src/config.rs`:

```rust
//! Application configuration.

use serde::Deserialize;

/// Server configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// Database configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

/// CORS configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_headers: Vec<String>,
}

/// Application configuration loaded from environment.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cors: CorsConfig,
    pub environment: String,
}

impl AppConfig {
    /// Load configuration from environment variables.
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
```

Create `apps/nursery/crates/infra/src/tracing.rs`:

```rust
//! Tracing and observability setup.

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Initialize tracing with JSON logging in production, pretty in development.
pub fn init_tracing() {
    let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into());

    let registry = tracing_subscriber::Registry::default();

    if environment == "production" {
        let env_filter = tracing_subscriber::EnvFilter::builder()
            .from_env_default()
            .expect("Failed to parse RUST_LOG");

        let json_layer = tracing_subscriber::fmt::layer()
            .json()
            .with_span_list(true)
            .finish();

        registry
            .with(env_filter)
            .with(json_layer)
            .init();
    } else {
        let env_filter = tracing_subscriber::EnvFilter::builder()
            .from_env_default()
            .expect("Failed to parse RUST_LOG");

        let pretty_layer = tracing_subscriber::fmt::layer()
            .pretty()
            .with_span_list(true)
            .finish();

        registry
            .with(env_filter)
            .with(pretty_layer)
            .init();
    }
}
```

## Step 5: Verify Build

```bash
cd apps/nursery
cargo check --workspace
cargo test --workspace
```

## Next Steps

With the Rust backend workspace set up, proceed to [050-database](./050-database.md) to configure the database layer.
