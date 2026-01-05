use http::header::{HeaderName, HeaderValue};
use tower_http::cors::{AllowHeaders, AllowOrigin, Any, CorsLayer};

use underlay_observability::REQUEST_ID_HEADER;

#[derive(Debug, Clone)]
pub struct CorsConfig {
    /// If true, uses `*` (useful for local dev and internal services).
    pub allow_any_origin: bool,
    /// Allowed origins when `allow_any_origin` is false.
    pub allowed_origins: Vec<HeaderValue>,
    /// Additional allowed headers.
    pub allowed_headers: Vec<HeaderName>,
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
        }
    }
}

pub fn cors_layer(config: CorsConfig) -> CorsLayer {
    let allow_origin = if config.allow_any_origin {
        AllowOrigin::any()
    } else {
        AllowOrigin::list(config.allowed_origins)
    };

    let allow_headers = AllowHeaders::list(config.allowed_headers);

    CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_headers(allow_headers)
        .allow_methods(Any)
}
