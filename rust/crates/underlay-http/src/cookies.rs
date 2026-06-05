//! Cookie utilities for authentication.
//!
//! Provides helpers for setting and clearing auth-related cookies in a
//! consistent, secure manner across applications.

mod builders;
mod config;
mod extractors;
mod headers;
mod policy;
mod typed_values;
mod validation;

#[cfg(test)]
use axum::http::{header, HeaderMap, HeaderValue};

pub use builders::{
    clear_csrf_token_cookie, clear_logged_in_cookie, clear_refresh_token_cookie, csrf_token_cookie,
    logged_in_cookie, refresh_token_cookie,
};
pub use config::AuthCookieConfig;
pub use extractors::{extract_csrf_token, extract_refresh_token, extract_refresh_token_default};
pub use headers::{clear_auth_cookies, clear_csrf_cookie, set_auth_cookies, set_csrf_cookie};
pub use policy::SameSite;
pub use typed_values::{CookieDomain, CookieName, CookiePath};
pub use validation::AuthCookieError;

#[cfg(test)]
#[path = "tests/cookies_tests.rs"]
mod tests;
