use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;

use crate::DefaultRegistry;

pub async fn metrics_handler(State(registry): State<DefaultRegistry>) -> impl IntoResponse {
    match registry.gather_text() {
        Ok(body) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/plain; version=0.0.4")],
            body,
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
