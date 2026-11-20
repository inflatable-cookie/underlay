use super::AuthCookieConfig;
use axum::http::{header, HeaderMap};

fn extract_cookie_value(headers: &HeaderMap, cookie_name: &str) -> Option<String> {
    let prefix = format!("{cookie_name}=");

    for header_value in headers.get_all(header::COOKIE) {
        let Ok(cookie_header) = header_value.to_str() else {
            continue;
        };

        for cookie in cookie_header.split(';') {
            let cookie = cookie.trim();
            if let Some(value) = cookie.strip_prefix(&prefix) {
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }

    None
}

/// Extract the refresh token from the Cookie header using the config's cookie name.
///
/// Returns None if the cookie is not present or malformed.
pub fn extract_refresh_token(headers: &HeaderMap, config: &AuthCookieConfig) -> Option<String> {
    extract_cookie_value(headers, &config.refresh_token_name())
}

/// Extract the CSRF token from the Cookie header using the config's cookie name.
///
/// Returns None if the cookie is not present or malformed.
pub fn extract_csrf_token(headers: &HeaderMap, config: &AuthCookieConfig) -> Option<String> {
    extract_cookie_value(headers, &config.csrf_token_name())
}

/// Extract the refresh token from the Cookie header using default cookie name.
///
/// This is a convenience function that uses the default "refresh_token" name.
/// For apps using custom cookie prefixes, use [`extract_refresh_token`] instead.
pub fn extract_refresh_token_default(headers: &HeaderMap) -> Option<String> {
    extract_refresh_token(headers, &AuthCookieConfig::default())
}
