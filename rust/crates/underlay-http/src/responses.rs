use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

use underlay_core::{ListResponse, SingleResponse};

pub fn ok<T>(data: T) -> Response
where
    T: Serialize,
{
    (StatusCode::OK, Json(SingleResponse { data })).into_response()
}

pub fn created<T>(data: T) -> Response
where
    T: Serialize,
{
    (StatusCode::CREATED, Json(SingleResponse { data })).into_response()
}

pub fn list_ok<T>(data: Vec<T>) -> Response
where
    T: Serialize,
{
    (StatusCode::OK, Json(ListResponse { data })).into_response()
}

pub fn no_content() -> Response {
    StatusCode::NO_CONTENT.into_response()
}
