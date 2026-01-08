# 070 - API Handlers and Routing

This document covers implementing HTTP handlers, routing, and middleware using Axum. The patterns here follow Acowtancy's canonical approach.

## Handler Structure

```
apps/nursery/crates/api/src/
├── main.rs                  # Application entry point + handlers inline
├── error.rs                 # Error handling
├── middleware/
│   └── mod.rs              # Custom middleware (optional)
```

## Handler Pattern

Following Acowtancy's canonical pattern, handlers are typically inline in `main.rs` for simplicity, using Underlay's response types directly.

### Step 1: Error Handling

Create `apps/nursery/crates/api/src/error.rs`:

```rust
use axum::{response::IntoResponse, Json};
use farmyard_core::AppError;
use underlay_core::ErrorEnvelope;

/// Convert AppError to HTTP response.
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = match self {
            AppError::NotFound(resource) => (404, "not_found", resource),
            AppError::Validation(msg) => (400, "validation", &msg),
            AppError::Unauthorized(msg) => (401, "unauthorized", msg),
            AppError::Forbidden(msg) => (403, "forbidden", msg),
            AppError::Conflict(msg) => (409, "conflict", &msg),
            AppError::Internal(msg) => (500, "internal", &msg),
        };

        let envelope = ErrorEnvelope::new(code, message);
        (status, Json(envelope)).into_response()
    }
}

/// Helper for handler results.
pub type HandlerResult<T> = Result<T, AppError>;
```

### Step 2: Application State

Create `apps/nursery/crates/api/src/state.rs`:

```rust
use std::sync::Arc;
use myapp_db::PgPool;
use myapp_auth::AuthProvider;

/// Application state shared across all handlers.
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool.
    pub pool: PgPool,

    /// Authentication provider.
    pub auth_provider: Arc<dyn AuthProvider>,

    // Add domain repositories here as needed
    // pub user_repo: Arc<dyn UserRepository>,
}

impl AppState {
    /// Create new application state.
    pub fn new(pool: PgPool, auth_provider: Arc<dyn AuthProvider>) -> Self {
        Self {
            pool,
            auth_provider,
        }
    }
}
```

### Step 3: Handler Implementation

Following Acowtancy's pattern, handlers use Underlay's extractors and response types directly:

```rust
// In apps/nursery/crates/api/src/main.rs

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use underlay_auth::Authenticated;
use underlay_core::{SingleResponse, ListResponse};

use crate::state::AppState;
use crate::error::HandlerResult;

/// Response DTO for a user.
#[derive(serde::Serialize)]
struct UserDto {
    id: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// GET /api/v1/users/:id
///
/// Requires authentication.
async fn get_user(
    State(state): State<AppState>,
    Authenticated(principal): Authenticated<myapp_auth::UserPrincipal>,
    Path(user_id): Path<String>,
) -> HandlerResult<Json<SingleResponse<UserDto>>> {
    // Parse UUID
    let user_id: uuid::Uuid = user_id
        .parse()
        .map_err(|_| AppError::Validation("Invalid user ID"))?;

    // Fetch from database
    let user = sqlx::query_as!(
        UserDto,
        r#"
        SELECT id::text, email, created_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(AppError::Internal)?
    .ok_or_else(|| AppError::NotFound("User not found"))?;

    Ok(Json(SingleResponse { data: user }))
}

/// GET /api/v1/users
///
/// List users (paginated).
async fn list_users(
    State(state): State<AppState>,
    Authenticated(_principal): Authenticated<myapp_auth::UserPrincipal>,
) -> HandlerResult<Json<ListResponse<UserDto>>> {
    let users = sqlx::query_as!(
        UserDto,
        r#"
        SELECT id::text, email, created_at
        FROM users
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(AppError::Internal)?;

    Ok(Json(ListResponse {
        items: users,
        pagination: None, // Add pagination if needed
    }))
}
```

## Router Setup

```rust
fn create_router(state: AppState) -> Router {
    use tower_http::cors::CorsLayer;
    use tower_http::trace::TraceLayer;

    let cors = CorsLayer::permissive();

    Router::new()
        // Health check (no auth)
        .route("/health", get(|| async { "ok" }))

        // API routes (auth required)
        .route("/api/v1/users", get(list_users))
        .route("/api/v1/users/:id", get(get_user))

        // Add state and middleware
        .layer(axum::Extension(state))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
```

## Using Underlay Extractors

Acowtancy uses Underlay's `Authenticated` extractor:

```rust
use underlay_auth::Authenticated;

async fn protected_handler(
    Authenticated(principal): Authenticated<myapp_auth::UserPrincipal>,
) {
    // principal.user_id contains the authenticated user's ID
    // principal.roles contains their roles
}
```

## Next Step

With API handlers implemented, proceed to [080-typescript-client](./080-typescript-client.md) to create the TypeScript API client.
