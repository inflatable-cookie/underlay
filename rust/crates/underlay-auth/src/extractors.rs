use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};

use underlay_core::AppError;

use crate::{AuthError, AuthProvider, Principal};

pub const AUTHORIZATION_HEADER: &str = "authorization";

/// Trait for application state types that can supply an auth provider.
///
/// This avoids forcing apps into a specific Axum state shape.
pub trait HasAuthProvider {
    fn auth_provider(&self) -> &dyn AuthProvider;
}

#[derive(Debug, Clone)]
pub struct Authenticated(pub Principal);

impl Authenticated {
    pub fn principal(&self) -> &Principal {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct OptionalAuthenticated(pub Option<Principal>);

impl<S> FromRequestParts<S> for Authenticated
where
    S: Send + Sync + HasAuthProvider,
{
    type Rejection = (StatusCode, axum::Json<underlay_core::ErrorEnvelope>);

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let token = extract_bearer(parts.headers.get(AUTHORIZATION_HEADER)).ok_or_else(|| {
                reject(
                    StatusCode::UNAUTHORIZED,
                    AuthError::Unauthorized.into_app_error(),
                )
            })?;

            let principal = state
                .auth_provider()
                .authenticate_bearer(&token)
                .await
                .map_err(|auth_err| {
                    let status = match auth_err {
                        AuthError::Forbidden => StatusCode::FORBIDDEN,
                        AuthError::RateLimited { .. } => StatusCode::TOO_MANY_REQUESTS,
                        AuthError::BadRequest(_) => StatusCode::BAD_REQUEST,
                        AuthError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
                        AuthError::Unauthorized
                        | AuthError::InvalidToken
                        | AuthError::TokenInvalid
                        | AuthError::TokenMalformed
                        | AuthError::TokenNotYetValid
                        | AuthError::TokenFingerprintMismatch
                        | AuthError::SessionExpired
                        | AuthError::SessionRevoked => StatusCode::UNAUTHORIZED,
                        _ => StatusCode::UNAUTHORIZED,
                    };
                    reject(status, auth_err.into_app_error())
                })?;

            Ok(Authenticated(principal))
        }
    }
}

impl<S> FromRequestParts<S> for OptionalAuthenticated
where
    S: Send + Sync + HasAuthProvider,
{
    type Rejection = (StatusCode, axum::Json<underlay_core::ErrorEnvelope>);

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let Some(token) = extract_bearer(parts.headers.get(AUTHORIZATION_HEADER)) else {
                return Ok(OptionalAuthenticated(None));
            };

            match state.auth_provider().authenticate_bearer(&token).await {
                Ok(principal) => Ok(OptionalAuthenticated(Some(principal))),
                Err(auth_err) => {
                    let status = match auth_err {
                        AuthError::Forbidden => StatusCode::FORBIDDEN,
                        AuthError::RateLimited { .. } => StatusCode::TOO_MANY_REQUESTS,
                        AuthError::BadRequest(_) => StatusCode::BAD_REQUEST,
                        AuthError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
                        AuthError::Unauthorized
                        | AuthError::InvalidToken
                        | AuthError::TokenInvalid
                        | AuthError::TokenMalformed
                        | AuthError::TokenNotYetValid
                        | AuthError::TokenFingerprintMismatch
                        | AuthError::SessionExpired
                        | AuthError::SessionRevoked => StatusCode::UNAUTHORIZED,
                        _ => StatusCode::UNAUTHORIZED,
                    };
                    Err(reject(status, auth_err.into_app_error()))
                }
            }
        }
    }
}

fn reject(
    status: StatusCode,
    err: AppError,
) -> (StatusCode, axum::Json<underlay_core::ErrorEnvelope>) {
    // Match `underlay-http` behaviour by setting `x-error-code` when apps use `underlay-http`.
    // (The rejection type itself stays pure.)
    (status, axum::Json(err.into_envelope()))
}

pub(crate) fn extract_bearer(header: Option<&http::HeaderValue>) -> Option<String> {
    let value = header?.to_str().ok()?;
    let value = value.trim();

    let prefix = "Bearer ";
    if !value.starts_with(prefix) {
        return None;
    }

    let token = value[prefix.len()..].trim();
    if token.is_empty() {
        return None;
    }

    Some(token.to_string())
}
