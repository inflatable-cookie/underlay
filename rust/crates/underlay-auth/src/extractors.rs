use axum::extract::FromRequestParts;
use axum::http::{header, request::Parts, HeaderMap, HeaderValue, StatusCode};

use crate::{AuthError, AuthProvider, Principal};

pub const AUTHORIZATION_HEADER: &str = "authorization";

type AuthRejection = (
    StatusCode,
    HeaderMap,
    axum::Json<underlay_core::ErrorEnvelope>,
);

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
    type Rejection = AuthRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = extract_bearer(parts.headers.get(AUTHORIZATION_HEADER))
            .ok_or_else(|| reject(AuthError::Unauthorized))?;

        let principal = state
            .auth_provider()
            .authenticate_bearer(&token)
            .await
            .map_err(reject)?;

        Ok(Authenticated(principal))
    }
}

impl<S> FromRequestParts<S> for OptionalAuthenticated
where
    S: Send + Sync + HasAuthProvider,
{
    type Rejection = AuthRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Some(token) = extract_bearer(parts.headers.get(AUTHORIZATION_HEADER)) else {
            return Ok(OptionalAuthenticated(None));
        };

        match state.auth_provider().authenticate_bearer(&token).await {
            Ok(principal) => Ok(OptionalAuthenticated(Some(principal))),
            Err(auth_err) => Err(reject(auth_err)),
        }
    }
}

fn reject(err: AuthError) -> AuthRejection {
    let mut headers = HeaderMap::new();
    let status = match err {
        AuthError::Forbidden => StatusCode::FORBIDDEN,
        AuthError::RateLimited {
            retry_after_seconds,
        } => {
            headers.insert(header::RETRY_AFTER, HeaderValue::from(retry_after_seconds));
            StatusCode::TOO_MANY_REQUESTS
        }
        AuthError::BadRequest(_) => StatusCode::BAD_REQUEST,
        AuthError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::UNAUTHORIZED,
    };
    (
        status,
        headers,
        axum::Json(err.into_app_error().into_envelope()),
    )
}

pub(crate) fn extract_bearer(header: Option<&http::HeaderValue>) -> Option<String> {
    let value = header?.to_str().ok()?;
    let value = value.trim();

    let (scheme, rest) = value.split_once(' ')?;
    if !scheme.eq_ignore_ascii_case("bearer") {
        return None;
    }

    let token = rest.trim();
    if token.is_empty() {
        return None;
    }

    Some(token.to_string())
}
