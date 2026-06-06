use std::net::IpAddr;

#[cfg(feature = "opentelemetry")]
use axum::http::HeaderMap;
#[cfg(feature = "opentelemetry")]
use underlay_observability::TraceContext;
use uuid::Uuid;

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

    pub(in crate::context) fn from_parts(
        request_id: String,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
        user_id: Option<Uuid>,
        #[cfg(feature = "opentelemetry")] trace_context: Option<TraceContext>,
    ) -> Self {
        Self {
            request_id,
            ip_address,
            user_agent,
            user_id,
            #[cfg(feature = "opentelemetry")]
            trace_context,
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
    pub(in crate::context) fn from_context(inner: RequestContext, user_id: Uuid) -> Self {
        Self { inner, user_id }
    }

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
