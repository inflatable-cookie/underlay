# 070 - API Handlers, Routing, and Errors

This document covers implementing HTTP handlers and routing using Axum.

The patterns here are intentionally simple and align with Underlay’s primitives:

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

## API Version Header (Optional)

If you use date-based versioning (Acowtancy-style), send a header like:

- `X-Api-Version: 2025-01-01`

Keep the URL stable (`/v1/...`). This makes it easier to evolve behavior without changing clients’ base URLs.

## Next Step

Proceed to [080-typescript-client](./080-typescript-client.md) to build a typed client that matches these envelopes and errors.
