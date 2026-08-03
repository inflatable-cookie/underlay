use http::header::{HeaderName, HeaderValue};
use http::Method;
use std::time::Duration;
use thiserror::Error;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer, ExposeHeaders};

use underlay_observability::{Environment, REQUEST_ID_HEADER};

/// Default max age for CORS preflight caching (1 hour).
pub const DEFAULT_CORS_MAX_AGE_SECS: u64 = 3600;

#[derive(Debug, Error)]
pub enum CorsConfigError {
    #[error("invalid CORS origin `{origin}`: {reason}")]
    InvalidOrigin { origin: String, reason: String },
    #[error(
        "mirror-origin with credentials echoes any Origin with cookies attached and is only \
         permitted in Local/Test environments (environment: {environment:?})"
    )]
    MirrorWithCredentialsOutsideLocal { environment: Environment },
}

/// Configuration for CORS (Cross-Origin Resource Sharing).
///
/// # Example
///
/// ```
/// use underlay_http::CorsConfig;
/// use http::header::HeaderValue;
///
/// // Default config: no cross-origin access (empty explicit origin list,
/// // no credentials). Wildcard is an explicit opt-in via `with_any_origin`.
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
    allow_any_origin: bool,
    /// If true, mirrors the requesting origin in the response.
    /// This allows credentials from any origin (useful for local dev).
    /// Takes precedence over `allow_any_origin` when `allow_credentials` is true.
    mirror_origin: bool,
    /// Allowed origins when `allow_any_origin` is false.
    allowed_origins: Vec<HeaderValue>,
    /// Additional allowed headers.
    allowed_headers: Vec<HeaderName>,
    /// Response headers readable by cross-origin browser clients.
    exposed_headers: Vec<HeaderName>,
    /// If true, allows credentials (cookies, authorization headers).
    allow_credentials: bool,
    /// Max age for preflight request caching in seconds (default: 3600).
    max_age_secs: u64,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allow_any_origin: false,
            mirror_origin: false,
            allowed_origins: vec![],
            allowed_headers: vec![
                HeaderName::from_static(REQUEST_ID_HEADER),
                HeaderName::from_static("authorization"),
                HeaderName::from_static("content-type"),
            ],
            exposed_headers: vec![
                HeaderName::from_static(REQUEST_ID_HEADER),
                HeaderName::from_static("etag"),
                HeaderName::from_static("x-error-code"),
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
    /// Local development only: combined with credentials this lets any site
    /// make credentialed requests. `cors_layer_for_env` refuses to build a
    /// mirror + credentials layer outside `Environment::Local`/`Test`.
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

    /// Set specific allowed origins, returning an error instead of dropping invalid values.
    pub fn try_with_origins<I, S>(mut self, origins: I) -> Result<Self, CorsConfigError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut parsed = Vec::new();
        for origin in origins {
            let origin = origin.as_ref();
            let value =
                HeaderValue::from_str(origin).map_err(|err| CorsConfigError::InvalidOrigin {
                    origin: origin.to_string(),
                    reason: err.to_string(),
                })?;
            parsed.push(value);
        }

        self.allowed_origins = parsed;
        self.allow_any_origin = false;
        self.mirror_origin = false;
        Ok(self)
    }

    /// Set already parsed allowed origins.
    pub fn with_origin_values<I>(mut self, origins: I) -> Self
    where
        I: IntoIterator<Item = HeaderValue>,
    {
        self.allowed_origins = origins.into_iter().collect();
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

    /// Add a response header that browser clients may read cross-origin.
    pub fn with_exposed_header(mut self, header: HeaderName) -> Self {
        self.exposed_headers.push(header);
        self
    }

    /// Set exposed response headers, replacing the defaults.
    pub fn with_exposed_headers(mut self, headers: Vec<HeaderName>) -> Self {
        self.exposed_headers = headers;
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

    pub fn allow_any_origin(&self) -> bool {
        self.allow_any_origin
    }

    pub fn mirror_origin(&self) -> bool {
        self.mirror_origin
    }

    pub fn allowed_origins(&self) -> &[HeaderValue] {
        &self.allowed_origins
    }

    pub fn allowed_headers(&self) -> &[HeaderName] {
        &self.allowed_headers
    }

    pub fn exposed_headers(&self) -> &[HeaderName] {
        &self.exposed_headers
    }

    pub fn allow_credentials(&self) -> bool {
        self.allow_credentials
    }

    pub fn max_age_secs(&self) -> u64 {
        self.max_age_secs
    }
}

/// Build a CORS layer, validating the config against the runtime environment.
///
/// Errors when mirror-origin is combined with credentials outside
/// `Environment::Local`/`Test`: that posture echoes any Origin with cookies
/// attached and must never activate in production.
pub fn try_cors_layer_for_env(
    config: CorsConfig,
    environment: Environment,
) -> Result<CorsLayer, CorsConfigError> {
    if config.allow_credentials && config.mirror_origin && !environment.is_local_dev() {
        return Err(CorsConfigError::MirrorWithCredentialsOutsideLocal { environment });
    }

    Ok(build_cors_layer(config))
}

/// Build a CORS layer for the given environment.
///
/// # Panics
///
/// Panics when mirror-origin is combined with credentials outside
/// `Environment::Local`/`Test`. Use [`try_cors_layer_for_env`] to handle the
/// error instead.
pub fn cors_layer_for_env(config: CorsConfig, environment: Environment) -> CorsLayer {
    try_cors_layer_for_env(config, environment).expect("invalid CORS configuration")
}

/// The canonical CORS config for Underlay admin APIs — the single construction
/// point so consumers do not clone per-app builder functions.
///
/// Shape:
/// - credentials allowed (cookie/token auth);
/// - `x-api-version`, `x-csrf-token`, `if-match` added to the default allowed
///   headers (they trigger browser preflight);
/// - when `explicit_origins` is empty and `environment.is_local_dev()`, the
///   request origin is mirrored (dev ergonomics — one stack, any port/host);
/// - otherwise the explicit list is used (invalid origins are a boot-time
///   panic, not a silent drop).
///
/// # Panics
///
/// Panics on an invalid origin value.
pub fn admin_cors_config(environment: Environment, explicit_origins: Vec<String>) -> CorsConfig {
    let mut config = CorsConfig::default()
        .with_header(HeaderName::from_static("x-api-version"))
        .with_header(HeaderName::from_static("x-csrf-token"))
        .with_header(HeaderName::from_static("if-match"))
        .with_credentials(true);

    if explicit_origins.is_empty() && environment.is_local_dev() {
        config = config.with_mirror_origin();
    } else if !explicit_origins.is_empty() {
        config = config
            .try_with_origins(&explicit_origins)
            .expect("invalid CORS origin in admin_cors_config");
    }

    config
}

/// The canonical CORS layer for Underlay admin APIs, built from
/// [`admin_cors_config`] and gated through [`cors_layer_for_env`].
///
/// # Panics
///
/// Panics on an invalid origin value, and on mirror-origin + credentials
/// outside local dev (via [`cors_layer_for_env`]).
pub fn admin_cors_layer(environment: Environment, explicit_origins: Vec<String>) -> CorsLayer {
    cors_layer_for_env(admin_cors_config(environment, explicit_origins), environment)
}

/// Build a CORS layer without environment context.
///
/// # Panics
///
/// Panics when mirror-origin is combined with credentials: that combination
/// requires an explicit environment gate - use
/// [`cors_layer_for_env`]`(config, Environment::Local)` for local dev.
pub fn cors_layer(config: CorsConfig) -> CorsLayer {
    if config.allow_credentials && config.mirror_origin {
        panic!(
            "mirror-origin with credentials requires an explicit environment gate; \
             use cors_layer_for_env(config, Environment::Local) for local development"
        );
    }

    build_cors_layer(config)
}

fn build_cors_layer(config: CorsConfig) -> CorsLayer {
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
    let expose_headers = ExposeHeaders::list(config.exposed_headers);

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
        .expose_headers(expose_headers)
        .allow_methods(allow_methods)
        .max_age(Duration::from_secs(config.max_age_secs));

    if config.allow_credentials {
        layer.allow_credentials(true)
    } else {
        layer
    }
}
