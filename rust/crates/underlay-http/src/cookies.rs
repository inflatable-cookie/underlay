//! Cookie utilities for authentication.
//!
//! Provides helpers for setting and clearing auth-related cookies in a
//! consistent, secure manner across applications.

use axum::http::{header, HeaderMap, HeaderValue};

/// Configuration for auth cookies.
#[derive(Debug, Clone)]
pub struct AuthCookieConfig {
    /// Domain for cookies (e.g., ".example.com" for cross-subdomain).
    /// If None, cookies are scoped to the current host only.
    pub domain: Option<String>,
    /// Whether to set Secure flag (should be true in production).
    pub secure: bool,
    /// Refresh token lifetime in seconds (default: 7 days).
    pub refresh_token_max_age: u64,
}

impl Default for AuthCookieConfig {
    fn default() -> Self {
        Self {
            domain: None,
            secure: true,
            refresh_token_max_age: 7 * 24 * 60 * 60, // 7 days
        }
    }
}

impl AuthCookieConfig {
    /// Create config for local development (insecure).
    pub fn local_dev() -> Self {
        Self {
            domain: None,
            secure: false,
            refresh_token_max_age: 7 * 24 * 60 * 60,
        }
    }
}

/// Build a Set-Cookie header value for the refresh token.
///
/// The refresh token cookie is:
/// - httpOnly (not accessible to JavaScript)
/// - Secure (only sent over HTTPS, unless config.secure is false)
/// - SameSite=Lax (sent with top-level navigations and GET from other sites)
/// - Path=/v1/auth (only sent to auth endpoints)
pub fn refresh_token_cookie(token: &str, config: &AuthCookieConfig) -> String {
    let mut cookie = format!(
        "refresh_token={}; HttpOnly; SameSite=Lax; Path=/v1/auth; Max-Age={}",
        token, config.refresh_token_max_age
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
    let mut cookie =
        "refresh_token=; HttpOnly; SameSite=Lax; Path=/v1/auth; Max-Age=0".to_string();

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
/// - SameSite=Lax
/// - Path=/ (available to all pages)
pub fn logged_in_cookie(config: &AuthCookieConfig) -> String {
    let mut cookie = format!(
        "logged_in=1; SameSite=Lax; Path=/; Max-Age={}",
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
    let mut cookie = "logged_in=; SameSite=Lax; Path=/; Max-Age=0".to_string();

    if config.secure {
        cookie.push_str("; Secure");
    }

    if let Some(domain) = &config.domain {
        cookie.push_str(&format!("; Domain={}", domain));
    }

    cookie
}

/// Extract the refresh token from the Cookie header.
///
/// Returns None if the cookie is not present or malformed.
pub fn extract_refresh_token(headers: &HeaderMap) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;

    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some(value) = cookie.strip_prefix("refresh_token=") {
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }

    None
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
mod tests {
    use super::*;

    #[test]
    fn refresh_token_cookie_secure() {
        let config = AuthCookieConfig::default();
        let cookie = refresh_token_cookie("test-token", &config);

        assert!(cookie.contains("refresh_token=test-token"));
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("Secure"));
        assert!(cookie.contains("SameSite=Lax"));
        assert!(cookie.contains("Path=/v1/auth"));
        assert!(cookie.contains("Max-Age="));
    }

    #[test]
    fn refresh_token_cookie_local_dev() {
        let config = AuthCookieConfig::local_dev();
        let cookie = refresh_token_cookie("test-token", &config);

        assert!(cookie.contains("refresh_token=test-token"));
        assert!(cookie.contains("HttpOnly"));
        assert!(!cookie.contains("Secure"));
    }

    #[test]
    fn logged_in_cookie_not_httponly() {
        let config = AuthCookieConfig::default();
        let cookie = logged_in_cookie(&config);

        assert!(cookie.contains("logged_in=1"));
        assert!(!cookie.contains("HttpOnly"));
        assert!(cookie.contains("Path=/"));
    }

    #[test]
    fn extract_refresh_token_present() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("other=value; refresh_token=my-token; another=thing"),
        );

        let token = extract_refresh_token(&headers);
        assert_eq!(token, Some("my-token".to_string()));
    }

    #[test]
    fn extract_refresh_token_missing() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("other=value; another=thing"),
        );

        let token = extract_refresh_token(&headers);
        assert_eq!(token, None);
    }

    #[test]
    fn extract_refresh_token_no_cookie_header() {
        let headers = HeaderMap::new();
        let token = extract_refresh_token(&headers);
        assert_eq!(token, None);
    }
}
