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
    extract::FromRequestParts,
    http::{header, request::Parts, HeaderMap, StatusCode},
};
use std::net::IpAddr;
#[cfg(feature = "opentelemetry")]
use underlay_observability::TraceContext;
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
    #[cfg(feature = "opentelemetry")]
    trace_context: Option<TraceContext>,
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
            #[cfg(feature = "opentelemetry")]
            trace_context: None,
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

    #[cfg(feature = "opentelemetry")]
    pub fn with_trace_context(mut self, trace_context: TraceContext) -> Self {
        self.trace_context = Some(trace_context);
        self
    }

    #[cfg(feature = "opentelemetry")]
    pub fn trace_context(&self) -> Option<&TraceContext> {
        self.trace_context.as_ref()
    }

    #[cfg(feature = "opentelemetry")]
    pub fn trace_id(&self) -> Option<&str> {
        self.trace_context().map(TraceContext::trace_id)
    }

    #[cfg(feature = "opentelemetry")]
    pub fn parent_span_id(&self) -> Option<&str> {
        self.trace_context().map(TraceContext::parent_span_id)
    }

    #[cfg(feature = "opentelemetry")]
    pub fn inject_trace_context(&self, headers: &mut HeaderMap) {
        if let Some(trace_context) = self.trace_context() {
            trace_context.inject_into(headers);
        }
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

    #[cfg(feature = "opentelemetry")]
    pub fn trace_context(&self) -> Option<&TraceContext> {
        self.inner.trace_context()
    }

    #[cfg(feature = "opentelemetry")]
    pub fn trace_id(&self) -> Option<&str> {
        self.inner.trace_id()
    }

    #[cfg(feature = "opentelemetry")]
    pub fn parent_span_id(&self) -> Option<&str> {
        self.inner.parent_span_id()
    }

    #[cfg(feature = "opentelemetry")]
    pub fn inject_trace_context(&self, headers: &mut HeaderMap) {
        self.inner.inject_trace_context(headers);
    }
}

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

        #[cfg(feature = "opentelemetry")]
        let trace_context = TraceContext::from_headers(headers);

        Ok(RequestContext {
            request_id,
            ip_address,
            user_agent,
            user_id,
            #[cfg(feature = "opentelemetry")]
            trace_context,
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
    #[cfg(feature = "opentelemetry")]
    let span = tracing::info_span!(
        "request",
        request_id = %ctx.request_id(),
        user_id = ?ctx.user_id(),
        ip = ?ctx.ip_address(),
        trace_id = tracing::field::Empty,
        parent_span_id = tracing::field::Empty,
        trace_flags = tracing::field::Empty,
        tracestate = tracing::field::Empty,
    );

    #[cfg(not(feature = "opentelemetry"))]
    let span = tracing::info_span!(
        "request",
        request_id = %ctx.request_id(),
        user_id = ?ctx.user_id(),
        ip = ?ctx.ip_address(),
    );

    #[cfg(feature = "opentelemetry")]
    if let Some(trace_context) = ctx.trace_context() {
        span.record("trace_id", trace_context.trace_id());
        span.record("parent_span_id", trace_context.parent_span_id());
        span.record("trace_flags", trace_context.trace_flags());
        if let Some(tracestate) = trace_context.tracestate() {
            span.record("tracestate", tracestate);
        }
    }

    span
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
        #[cfg(feature = "opentelemetry")]
        if let Some(trace_context) = self.trace_context() {
            span.record("trace_id", trace_context.trace_id());
            span.record("parent_span_id", trace_context.parent_span_id());
            span.record("trace_flags", trace_context.trace_flags());
            if let Some(tracestate) = trace_context.tracestate() {
                span.record("tracestate", tracestate);
            }
        }
    }
}

#[cfg(test)]
#[path = "tests/context_tests.rs"]
mod tests;
