use super::{validation::validate_cookie_value, AuthCookieConfig, AuthCookieError};

/// Build a Set-Cookie header value for the refresh token.
///
/// The refresh token cookie is:
/// - httpOnly (not accessible to JavaScript)
/// - Secure (only sent over HTTPS, unless config.secure is false)
/// - SameSite configurable (default: Lax)
/// - Path configurable (default: /v1/auth)
pub fn refresh_token_cookie(
    token: &str,
    config: &AuthCookieConfig,
) -> Result<String, AuthCookieError> {
    config.validate()?;
    validate_cookie_value(token)?;
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

    Ok(cookie)
}

/// Build a Set-Cookie header value to clear the refresh token.
pub fn clear_refresh_token_cookie(config: &AuthCookieConfig) -> Result<String, AuthCookieError> {
    config.validate()?;
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

    Ok(cookie)
}

/// Build a Set-Cookie header value for the logged_in indicator.
///
/// The logged_in cookie is:
/// - NOT httpOnly (must be readable by JavaScript for CSS switching)
/// - Secure (only sent over HTTPS, unless config.secure is false)
/// - SameSite configurable (default: Lax)
/// - Path=/ (available to all pages)
pub fn logged_in_cookie(config: &AuthCookieConfig) -> Result<String, AuthCookieError> {
    config.validate()?;
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

    Ok(cookie)
}

/// Build a Set-Cookie header value to clear the logged_in indicator.
pub fn clear_logged_in_cookie(config: &AuthCookieConfig) -> Result<String, AuthCookieError> {
    config.validate()?;
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

    Ok(cookie)
}

/// Build a Set-Cookie header value for the CSRF token.
///
/// The CSRF token cookie is readable by JavaScript so browser clients can send
/// it back in a request header. It uses the same domain, Secure, SameSite, and
/// lifetime settings as the auth cookies.
pub fn csrf_token_cookie(
    token: &str,
    config: &AuthCookieConfig,
) -> Result<String, AuthCookieError> {
    config.validate()?;
    validate_cookie_value(token)?;
    let mut cookie = format!(
        "{}={}; SameSite={}; Path=/; Max-Age={}",
        config.csrf_token_name(),
        token,
        config.same_site.as_str(),
        config.refresh_token_max_age
    );

    if config.secure {
        cookie.push_str("; Secure");
    }

    if let Some(domain) = &config.domain {
        cookie.push_str(&format!("; Domain={}", domain));
    }

    Ok(cookie)
}

/// Build a Set-Cookie header value to clear the CSRF token.
pub fn clear_csrf_token_cookie(config: &AuthCookieConfig) -> Result<String, AuthCookieError> {
    config.validate()?;
    let mut cookie = format!(
        "{}=; SameSite={}; Path=/; Max-Age=0",
        config.csrf_token_name(),
        config.same_site.as_str()
    );

    if config.secure {
        cookie.push_str("; Secure");
    }

    if let Some(domain) = &config.domain {
        cookie.push_str(&format!("; Domain={}", domain));
    }

    Ok(cookie)
}
