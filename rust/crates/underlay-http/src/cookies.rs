//! Cookie utilities for authentication.
//!
//! Provides helpers for setting and clearing auth-related cookies in a
//! consistent, secure manner across applications.

use axum::http::{header, HeaderMap, HeaderValue};

/// SameSite cookie policy.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SameSite {
    /// Cookie is sent with same-site and cross-site top-level navigations.
    #[default]
    Lax,
    /// Cookie is only sent with same-site requests.
    Strict,
    /// Cookie is sent with all requests (requires Secure flag).
    None,
}

impl SameSite {
    fn as_str(&self) -> &'static str {
        match self {
            SameSite::Lax => "Lax",
            SameSite::Strict => "Strict",
            SameSite::None => "None",
        }
    }
}

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
///     .with_domain(".example.com")
///     .with_refresh_token_max_age(14 * 24 * 60 * 60) // 14 days
///     .with_cookie_prefix("myapp_");
/// ```
#[derive(Debug, Clone)]
pub struct AuthCookieConfig {
    /// Domain for cookies (e.g., ".example.com" for cross-subdomain).
    /// If None, cookies are scoped to the current host only.
    pub domain: Option<String>,
    /// Whether to set Secure flag (should be true in production).
    pub secure: bool,
    /// Refresh token lifetime in seconds (default: 7 days).
    pub refresh_token_max_age: u64,
    /// SameSite policy for cookies (default: Lax).
    pub same_site: SameSite,
    /// Prefix for cookie names (default: empty).
    /// E.g., "acme_" produces cookies named "acme_refresh_token" and "acme_logged_in".
    pub cookie_prefix: String,
    /// Path for the refresh token cookie (default: "/v1/auth").
    pub refresh_token_path: String,
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

    /// Set the cookie domain.
    pub fn with_domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
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

    /// Set a prefix for cookie names.
    ///
    /// E.g., `with_cookie_prefix("acme_")` produces cookies named
    /// "acme_refresh_token" and "acme_logged_in".
    pub fn with_cookie_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.cookie_prefix = prefix.into();
        self
    }

    /// Set the path for the refresh token cookie.
    ///
    /// Default is "/v1/auth". Set to "/" to send refresh token with all requests.
    pub fn with_refresh_token_path(mut self, path: impl Into<String>) -> Self {
        self.refresh_token_path = path.into();
        self
    }

    /// Get the full refresh token cookie name (with prefix).
    pub fn refresh_token_name(&self) -> String {
        format!("{}refresh_token", self.cookie_prefix)
    }

    /// Get the full logged_in cookie name (with prefix).
    pub fn logged_in_name(&self) -> String {
        format!("{}logged_in", self.cookie_prefix)
    }
}

/// Build a Set-Cookie header value for the refresh token.
///
/// The refresh token cookie is:
/// - httpOnly (not accessible to JavaScript)
/// - Secure (only sent over HTTPS, unless config.secure is false)
/// - SameSite configurable (default: Lax)
/// - Path configurable (default: /v1/auth)
pub fn refresh_token_cookie(token: &str, config: &AuthCookieConfig) -> String {
    let mut cookie = format!(
        "{}={}; HttpOnly; SameSite={}; Path={}; Max-Age={}",
        config.refresh_token_name(),
        token,
        config.same_site.as_str(),
        config.refresh_token_path,
        config.refresh_token_max_age
    );

    if config.secure {
        cookie.push_str("; Secure");
    }

    if let Some(domain) = &config.domain {
        cookie.push_str(&format!("; Domain={}", domain));
    }

    cookie
}

/// Build a Set-Cookie header value to clear the refresh token.
pub fn clear_refresh_token_cookie(config: &AuthCookieConfig) -> String {
    let mut cookie = format!(
        "{}=; HttpOnly; SameSite={}; Path={}; Max-Age=0",
        config.refresh_token_name(),
        config.same_site.as_str(),
        config.refresh_token_path
    );

    if config.secure {
        cookie.push_str("; Secure");
    }

    if let Some(domain) = &config.domain {
        cookie.push_str(&format!("; Domain={}", domain));
    }

    cookie
}

/// Build a Set-Cookie header value for the logged_in indicator.
///
/// The logged_in cookie is:
/// - NOT httpOnly (must be readable by JavaScript for CSS switching)
/// - Secure (only sent over HTTPS, unless config.secure is false)
/// - SameSite configurable (default: Lax)
/// - Path=/ (available to all pages)
pub fn logged_in_cookie(config: &AuthCookieConfig) -> String {
    let mut cookie = format!(
        "{}=1; SameSite={}; Path=/; Max-Age={}",
        config.logged_in_name(),
        config.same_site.as_str(),
        config.refresh_token_max_age
    );

    if config.secure {
        cookie.push_str("; Secure");
    }

    if let Some(domain) = &config.domain {
        cookie.push_str(&format!("; Domain={}", domain));
    }

    cookie
}

/// Build a Set-Cookie header value to clear the logged_in indicator.
pub fn clear_logged_in_cookie(config: &AuthCookieConfig) -> String {
    let mut cookie = format!(
        "{}=; SameSite={}; Path=/; Max-Age=0",
        config.logged_in_name(),
        config.same_site.as_str()
    );

    if config.secure {
        cookie.push_str("; Secure");
    }

    if let Some(domain) = &config.domain {
        cookie.push_str(&format!("; Domain={}", domain));
    }

    cookie
}

/// Extract the refresh token from the Cookie header using the config's cookie name.
///
/// Returns None if the cookie is not present or malformed.
pub fn extract_refresh_token(headers: &HeaderMap, config: &AuthCookieConfig) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;
    let cookie_name = config.refresh_token_name();
    let prefix = format!("{}=", cookie_name);

    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some(value) = cookie.strip_prefix(&prefix) {
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }

    None
}

/// Extract the refresh token from the Cookie header using default cookie name.
///
/// This is a convenience function that uses the default "refresh_token" name.
/// For apps using custom cookie prefixes, use [`extract_refresh_token`] instead.
pub fn extract_refresh_token_default(headers: &HeaderMap) -> Option<String> {
    extract_refresh_token(headers, &AuthCookieConfig::default())
}

/// Add auth cookies to a response's headers.
///
/// Sets both the refresh_token (httpOnly) and logged_in cookies.
pub fn set_auth_cookies(
    headers: &mut HeaderMap,
    refresh_token: &str,
    config: &AuthCookieConfig,
) -> Result<(), http::header::InvalidHeaderValue> {
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&refresh_token_cookie(refresh_token, config))?,
    );
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&logged_in_cookie(config))?,
    );
    Ok(())
}

/// Add cookie-clearing headers to a response.
///
/// Clears both the refresh_token and logged_in cookies.
pub fn clear_auth_cookies(
    headers: &mut HeaderMap,
    config: &AuthCookieConfig,
) -> Result<(), http::header::InvalidHeaderValue> {
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&clear_refresh_token_cookie(config))?,
    );
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&clear_logged_in_cookie(config))?,
    );
    Ok(())
}

#[cfg(test)]
#[path = "tests/cookies_tests.rs"]
mod tests;
