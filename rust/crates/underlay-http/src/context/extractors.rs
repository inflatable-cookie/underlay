use axum::extract::FromRequestParts;
use axum::http::{header, request::Parts, StatusCode};

use super::error::ContextError;
use super::model::{AuthenticatedContext, AuthenticatedUser, RequestContext};
use super::parse::{extract_ip_address, extract_request_id};
#[cfg(feature = "opentelemetry")]
use underlay_observability::TraceContext;

impl<S> FromRequestParts<S> for AuthenticatedContext
where
    S: Send + Sync,
{
    type Rejection = ContextError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx = RequestContext::from_request_parts(parts, state)
            .await
            .map_err(|_| ContextError::MissingField("request context"))?;

        let user_id = ctx.user_id().ok_or(ContextError::Unauthenticated)?;

        Ok(AuthenticatedContext::from_context(ctx, user_id))
    }
}

impl<S> FromRequestParts<S> for RequestContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;

        let request_id = extract_request_id(headers);
        let ip_address = extract_ip_address(headers);
        let user_agent = headers
            .get(header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let user_id = parts.extensions.get::<AuthenticatedUser>().map(|u| u.0);

        #[cfg(feature = "opentelemetry")]
        let trace_context = TraceContext::from_headers(headers);

        Ok(RequestContext::from_parts(
            request_id,
            ip_address,
            user_agent,
            user_id,
            #[cfg(feature = "opentelemetry")]
            trace_context,
        ))
    }
}
