use super::{validation, AuthCookieError, CookieDomain, CookieName, CookiePath, SameSite};

/// Configuration for auth cookies.
///
/// # Example
///
/// ```
/// use underlay_http::cookies::AuthCookieConfig;
///
/// // Default config (secure, 7-day refresh token)
/// let config = AuthCookieConfig::default();
///
/// // Custom config with builder pattern
/// let config = AuthCookieConfig::default()
///     .try_with_domain(".example.com")?
///     .with_refresh_token_max_age(14 * 24 * 60 * 60) // 14 days
///     .try_with_cookie_prefix("myapp_")?;
/// # Ok::<(), underlay_http::cookies::AuthCookieError>(())
/// ```
#[derive(Debug, Clone)]
pub struct AuthCookieConfig {
    /// Domain for cookies (e.g., ".example.com" for cross-subdomain).
    /// If None, cookies are scoped to the current host only.
    pub(crate) domain: Option<String>,
    /// Whether to set Secure flag (should be true in production).
    pub(crate) secure: bool,
    /// Refresh token lifetime in seconds (default: 7 days).
    pub(crate) refresh_token_max_age: u64,
    /// SameSite policy for cookies (default: Lax).
    pub(crate) same_site: SameSite,
    /// Prefix for cookie names (default: empty).
    /// E.g., "acme_" produces cookies named "acme_refresh_token" and "acme_logged_in".
    pub(crate) cookie_prefix: String,
    /// Path for the refresh token cookie (default: "/v1/auth").
    pub(crate) refresh_token_path: String,
}

impl Default for AuthCookieConfig {
    fn default() -> Self {
        Self {
            domain: None,
            secure: true,
            refresh_token_max_age: 7 * 24 * 60 * 60, // 7 days
            same_site: SameSite::Lax,
            cookie_prefix: String::new(),
            refresh_token_path: "/v1/auth".to_string(),
        }
    }
}

impl AuthCookieConfig {
    /// Create a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create config for local development (insecure).
    pub fn local_dev() -> Self {
        Self {
            domain: None,
            secure: false,
            refresh_token_max_age: 7 * 24 * 60 * 60,
            same_site: SameSite::Lax,
            cookie_prefix: String::new(),
            refresh_token_path: "/v1/auth".to_string(),
        }
    }

    /// Set a pre-validated cookie domain.
    pub fn with_cookie_domain(mut self, domain: CookieDomain) -> Self {
        self.domain = Some(domain.to_string());
        self
    }

    /// Validate and set the cookie domain.
    pub fn try_with_domain(mut self, domain: impl AsRef<str>) -> Result<Self, AuthCookieError> {
        self.domain = Some(CookieDomain::parse(domain)?.to_string());
        Ok(self)
    }

    /// Set whether cookies should be secure (HTTPS only).
    pub fn with_secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// Set the refresh token max age in seconds.
    pub fn with_refresh_token_max_age(mut self, seconds: u64) -> Self {
        self.refresh_token_max_age = seconds;
        self
    }

    /// Set the SameSite policy.
    pub fn with_same_site(mut self, same_site: SameSite) -> Self {
        self.same_site = same_site;
        self
    }

    /// Validate and set a prefix for cookie names.
    pub fn try_with_cookie_prefix(
        mut self,
        prefix: impl AsRef<str>,
    ) -> Result<Self, AuthCookieError> {
        validation::validate_cookie_name_prefix(prefix.as_ref())?;
        self.cookie_prefix = prefix.as_ref().to_string();
        Ok(self)
    }

    /// Set a pre-validated path for the refresh token cookie.
    pub fn with_refresh_cookie_path(mut self, path: CookiePath) -> Self {
        self.refresh_token_path = path.to_string();
        self
    }

    /// Validate and set the path for the refresh token cookie.
    pub fn try_with_refresh_token_path(
        mut self,
        path: impl AsRef<str>,
    ) -> Result<Self, AuthCookieError> {
        self.refresh_token_path = CookiePath::parse(path)?.to_string();
        Ok(self)
    }

    /// Get the full refresh token cookie name (with prefix).
    pub fn refresh_token_name(&self) -> String {
        format!("{}refresh_token", self.cookie_prefix)
    }

    /// Get the validated refresh token cookie name.
    pub fn refresh_token_cookie_name(&self) -> Result<CookieName, AuthCookieError> {
        CookieName::parse(self.refresh_token_name())
    }

    /// Get the full logged_in cookie name (with prefix).
    pub fn logged_in_name(&self) -> String {
        format!("{}logged_in", self.cookie_prefix)
    }

    /// Get the validated logged_in cookie name.
    pub fn logged_in_cookie_name(&self) -> Result<CookieName, AuthCookieError> {
        CookieName::parse(self.logged_in_name())
    }

    /// Get the full CSRF token cookie name (with prefix).
    pub fn csrf_token_name(&self) -> String {
        format!("{}csrf_token", self.cookie_prefix)
    }

    /// Get the validated CSRF token cookie name.
    pub fn csrf_token_cookie_name(&self) -> Result<CookieName, AuthCookieError> {
        CookieName::parse(self.csrf_token_name())
    }

    pub fn validate(&self) -> Result<(), AuthCookieError> {
        validation::validate_config(self)
    }

    /// Cookie domain, if configured.
    pub fn domain(&self) -> Option<&str> {
        self.domain.as_deref()
    }

    /// Whether generated cookies use the Secure flag.
    pub fn secure(&self) -> bool {
        self.secure
    }

    /// Refresh token lifetime in seconds.
    pub fn refresh_token_max_age(&self) -> u64 {
        self.refresh_token_max_age
    }

    /// SameSite policy for generated cookies.
    pub fn same_site(&self) -> SameSite {
        self.same_site
    }

    /// Prefix used for generated auth cookie names.
    pub fn cookie_prefix(&self) -> &str {
        &self.cookie_prefix
    }

    /// Path used for the refresh token cookie.
    pub fn refresh_token_path(&self) -> &str {
        &self.refresh_token_path
    }
}
