use http::header::{HeaderName, HeaderValue};
use tower_http::cors::{AllowHeaders, AllowOrigin, Any, CorsLayer};

use underlay_observability::REQUEST_ID_HEADER;

#[derive(Debug, Clone)]
pub struct CorsConfig {
    /// If true, uses `*` (useful for local dev and internal services).
    /// Note: Cannot be true if `allow_credentials` is true.
    pub allow_any_origin: bool,
    /// Allowed origins when `allow_any_origin` is false.
    pub allowed_origins: Vec<HeaderValue>,
    /// Additional allowed headers.
    pub allowed_headers: Vec<HeaderName>,
    /// If true, allows credentials (cookies, authorization headers).
    /// When true, `allow_any_origin` must be false and explicit origins must be specified.
    pub allow_credentials: bool,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allow_any_origin: true,
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
    // When credentials are allowed, we cannot use wildcard origin
    let allow_origin = if config.allow_credentials {
        // Must use explicit origins when credentials are enabled
        AllowOrigin::list(config.allowed_origins)
    } else if config.allow_any_origin {
        AllowOrigin::any()
    } else {
        AllowOrigin::list(config.allowed_origins)
    };

    let allow_headers = AllowHeaders::list(config.allowed_headers);

    let layer = CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_headers(allow_headers)
        .allow_methods(Any);

    if config.allow_credentials {
        layer.allow_credentials(true)
    } else {
        layer
    }
}
