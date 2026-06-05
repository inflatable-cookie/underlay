use thiserror::Error;

use super::{AuthCookieConfig, SameSite};

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum AuthCookieError {
    #[error("cookie prefix contains invalid characters")]
    InvalidCookiePrefix,
    #[error("cookie domain is invalid")]
    InvalidDomain,
    #[error("cookie path is invalid")]
    InvalidPath,
    #[error("cookie value contains invalid characters")]
    InvalidValue,
    #[error("SameSite=None requires Secure")]
    SameSiteNoneRequiresSecure,
    #[error("invalid Set-Cookie header value")]
    InvalidHeaderValue,
}

impl From<http::header::InvalidHeaderValue> for AuthCookieError {
    fn from(_: http::header::InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue
    }
}

pub(super) fn validate_config(config: &AuthCookieConfig) -> Result<(), AuthCookieError> {
    if config.same_site == SameSite::None && !config.secure {
        return Err(AuthCookieError::SameSiteNoneRequiresSecure);
    }
    validate_cookie_name_prefix(&config.cookie_prefix)?;
    validate_cookie_path(&config.refresh_token_path)?;
    if let Some(domain) = &config.domain {
        validate_cookie_domain(domain)?;
    }
    Ok(())
}

pub(super) fn validate_cookie_name(value: &str) -> Result<(), AuthCookieError> {
    if !value.is_empty()
        && value
            .bytes()
            .all(|b| matches!(b, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-'))
    {
        Ok(())
    } else {
        Err(AuthCookieError::InvalidCookiePrefix)
    }
}

pub(super) fn validate_cookie_value(value: &str) -> Result<(), AuthCookieError> {
    if !value.is_empty()
        && value
            .bytes()
            .all(|b| !matches!(b, 0x00..=0x20 | 0x7f | b';' | b'"'))
    {
        Ok(())
    } else {
        Err(AuthCookieError::InvalidValue)
    }
}

pub(super) fn validate_cookie_name_prefix(prefix: &str) -> Result<(), AuthCookieError> {
    if prefix
        .bytes()
        .all(|b| matches!(b, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-'))
    {
        Ok(())
    } else {
        Err(AuthCookieError::InvalidCookiePrefix)
    }
}

pub(super) fn validate_cookie_path(path: &str) -> Result<(), AuthCookieError> {
    if path.starts_with('/')
        && !path.is_empty()
        && path
            .bytes()
            .all(|b| !matches!(b, 0x00..=0x20 | 0x7f | b';' | b'"'))
    {
        Ok(())
    } else {
        Err(AuthCookieError::InvalidPath)
    }
}

pub(super) fn validate_cookie_domain(domain: &str) -> Result<(), AuthCookieError> {
    let trimmed = domain.strip_prefix('.').unwrap_or(domain);
    if trimmed.is_empty()
        || trimmed.contains("..")
        || !trimmed
            .bytes()
            .all(|b| matches!(b, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'-'))
        || !trimmed
            .split('.')
            .all(|label| !label.is_empty() && !label.starts_with('-') && !label.ends_with('-'))
    {
        return Err(AuthCookieError::InvalidDomain);
    }

    Ok(())
}
