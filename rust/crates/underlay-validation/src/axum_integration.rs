//! Axum integration for validation.
//!
//! Provides `ValidatedJson` extractor that automatically validates request bodies
//! and returns proper HTTP error responses.

use axum::{
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;

use crate::{Validate, ValidationError};

/// A JSON extractor that validates the request body.
///
/// This works like `axum::Json`, but additionally validates the deserialized
/// value using the `Validate` trait. If validation fails, it returns a 400
/// response with field-level errors.
///
/// # Example
///
/// ```rust,ignore
/// use axum::{routing::post, Router};
/// use underlay_validation::{ValidatedJson, Validate, ValidationError, validators};
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct CreateUser {
///     email: String,
///     password: String,
/// }
///
/// impl Validate for CreateUser {
///     fn validate(&self) -> Result<(), ValidationError> {
///         let mut errors = ValidationError::new();
///         if let Err(e) = validators::email(&self.email) {
///             errors.add_field("email", e);
///         }
///         errors.into_result()
///     }
/// }
///
/// async fn create_user(ValidatedJson(payload): ValidatedJson<CreateUser>) {
///     // payload is guaranteed to be valid here
/// }
///
/// let app = Router::new().route("/users", post(create_user));
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

/// Error type for `ValidatedJson` extraction failures.
#[derive(Debug)]
pub enum ValidatedJsonRejection {
    /// JSON deserialization failed.
    JsonRejection(axum::extract::rejection::JsonRejection),
    /// Validation failed.
    ValidationError(ValidationError),
}

impl IntoResponse for ValidatedJsonRejection {
    fn into_response(self) -> Response {
        match self {
            Self::JsonRejection(rejection) => {
                let body = serde_json::json!({
                    "error": {
                        "code": "json.invalid",
                        "message": rejection.body_text(),
                    }
                });
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }
            Self::ValidationError(error) => {
                let body = serde_json::json!({
                    "error": {
                        "code": error.code.as_deref().unwrap_or("validation.failed"),
                        "message": error.message.as_deref().unwrap_or("Validation failed"),
                        "field_errors": error.field_errors,
                    }
                });
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }
        }
    }
}

impl From<axum::extract::rejection::JsonRejection> for ValidatedJsonRejection {
    fn from(rejection: axum::extract::rejection::JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

impl From<ValidationError> for ValidatedJsonRejection {
    fn from(error: ValidationError) -> Self {
        Self::ValidationError(error)
    }
}

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = ValidatedJsonRejection;

    fn from_request(
        req: Request,
        state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // First, extract and deserialize the JSON
            let Json(value) = Json::<T>::from_request(req, state).await?;

            // Then validate it
            value.validate()?;

            Ok(ValidatedJson(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validators;

    #[derive(Debug, serde::Deserialize)]
    struct TestRequest {
        email: String,
    }

    impl Validate for TestRequest {
        fn validate(&self) -> crate::ValidationResult<()> {
            let mut errors = ValidationError::new();
            if let Err(e) = validators::email(&self.email) {
                errors.add_field("email", e);
            }
            errors.into_result()
        }
    }

    #[test]
    fn test_validation_error_response() {
        let mut error = ValidationError::with_message("Validation failed");
        error.add_field("email", "Invalid email");

        let rejection = ValidatedJsonRejection::ValidationError(error);
        let response = rejection.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
