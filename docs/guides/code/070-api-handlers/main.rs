use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use underlay_core::{AppError, ListResponse, SingleResponse};

#[derive(Clone)]
struct AppState {
    pool: sqlx::PgPool,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct UserDto {
    user_id: String,
    email: String,
}

async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    let rows = match sqlx::query!(r#"SELECT id::text as user_id, email FROM users LIMIT 100"#)
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

    let data = rows
        .into_iter()
        .map(|r| UserDto {
            user_id: r.user_id,
            email: r.email,
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
        r#"SELECT id::text as user_id, email FROM users WHERE id = $1"#,
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
    };

    (StatusCode::OK, Json(SingleResponse { data: dto })).into_response()
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/v1/users", get(list_users))
        .route("/v1/users/:id", get(get_user))
        .with_state(state)
}
