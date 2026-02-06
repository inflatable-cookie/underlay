use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// OpenAPI schema representation of `underlay_core::Uuid`.
///
/// Underlay uses UUIDv7 values, but the wire format is the standard UUID string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
#[schema(value_type = String, format = "uuid")]
pub struct ApiUuid(pub underlay_core::Uuid);

impl From<underlay_core::Uuid> for ApiUuid {
    fn from(value: underlay_core::Uuid) -> Self {
        Self(value)
    }
}

impl From<ApiUuid> for underlay_core::Uuid {
    fn from(value: ApiUuid) -> Self {
        value.0
    }
}

/// OpenAPI schema representation of `underlay_core::SingleResponse<T>`.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ApiSingleResponse<T> {
    pub data: T,
}

/// OpenAPI schema representation of `underlay_core::ListResponse<T>`.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ApiListResponse<T> {
    pub data: Vec<T>,
}

/// OpenAPI schema representation of `underlay_core::ErrorEnvelope`.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ApiErrorEnvelope {
    pub error: ApiErrorBody,
}

/// OpenAPI schema representation of `underlay_core::ErrorBody`.
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<HashMap<String, String>>,
}
