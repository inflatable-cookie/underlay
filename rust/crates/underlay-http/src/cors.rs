use http::header::{HeaderName, HeaderValue};
use http::Method;
use std::time::Duration;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use underlay_observability::REQUEST_ID_HEADER;

/// Default max age for CORS preflight caching (1 hour).
pub const DEFAULT_CORS_MAX_AGE_SECS: u64 = 3600;

/// Configuration for CORS (Cross-Origin Resource Sharing).
///
/// # Example
///
/// ```
/// use underlay_http::CorsConfig;
/// use http::header::HeaderValue;
///
/// // Default config (allow any origin, no credentials)
/// let config = CorsConfig::default();
///
/// // Production config with specific origins
/// let config = CorsConfig::default()
///     .with_origins(["https://app.example.com", "https://admin.example.com"])
///     .with_credentials(true)
///     .with_max_age(7200); // 2 hours
/// ```
#[derive(Debug, Clone)]
pub struct CorsConfig {
    /// If true, uses `*` (useful for internal services without credentials).
    /// Note: Cannot be true if `allow_credentials` is true.
    pub allow_any_origin: bool,
    /// If true, mirrors the requesting origin in the response.
    /// This allows credentials from any origin (useful for local dev).
    /// Takes precedence over `allow_any_origin` when `allow_credentials` is true.
    pub mirror_origin: bool,
    /// Allowed origins when `allow_any_origin` is false.
    pub allowed_origins: Vec<HeaderValue>,
    /// Additional allowed headers.
    pub allowed_headers: Vec<HeaderName>,
    /// If true, allows credentials (cookies, authorization headers).
    pub allow_credentials: bool,
    /// Max age for preflight request caching in seconds (default: 3600).
    pub max_age_secs: u64,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allow_any_origin: true,
            mirror_origin: false,
            allowed_origins: vec![],
            allowed_headers: vec![
                HeaderName::from_static(REQUEST_ID_HEADER),
                HeaderName::from_static("authorization"),
                HeaderName::from_static("content-type"),
            ],
            allow_credentials: false,
            max_age_secs: DEFAULT_CORS_MAX_AGE_SECS,
        }
    }
}

impl CorsConfig {
    /// Create a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Allow any origin (uses `*`).
    ///
    /// Note: Cannot be used with credentials.
    pub fn with_any_origin(mut self) -> Self {
        self.allow_any_origin = true;
        self.mirror_origin = false;
        self
    }

    /// Mirror the requesting origin in the response.
    ///
    /// Useful for local development with credentials.
    pub fn with_mirror_origin(mut self) -> Self {
        self.mirror_origin = true;
        self.allow_any_origin = false;
        self
    }

    /// Set specific allowed origins.
    pub fn with_origins<I, S>(mut self, origins: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.allowed_origins = origins
            .into_iter()
            .filter_map(|s| HeaderValue::from_str(s.as_ref()).ok())
            .collect();
        self.allow_any_origin = false;
        self.mirror_origin = false;
        self
    }

    /// Add an additional allowed header.
    pub fn with_header(mut self, header: HeaderName) -> Self {
        self.allowed_headers.push(header);
        self
    }

    /// Set the allowed headers (replaces defaults).
    pub fn with_headers(mut self, headers: Vec<HeaderName>) -> Self {
        self.allowed_headers = headers;
        self
    }

    /// Enable or disable credentials.
    pub fn with_credentials(mut self, allow: bool) -> Self {
        self.allow_credentials = allow;
        self
    }

    /// Set the max age for preflight caching in seconds.
    pub fn with_max_age(mut self, seconds: u64) -> Self {
        self.max_age_secs = seconds;
        self
    }
}

pub fn cors_layer(config: CorsConfig) -> CorsLayer {
    // Determine origin handling:
    // 1. If credentials + mirror_origin: echo the requesting origin (for local dev)
    // 2. If credentials + explicit origins: use the explicit list
    // 3. If no credentials + allow_any: use wildcard *
    // 4. Otherwise: use explicit list
    let allow_origin = if config.allow_credentials && config.mirror_origin {
        // Mirror mode: echo back the requesting origin (works with credentials)
        AllowOrigin::mirror_request()
    } else if config.allow_credentials {
        // Must use explicit origins when credentials are enabled
        AllowOrigin::list(config.allowed_origins)
    } else if config.allow_any_origin {
        AllowOrigin::any()
    } else {
        AllowOrigin::list(config.allowed_origins)
    };

    let allow_headers = AllowHeaders::list(config.allowed_headers);

    // When credentials are enabled, we can't use wildcard for methods either
    let allow_methods = if config.allow_credentials {
        AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
    } else {
        AllowMethods::any()
    };

    let layer = CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_headers(allow_headers)
        .allow_methods(allow_methods)
        .max_age(Duration::from_secs(config.max_age_secs));

    if config.allow_credentials {
        layer.allow_credentials(true)
    } else {
        layer
    }
}
