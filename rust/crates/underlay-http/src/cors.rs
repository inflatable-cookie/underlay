use http::header::{HeaderName, HeaderValue};
use http::Method;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use underlay_observability::REQUEST_ID_HEADER;

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
        }
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
        .allow_methods(allow_methods);

    if config.allow_credentials {
        layer.allow_credentials(true)
    } else {
        layer
    }
}
