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

### Error Logging

> **Quick Start**: Underlay provides production-ready error logging via the `underlay-http` crate's `error-logging` feature. For full documentation, see [rust/crates/underlay-http/ERROR_LOGGING.md](../../rust/crates/underlay-http/ERROR_LOGGING.md).

Log errors to a database for monitoring and debugging. This pattern is used in Acowtancy's Farmyard.

**Benefits:**
- Centralized error tracking
- Correlation with request IDs
- Debug production issues
- Monitor error trends
- Query by time range, status code, or endpoint

#### Setup

Add to your `Cargo.toml`:

```toml
[dependencies]
underlay-http = { path = "../../rust/crates/underlay-http", features = ["error-logging"] }
```

Sync migrations to your app:

```bash
cargo run --bin underlay-devtools -- sync-migrations --target ./migrations
```

Then run your migrations. This creates the `infra.error_log` table with indexes.

#### Basic Usage in Handlers

Log errors directly from error handlers:

```rust
use underlay_http::error_logging::append_error_log;
use underlay_observability::RequestId;

async fn get_user(
    State(state): State<AppState>,
    request_id: RequestId,
    Path(user_id): Path<String>
) -> impl IntoResponse {
    let id: uuid::Uuid = match user_id.parse() {
        Ok(id) => id,
        Err(_) => {
            let error = AppError::new("validation.invalid_id", "Invalid user ID");
            
            // Log the error asynchronously (non-blocking)
            let pool = state.pool.clone();
            tokio::spawn(async move {
                let _ = append_error_log(
                    &pool,
                    "/v1/users/:id",                    // endpoint
                    "GET",                               // method
                    400,                                 // status code
                    "validation.invalid_id",             // error code
                    "Invalid user ID",                   // message
                    &request_id.to_string(),             // correlation ID
                    serde_json::json!({"user_id": user_id}), // context
                ).await;
            });
            
            return underlay_http::error_response(
                StatusCode::BAD_REQUEST,
                error,
            );
        }
    };
    
    // ... rest of handler
}
```

**Note**: Always wrap `append_error_log()` in `tokio::spawn()` to avoid blocking the request response.

#### Querying Error Logs

Query recent errors for debugging:

```rust
use underlay_http::error_logging::{list_error_logs, ErrorLogFilters};
use chrono::{Utc, Duration};

// Get all 500 errors in the last hour
let filters = ErrorLogFilters {
    since: Some(Utc::now() - Duration::hours(1)),
    status_code: Some(500),
    limit: 50,
    ..Default::default()
};

let errors = list_error_logs(&pool, filters).await?;

for error in errors {
    println!("{}: {} - {}", 
        error.occurred_at, 
        error.endpoint, 
        error.message
    );
}
```

#### Advanced: Middleware-Based Logging

For automatic error logging across all endpoints, implement a Tower middleware layer (planned in Phase 8.3 task 3). The current implementation provides the building blocks:

- `append_error_log()` - Direct database insertion
- `DbErrorLogSink` - Implements `ErrorLogSink` trait
- `ErrorLogFilters` - Query builder for error logs

See [rust/crates/underlay-http/ERROR_LOGGING.md](../../rust/crates/underlay-http/ERROR_LOGGING.md) for:
- Complete API documentation
- Database schema details
- Best practices for correlation IDs
- Indexing and query optimization
- Future roadmap (Tower middleware, retention policies)

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

