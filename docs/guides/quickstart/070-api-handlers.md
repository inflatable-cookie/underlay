# 070 - API Handlers

> **Reference Implementation**: This guide includes patterns from Acowtancy, a production application built with Underlay. These serve as working examples of best practices.

This document covers implementing HTTP handlers and routing using Axum.

The patterns here are intentionally simple and align with Underlay's primitives:

- API routes use the `/v1/...` prefix.
- Responses use `underlay_core::{SingleResponse, ListResponse}`.
- Errors use `underlay_core::AppError` wrapped via `underlay_http::error_response`.

## Handler Structure

For small-to-medium services, keep things inline in `main.rs`:

```
apps/nursery/crates/api/src/
├── main.rs        # Router + handlers + DTOs
└── state.rs       # AppState (optional)
```

As the API grows, split by domain:

```
apps/nursery/crates/api/src/
├── main.rs
├── state.rs
├── error.rs
└── handlers/
    ├── mod.rs
    ├── users.rs
    ├── artists.rs
    └── admin.rs
```

## Response Envelopes

Underlay defines canonical response shapes:

```rust
use underlay_core::{ListResponse, SingleResponse};

// list
ListResponse { data: vec![/* ... */] }

// single
SingleResponse { data: /* ... */ }
```

## Errors

Underlay’s error envelope is:

```json
{
  "error": {
    "code": "resource.not_found",
    "message": "User not found",
    "fieldErrors": {
      "email": "Must be a valid email"
    }
  }
}
```

In Rust, build errors with `underlay_core::AppError` and return them using `underlay_http::error_response`:

```rust
use axum::{http::StatusCode, response::IntoResponse};
use underlay_core::AppError;

fn not_found(resource: &str) -> impl IntoResponse {
    underlay_http::error_response(
        StatusCode::NOT_FOUND,
        AppError::new("resource.not_found", format!("{} not found", resource)),
    )
}
```

## AppState

Keep `AppState` minimal and explicit:

```rust
use std::sync::Arc;
use myapp_db::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub auth_provider: Arc<dyn underlay_auth::AuthProvider>,
}

impl underlay_auth::HasAuthProvider for AppState {
    fn auth_provider(&self) -> &dyn underlay_auth::AuthProvider {
        self.auth_provider.as_ref()
    }
}
```

## Example Handlers

These examples use `/v1/...` routes and the Underlay envelopes.

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use underlay_core::{ListResponse, SingleResponse};
use underlay_core::AppError;

use crate::state::AppState;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct UserDto {
    user_id: String,
    email: String,
    created_at: String,
}

async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    let rows = match sqlx::query!(
        r#"
        SELECT id::text as user_id, email, created_at
        FROM users
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&state.pool)
    .await
    {
        Ok(rows) => rows,
        Err(err) => {
            return underlay_http::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                AppError::new("db.query_failed", err.to_string()),
            );
        }
    };

    let data: Vec<UserDto> = rows
        .into_iter()
        .map(|r| UserDto {
            user_id: r.user_id,
            email: r.email,
            created_at: r.created_at.to_rfc3339(),
        })
        .collect();

    (StatusCode::OK, Json(ListResponse { data })).into_response()
}

async fn get_user(State(state): State<AppState>, Path(user_id): Path<String>) -> impl IntoResponse {
    let id: uuid::Uuid = match user_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return underlay_http::error_response(
                StatusCode::BAD_REQUEST,
                AppError::new("validation.invalid_id", "Invalid user ID"),
            );
        }
    };

    let row = match sqlx::query!(
        r#"
        SELECT id::text as user_id, email, created_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await
    {
        Ok(row) => row,
        Err(err) => {
            return underlay_http::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                AppError::new("db.query_failed", err.to_string()),
            );
        }
    };

    let Some(row) = row else {
        return underlay_http::error_response(
            StatusCode::NOT_FOUND,
            AppError::new("resource.not_found", "User not found"),
        );
    };

    let dto = UserDto {
        user_id: row.user_id,
        email: row.email,
        created_at: row.created_at.to_rfc3339(),
    };

    (StatusCode::OK, Json(SingleResponse { data: dto })).into_response()
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/v1/users", get(list_users))
        .route("/v1/users/:id", get(get_user))
        .with_state(state)
}
```

## Production Patterns

### Error Logging Middleware

Log errors to a database for monitoring and debugging. This pattern is used in Acowtancy's Farmyard.

**Benefits:**
- Centralized error tracking
- Correlation with request IDs
- Debug production issues
- Monitor error trends

#### Implementation

Create `apps/nursery/crates/api/src/middleware/error_logger.rs`:

```rust
use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::spawn;

/// Middleware to log errors to database.
pub async fn error_logger_middleware(
    req: Request,
    next: Next,
) -> Response {
    let request_id = req
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let method = req.method().clone();
    let uri = req.uri().clone();

    // Get pool from extensions if available
    let pool = req
        .extensions()
        .get::<Arc<PgPool>>()
        .cloned();

    let response = next.run(req).await;

    // Log errors (4xx/5xx) asynchronously
    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        if let Some(pool) = pool {
            // Extract error message from response if possible
            // (In practice, you might need to clone the body)
            let error_message = format!("{} {} -> {}", method, uri, status);

            // Spawn async task to avoid blocking response
            spawn(async move {
                if let Err(e) = log_error_to_db(
                    &pool,
                    &request_id,
                    status.as_u16(),
                    &error_message,
                )
                .await
                {
                    eprintln!("Failed to log error: {}", e);
                }
            });
        }
    }

    response
}

async fn log_error_to_db(
    pool: &PgPool,
    request_id: &str,
    status_code: u16,
    message: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO platform.error_log (request_id, status_code, message, occurred_at)
        VALUES ($1, $2, $3, NOW())
        "#,
    )
    .bind(request_id)
    .bind(status_code as i32)
    .bind(message)
    .execute(pool)
    .await?;

    Ok(())
}
```

#### Database Schema

```sql
CREATE SCHEMA IF NOT EXISTS platform;

CREATE TABLE platform.error_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    request_id VARCHAR(255) NOT NULL,
    status_code INTEGER NOT NULL,
    message TEXT NOT NULL,
    user_id UUID,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_error_log_request_id ON platform.error_log(request_id);
CREATE INDEX idx_error_log_occurred_at ON platform.error_log(occurred_at DESC);
CREATE INDEX idx_error_log_status_code ON platform.error_log(status_code);
```

#### Usage in Router

```rust
use axum::{middleware, Router};
use std::sync::Arc;

pub fn create_router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/v1/users", get(list_users))
        // ... other routes
        .layer(middleware::from_fn(error_logger_middleware))
        .layer(Extension(pool))
}
```

**Note:** This is a simplified example. In production, you might:
- Extract error details from response body
- Include user ID from authentication
- Add IP address, user agent
- Implement sampling to reduce database load

### Request Tracing

Use Underlay's observability layer for structured logging:

```rust
use underlay_observability::init_tracing;

#[tokio::main]
async fn main() {
    // Initialize tracing (pretty for dev, JSON for prod)
    init_tracing();

    // Handlers automatically get tracing context
    let app = Router::new()
        .route("/v1/users", get(list_users))
        .layer(underlay_observability::trace_layer());

    // ...
}

async fn list_users() -> impl IntoResponse {
    tracing::info!("Listing users");  // Includes request ID automatically
    // ...
}
```

### Rate Limiting (Optional)

Protect endpoints from abuse:

```rust
use tower_governor::{
    governor::GovernorConfigBuilder,
    GovernorLayer,
};

pub fn create_router() -> Router {
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(10)
            .burst_size(20)
            .finish()
            .unwrap(),
    );

    Router::new()
        .route("/v1/auth/login", post(login))
        .layer(GovernorLayer {
            config: Box::leak(governor_conf),
        })
}
```

Add to `Cargo.toml`:
```toml
tower-governor = "0.1"
```

## API Version Header (Optional)

If you use date-based versioning (Acowtancy-style), send a header like:

- `X-Api-Version: 2025-01-01`

Keep the URL stable (`/v1/...`). This makes it easier to evolve behavior without changing clients' base URLs.

## See Also

**Related Guides:**
- **[060-authentication.md](./060-authentication.md)** - Authentication setup for protected endpoints
- **[067-authorization.md](./067-authorization.md)** - Role-based access control in handlers
- **[075-validation.md](./075-validation.md)** - Request validation patterns
- **[130-testing.md](./130-testing.md)** - Testing API handlers and integration tests

**Key Patterns:**
- Use `SingleResponse` and `ListResponse` for consistency
- Handle errors with `AppError` and `error_response`
- Add request ID middleware for traceability
- Log errors to database for monitoring
- Test handlers with mock repositories

## Next Steps

Proceed to [080-typescript-client](./080-typescript-client.md) to build a typed client that matches these envelopes and errors.

