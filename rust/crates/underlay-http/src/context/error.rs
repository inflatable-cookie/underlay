use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Error type for context extraction failures
#[derive(Debug, Clone)]
pub enum ContextError {
    /// User is not authenticated (no user ID in context)
    Unauthenticated,
    /// Required context field is missing
    MissingField(&'static str),
}

impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextError::Unauthenticated => write!(f, "Authentication required"),
            ContextError::MissingField(field) => write!(f, "Missing required field: {}", field),
        }
    }
}

impl std::error::Error for ContextError {}

impl IntoResponse for ContextError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ContextError::Unauthenticated => (StatusCode::UNAUTHORIZED, "Authentication required"),
            ContextError::MissingField(_) => (StatusCode::BAD_REQUEST, "Missing required context"),
        };
        (status, message).into_response()
    }
}
