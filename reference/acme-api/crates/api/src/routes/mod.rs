//! Route handlers for the Acme API.

use axum::http::header::HeaderName;
use axum::routing::{get, patch, post};
use axum::Router;
use underlay_http::{cors_layer, CorsConfig};

use crate::state::AppState;

mod shared;
mod tasks;

/// Build the main API router with all routes configured.
pub fn build_router() -> Router<AppState> {
    let cors = build_cors_layer();

    Router::new()
        // Health
        .route("/v1/health", get(shared::health::health))
        // Auth routes
        .route("/v1/auth/register", post(shared::auth::register))
        .route("/v1/auth/login", post(shared::auth::login))
        .route("/v1/auth/login/start", post(shared::auth::login_start))
        .route("/v1/auth/login/finish", post(shared::auth::login_finish))
        .route("/v1/auth/refresh", post(shared::auth::refresh))
        .route("/v1/auth/logout", post(shared::auth::logout))
        .route("/v1/auth/me", get(shared::auth::me))
        .route(
            "/v1/auth/password/change",
            post(shared::auth::change_password),
        )
        .route(
            "/v1/auth/password/requirements",
            get(shared::auth::password_requirements),
        )
        .route(
            "/v1/auth/password/change-2fa",
            post(shared::auth::change_password_with_verification),
        )
        // TOTP routes
        .route("/v1/auth/totp/status", get(shared::auth::totp_status))
        .route("/v1/auth/totp/setup", post(shared::auth::totp_setup))
        .route("/v1/auth/totp/enable", post(shared::auth::totp_enable))
        .route("/v1/auth/totp/disable", post(shared::auth::totp_disable))
        .route("/v1/auth/totp/verify", post(shared::auth::totp_verify))
        // 2FA status route
        .route("/v1/auth/2fa-status", get(shared::auth::two_factor_status))
        // Email TOTP routes
        .route(
            "/v1/auth/email-totp/request",
            post(shared::auth::email_totp_request),
        )
        .route(
            "/v1/auth/email-totp/verify",
            post(shared::auth::email_totp_verify),
        )
        // Session routes
        .route("/v1/auth/sessions", get(shared::auth::list_sessions))
        .route(
            "/v1/auth/sessions/:session_id/revoke",
            post(shared::auth::revoke_session),
        )
        // Account routes
        .route(
            "/v1/account/profile",
            get(shared::account::get_profile).patch(shared::account::update_profile),
        )
        // Project routes
        .route(
            "/v1/projects",
            get(tasks::list_projects).post(tasks::create_project),
        )
        .route(
            "/v1/projects/:project_id",
            get(tasks::get_project)
                .patch(tasks::update_project)
                .delete(tasks::delete_project),
        )
        // Task routes
        .route(
            "/v1/projects/:project_id/tasks",
            get(tasks::list_tasks).post(tasks::create_task),
        )
        .route(
            "/v1/projects/:project_id/tasks/:task_id",
            patch(tasks::update_task).delete(tasks::delete_task),
        )
        .layer(cors)
}

fn build_cors_layer() -> tower_http::cors::CorsLayer {
    // Underlay CORS policy (matches guide patterns):
    // - Use `CORS_ORIGINS` in production.
    // - In local/dev, if `CORS_ORIGINS` is unset, mirror the request origin.
    // - Allow credentials so cookie-based auth can be enabled without reworking CORS.

    let env = std::env::var("ENVIRONMENT")
        .or_else(|_| std::env::var("ACME_ENV"))
        .unwrap_or_else(|_| "local".to_string());

    let origins = parse_cors_origins();

    // If no explicit origins are set and we're in local/dev, mirror request origin.
    let mirror_origin = origins.is_empty() && (env == "local" || env == "dev");

    // NOTE: the browser will preflight if we send `X-Api-Version`.
    // Add it explicitly to allowed headers.
    let mut allowed_headers = CorsConfig::default().allowed_headers;
    allowed_headers.push(HeaderName::from_static("x-api-version"));

    cors_layer(CorsConfig {
        allow_any_origin: false,
        mirror_origin,
        allowed_origins: origins,
        allowed_headers,
        allow_credentials: true,
    })
}

fn parse_cors_origins() -> Vec<axum::http::HeaderValue> {
    let raw = std::env::var("CORS_ORIGINS").unwrap_or_default();
    if raw.trim().is_empty() {
        return vec![];
    }

    raw.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter_map(|s| axum::http::HeaderValue::from_str(s).ok())
        .collect()
}
