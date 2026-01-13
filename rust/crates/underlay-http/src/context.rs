//! Request context extraction for Axum handlers
//!
//! Provides a unified way to extract common request metadata like request ID,
//! client IP, user agent, and user identity from incoming requests.
//!
//! # Example
//!
//! ```ignore
//! use underlay_http::context::RequestContext;
//! use axum::Json;
//!
//! async fn my_handler(ctx: RequestContext) -> Json<String> {
//!     tracing::info!(
//!         request_id = %ctx.request_id(),
//!         ip = ?ctx.ip_address(),
//!         "Processing request"
//!     );
//!     Json("ok".to_string())
//! }
//! ```

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, HeaderMap, StatusCode},
};
use std::net::IpAddr;
use uuid::Uuid;

/// Common request header names
pub mod headers {
    /// Standard request ID header
    pub const X_REQUEST_ID: &str = "x-request-id";
    /// Cloudflare/proxy forwarded IP
    pub const X_FORWARDED_FOR: &str = "x-forwarded-for";
    /// Real IP from reverse proxy
    pub const X_REAL_IP: &str = "x-real-ip";
    /// Cloudflare connecting IP
    pub const CF_CONNECTING_IP: &str = "cf-connecting-ip";
}

/// Request context containing common metadata extracted from the request.
///
/// This struct is designed to be used as an Axum extractor, automatically
/// extracting request metadata from headers.
#[derive(Debug, Clone)]
pub struct RequestContext {
    request_id: String,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
    // User ID would come from auth middleware, stored in extensions
    user_id: Option<Uuid>,
}

impl RequestContext {
    /// Create a new RequestContext with the given values
    pub fn new(
        request_id: String,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
        user_id: Option<Uuid>,
    ) -> Self {
        Self {
            request_id,
            ip_address,
            user_agent,
            user_id,
        }
    }

    /// Get the request ID (always present, generated if not in headers)
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    /// Get the client IP address if available
    pub fn ip_address(&self) -> Option<IpAddr> {
        self.ip_address
    }

    /// Get the user agent string if available
    pub fn user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }

    /// Get the authenticated user ID if available
    ///
    /// This is typically set by auth middleware and stored in request extensions.
    pub fn user_id(&self) -> Option<Uuid> {
        self.user_id
    }

    /// Check if the request is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.user_id.is_some()
    }
}

/// Extension type for storing user ID in request extensions
#[derive(Debug, Clone)]
pub struct AuthenticatedUser(pub Uuid);

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

impl axum::response::IntoResponse for ContextError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ContextError::Unauthenticated => (StatusCode::UNAUTHORIZED, "Authentication required"),
            ContextError::MissingField(_) => (StatusCode::BAD_REQUEST, "Missing required context"),
        };
        (status, message).into_response()
    }
}

/// An authenticated request context that guarantees a user ID is present.
///
/// Use this extractor when an endpoint requires authentication.
/// It will return a 401 Unauthorized if no user is authenticated.
///
/// # Example
///
/// ```ignore
/// use underlay_http::context::AuthenticatedContext;
///
/// async fn protected_handler(ctx: AuthenticatedContext) -> Json<String> {
///     // ctx.user_id() is guaranteed to return a value
///     let user_id = ctx.user_id();
///     Json(format!("Hello, user {}", user_id))
/// }
/// ```
#[derive(Debug, Clone)]
pub struct AuthenticatedContext {
    inner: RequestContext,
    user_id: Uuid,
}

impl AuthenticatedContext {
    /// Get the request ID
    pub fn request_id(&self) -> &str {
        self.inner.request_id()
    }

    /// Get the client IP address if available
    pub fn ip_address(&self) -> Option<IpAddr> {
        self.inner.ip_address()
    }

    /// Get the user agent string if available
    pub fn user_agent(&self) -> Option<&str> {
        self.inner.user_agent()
    }

    /// Get the authenticated user ID (guaranteed to be present)
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    /// Get the underlying RequestContext
    pub fn context(&self) -> &RequestContext {
        &self.inner
    }
}

#[async_trait]
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

        Ok(AuthenticatedContext {
            inner: ctx,
            user_id,
        })
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for RequestContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;

        // Extract or generate request ID
        let request_id = extract_request_id(headers);

        // Extract client IP from various headers
        let ip_address = extract_ip_address(headers);

        // Extract user agent
        let user_agent = headers
            .get(header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // Extract user ID from extensions (set by auth middleware)
        let user_id = parts.extensions.get::<AuthenticatedUser>().map(|u| u.0);

        Ok(RequestContext {
            request_id,
            ip_address,
            user_agent,
            user_id,
        })
    }
}

/// Extract request ID from headers or generate a new one
fn extract_request_id(headers: &HeaderMap) -> String {
    headers
        .get(headers::X_REQUEST_ID)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::now_v7().to_string())
}

/// Extract client IP address from various headers
///
/// Checks headers in order of priority:
/// 1. CF-Connecting-IP (Cloudflare)
/// 2. X-Real-IP (nginx)
/// 3. X-Forwarded-For (first IP)
fn extract_ip_address(headers: &HeaderMap) -> Option<IpAddr> {
    // Try CF-Connecting-IP first (Cloudflare)
    if let Some(ip) = headers
        .get(headers::CF_CONNECTING_IP)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
    {
        return Some(ip);
    }

    // Try X-Real-IP (common in nginx setups)
    if let Some(ip) = headers
        .get(headers::X_REAL_IP)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
    {
        return Some(ip);
    }

    // Try X-Forwarded-For (take first IP)
    if let Some(ip) = headers
        .get(headers::X_FORWARDED_FOR)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim())
        .and_then(|s| s.parse().ok())
    {
        return Some(ip);
    }

    None
}

/// Create a tracing span with request context fields.
///
/// This is useful for structured logging with request context.
///
/// # Example
///
/// ```ignore
/// use underlay_http::context::{RequestContext, make_request_span};
///
/// async fn my_handler(ctx: RequestContext) -> Json<String> {
///     let span = make_request_span(&ctx);
///     let _guard = span.enter();
///     
///     tracing::info!("Processing request");
///     Json("ok".to_string())
/// }
/// ```
#[cfg(feature = "tracing")]
pub fn make_request_span(ctx: &RequestContext) -> tracing::Span {
    tracing::info_span!(
        "request",
        request_id = %ctx.request_id(),
        user_id = ?ctx.user_id(),
        ip = ?ctx.ip_address(),
    )
}

impl RequestContext {
    /// Record context fields to the current tracing span.
    ///
    /// Call this to add request context to structured logs.
    #[cfg(feature = "tracing")]
    pub fn record_to_span(&self, span: &tracing::Span) {
        span.record("request_id", self.request_id());
        if let Some(user_id) = self.user_id() {
            span.record("user_id", tracing::field::display(user_id));
        }
        if let Some(ip) = self.ip_address() {
            span.record("ip", tracing::field::display(ip));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn test_extract_request_id_from_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            headers::X_REQUEST_ID,
            HeaderValue::from_static("test-request-id"),
        );

        let request_id = extract_request_id(&headers);
        assert_eq!(request_id, "test-request-id");
    }

    #[test]
    fn test_extract_request_id_generates_uuid() {
        let headers = HeaderMap::new();
        let request_id = extract_request_id(&headers);

        // Should be a valid UUID
        assert!(Uuid::parse_str(&request_id).is_ok());
    }

    #[test]
    fn test_extract_ip_from_cf_connecting_ip() {
        let mut headers = HeaderMap::new();
        headers.insert(
            headers::CF_CONNECTING_IP,
            HeaderValue::from_static("192.168.1.1"),
        );

        let ip = extract_ip_address(&headers);
        assert_eq!(ip, Some("192.168.1.1".parse().unwrap()));
    }

    #[test]
    fn test_extract_ip_from_x_real_ip() {
        let mut headers = HeaderMap::new();
        headers.insert(headers::X_REAL_IP, HeaderValue::from_static("10.0.0.1"));

        let ip = extract_ip_address(&headers);
        assert_eq!(ip, Some("10.0.0.1".parse().unwrap()));
    }

    #[test]
    fn test_extract_ip_from_x_forwarded_for() {
        let mut headers = HeaderMap::new();
        headers.insert(
            headers::X_FORWARDED_FOR,
            HeaderValue::from_static("203.0.113.195, 70.41.3.18, 150.172.238.178"),
        );

        let ip = extract_ip_address(&headers);
        assert_eq!(ip, Some("203.0.113.195".parse().unwrap()));
    }

    #[test]
    fn test_extract_ip_priority() {
        let mut headers = HeaderMap::new();
        // CF-Connecting-IP should win
        headers.insert(
            headers::CF_CONNECTING_IP,
            HeaderValue::from_static("1.1.1.1"),
        );
        headers.insert(headers::X_REAL_IP, HeaderValue::from_static("2.2.2.2"));
        headers.insert(
            headers::X_FORWARDED_FOR,
            HeaderValue::from_static("3.3.3.3"),
        );

        let ip = extract_ip_address(&headers);
        assert_eq!(ip, Some("1.1.1.1".parse().unwrap()));
    }

    #[test]
    fn test_extract_ip_none_when_missing() {
        let headers = HeaderMap::new();
        let ip = extract_ip_address(&headers);
        assert!(ip.is_none());
    }

    #[test]
    fn test_request_context_methods() {
        let ctx = RequestContext::new(
            "req-123".to_string(),
            Some("192.168.1.1".parse().unwrap()),
            Some("Mozilla/5.0".to_string()),
            Some(Uuid::nil()),
        );

        assert_eq!(ctx.request_id(), "req-123");
        assert_eq!(ctx.ip_address(), Some("192.168.1.1".parse().unwrap()));
        assert_eq!(ctx.user_agent(), Some("Mozilla/5.0"));
        assert_eq!(ctx.user_id(), Some(Uuid::nil()));
        assert!(ctx.is_authenticated());
    }

    #[test]
    fn test_request_context_unauthenticated() {
        let ctx = RequestContext::new("req-456".to_string(), None, None, None);

        assert!(!ctx.is_authenticated());
        assert!(ctx.user_id().is_none());
    }
}
