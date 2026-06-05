use super::{
    clear_csrf_token_cookie, clear_logged_in_cookie, clear_refresh_token_cookie, csrf_token_cookie,
    logged_in_cookie, refresh_token_cookie, AuthCookieConfig, AuthCookieError,
};
use axum::http::{header, HeaderMap, HeaderValue};

/// Add auth cookies to a response's headers.
///
/// Sets both the refresh_token (httpOnly) and logged_in cookies.
pub fn set_auth_cookies(
    headers: &mut HeaderMap,
    refresh_token: &str,
    config: &AuthCookieConfig,
) -> Result<(), AuthCookieError> {
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&refresh_token_cookie(refresh_token, config)?)?,
    );
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&logged_in_cookie(config)?)?,
    );
    Ok(())
}

/// Add a CSRF token cookie to a response's headers.
pub fn set_csrf_cookie(
    headers: &mut HeaderMap,
    csrf_token: &str,
    config: &AuthCookieConfig,
) -> Result<(), AuthCookieError> {
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&csrf_token_cookie(csrf_token, config)?)?,
    );
    Ok(())
}

/// Add cookie-clearing headers to a response.
///
/// Clears both the refresh_token and logged_in cookies.
pub fn clear_auth_cookies(
    headers: &mut HeaderMap,
    config: &AuthCookieConfig,
) -> Result<(), AuthCookieError> {
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&clear_refresh_token_cookie(config)?)?,
    );
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&clear_logged_in_cookie(config)?)?,
    );
    Ok(())
}

/// Add a CSRF token clearing header to a response.
pub fn clear_csrf_cookie(
    headers: &mut HeaderMap,
    config: &AuthCookieConfig,
) -> Result<(), AuthCookieError> {
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&clear_csrf_token_cookie(config)?)?,
    );
    Ok(())
}
