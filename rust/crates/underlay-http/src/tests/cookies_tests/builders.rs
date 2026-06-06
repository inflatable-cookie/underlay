use super::*;

#[test]
fn refresh_token_cookie_secure() {
    let config = AuthCookieConfig::default();
    let cookie = refresh_token_cookie("test-token", &config).unwrap();

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
    let cookie = refresh_token_cookie("test-token", &config).unwrap();

    assert!(cookie.contains("refresh_token=test-token"));
    assert!(cookie.contains("HttpOnly"));
    assert!(!cookie.contains("Secure"));
}

#[test]
fn logged_in_cookie_not_httponly() {
    let config = AuthCookieConfig::default();
    let cookie = logged_in_cookie(&config).unwrap();

    assert!(cookie.contains("logged_in=1"));
    assert!(!cookie.contains("HttpOnly"));
    assert!(cookie.contains("Path=/"));
}

#[test]
fn csrf_token_cookie_not_httponly() {
    let config = AuthCookieConfig::default();
    let cookie = csrf_token_cookie("csrf-token", &config).unwrap();

    assert!(cookie.contains("csrf_token=csrf-token"));
    assert!(!cookie.contains("HttpOnly"));
    assert!(cookie.contains("Secure"));
    assert!(cookie.contains("SameSite=Lax"));
    assert!(cookie.contains("Path=/"));
    assert!(cookie.contains("Max-Age="));
}

#[test]
fn same_site_strict() {
    let config = AuthCookieConfig::default().with_same_site(SameSite::Strict);
    let cookie = refresh_token_cookie("test-token", &config).unwrap();

    assert!(cookie.contains("SameSite=Strict"));
}

#[test]
fn custom_refresh_token_path() {
    let config = AuthCookieConfig::default().with_refresh_token_path("/api/auth");
    let cookie = refresh_token_cookie("test-token", &config).unwrap();

    assert!(cookie.contains("Path=/api/auth"));
}

#[test]
fn same_site_none_requires_secure() {
    let config = AuthCookieConfig::local_dev().with_same_site(SameSite::None);

    assert_eq!(
        refresh_token_cookie("test-token", &config),
        Err(AuthCookieError::SameSiteNoneRequiresSecure)
    );
}

#[test]
fn rejects_invalid_cookie_value() {
    let config = AuthCookieConfig::default();

    assert_eq!(
        refresh_token_cookie("bad;token", &config),
        Err(AuthCookieError::InvalidValue)
    );
}

#[test]
fn rejects_empty_cookie_value() {
    let config = AuthCookieConfig::default();

    assert_eq!(
        refresh_token_cookie("", &config),
        Err(AuthCookieError::InvalidValue)
    );
}

#[test]
fn csrf_cookie_reuses_value_validation() {
    let config = AuthCookieConfig::default();

    assert_eq!(
        csrf_token_cookie("bad;token", &config),
        Err(AuthCookieError::InvalidValue)
    );
}
