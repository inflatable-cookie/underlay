use std::net::SocketAddr;

use axum::extract::{ConnectInfo, FromRequestParts};
use axum::http::{header, request::Parts};

use super::error::ContextError;
use super::model::{AuthenticatedContext, AuthenticatedUser, RequestContext};
use super::parse::{extract_request_id, resolve_ip_address, TrustedProxyConfig};
#[cfg(feature = "opentelemetry")]
use underlay_observability::TraceContext;

impl<S> FromRequestParts<S> for AuthenticatedContext
where
    S: Send + Sync,
{
    type Rejection = ContextError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx = RequestContext::from_request_parts(parts, state).await?;

        let user_id = ctx.user_id().ok_or(ContextError::Unauthenticated)?;

        Ok(AuthenticatedContext::from_context(ctx, user_id))
    }
}

impl<S> FromRequestParts<S> for RequestContext
where
    S: Send + Sync,
{
    type Rejection = ContextError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;

        let request_id = extract_request_id(headers);

        let trusted_proxy = parts
            .extensions
            .get::<TrustedProxyConfig>()
            .cloned()
            .unwrap_or_default();
        let socket_ip = parts
            .extensions
            .get::<ConnectInfo<SocketAddr>>()
            .map(|info| info.0.ip());
        let ip_address = resolve_ip_address(headers, &trusted_proxy, socket_ip);
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
